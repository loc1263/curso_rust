mod almacen;
mod tarea;

use std::env;
use std::path::{Path, PathBuf};
use std::process;
use tarea::Tarea;

const AYUDA: &str = "\
Uso: todo <comando> [argumentos]

Comandos:
  add <título>     Añade una tarea
  list             Lista las tareas (comando por defecto)
  done <id>        Marca una tarea como hecha
  remove <id>      Elimina una tarea
  help             Muestra esta ayuda

El archivo de datos es `tareas.txt` (cámbialo con la variable TODO_FILE).";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let ruta = PathBuf::from(env::var("TODO_FILE").unwrap_or_else(|_| "tareas.txt".to_string()));

    if let Err(e) = ejecutar(&args, &ruta) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn ejecutar(args: &[String], ruta: &Path) -> Result<(), String> {
    let mut tareas = almacen::cargar(ruta)
        .map_err(|e| format!("no se pudo leer {}: {e}", ruta.display()))?;

    match args.first().map(String::as_str) {
        Some("add") => {
            let titulo = args[1..].join(" ");
            if titulo.trim().is_empty() {
                return Err("uso: add <título>".to_string());
            }
            let id = tareas.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            tareas.push(Tarea::new(id, titulo));
            guardar(ruta, &tareas)?;
            println!("Tarea {id} añadida");
        }
        Some("list") | None => listar(&tareas),
        Some("done") => {
            let id = leer_id(args)?;
            let tarea = tareas
                .iter_mut()
                .find(|t| t.id == id)
                .ok_or(format!("no existe la tarea {id}"))?;
            tarea.hecha = true;
            guardar(ruta, &tareas)?;
            println!("Tarea {id} completada");
        }
        Some("remove") => {
            let id = leer_id(args)?;
            let antes = tareas.len();
            tareas.retain(|t| t.id != id);
            if tareas.len() == antes {
                return Err(format!("no existe la tarea {id}"));
            }
            guardar(ruta, &tareas)?;
            println!("Tarea {id} eliminada");
        }
        Some("help") | Some("--help") | Some("-h") => println!("{AYUDA}"),
        Some(otro) => return Err(format!("comando desconocido '{otro}'\n\n{AYUDA}")),
    }
    Ok(())
}

fn leer_id(args: &[String]) -> Result<u32, String> {
    args.get(1)
        .ok_or("falta el id de la tarea".to_string())?
        .parse()
        .map_err(|_| "el id debe ser un número positivo".to_string())
}

fn guardar(ruta: &Path, tareas: &[Tarea]) -> Result<(), String> {
    almacen::guardar(ruta, tareas).map_err(|e| format!("no se pudo guardar {}: {e}", ruta.display()))
}

fn listar(tareas: &[Tarea]) {
    if tareas.is_empty() {
        println!("No hay tareas. Añade una con: todo add <título>");
        return;
    }
    for t in tareas {
        let marca = if t.hecha { "x" } else { " " };
        println!("[{marca}] {:>3}  {}", t.id, t.titulo);
    }
    let hechas = tareas.iter().filter(|t| t.hecha).count();
    println!("\n{hechas}/{} completadas", tareas.len());
}
