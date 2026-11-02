# Módulo 13 — Traits estándar

## La idea en 2 minutos

La biblioteca estándar define traits que "enseñan" a tus tipos a comportarse como los de siempre: imprimirse (`Display`), compararse (`PartialEq`, `Ord`), copiarse (`Clone`), convertirse (`From`)...

```rust
#[derive(Debug, Clone, PartialEq)]   // tres traits gratis con una línea
struct Punto { x: i32, y: i32 }

let p = Punto { x: 1, y: 2 };
println!("{p:?}");          // gracias a Debug
assert_eq!(p, p.clone());   // gracias a Clone + PartialEq
```

`derive` escribe la implementación obvia por ti. Cuando lo "obvio" no sirve (p. ej., ordenar por brillo en vez de por campos), la implementas a mano.
Regla útil: **si un tipo tuyo representa un valor, casi siempre quieres `Debug`, `Clone` y `PartialEq`.**

**Orden recomendado:** `derive_comparaciones` → `display_from_into` → `drop_deref_iterator`.

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
