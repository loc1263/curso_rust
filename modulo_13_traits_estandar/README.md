# Módulo 13 — Traits estándar

## Objetivos

- Conocer los traits de la biblioteca estándar que implementa casi todo tipo propio.
- Usar `#[derive]` y saber cuándo escribir la implementación a mano.
- Convertir entre tipos con `From`/`Into`/`TryFrom`/`FromStr`.

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m13_traits_estandar --example derive_comparaciones` | `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Ord`, `Hash`, `Default` |
| `cargo run -p m13_traits_estandar --example display_from_into` | `Display`, `From`/`Into`, `TryFrom`, `FromStr`, `AsRef`, `Ord` manual |
| `cargo run -p m13_traits_estandar --example drop_deref_iterator` | `Drop`, `Deref`, `Index`, `IntoIterator` |

## Conceptos para recordar

- `derive` genera la implementación obvia; el orden de comparación sigue el orden de declaración de los campos.
- `Copy` exige `Clone` y que todos los campos sean `Copy`. Un `String` nunca es `Copy`.
- `Eq`/`Ord`/`Hash` no existen para `f64` por culpa de `NaN`.
- Implementa `From<A> for B` y obtienes `Into<B> for A` gratis. Para conversiones que pueden fallar usa `TryFrom`.
- `Display` es para humanos; `Debug` para depuración. Implementar `Display` da `to_string()`.
- `Drop` implementa RAII: la liberación es automática y determinista (orden inverso a la declaración).
- `Deref` habilita la *deref coercion* (`&Texto` → `&String` → `&str`).

## Ejercicios

1. Crea `struct Temperatura(f64)` con `Display` ("21.5°C") y `From<f64>`.
2. Implementa `FromStr` para un `struct Punto { x, y }` que parsee `"3,4"`.
3. Ordena un `Vec<Persona>` por edad usando `Ord` manual, y luego por nombre con `sort_by_key`.
4. Escribe un `struct Archivo` que imprima "cerrando…" en `Drop` y comprueba el orden con varios valores.
5. Implementa `TryFrom<i32> for DiaSemana` (1–7).
6. ¿Por qué `HashSet<f64>` no compila? Reemplázalo con un wrapper que implemente `Eq` y `Hash` usando `to_bits`.
