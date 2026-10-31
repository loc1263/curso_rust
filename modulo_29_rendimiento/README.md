# Módulo 29 — Rendimiento

## Objetivos

- Medir antes de optimizar: benchmarks y profiling.
- Entender el layout de memoria y su efecto en la caché.
- Evitar asignaciones innecesarias, usar SIMD y paralelismo de datos.

## Contenido

| Comando | Tema |
|---------|------|
| `cargo bench -p m29_rendimiento` | benchmarks con **criterion** (`benches/algoritmos.rs`) |
| `cargo run --release -p m29_rendimiento --example rayon_paralelo` | `par_iter`, `join`, `par_sort`: cuándo compensa |
| `cargo run --release -p m29_rendimiento --example layout_memoria` | `size_of`/`align_of`, `repr`, nichos, AoS vs SoA, caché |
| `cargo run --release -p m29_rendimiento --example evitar_allocaciones` | `#[global_allocator]` que cuenta asignaciones |
| `cargo run --release -p m29_rendimiento --example simd_basico` | autovectorización y `std::arch` con AVX |

> **Siempre `--release`** para medir. El perfil `dev` no optimiza y las cifras no sirven.

## Metodología

1. **Define la métrica** (latencia, throughput, memoria) y un benchmark reproducible.
2. **Perfila** para encontrar el cuello de botella real (no adivines).
3. **Cambia una cosa** y vuelve a medir.
4. Prefiere mejorar el **algoritmo y las estructuras de datos** antes que micro-optimizar.

## Profiling

| Herramienta | Plataforma | Uso |
|-------------|-----------|-----|
| `cargo flamegraph` | Linux/macOS/Windows | `cargo install flamegraph` → `cargo flamegraph --example evitar_allocaciones -p m29_rendimiento` |
| `perf` | Linux | `perf record --call-graph dwarf ./target/release/mi_bin && perf report` |
| `samply` | Linux/macOS/Windows | `cargo install samply` → `samply record ./target/release/mi_bin` (abre Firefox Profiler) |
| Visual Studio / WPA | Windows | perfilador nativo con símbolos `.pdb` |
| `heaptrack` / `dhat` | Linux / multiplataforma | perfil de memoria; el crate `dhat` es un asignador instrumentado |
| `cargo asm` / Compiler Explorer | todas | inspeccionar el ensamblador generado |

Para tener símbolos legibles en el profiler, añade al `Cargo.toml` de la raíz:

```toml
[profile.release]
debug = true      # símbolos de depuración (no afecta a la velocidad)
```

## Ajustes del perfil `release`

```toml
[profile.release]
opt-level = 3          # (por defecto) máximo; "s"/"z" optimizan tamaño
lto = "fat"            # optimización entre crates (más lento compilando, binario más rápido/pequeño)
codegen-units = 1      # menos paralelismo al compilar, mejor optimización
panic = "abort"        # sin desenrollado: binario más pequeño (pierdes catch_unwind)
strip = true           # elimina símbolos
```

Más: `RUSTFLAGS="-C target-cpu=native"` y PGO (`cargo-pgo`) para exprimir el último 10–20 %.

## Conceptos para recordar

- Las abstracciones de coste cero (iteradores, genéricos) se compilan a código tan bueno como el manual.
- `black_box` evita que el compilador elimine el código que estás midiendo.
- **Caché**: recorrer memoria contigua es mucho más rápido que saltar. Prefiere `Vec` a listas enlazadas; SoA a AoS en bucles calientes.
- **Asignaciones**: `with_capacity`, reutilizar buffers (`clear`), `&str`/`&[T]` en vez de clonar, `enum` en vez de `Box<dyn>`, arrays en pila.
- **Paralelismo**: rayon reparte trabajo con *work stealing*; solo compensa si el trabajo por elemento es grande o los datos enormes. Mide.
- `sort_unstable` es más rápido que `sort` si no necesitas estabilidad.
- Bounds checks: itera con iteradores en vez de índices; si hay que indexar, comprueba con `assert!(i < len)` antes del bucle.
- `#[inline]` solo ayuda entre crates o con funciones pequeñas; confía primero en el optimizador.

## Ejercicios

1. Ejecuta `cargo bench -p m29_rendimiento` y explica las diferencias entre `sort`, `sort_unstable` e inserción.
2. Reescribe `contar_largas_con_alloc` para usar `eq_ignore_ascii_case` o similar y mide con `evitar_allocaciones`.
3. Genera un flamegraph de `rayon_paralelo` e identifica dónde se va el tiempo.
4. Convierte `ParticulaAoS` a SoA en un programa que muevan 1 M de partículas y mide la mejora.
5. Prueba `[profile.release] lto = "fat", codegen-units = 1` y compara tiempos y tamaño de binario.
6. Implementa un `suma_avx` equivalente con el crate `wide` (estable y portable).
