# Módulo 16 — Iteradores

## Objetivos

- Reemplazar bucles manuales por cadenas de adaptadores.
- Entender la evaluación perezosa.
- Implementar `Iterator` para tipos propios.

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m16_iteradores --example adaptadores` | `map`, `filter`, `fold`, `zip`, `flat_map`, `partition`, `collect` |
| `cargo run -p m16_iteradores --example perezosos` | pereza, secuencias infinitas, `successors`, `peekable` |
| `cargo run -p m16_iteradores --example iterador_propio` | implementar `Iterator`, `size_hint`, extension traits |

## Conceptos para recordar

- Un adaptador (`map`, `filter`, …) devuelve otro iterador y **no ejecuta nada**; un consumidor (`collect`, `sum`, `for`) lo pone en marcha.
- `iter()` presta `&T`, `iter_mut()` presta `&mut T`, `into_iter()` entrega `T`.
- Los iteradores se compilan a código tan rápido como un bucle `for` manual (abstracción de coste cero).
- `collect()` necesita saber el tipo destino: `Vec<_>`, `HashMap<_, _>`, `String`, `Result<Vec<_>, _>`…
- Para tu propio iterador solo implementas `next`; el resto de métodos vienen de regalo.
- Los iteradores infinitos son seguros mientras acabes con `take`, `find`, `take_while`…

## Ejercicios

1. Dada una `Vec<String>`, obtén las que empiezan por mayúscula y conviértelas a minúsculas.
2. Cuenta la frecuencia de palabras de un texto con `fold` o `entry`.
3. Implementa un iterador `Primos` infinito y obtén los 20 primeros.
4. Usa `zip` y `map` para calcular el producto escalar de dos `Vec<f64>`.
5. Escribe `fn media(v: &[f64]) -> Option<f64>` sin bucles `for`.
6. Convierte `Vec<&str>` a `Result<Vec<u32>, ParseIntError>` con `collect`.
