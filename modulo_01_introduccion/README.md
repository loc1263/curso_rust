# Módulo 1 — Introducción

## Objetivos

- Entender qué es Rust y por qué usarlo.
- Compilar con `rustc` y con `cargo`.
- Conocer la estructura de un proyecto y el archivo `Cargo.toml`.

## Estructura

```
modulo_01_introduccion/
├── rustc/                  # compilación manual con rustc
│   ├── 01_hola_mundo.rs
│   ├── 02_macros_println.rs
│   └── 03_error_compilador.rs   # no compila a propósito
└── hola_cargo/             # proyecto creado con `cargo new`
    ├── Cargo.toml
    ├── src/
    │   ├── main.rs
    │   └── bin/contador.rs
    └── examples/saludo.rs
```

## Comandos clave

| Comando | Qué hace |
|---------|----------|
| `rustc archivo.rs` | Compila un solo archivo |
| `cargo new nombre` | Crea un proyecto nuevo |
| `cargo check` | Verifica que compila, sin generar binario (rápido) |
| `cargo build` | Compila en modo debug (`target/debug`) |
| `cargo build --release` | Compila optimizado (`target/release`) |
| `cargo run` | Compila y ejecuta |
| `cargo run --bin contador` | Ejecuta un binario de `src/bin/` |
| `cargo run --example saludo -- Ana` | Ejecuta un ejemplo de `examples/` |
| `cargo add <crate>` | Añade una dependencia |
| `cargo fmt` / `cargo clippy` | Formatea / analiza el código |

## Ejercicios

1. Compila y ejecuta `rustc/01_hola_mundo.rs` con `rustc`.
2. Lee el error de `03_error_compilador.rs`, corrígelo y vuelve a compilar.
3. En `hola_cargo`, ejecuta `cargo run`, `cargo run --bin contador` y `cargo run --example saludo -- tu nombre`.
4. Compara el tiempo de `cargo build` y `cargo build --release`, y mira el tamaño de ambos binarios.
5. Ejecuta `cargo add rand`, mira cómo cambia `Cargo.toml` y usa `rand::random::<u8>()` en `main.rs`.
6. Crea tu propio proyecto con `cargo new mi_proyecto` e imprime tu nombre y tu edad con `println!`.

## Conceptos para recordar

- `fn main()` es el punto de entrada.
- `println!` es una **macro** (el `!` lo indica).
- `Cargo.toml` define nombre, versión, edición y dependencias.
- `Cargo.lock` fija las versiones exactas; `target/` contiene lo compilado y no se sube a git.
