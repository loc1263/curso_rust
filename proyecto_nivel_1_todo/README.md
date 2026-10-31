# Proyecto integrador del Nivel 1 — CLI de tareas (TODO)

Aplicación de línea de comandos que guarda tareas en un archivo de texto. Solo usa la biblioteca estándar.

## Qué practica cada módulo

| Módulo | Dónde se ve |
|--------|-------------|
| 1 Introducción | `cargo new`, `cargo run`, `cargo test` |
| 2 Fundamentos | tipos `u32`, `bool`, constantes (`AYUDA`) |
| 3 Funciones y control de flujo | `ejecutar`, `leer_id`, bucles `for` en `listar` |
| 4 Ownership | `Tarea` es dueña de su `String`; `tareas` se mueve y se presta |
| 5 Referencias | `&[Tarea]`, `&mut` con `iter_mut`, `&Path` |
| 6 Structs y métodos | `Tarea` con `new`, `a_linea`, `desde_linea` |
| 7 Enums y matching | `match` sobre el comando, `Option` en `desde_linea` |
| 8 Colecciones | `Vec<Tarea>`, `String`, `join`, `retain` |
| 9 Errores | `Result<_, String>`, `?`, `map_err`, `io::Result` |
| 10 Módulos | `mod tarea;` y `mod almacen;` en archivos propios |

## Uso

```bash
cargo run -- add Comprar pan
cargo run -- add Estudiar ownership
cargo run -- list
cargo run -- done 1
cargo run -- remove 2
cargo run -- help
```

O compila una vez y usa el binario: `cargo build --release` y luego `target/release/todo list`.

Para usar otro archivo de datos: `TODO_FILE=otra.txt cargo run -- list` (en PowerShell: `$env:TODO_FILE="otra.txt"`).

## Formato del archivo

Una tarea por línea: `id|hecha|título`, por ejemplo:

```
1|1|Comprar pan
2|0|Estudiar ownership
```

## Tests

```bash
cargo test -p todo
```

## Retos para ampliarlo

1. Añade el comando `edit <id> <nuevo título>`.
2. Añade `list --pendientes` y `list --hechas`.
3. Añade prioridades (`enum Prioridad { Baja, Media, Alta }`) y ordena por prioridad.
4. Añade una fecha límite como `String` en formato `AAAA-MM-DD` y valida el formato.
5. Cambia el tipo de error `String` por un `enum ErrorTodo` con `Display` y `From<io::Error>`.
6. Escribe tests de integración en `tests/` que ejecuten el binario.
