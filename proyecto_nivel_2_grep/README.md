# Proyecto integrador — Nivel 2: `rgrep`

Un mini `grep` escrito con lo aprendido en el Nivel 2: **traits**, **genéricos**, **iteradores**, **closures**, **errores tipados** (`thiserror` + `anyhow`), **testing** y **`clap`**.

## Uso

```bash
cargo run -p rgrep -- <patrón> <rutas...> [opciones]

# Ejemplos
cargo run -p rgrep -- fn proyecto_nivel_2_grep/src/lib.rs -n
cargo run -p rgrep -- -r -n -i todo modulo_11_genericos --ext rs
cargo run -p rgrep -- -c -r "impl" . --ext rs
cargo run -p rgrep -- -w -i rust README.md
cargo run -p rgrep -- -v -i rust README.md        # líneas que NO contienen el texto
```

| Opción | Efecto |
|--------|--------|
| `-i`, `--ignorar-mayusculas` | no distingue mayúsculas |
| `-n`, `--numeros` | muestra el número de línea |
| `-v`, `--invertir` | líneas que **no** coinciden |
| `-w`, `--palabra` | solo palabras completas |
| `-c`, `--contar` | muestra el número de coincidencias por archivo |
| `-r`, `--recursivo` | recorre directorios |
| `-e`, `--ext <EXT>` | limita a una extensión |

Códigos de salida (como `grep`): `0` hubo coincidencias, `1` ninguna, `2` error.

## Arquitectura

```
src/lib.rs    Buscador, Opciones, Coincidencia, ErrorBusqueda (thiserror), listar_archivos
              + tests unitarios
src/main.rs   CLI con clap (derive) + anyhow para el contexto de errores
tests/cli.rs  tests de integración que ejecutan el binario real
```

Decisiones de diseño relacionadas con el curso:

- **Biblioteca + binario**: la lógica es testeable sin pasar por la terminal.
- **Errores tipados en la lib, `anyhow` en el binario** (módulo 18).
- **Iteradores** (`lines().enumerate().filter().map().collect()`) en lugar de bucles manuales (módulo 16).
- **Un archivo ilegible no detiene la búsqueda**: se avisa por `stderr` y se continúa.

## Cómo probarlo

```bash
cargo test -p rgrep
cargo clippy -p rgrep -- -D warnings
```

## Mejoras propuestas

1. Soportar expresiones regulares con el crate `regex` (`--regex`).
2. Mostrar líneas de contexto antes y después (`-A`, `-B`, `-C`).
3. Resaltar la coincidencia con colores (crate `colored` o secuencias ANSI).
4. Respetar `.gitignore` (crate `ignore`) y buscar en paralelo con `rayon`.
5. Leer de `stdin` cuando no se pasen rutas, para usarlo en pipes.
6. Añadir un benchmark con `criterion` sobre un archivo grande.
