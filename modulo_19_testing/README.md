# Módulo 19 — Testing

## Objetivos

- Escribir tests unitarios, de integración y doc tests.
- Probar casos de éxito, de error y de pánico.
- Medir rendimiento de forma básica.

## Estructura

```
modulo_19_testing/
├── src/lib.rs               # código + tests unitarios (#[cfg(test)]) + doc tests (///)
├── tests/integracion.rs     # tests de integración (solo API pública)
├── tests/comun/mod.rs       # helpers compartidos entre tests de integración
└── examples/benchmark_basico.rs
```

## Comandos

| Comando | Qué hace |
|---------|----------|
| `cargo test -p m19_testing` | todos los tests (unitarios + integración + doc) |
| `cargo test -p m19_testing palindromos` | solo los que contengan ese nombre |
| `cargo test -p m19_testing -- --nocapture` | muestra los `println!` de los tests |
| `cargo test -p m19_testing -- --ignored` | ejecuta los marcados con `#[ignore]` |
| `cargo test -p m19_testing --lib` / `--test integracion` / `--doc` | solo un tipo |
| `cargo run --release -p m19_testing --example benchmark_basico` | benchmark casero |

## Conceptos para recordar

- `#[test]` marca una función; falla si hace `panic!` (o si devuelve `Err`).
- Macros: `assert!`, `assert_eq!`, `assert_ne!` (admiten mensaje personalizado).
- `#[should_panic(expected = "...")]` comprueba que se produzca un pánico concreto.
- `#[cfg(test)]` compila el módulo solo al ejecutar tests.
- Los **tests unitarios** viven junto al código y pueden ver lo privado; los de **integración** solo ven lo `pub`.
- Los **doc tests** garantizan que los ejemplos de la documentación compilan y funcionan.
- Los tests se ejecutan en paralelo: no compartas estado global mutable entre ellos.
- Para benchmarks serios usa `criterion` (módulo 29); `#[bench]` sigue siendo inestable.

## Ejercicios

1. Añade `fn factorial(n: u32) -> Option<u64>` (con `checked_mul`) y escribe tests para 0, 5 y overflow.
2. Escribe un test con `#[should_panic]` para una función que haga `panic!` con índice fuera de rango.
3. Añade un doc test con un ejemplo que **debe fallar al compilar** (`compile_fail`) y otro `no_run`.
4. Crea otro archivo en `tests/` y comparte datos mediante `tests/comun/mod.rs`.
5. Escribe tests parametrizados con un array de casos `[(entrada, esperado)]` y un `for`.
6. Mide cuánto tarda `es_palindromo` con un texto de 1 millón de caracteres.
