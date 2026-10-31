# Módulo 9 — Manejo de errores

## Objetivos

- Distinguir errores irrecuperables (`panic!`) de recuperables (`Result`).
- Propagar errores con `?`.
- Definir tipos de error propios.

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m09_manejo_errores --example panic_result` | `panic!`, `Result`, combinadores, `ErrorKind` |
| `cargo run -p m09_manejo_errores --example panic_result -- --panic` | ver un panic real |
| `cargo run -p m09_manejo_errores --example operador_pregunta` | `?` con `Result` y `Option`, `Box<dyn Error>` |
| `cargo run -p m09_manejo_errores --example errores_propios` | `enum` de errores, `Display`, `Error`, `From` |

## Conceptos para recordar

- `panic!` es para bugs e invariantes rotos; los errores esperables se devuelven con `Result`.
- `unwrap()` y `expect()` entran en pánico: úsalos en prototipos, pruebas o cuando el fallo sea imposible.
- `?` equivale a "si es `Err`, retorna ese error (convertido con `From`)".
- `?` solo puede usarse en funciones que devuelven `Result` u `Option` (o `main` con `-> Result`).
- `Box<dyn Error>` es cómodo para aplicaciones; un `enum` propio es mejor para bibliotecas.
- `RUST_BACKTRACE=1` muestra la traza de un panic.

## Ejercicios

1. Escribe `fn leer_y_sumar(ruta: &str) -> Result<i32, Box<dyn Error>>` que sume los números de un archivo (uno por línea).
2. Crea `enum ErrorBanco { SaldoInsuficiente { necesario: f64, disponible: f64 }, CuentaInexistente(String) }` con su `Display`.
3. Implementa `fn retirar(&mut self, monto: f64) -> Result<(), ErrorBanco>` para una cuenta.
4. Reescribe una cadena de `match` anidados de `Result` usando `?`.
5. Prueba `RUST_BACKTRACE=1 cargo run --example panic_result -- --panic` y lee la traza.
6. Añade `impl From<std::io::Error>` a tu enum de errores y usa `?` sobre `fs::read_to_string`.
