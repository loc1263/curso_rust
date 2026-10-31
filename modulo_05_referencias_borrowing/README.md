# Módulo 5 — Referencias y borrowing

## Objetivos

- Usar referencias `&` y `&mut` para prestar valores sin moverlos.
- Conocer las reglas del borrow checker.
- Trabajar con slices (`&str`, `&[T]`).

## Las reglas del préstamo

En cualquier momento puedes tener **una de dos cosas**:

- cualquier número de referencias inmutables (`&T`), **o**
- exactamente una referencia mutable (`&mut T`).

Y las referencias siempre deben ser válidas (no existen referencias colgantes).

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m05_referencias_borrowing --example referencias` | `&`, `&mut`, desreferencia con `*` |
| `cargo run -p m05_referencias_borrowing --example slices` | `&str`, `&[T]`, rangos |
| `cargo run -p m05_referencias_borrowing --example borrow_checker` | patrones válidos, NLL, `iter_mut` |

### Errores a propósito

En `errores_a_proposito/` hay tres archivos que no compilan (`rustc archivo.rs`):

| Archivo | Error |
|---------|-------|
| `dos_mutables.rs` | E0499: dos préstamos mutables simultáneos |
| `mutable_e_inmutable.rs` | E0502: mutar mientras hay una referencia viva |
| `referencia_colgante.rs` | E0106: devolver una referencia a un local |

## Conceptos para recordar

- Las funciones deben recibir `&str` y `&[T]` en lugar de `&String` y `&Vec<T>`: aceptan más tipos.
- Una referencia vive hasta su **último uso** (NLL), no hasta el final del bloque.
- Los slices guardan puntero + longitud; no copian datos.
- Los índices de `&str` son bytes y deben caer en límites de carácter UTF-8.

## Ejercicios

1. Corrige los tres archivos de `errores_a_proposito/`.
2. Escribe `fn ultima_palabra(s: &str) -> &str`.
3. Escribe `fn incrementar_todos(v: &mut [i32])` que sume 1 a cada elemento.
4. Escribe `fn mayor(v: &[i32]) -> Option<&i32>` sin usar `iter().max()`.
5. Escribe `fn contar_vocales(s: &str) -> usize` y llámala con un `&String` y con un literal.
6. Explica por qué el siguiente código compila: `let r = &mut v; r.push(1); v.push(2);`.
