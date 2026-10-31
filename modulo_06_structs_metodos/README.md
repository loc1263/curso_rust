# Módulo 6 — Structs y métodos

## Objetivos

- Agrupar datos relacionados con `struct`.
- Añadir comportamiento con bloques `impl`.
- Conocer tuple structs, unit structs y el patrón newtype.

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m06_structs_metodos --example structs` | campos, shorthand, `..` de actualización, `{:#?}` |
| `cargo run -p m06_structs_metodos --example metodos` | `&self`, `&mut self`, `self`, funciones asociadas |
| `cargo run -p m06_structs_metodos --example tuple_structs` | tuple/unit structs, newtype, `derive` |

## Conceptos para recordar

- `Self` es un alias del tipo del bloque `impl`.
- Primer parámetro: `&self` (leer), `&mut self` (modificar), `self` (consumir).
- Sin `self` es una **función asociada**, típica para constructores (`new`).
- La mutabilidad es de toda la instancia; no existen campos `mut` individuales.
- `#[derive(Debug, Clone, Copy, PartialEq, Default)]` genera implementaciones automáticas.
- Con `..otra` se reutilizan campos, y eso puede **mover** los que no son `Copy`.

## Ejercicios

1. Crea `struct Cuenta { titular: String, saldo: f64 }` con `depositar`, `retirar` (que devuelva `bool`) y `saldo`.
2. Crea `struct Circulo` con `area()` y `circunferencia()`, y una función asociada `unitario()`.
3. Implementa `struct Temperatura(f64)` con los métodos `a_celsius` y `a_fahrenheit`.
4. Añade `#[derive(PartialEq, PartialOrd)]` a un struct y compara dos instancias.
5. Crea `struct Libro` con `titulo`, `autor`, `paginas` y una función que reciba `&[Libro]` y devuelva el más largo.
6. Explica por qué `r.into_tupla()` impide usar `r` después.
