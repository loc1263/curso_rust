# Módulo 2 — Fundamentos

## Objetivos

- Declarar variables, entender la inmutabilidad y el shadowing.
- Conocer los tipos escalares (enteros, flotantes, `bool`, `char`) y los compuestos (tuplas, arrays).
- Convertir entre tipos de forma explícita.

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m02_fundamentos --example variables` | `let`, `mut`, shadowing, `const`, `static` |
| `cargo run -p m02_fundamentos --example tipos` | enteros, desbordamiento, flotantes, `char`, `as`, `parse` |
| `cargo run -p m02_fundamentos --example tuplas_arrays` | tuplas, arrays, acceso seguro con `get` |

(Desde esta carpeta basta con `cargo run --example variables`.)

## Conceptos para recordar

- Las variables son **inmutables por defecto**; `mut` las hace mutables.
- Shadowing (`let x = ...` de nuevo) crea otra variable y permite cambiar el tipo.
- Rust infiere tipos, pero a veces hay que anotarlos (`let n: i32 = "5".parse().unwrap()`).
- En debug el desbordamiento entra en pánico; usa `checked_*`, `wrapping_*` o `saturating_*` para elegir el comportamiento.
- `as` puede truncar; prefiere `From`/`TryFrom` cuando importe la seguridad.
- Un array tiene tamaño fijo conocido en compilación; para tamaño variable se usa `Vec` (módulo 8).

## Ejercicios

1. Convierte grados Celsius a Fahrenheit usando `f64`.
2. Genera el n-ésimo número de Fibonacci con `u64` y comprueba qué `n` desborda; usa `checked_add` para detectarlo.
3. Imprime la letra y el código numérico de cada carácter de `"Rust"`.
4. Crea una tupla `(nombre, edad, altura)`, desestructúrala y muestra cada campo.
5. Dado `[3, 8, 1, 9, 4]`, calcula el mínimo, el máximo y la media sin usar `Vec`.
6. Intenta asignar dos veces a una variable inmutable y lee el mensaje del compilador.
