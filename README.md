# Curso de Rust

Ruta completa en [ROADMAP.md](ROADMAP.md).

## Nivel 1 — Básico

| Módulo | Carpeta |
|--------|---------|
| 1. Introducción | [modulo_01_introduccion](modulo_01_introduccion/README.md) |
| 2. Fundamentos | [modulo_02_fundamentos](modulo_02_fundamentos/README.md) |
| 3. Funciones y control de flujo | [modulo_03_funciones_control_flujo](modulo_03_funciones_control_flujo/README.md) |
| 4. Ownership | [modulo_04_ownership](modulo_04_ownership/README.md) |
| 5. Referencias y borrowing | [modulo_05_referencias_borrowing](modulo_05_referencias_borrowing/README.md) |
| 6. Structs y métodos | [modulo_06_structs_metodos](modulo_06_structs_metodos/README.md) |
| 7. Enums y pattern matching | [modulo_07_enums_pattern_matching](modulo_07_enums_pattern_matching/README.md) |
| 8. Colecciones | [modulo_08_colecciones](modulo_08_colecciones/README.md) |
| 9. Manejo de errores | [modulo_09_manejo_errores](modulo_09_manejo_errores/README.md) |
| 10. Módulos y paquetes | [modulo_10_modulos_paquetes](modulo_10_modulos_paquetes/README.md) |
| Proyecto integrador | [proyecto_nivel_1_todo](proyecto_nivel_1_todo/README.md) |

## Nivel 2 — Intermedio

| Módulo | Carpeta |
|--------|---------|
| 11. Genéricos | [modulo_11_genericos](modulo_11_genericos/README.md) |
| 12. Traits | [modulo_12_traits](modulo_12_traits/README.md) |
| 13. Traits estándar | [modulo_13_traits_estandar](modulo_13_traits_estandar/README.md) |
| 14. Lifetimes | [modulo_14_lifetimes](modulo_14_lifetimes/README.md) |
| 15. Closures | [modulo_15_closures](modulo_15_closures/README.md) |
| 16. Iteradores | [modulo_16_iteradores](modulo_16_iteradores/README.md) |
| 17. Smart pointers | [modulo_17_smart_pointers](modulo_17_smart_pointers/README.md) |
| 18. Errores avanzados | [modulo_18_errores_avanzados](modulo_18_errores_avanzados/README.md) |
| 19. Testing | [modulo_19_testing](modulo_19_testing/README.md) |
| 20. Concurrencia | [modulo_20_concurrencia](modulo_20_concurrencia/README.md) |
| 21. Ecosistema | [modulo_21_ecosistema](modulo_21_ecosistema/README.md) |
| Proyecto integrador (`rgrep`) | [proyecto_nivel_2_grep](proyecto_nivel_2_grep/README.md) |

## Nivel 3 — Avanzado

| Módulo | Carpeta |
|--------|---------|
| 22. Async/await | [modulo_22_async_await](modulo_22_async_await/README.md) |
| 23. Pin y Futures a fondo | [modulo_23_pin_futures](modulo_23_pin_futures/README.md) |
| 24. Programación web / red | [modulo_24_web_red](modulo_24_web_red/README.md) |
| 25. Unsafe Rust | [modulo_25_unsafe](modulo_25_unsafe/README.md) |
| 26. FFI | [modulo_26_ffi](modulo_26_ffi/README.md) |
| 27. Macros | [modulo_27_macros](modulo_27_macros/README.md) |
| 28. Sistema de tipos avanzado | [modulo_28_tipos_avanzados](modulo_28_tipos_avanzados/README.md) |
| 29. Rendimiento | [modulo_29_rendimiento](modulo_29_rendimiento/README.md) |
| 30. Concurrencia avanzada | [modulo_30_concurrencia_avanzada](modulo_30_concurrencia_avanzada/README.md) |
| 31. `no_std`, embebido y WebAssembly | [modulo_31_no_std_embebido](modulo_31_no_std_embebido/README.md) |
| 32. Producción | [modulo_32_produccion](modulo_32_produccion/README.md) |
| Proyecto integrador (`kvdb`) | [proyecto_nivel_3_kv](proyecto_nivel_3_kv/README.md) |

## Cómo ejecutar los ejemplos

La raíz es un *workspace* de Cargo, así que desde aquí funciona:

```bash
cargo run -p m04_ownership --example move_clone
cargo run -p m16_iteradores --example perezosos
cargo run --release -p m29_rendimiento --example layout_memoria   # rendimiento: siempre --release
cargo test --workspace
cargo build --workspace --examples
```

Dentro de cada carpeta de módulo basta con `cargo run --example <nombre>`.
Cada README de módulo lista el comando exacto de sus ejemplos.

> **Nota:** los módulos 18, 21, 22, 24, 27 y 29 y los proyectos de Nivel 2 y 3 usan crates externos
> (`anyhow`, `thiserror`, `serde`, `clap`, `tokio`, `axum`, `reqwest`, `rayon`, `criterion`, `syn`…);
> la primera compilación los descarga y tarda más. Los módulos 23, 25, 26 (parte Rust) y 28, 30 solo usan la biblioteca estándar.

## Proyectos integradores

```bash
# Nivel 1: gestor de tareas por consola
cargo run -p todo

# Nivel 2: mini grep con clap
cargo run -p rgrep -- -n -i fn proyecto_nivel_2_grep/src/lib.rs

# Nivel 3: base de datos clave-valor async (dos terminales)
cargo run -p kvdb --bin kv-server -- --aof datos.aof
cargo run -p kvdb --bin kv-cli
```
