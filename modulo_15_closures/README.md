# Módulo 15 — Closures

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
