# Módulo 4 — Ownership

## Objetivos

- Entender las tres reglas de la propiedad.
- Distinguir **move**, **clone** y **copy**.
- Saber qué vive en el stack y qué en el heap, y cuándo se libera la memoria.

## Las reglas

1. Cada valor tiene un único **dueño**.
2. Solo puede haber un dueño a la vez.
3. Cuando el dueño sale de ámbito, el valor se **libera** (`Drop`).

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m04_ownership --example move_clone` | move, `clone()`, tipos `Copy` |
| `cargo run -p m04_ownership --example funciones_ownership` | propiedad al pasar y devolver valores |
| `cargo run -p m04_ownership --example stack_heap` | tamaños, `String` en el heap, `Box`, `Drop` |

### Errores a propósito

`errores_a_proposito/use_after_move.rs` no compila. Ejecútalo con `rustc` desde esa carpeta, lee el error y corrígelo.

## Conceptos para recordar

- Tipos `Copy` (enteros, `bool`, `char`, flotantes, tuplas de `Copy`) se duplican al asignar.
- Tipos con recursos en el heap (`String`, `Vec`, `Box`) se **mueven**.
- `clone()` es una copia profunda y explícita, con coste.
- Pasar un valor a una función = moverlo (o copiarlo si es `Copy`).
- No hay recolector de basura: el compilador inserta `drop` al final del ámbito.

## Ejercicios

1. Arregla `use_after_move.rs` de dos maneras distintas.
2. Escribe `fn mayusculas(s: String) -> String` y úsala encadenando llamadas.
3. Implementa `Drop` para un struct tuyo y comprueba el orden de liberación con tres variables.
4. Predice, sin ejecutar, qué líneas fallan al compilar y por qué:
   ```rust
   let a = vec![1, 2];
   let b = a;
   let c = b.clone();
   println!("{:?} {:?} {:?}", a, b, c);
   ```
5. ¿Por qué `let x = 5; let y = x;` no mueve `x` pero `let s = String::new(); let t = s;` sí?
