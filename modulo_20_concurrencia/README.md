# Módulo 20 — Concurrencia

## Objetivos

- Lanzar hilos y compartir datos de forma segura.
- Comunicar hilos con canales o memoria compartida.
- Entender `Send` y `Sync` como base de la "concurrencia sin miedo".

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m20_concurrencia --example hilos` | `spawn`, `join`, `move`, `thread::scope`, pánicos en hilos |
| `cargo run -p m20_concurrencia --example canales_mpsc` | `mpsc::channel`, varios productores, patrón *worker* |
| `cargo run -p m20_concurrencia --example estado_compartido` | `Arc`, `Mutex`, `RwLock`, `Condvar`, `OnceLock` |
| `cargo run -p m20_concurrencia --example send_sync` | `Send`, `Sync` y por qué `Rc` no cruza hilos |

## Conceptos para recordar

- `thread::spawn` exige `'static`: el closure debe ser dueño de sus datos (`move`). Con `thread::scope` puedes prestar datos locales.
- **Canales** (`mpsc`): pasas la propiedad del mensaje; no hay acceso simultáneo.
- **Memoria compartida**: `Arc<Mutex<T>>` (un acceso a la vez) o `Arc<RwLock<T>>` (muchos lectores / un escritor).
- Un `Mutex` envenenado ocurre si un hilo entra en pánico con el lock; se puede recuperar con `into_inner`.
- `Send` = se puede mover a otro hilo. `Sync` = se puede compartir por referencia entre hilos.
- Las data races son **errores de compilación**; los deadlocks y las condiciones de carrera lógicas no lo son.
- Evita deadlocks: orden fijo al tomar varios locks, secciones críticas cortas, no hagas E/S con el lock tomado.

## Ejercicios

1. Calcula la suma de un `Vec` de 10 millones de elementos dividiéndolo entre N hilos con `thread::scope`.
2. Implementa un *pool* de hilos: N workers que leen trabajos de un `mpsc` compartido (`Arc<Mutex<Receiver<_>>>`).
3. Simula una cuenta bancaria compartida con `Arc<Mutex<Cuenta>>` y 100 hilos que depositan.
4. Provoca un deadlock a propósito con dos mutex tomados en orden inverso y arréglalo.
5. Usa un `RwLock<HashMap>` como caché con muchos lectores y un escritor ocasional.
6. Comprueba el error al enviar un `Rc` a un hilo y reemplázalo por `Arc`.
