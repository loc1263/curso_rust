# Proyecto integrador — Nivel 3: `kvdb`

Una mini base de datos **clave-valor async** (estilo Redis) que reúne lo aprendido en el Nivel 3:
**async/tokio** (22), **Futures y `select!`** (23), **red/TCP** (24), **concurrencia con estado compartido** (20, 30),
**errores tipados** (18), **tests de extremo a extremo** (19) y **prácticas de producción** (32): límites de recursos,
apagado ordenado, persistencia y observabilidad básica.

## Uso

```bash
# Terminal 1: servidor (con persistencia en disco)
cargo run -p kvdb --bin kv-server -- --addr 127.0.0.1:6380 --aof datos.aof

# Terminal 2: cliente interactivo
cargo run -p kvdb --bin kv-cli -- 127.0.0.1:6380
> SET saludo "hola mundo"
OK
> GET saludo
"hola mundo"
> INCR visitas
(entero) 1
> SET sesion abc EX 30
OK
> TTL sesion
(entero) 30
> KEYS
(3 elementos)
  saludo
  sesion
  visitas
```

También funciona con cualquier cliente TCP de texto (`telnet`, `nc`, `ncat`) o por tubería:
`echo PING | cargo run -q -p kvdb --bin kv-cli`.

Detén el servidor con **Ctrl+C**: deja terminar las conexiones abiertas y vuelca el AOF a disco antes de salir.

## Comandos

| Comando | Respuesta | Descripción |
|---------|-----------|-------------|
| `PING` | `+PONG` | comprobación de vida |
| `SET clave valor [EX seg \| PX ms]` | `+OK` | guarda (con caducidad opcional) |
| `GET clave` | `$valor` ó `_` | lee; `_` = no existe |
| `DEL clave` | `:1` / `:0` | borra |
| `INCR clave` | `:n` | incrementa un entero (lo crea en 1 si no existe) |
| `TTL clave` | `:n` | segundos restantes; `-1` sin caducidad, `-2` no existe |
| `KEYS` | `*n` + `$clave`… | claves vivas, ordenadas |
| `INFO` | `*n` + `$k:v`… | versión, claves, conexiones, comandos, persistencia |
| `QUIT` | `+BYE` | cierra la conexión |

Valores con espacios entre comillas dobles (`SET k "a b"`, con `\"` y `\\`). Errores: `-ERR mensaje` (la conexión sigue abierta).
Configuración: `--addr`/`KV_ADDR` (por defecto `127.0.0.1:6380`) y `--aof`/`KV_AOF`.

## Arquitectura

```
src/protocolo.rs   Comando, Respuesta, tokenizador con comillas, parseo           (puro, con tests)
src/almacen.rs     Almacen (Arc<Mutex<HashMap>>), TTL, AOF (escritor en una tarea) (con tests)
src/servidor.rs    accept loop, tarea por conexión, límites, timeouts, apagado     (tokio)
src/lib.rs         ejecutar(): abre el AOF, sirve y espera al volcado final
src/main.rs        kv-server: argumentos/entorno, Ctrl+C
src/bin/kv-cli.rs  cliente interactivo
tests/integracion.rs  servidor real + clientes TCP reales
```

```
 cliente ──TCP──▶ [manejar: read_line ▸ parsear ▸ despachar ▸ write] ──▶ Almacen ──mpsc──▶ [escritor AOF] ──▶ disco
                      ▲ una tarea tokio por conexión              (Mutex, sección corta)        (tarea aparte)
                      └── watch<bool> "apagar" ──▶ todas las tareas + purga periódica de TTL
```

### Decisiones de diseño (y por qué)

| Decisión | Motivo |
|----------|--------|
| `std::sync::Mutex` y no `tokio::sync::Mutex` | las secciones críticas son cortas y **nunca cruzan un `.await`** (módulo 22) |
| AOF escrito por una tarea con `mpsc` | los clientes no esperan al disco; el envío se hace con el lock tomado, así el orden del log = el de la memoria |
| Se registra también cada caducidad como `D` | la reproducción del log es determinista aunque pase el tiempo |
| `Semaphore` de 128 conexiones **antes** de `accept` | evita agotar recursos; el exceso espera en la cola del SO |
| `take(MAX_LINEA)` sobre el lector | un cliente no puede hacernos reservar memoria sin límite |
| Timeout de inactividad (5 min) | libera conexiones muertas |
| `watch` + `JoinSet` para el apagado | deja de aceptar, avisa a las conexiones y espera con tope de 5 s |
| `kvdb::ejecutar` espera a la tarea del AOF | al soltar el último `Almacen` el canal se cierra y el escritor hace `flush` + `fsync` |
| Lógica en una **biblioteca** y `main` finos | permite tests de integración con un servidor en un puerto efímero (`:0`) |

### Garantías y límites (honestidad sobre lo que NO hace)

- Tras un **cierre ordenado** no se pierde ninguna escritura. Ante un **corte brusco** (kill -9, apagón) pueden perderse las últimas
  operaciones aún no volcadas (no hay `fsync` por operación; es el equivalente a `appendfsync no`).
- Una línea truncada al final del AOF se ignora con un aviso al reiniciar.
- El AOF crece indefinidamente (no hay compactación).
- Sin autenticación ni TLS: **no lo expongas a Internet**; escucha solo en `127.0.0.1`.
- Un único `Mutex` global: suficiente para miles de operaciones por segundo; después conviene fragmentarlo (*sharding*).

## Cómo probarlo

```bash
cargo test -p kvdb                      # 10 tests unitarios + 10 de extremo a extremo
cargo clippy -p kvdb --all-targets
cargo build -p kvdb --release
```

Los tests de integración cubren: comandos básicos, comillas, errores sin cerrar la conexión, `INCR`/`KEYS`/`INFO`,
TTL, **20 clientes concurrentes × 50 `INCR` = 1000 exactos**, `QUIT`, líneas demasiado largas, apagado con clientes
conectados y **persistencia a través de un reinicio** (incluyendo caducidades y TTL conservado).

## Mejoras propuestas

1. **Compactación del AOF** (reescribir el log con el estado actual) cuando supere cierto tamaño.
2. **Sharding** del `HashMap` en N cubos con su propio lock para escalar con los núcleos.
3. Comandos `EXPIRE`, `MGET/MSET`, `LPUSH/LRANGE` (listas) y `SUBSCRIBE/PUBLISH` con `tokio::sync::broadcast`.
4. **Autenticación** (`AUTH clave`) y TLS con `tokio-rustls`.
5. **Métricas Prometheus** y logs estructurados con `tracing` (módulos 21 y 32).
6. Benchmark de carga con varios clientes (`criterion` o un cliente propio) y **flamegraph** (módulo 29).
7. Reemplazar el protocolo de texto por **RESP** real para usar `redis-cli`.
8. Dockerfile y CI con las plantillas del módulo 32.
9. Replicación líder-seguidor: el seguidor se suscribe al flujo del AOF.
