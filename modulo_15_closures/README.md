# Módulo 15 — Closures

## La idea en 2 minutos

Un **closure** es una función sin nombre que puede "recordar" variables de donde fue creada, como una nota adhesiva que lleva escrito lo que necesita.

```rust
let iva = 0.21;
let con_iva = |precio: f64| precio * (1.0 + iva);   // captura `iva`
println!("{}", con_iva(100.0));
```

La pregunta clave es **cómo captura** lo que usa, y eso determina qué trait cumple:

| Si el closure... | Trait | Se puede llamar... |
|---|---|---|
| solo lee | `Fn` | muchas veces |
| modifica lo capturado | `FnMut` | muchas veces (necesita `mut`) |
| se lleva (consume) algo | `FnOnce` | una sola vez |

**Orden recomendado:** `captura` → `parametros_retornos`.

## Objetivos

- Escribir funciones anónimas que capturan su entorno.
- Distinguir `Fn`, `FnMut` y `FnOnce`.
- Pasar y devolver closures.

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m15_closures --example captura` | captura por `&`, `&mut` y valor; `move` |
| `cargo run -p m15_closures --example parametros_retornos` | closures como argumentos, retornos (`impl Fn`, `Box<dyn Fn>`), en structs |

## Conceptos para recordar

- El compilador elige el modo de captura **más débil** que le baste: `&`, luego `&mut`, luego valor.
- `Fn`: solo lee. `FnMut`: modifica estado. `FnOnce`: consume capturas (solo se llama una vez).
- Toda `Fn` es `FnMut` y toda `FnMut` es `FnOnce`.
- `move` fuerza capturar por valor; es obligatorio cuando el closure sobrevive a su ámbito (hilos, retornos).
- `impl Fn(..)` cuando devuelves siempre el mismo closure; `Box<dyn Fn(..)>` cuando pueden ser varios.
- Los nombres de funciones normales también cumplen `Fn`.

## Ejercicios

1. Escribe `fn componer<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C`.
2. Crea un generador de IDs: `fn generador() -> impl FnMut() -> u32` que devuelva 1, 2, 3…
3. Implementa `fn aplicar_n(f: impl Fn(i32) -> i32, n: usize, x: i32) -> i32`.
4. Guarda en un `HashMap<String, Box<dyn Fn(f64) -> f64>>` varias funciones matemáticas.
5. Provoca el error "closure may outlive the current function" y soluciónalo con `move`.
6. Escribe un `struct Boton` con un `on_click: Box<dyn FnMut()>` y llámalo varias veces.
