# Módulo 25 — Unsafe Rust

## Objetivos

- Saber qué permite (y qué NO) un bloque `unsafe`.
- Escribir abstracciones seguras sobre código `unsafe` documentando invariantes.
- Conocer las herramientas de auditoría: Miri, `#![forbid(unsafe_code)]`, `cargo geiger`.

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m25_unsafe --example punteros_crudos` | los 5 superpoderes: punteros crudos, `unsafe fn`, `unsafe trait`, `static mut`, `union` |
| `cargo run -p m25_unsafe --example abstraccion_segura` | `split_at_mut` propio, invariantes en constructores, `# Safety` |
| `cargo run -p m25_unsafe --example maybe_uninit` | `MaybeUninit`, `ManuallyDrop`, una pila fija con `Drop` |

## Los cinco superpoderes de `unsafe`

1. Desreferenciar punteros crudos (`*const T`, `*mut T`).
2. Llamar a funciones `unsafe` (incluidas las de FFI).
3. Acceder o modificar un `static mut`.
4. Implementar un `unsafe trait` (`Send`, `Sync`…).
5. Acceder a campos de una `union`.

`unsafe` **no** desactiva el borrow checker ni el sistema de tipos; solo te deja hacer estas cinco cosas bajo tu responsabilidad.

## Reglas de oro

- **Minimiza** el bloque `unsafe` y rodéalo de comprobaciones seguras.
- Cada `unsafe { }` lleva un comentario `// SAFETY:` que explica por qué se cumple el contrato.
- Cada `unsafe fn` o `unsafe trait` documenta el contrato en una sección `# Safety`.
- Los invariantes se protegen con **privacidad**: campos privados y constructores que los verifican.
- **Comportamiento indefinido (UB)**: referencias colgantes o desalineadas, data races, valores inválidos (`bool` ≠ 0/1, `&T` nulo), `&mut` aliased, leer memoria sin inicializar, violar el contrato de `from_utf8_unchecked`… El compilador asume que nunca ocurre y puede producir cualquier cosa.
- Una fuga de memoria (`mem::forget`) **no** es UB; por eso es una función segura.
- Panics dentro de `unsafe` pueden dejar invariantes rotos a medias: piensa en la seguridad ante excepciones (*exception safety*).

## Herramientas de auditoría

| Herramienta | Uso |
|-------------|-----|
| **Miri** | `rustup +nightly component add miri` → `cargo +nightly miri test`: intérprete que detecta UB |
| `#![forbid(unsafe_code)]` | prohíbe `unsafe` en tu crate (recomendado salvo que lo necesites) |
| `#![deny(unsafe_op_in_unsafe_fn)]` | exige bloque `unsafe {}` incluso dentro de `unsafe fn` (ya es *warn* en edición 2024) |
| `cargo geiger` | cuenta el `unsafe` de tu árbol de dependencias |
| `cargo clippy` | lints `undocumented_unsafe_blocks`, `missing_safety_doc` |
| Sanitizers | `RUSTFLAGS=-Zsanitizer=address` (nightly) |
| `loom` | explora intercalados de hilos en código concurrente |

## Ejercicios

1. Implementa `fn dividir_en_tres<T>(s: &mut [T], a: usize, b: usize) -> (&mut [T], &mut [T], &mut [T])`.
2. Añade `iter()` a `PilaFija` y comprueba con Miri que no hay UB.
3. Escribe `unsafe fn leer_u32_no_alineado(bytes: &[u8], off: usize) -> u32` con `ptr::read_unaligned`.
4. Implementa una lista enlazada simple con `*mut Nodo` y su `Drop` (sin fugas).
5. Reproduce un UB sencillo (p. ej. dos `&mut` al mismo valor con `transmute`) y observa cómo Miri lo detecta.
6. Reescribe `static mut CONTADOR_GLOBAL` con `AtomicU32` y compara.
