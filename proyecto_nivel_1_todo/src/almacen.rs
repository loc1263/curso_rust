use crate::tarea::Tarea;
use std::fs;
use std::io;
use std::path::Path;

/// Lee las tareas del archivo. Si no existe todavía, devuelve una lista vacía.
pub fn cargar(ruta: &Path) -> io::Result<Vec<Tarea>> {
    match fs::read_to_string(ruta) {
        Ok(contenido) => Ok(contenido.lines().filter_map(Tarea::desde_linea).collect()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(e),
    }
}

pub fn guardar(ruta: &Path, tareas: &[Tarea]) -> io::Result<()> {
    let contenido: String = tareas.iter().map(|t| t.a_linea() + "\n").collect();
    fs::write(ruta, contenido)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guarda_y_carga() {
        let ruta = std::env::temp_dir().join("todo_test_guarda_y_carga.txt");
        let tareas = vec![
            Tarea::new(1, "a".to_string()),
            Tarea::new(2, "b".to_string()),
        ];
        guardar(&ruta, &tareas).unwrap();
        assert_eq!(cargar(&ruta).unwrap(), tareas);
        fs::remove_file(&ruta).unwrap();
    }

    #[test]
    fn archivo_inexistente_es_lista_vacia() {
        let ruta = std::env::temp_dir().join("todo_test_no_existe.txt");
        let _ = fs::remove_file(&ruta);
        assert!(cargar(&ruta).unwrap().is_empty());
    }
}
