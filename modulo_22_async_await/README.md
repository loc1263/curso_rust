# Módulo 22 — Async/await

## Objetivos

- Entender qué es un `Future` y cómo `async`/`await` los compone.
- Usar `tokio` para tareas concurrentes, temporizadores y canales.
- Evitar los errores clásicos: bloquear el runtime y mantener locks a través de `.await`.

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m22_async_await --example basico` | `async fn`, `join!`, `spawn`, `spawn_blocking`, panics en tareas |
| `cargo run -p m22_async_await --example select_timeout` | `select!`, `timeout`, `interval`, cancelación |
| `cargo run -p m22_async_await --example canales_async` | `mpsc`, `oneshot`, `broadcast`, `watch`, `Mutex`, `Semaphore` |

## Conceptos para recordar

- `async fn` devuelve un `Future` **perezoso**: no hace nada hasta que lo `.await`es (o lo pasas a `spawn`).
- Rust no incluye ejecutor: necesitas un runtime (`tokio`, `async-std`, `smol`…).
- `join!` ejecuta varios futures concurrentemente **en la misma tarea**; `spawn` crea tareas independientes (requieren `Send + 'static`).
- Concurrencia ≠ paralelismo: un runtime *multi-thread* reparte las tareas entre hilos; `current_thread` las ejecuta en uno solo.
- **Nunca bloquees** dentro de código async (`std::thread::sleep`, E/S síncrona, cálculos largos): usa `tokio::time::sleep`, E/S de tokio o `spawn_blocking`.
- **Cancelación**: descartar un future lo cancela en su siguiente punto `.await`. `select!` cancela las ramas perdedoras.
- No mantengas un `std::sync::MutexGuard` a través de un `.await` (el future dejaría de ser `Send`). Usa `tokio::sync::Mutex` o acorta el ámbito del guard.
- Elección de canal: `mpsc` (cola), `oneshot` (una respuesta), `broadcast` (todos reciben todo), `watch` (último valor).

## Ejercicios

1. Descarga (simula con `sleep`) 10 "páginas" con un máximo de 3 simultáneas usando `Semaphore`.
2. Escribe `async fn con_reintentos` que reintente una operación fallida 3 veces con espera creciente.
3. Implementa un servidor de eco TCP con `tokio::net::TcpListener` y una tarea por conexión.
4. Crea un pipeline productor → procesador → consumidor con dos `mpsc` encadenados.
5. Añade apagado ordenado con `tokio::signal::ctrl_c()` y un `watch`/`broadcast` de shutdown.
6. Provoca un error `future cannot be sent between threads safely` guardando un `Rc` a través de un `.await` y corrígelo.
