# Roadmap del Curso de Rust

Ruta de aprendizaje en tres niveles: **Básico → Intermedio → Avanzado**.
Cada módulo termina con ejercicios y, al final de cada nivel, un proyecto integrador.

## Requisitos previos

- Nociones básicas de programación (variables, funciones, condicionales, bucles).
- Manejo básico de la terminal.
- Tener instalado `rustup`, `cargo` y un editor con `rust-analyzer` (VS Code recomendado).

---

## Nivel 1 — Básico

**Objetivo:** escribir programas pequeños en Rust y entender el modelo de propiedad.

| # | Módulo | Contenido |
|---|--------|-----------|
| 1 | Introducción | Qué es Rust, instalación, `rustc`, `cargo new/build/run/check`, estructura de un proyecto, `Cargo.toml` |
| 2 | Fundamentos | Variables e inmutabilidad, shadowing, tipos escalares, tuplas, arrays, comentarios, `println!` |
| 3 | Funciones y control de flujo | Funciones, expresiones vs sentencias, `if`, `loop`, `while`, `for`, rangos |
| 4 | Ownership | Reglas de propiedad, move, `Clone`/`Copy`, stack vs heap |
| 5 | Referencias y borrowing | `&` y `&mut`, reglas del borrow checker, slices (`&str`, `&[T]`) |
| 6 | Structs y métodos | Definición de structs, `impl`, métodos y funciones asociadas, tuple structs |
| 7 | Enums y pattern matching | `enum`, `Option<T>`, `match`, `if let`, `while let` |
| 8 | Colecciones | `Vec`, `String`, `HashMap`, iteración básica |
| 9 | Manejo de errores | `panic!`, `Result<T, E>`, operador `?`, `unwrap`/`expect` |
| 10 | Módulos y paquetes | `mod`, `pub`, `use`, crates, uso de dependencias de crates.io |

**Proyecto integrador:** aplicación CLI de gestión de tareas (TODO) con persistencia en archivo.

**Al terminar sabrás:** leer errores del compilador, usar ownership/borrowing con soltura y estructurar un proyecto pequeño.

---

## Nivel 2 — Intermedio

**Objetivo:** escribir código idiomático y abstracciones reutilizables.

| # | Módulo | Contenido |
|---|--------|-----------|
| 11 | Genéricos | Funciones y tipos genéricos, `where`, monomorfización |
| 12 | Traits | Definir e implementar traits, métodos por defecto, trait bounds, `impl Trait`, `dyn Trait` |
| 13 | Traits estándar | `Debug`, `Display`, `Clone`, `PartialEq`, `Ord`, `Default`, `From`/`Into`, `derive` |
| 14 | Lifetimes | Anotaciones, elisión, lifetimes en structs y funciones, `'static` |
| 15 | Closures | Captura (`Fn`, `FnMut`, `FnOnce`), `move`, closures como parámetros y retornos |
| 16 | Iteradores | Trait `Iterator`, `map`/`filter`/`fold`/`collect`, iteradores perezosos, iteradores propios |
| 17 | Smart pointers | `Box`, `Rc`, `Arc`, `RefCell`, `Cell`, `Deref`, `Drop`, ciclos con `Weak` |
| 18 | Errores avanzados | Tipos de error propios, `Box<dyn Error>`, crates `thiserror` y `anyhow` |
| 19 | Testing | Tests unitarios y de integración, `#[cfg(test)]`, doc tests, benchmarks básicos |
| 20 | Concurrencia | `thread`, `mpsc`, `Mutex`, `RwLock`, `Arc`, `Send` y `Sync`, data races en tiempo de compilación |
| 21 | Ecosistema | `serde` / `serde_json`, `clap`, `rand`, `chrono`, `tracing`, `clippy`, `rustfmt` |

**Proyecto integrador:** herramienta CLI tipo `grep` (con `clap`) o servidor HTTP básico con un parser de JSON.

**Al terminar sabrás:** diseñar APIs con traits y genéricos, manejar lifetimes y escribir código concurrente seguro.

---

## Nivel 3 — Avanzado

**Objetivo:** dominar async, `unsafe`, rendimiento y proyectos de producción.

| # | Módulo | Contenido |
|---|--------|-----------|
| 22 | Async/await | `Future`, `async`/`await`, ejecutores, `tokio`, tareas, `select!`, canales async |
| 23 | Pin y Futures a fondo | `Pin`, `Unpin`, cómo funciona un ejecutor, implementar un `Future` manualmente |
| 24 | Programación web / red | `axum` o `actix-web`, `reqwest`, bases de datos con `sqlx`, middleware, autenticación |
| 25 | Unsafe Rust | Raw pointers, `unsafe fn`/`trait`, invariantes, `MaybeUninit`, buenas prácticas y auditoría |
| 26 | FFI | Interoperar con C (`extern "C"`, `bindgen`, `cbindgen`), exponer Rust a Python/Node (`PyO3`, `napi-rs`) |
| 27 | Macros | `macro_rules!`, macros procedurales (derive, attribute, function-like), `syn` y `quote` |
| 28 | Sistema de tipos avanzado | Tipos asociados, GATs, const generics, `PhantomData`, typestate, newtype, HRTB |
| 29 | Rendimiento | Profiling (`perf`, `flamegraph`), `criterion`, layout de memoria, SIMD, evitar allocations, `rayon` |
| 30 | Concurrencia avanzada | Atomics y orderings, estructuras lock-free, modelo de memoria |
| 31 | `no_std` y embebido | `core`/`alloc`, `no_std`, introducción a embedded (`embassy`), WebAssembly (`wasm-bindgen`) |
| 32 | Producción | Workspaces, features, CI/CD, `cargo-deny`/`audit`, publicación en crates.io, observabilidad, Docker |

**Proyecto integrador (a elegir):**
- Servidor web async con base de datos, autenticación y despliegue en contenedor.
- Runtime async mínimo o un pequeño motor de base de datos clave-valor.
- Extensión nativa para Python/Node escrita en Rust con benchmarks comparativos.

**Al terminar sabrás:** construir sistemas de alto rendimiento, escribir `unsafe` con criterio y publicar y mantener crates.

---

## Recursos recomendados

- *The Rust Programming Language* ("The Book") — https://doc.rust-lang.org/book/
- *Rust by Example* — https://doc.rust-lang.org/rust-by-example/
- *Rustlings* (ejercicios interactivos) — https://github.com/rust-lang/rustlings
- *Rust Standard Library* docs — https://doc.rust-lang.org/std/
- *Rust Async Book* — https://rust-lang.github.io/async-book/
- *The Rustonomicon* (unsafe) — https://doc.rust-lang.org/nomicon/
- *The Little Book of Rust Macros* — https://veykril.github.io/tlborm/
- *Rust Design Patterns* — https://rust-unofficial.github.io/patterns/
- *Comprehensive Rust* (curso de Google) — https://google.github.io/comprehensive-rust/

## Duración estimada

| Nivel | Duración sugerida (≈ 6–8 h/semana) |
|-------|-----------------------------------|
| Básico | 6–8 semanas |
| Intermedio | 8–10 semanas |
| Avanzado | 10–14 semanas |

## Seguimiento

- [ ] Nivel 1 completado + proyecto CLI TODO
- [ ] Nivel 2 completado + proyecto CLI/HTTP
- [ ] Nivel 3 completado + proyecto final
