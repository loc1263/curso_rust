//! Núcleo de `rgrep`: búsqueda de texto en líneas, archivos y directorios.
//!
//! La lógica vive en la biblioteca (fácil de probar) y `main.rs` solo se ocupa de la CLI.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errores de la biblioteca (tipados, porque es una lib: ver módulo 18).
#[derive(Debug, Error)]
pub enum ErrorBusqueda {
    #[error("no se pudo leer '{ruta}': {fuente}")]
    Lectura {
        ruta: PathBuf,
        #[source]
        fuente: io::Error,
    },
    #[error("el patrón no puede estar vacío")]
    PatronVacio,
}

/// Opciones de búsqueda.
#[derive(Debug, Clone, Default)]
pub struct Opciones {
    pub ignorar_mayusculas: bool,
    pub invertir: bool,
    pub palabra_completa: bool,
}

/// Una línea que coincide.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coincidencia {
    pub numero_linea: usize,
    pub linea: String,
}

/// Buscador con el patrón ya preparado.
pub struct Buscador {
    patron: String,
    opciones: Opciones,
}

impl Buscador {
    pub fn nuevo(patron: &str, opciones: Opciones) -> Result<Self, ErrorBusqueda> {
        if patron.is_empty() {
            return Err(ErrorBusqueda::PatronVacio);
        }
        let patron = if opciones.ignorar_mayusculas {
            patron.to_lowercase()
        } else {
            patron.to_string()
        };
        Ok(Buscador { patron, opciones })
    }

    /// ¿Esta línea contiene el patrón (según las opciones)?
    pub fn coincide(&self, linea: &str) -> bool {
        let encontrada = if self.opciones.ignorar_mayusculas {
            self.contiene(&linea.to_lowercase())
        } else {
            self.contiene(linea)
        };
        encontrada != self.opciones.invertir
    }

    fn contiene(&self, texto: &str) -> bool {
        if !self.opciones.palabra_completa {
            return texto.contains(&self.patron);
        }
        // Palabra completa: el patrón debe estar rodeado de caracteres que no sean alfanuméricos
        let es_palabra = |c: char| c.is_alphanumeric() || c == '_';
        texto.match_indices(&self.patron).any(|(i, m)| {
            let antes = texto[..i]
                .chars()
                .next_back()
                .is_none_or(|c| !es_palabra(c));
            let despues = texto[i + m.len()..]
                .chars()
                .next()
                .is_none_or(|c| !es_palabra(c));
            antes && despues
        })
    }

    /// Busca en un texto completo.
    pub fn buscar_en_texto(&self, texto: &str) -> Vec<Coincidencia> {
        texto
            .lines()
            .enumerate()
            .filter(|(_, l)| self.coincide(l))
            .map(|(i, l)| Coincidencia {
                numero_linea: i + 1,
                linea: l.to_string(),
            })
            .collect()
    }

    /// Busca en un archivo. Los archivos que no son texto UTF-8 se informan como error de lectura.
    pub fn buscar_en_archivo(&self, ruta: &Path) -> Result<Vec<Coincidencia>, ErrorBusqueda> {
        let texto = fs::read_to_string(ruta).map_err(|fuente| ErrorBusqueda::Lectura {
            ruta: ruta.to_path_buf(),
            fuente,
        })?;
        Ok(self.buscar_en_texto(&texto))
    }
}

/// Lista archivos de forma recursiva (ordenados para tener una salida estable).
/// `extension` limita a archivos con esa extensión (sin el punto).
pub fn listar_archivos(raiz: &Path, extension: Option<&str>) -> io::Result<Vec<PathBuf>> {
    let mut resultado = Vec::new();
    if raiz.is_file() {
        resultado.push(raiz.to_path_buf());
        return Ok(resultado);
    }
    let mut pendientes = vec![raiz.to_path_buf()];
    while let Some(dir) = pendientes.pop() {
        for entrada in fs::read_dir(&dir)? {
            let ruta = entrada?.path();
            if ruta.is_dir() {
                pendientes.push(ruta);
            } else if extension.is_none_or(|e| ruta.extension().is_some_and(|x| x == e)) {
                resultado.push(ruta);
            }
        }
    }
    resultado.sort();
    Ok(resultado)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXTO: &str = "Rust es rápido\nrusty metal\nPython es lento\nLa herramienta rust\n";

    fn buscador(patron: &str, op: Opciones) -> Buscador {
        Buscador::nuevo(patron, op).unwrap()
    }

    #[test]
    fn distingue_mayusculas_por_defecto() {
        let r = buscador("rust", Opciones::default()).buscar_en_texto(TEXTO);
        let lineas: Vec<usize> = r.iter().map(|c| c.numero_linea).collect();
        assert_eq!(lineas, vec![2, 4]);
    }

    #[test]
    fn ignorar_mayusculas() {
        let op = Opciones {
            ignorar_mayusculas: true,
            ..Default::default()
        };
        let r = buscador("RUST", op).buscar_en_texto(TEXTO);
        assert_eq!(r.len(), 3);
        assert_eq!(r[0].linea, "Rust es rápido");
    }

    #[test]
    fn invertir_devuelve_las_que_no_coinciden() {
        let op = Opciones {
            invertir: true,
            ..Default::default()
        };
        let r = buscador("es", op).buscar_en_texto(TEXTO);
        assert_eq!(r.len(), 2);
    }

    #[test]
    fn palabra_completa_descarta_rusty() {
        let op = Opciones {
            palabra_completa: true,
            ignorar_mayusculas: true,
            ..Default::default()
        };
        let r = buscador("rust", op).buscar_en_texto(TEXTO);
        let lineas: Vec<usize> = r.iter().map(|c| c.numero_linea).collect();
        assert_eq!(lineas, vec![1, 4]);
    }

    #[test]
    fn patron_vacio_es_error() {
        assert!(matches!(
            Buscador::nuevo("", Opciones::default()),
            Err(ErrorBusqueda::PatronVacio)
        ));
    }

    #[test]
    fn archivo_inexistente_da_error_de_lectura() {
        let b = buscador("x", Opciones::default());
        let e = b.buscar_en_archivo(Path::new("no/existe.txt")).unwrap_err();
        assert!(matches!(e, ErrorBusqueda::Lectura { .. }));
        assert!(e.to_string().contains("no/existe.txt"));
    }

    #[test]
    fn unicode_con_ignorar_mayusculas() {
        let op = Opciones {
            ignorar_mayusculas: true,
            ..Default::default()
        };
        let r = buscador("ÁRBOL", op).buscar_en_texto("el árbol\notro");
        assert_eq!(r.len(), 1);
    }
}
