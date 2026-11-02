# Módulo 30 — Concurrencia avanzada

## La idea en 2 minutos

Un `Mutex` es como una puerta con cerradura: entra uno y los demás esperan. Los **atómicos** son operaciones que el procesador garantiza indivisibles, sin cerradura. El reto: cuando varios núcleos tienen cachés propias, ¿en qué orden ven los cambios? Eso es el **modelo de memoria**, y los *orderings* son cuánto orden exiges:

| Ordering | Analogía |
|---|---|
| `Relaxed` | "solo cuenta bien; no me importa el resto" (contadores) |
| `Release` + `Acquire` | "cuando levante la bandera, todo lo que dejé escrito ya es visible" (publicar datos) |
| `SeqCst` | "todos vemos los sucesos en el mismo orden" (lo más simple y lo más caro) |

```rust
dato.store(42, Relaxed);
bandera.store(true, Release);          // publica
while !bandera.load(Acquire) {}        // espera la publicación; ahora `dato` es visible
```

Es el módulo más difícil del curso: ve despacio y usa `Mutex` salvo que midas que no basta.

**Orden recomendado:** `atomics_basicos` → `orderings` → `lock_free`.

## Objetivos

- Usar atómicos en lugar de locks cuando tiene sentido.
- Entender los *memory orderings* (`Relaxed`, `Acquire`/`Release`, `SeqCst`) y el modelo de memoria.
- Construir un spinlock y una cola SPSC sin bloqueos; reconocer el *false sharing*.

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m30_concurrencia_avanzada --example atomics_basicos` | `fetch_add`, `compare_exchange`, bucle CAS, `try_update`, atómico vs `Mutex` |
| `cargo run --release -p m30_concurrencia_avanzada --example orderings` | `Release`/`Acquire`, `Relaxed`, `fence`, `SeqCst` (Dekker) |
| `cargo run --release -p m30_concurrencia_avanzada --example lock_free` | `Spinlock<T>`, cola SPSC de anillo, false sharing |

## Orderings en una tabla

| Ordering | Garantiza | Úsalo para |
|----------|-----------|-----------|
| `Relaxed` | solo atomicidad de esa variable | contadores, estadísticas, IDs |
| `Release` (store) + `Acquire` (load) | lo escrito antes del `Release` es visible tras el `Acquire` que lee ese valor | publicar datos, locks, colas |
| `AcqRel` | ambos, en operaciones RMW (`fetch_*`, `compare_exchange`) | contadores de referencias, estados |
| `SeqCst` | además un único orden total global | cuando necesitas razonar con varias variables a la vez; en la duda |

Idea clave: **happens-before**. Un `store(Release)` que es leído por un `load(Acquire)` crea una relación "ocurre-antes" entre los hilos; sin ella, las escrituras a otros datos pueden verse en cualquier orden (o no verse).

## Conceptos para recordar

- Un atómico no es "un Mutex más rápido": es una herramienta distinta. Útil para contadores, banderas, estados y como base de estructuras sin bloqueos.
- `compare_exchange_weak` puede fallar "espuriamente"; úsalo dentro de un bucle (más eficiente en ARM). `compare_exchange` para un único intento.
- **ABA**: un CAS comprueba solo el *valor*; si cambió A→B→A, no lo detecta. Se resuelve con contadores de versión o con reclamación basada en épocas (crossbeam-epoch).
- **Lock-free ≠ más rápido**: la contención de caché (*false sharing*, rebote de líneas) puede dominar. Mide.
- x86 tiene un modelo de memoria fuerte; los errores de orden pueden no manifestarse allí y romper en ARM (Apple Silicon, móviles, servidores Graviton).
- `Send`/`Sync` siguen aplicando: un tipo con `UnsafeCell` solo es `Sync` si **tú** haces `unsafe impl Sync` y garantizas la sincronización.
- Herramientas: **Miri** (`cargo +nightly miri run`) detecta data races en código unsafe; **loom** explora sistemáticamente los intercalados; **ThreadSanitizer** (`-Zsanitizer=thread`).
- En producción usa `crossbeam` (canales, colas, `CachePadded`, épocas) y `parking_lot` (mutex más rápidos) antes de escribir estructuras propias.

## Ejercicios

1. Implementa un contador de referencias propio (`MiArc<T>`) con `fetch_add`/`fetch_sub` y el `fence(Acquire)` necesario antes de liberar.
2. Añade a `Spinlock` un método `try_lock` y un `Drop` correcto; prueba con Miri.
3. Cambia los orderings de `publicar_con_release_acquire` a `Relaxed` y ejecútalo con Miri para ver la data race detectada.
4. Implementa un *seqlock* para leer una estructura pequeña sin bloquear al escritor.
5. Convierte la cola SPSC en una MPSC añadiendo un lock solo del lado productor y mide el coste.
6. Usa el crate `loom` para comprobar exhaustivamente el ejemplo de `Release`/`Acquire`.
