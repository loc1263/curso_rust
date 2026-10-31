# Módulo 7 — Enums y pattern matching

## Objetivos

- Modelar datos que pueden ser "una cosa u otra" con `enum`.
- Usar `Option<T>` en lugar de `null`.
- Dominar `match`, `if let`, `while let` y `let else`.

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m07_enums_pattern_matching --example enums` | variantes con y sin datos, métodos en enums |
| `cargo run -p m07_enums_pattern_matching --example option` | `Some`/`None`, `map`, `and_then`, `unwrap_or`, `ok_or` |
| `cargo run -p m07_enums_pattern_matching --example match_patrones` | rangos, guardas, `@`, struct patterns, `let else`, `matches!` |

## Conceptos para recordar

- Cada variante puede contener datos distintos; así se hacen tipos suma (*sum types*).
- `match` es **exhaustivo**: el compilador te obliga a cubrir todos los casos.
- `Option<T>` = `Some(T)` o `None`. No hay valores nulos implícitos.
- Prefiere `match`/`if let`/combinadores (`map`, `unwrap_or`) a `unwrap()`.
- Patrones útiles: `a | b`, `1..=5`, `n @ 1..=5`, `if guarda`, `..` para ignorar el resto.

## Ejercicios

1. Define `enum Figura { Circulo(f64), Rectangulo(f64, f64), Triangulo { base: f64, altura: f64 } }` con un método `area`.
2. Crea `enum Semaforo { Rojo, Amarillo, Verde }` con `siguiente()` y `duracion_segundos()`.
3. Escribe `fn segundo(v: &[i32]) -> Option<i32>` y úsala con `match`, `if let` y `unwrap_or`.
4. Crea `enum Comando { Avanzar(u32), Girar(i32), Parar }` y simula un robot en una posición `(x, y)`.
5. Reescribe una función con `if/else if` encadenados usando `match` con rangos y guardas.
6. Escribe una función con `let else` que extraiga el primer número de un `&str` como `"edad=42"`.
