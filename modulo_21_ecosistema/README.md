# Módulo 21 — Ecosistema

## Objetivos

- Conocer los crates imprescindibles de la comunidad.
- Usar las herramientas de calidad de `cargo`: `clippy`, `rustfmt`, `doc`.
- Aprender a evaluar y añadir dependencias.

## Ejemplos

| Comando | Crate | Tema |
|---------|-------|------|
| `cargo run -p m21_ecosistema --example serde_json_demo` | `serde`, `serde_json` | (de)serialización, atributos, enums, `json!` |
| `cargo run -p m21_ecosistema --example clap_cli -- --help` | `clap` | CLI con derive, subcomandos, enums de valores |
| `cargo run -p m21_ecosistema --example chrono_rand` | `chrono`, `rand` | fechas, duraciones, aleatorios con y sin semilla |
| `cargo run -p m21_ecosistema --example tracing_demo` | `tracing` | logs estructurados y spans |

## Herramientas de cargo

| Comando | Para qué |
|---------|----------|
| `cargo fmt` | formatea el código (rustfmt); `cargo fmt --check` en CI |
| `cargo clippy` | linter con cientos de consejos de código idiomático |
| `cargo clippy -- -D warnings` | convierte los avisos en errores (CI) |
| `cargo doc --open` | genera y abre la documentación de tu crate y sus dependencias |
| `cargo add <crate>` | añade una dependencia a `Cargo.toml` |
| `cargo tree` | árbol de dependencias |
| `cargo update` | actualiza `Cargo.lock` |

## Conceptos para recordar

- Las dependencias van en `[dependencies]` de `Cargo.toml`; las *features* activan partes opcionales (`features = ["derive"]`).
- `Cargo.lock` fija versiones exactas: se versiona en binarios y es opcional en bibliotecas.
- Antes de depender de un crate revisa: descargas, mantenimiento reciente, licencia y `unsafe`.
- `serde` es la base de casi todo el ecosistema de formatos (JSON, TOML, YAML, bincode…).
- Para logs usa `tracing` (aplicaciones async o con contexto) o `log` + `env_logger` (simple).
- Otros crates útiles: `regex`, `itertools`, `once_cell`/`LazyLock`, `walkdir`, `indicatif`, `colored`.

## Ejercicios

1. Define un `struct Config` con `serde`, léelo de un archivo JSON y escribe una versión modificada.
2. Crea una CLI con `clap` que reciba una ruta y las flags `--recursivo` y `--ext <EXT>`.
3. Calcula cuántos días faltan hasta el próximo 1 de enero con `chrono`.
4. Genera una contraseña aleatoria de 16 caracteres eligiendo de un alfabeto.
5. Añade `#[instrument]` a una función recursiva y observa el árbol de spans con `RUST_LOG=trace`.
6. Pasa `cargo clippy` a tu proyecto del Nivel 1 y corrige todas las sugerencias.
