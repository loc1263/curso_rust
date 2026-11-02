# Módulo 23 — Pin y Futures a fondo

## La idea en 2 minutos

En el módulo 22 usaste async como una caja negra. Aquí la abrimos. Un `Future` es solo un objeto con un método:

```rust
fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Output>
//  devuelve Poll::Ready(valor)  -> "ya terminé"
//  o        Poll::Pending       -> "aún no; avísame cuando pueda seguir"
```

El **ejecutor** llama a `poll` una y otra vez; el **Waker** es el timbre con el que un Future dice "ya puedes volver a preguntarme". Un `async fn` es solo una máquina de estados escrita por el compilador con este patrón.

¿Y `Pin`? Una máquina de estados puede guardar referencias a sus propias variables; si se moviera en memoria, esas referencias quedarían apuntando a ningún sitio. `Pin` es la promesa de **"este valor ya no se moverá"**.

**Orden recomendado:** `future_manual` → `ejecutor_mini` → `pin_unpin` (el más abstracto, déjalo para el final).

## Objetivos

- Saber qué hay "bajo el capó" de `async`/`await`.
- Implementar un `Future` a mano y un ejecutor mínimo.
- Entender para qué sirven `Pin` y `Unpin`.

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m23_pin_futures --example future_manual` | trait `Future`, `poll`, `Waker`, `block_on` casero |
| `cargo run -p m23_pin_futures --example ejecutor_mini` | ejecutor con cola de tareas y temporizador async |
| `cargo run -p m23_pin_futures --example pin_unpin` | estructuras autorreferenciadas, `Pin`, `PhantomPinned` |

## El modelo en una imagen

```
            ┌──────────────── ejecutor ────────────────┐
  spawn ──▶ │ cola de tareas listas                    │
            │   │                                      │
            │   ▼  poll(cx)                            │
            │ Future ──▶ Ready(v)   → tarea terminada  │
            │        └─▶ Pending    → se guarda        │
            │                             ▲            │
            │   waker.wake() ─────────────┘            │
            └──────────────────────────────────────────┘
   (un reactor/hilo externo —timer, socket— llama a wake cuando hay progreso)
```

## Conceptos para recordar

- `trait Future { type Output; fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>; }`
- `poll` **nunca debe bloquear**. Devuelve `Pending` si no puede avanzar, después de registrar el `Waker` de `cx`.
- Un `async fn` se compila a una **máquina de estados**: cada `.await` es un punto de suspensión.
- Esa máquina puede guardar referencias a sus propias variables → es autorreferenciada → no debe moverse → `Pin`.
- `Unpin` es un *auto trait*: casi todo lo es. Los futures de `async` son `!Unpin`.
- Para fijar: `Box::pin(fut)` (heap), `std::pin::pin!(fut)` (pila), `Pin::new(&mut x)` (solo si `x: Unpin`).
- El `Waker` es el puente entre el reactor (quien sabe cuándo hay datos) y el ejecutor (quien hace poll).
- Un ejecutor real (tokio) añade: reactor de E/S (epoll/IOCP), temporizadores, reparto entre hilos (*work stealing*).

## Ejercicios

1. Implementa `struct Ceder(bool)` — un future que devuelve `Pending` una vez (y se despierta) y luego `Ready` — como `tokio::task::yield_now`.
2. Escribe `fn unir<A: Future, B: Future>(a: A, b: B) -> impl Future<Output = (A::Output, B::Output)>` (tu propio `join!`).
3. Añade a `ejecutor_mini` un método `lanzar` que devuelva un `JoinHandle` para recoger el resultado.
4. Haz que `ejecutor_mini` use varios hilos que comparten la cola (`Arc<Mutex<Receiver>>`).
5. Implementa un future `Timeout<F>` que falle si `F` no termina antes de `Duration`.
6. Explica con un dibujo por qué mover un `AutoRef` rompe el puntero interno.
