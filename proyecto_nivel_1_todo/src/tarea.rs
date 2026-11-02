#[derive(Debug, Clone, PartialEq)]
pub struct Tarea {
    pub id: u32,
    pub titulo: String,
    pub hecha: bool,
}

impl Tarea {
    pub fn new(id: u32, titulo: String) -> Self {
        Self {
            id,
            titulo,
            hecha: false,
        }
    }

    /// Formato en disco: `id|hecha|titulo` (hecha es 0 o 1).
    pub fn a_linea(&self) -> String {
        format!("{}|{}|{}", self.id, u8::from(self.hecha), self.titulo)
    }

    /// Devuelve `None` si la línea está mal formada.
    pub fn desde_linea(linea: &str) -> Option<Tarea> {
        let mut partes = linea.splitn(3, '|');
        let id = partes.next()?.parse().ok()?;
        let hecha = partes.next()? == "1";
        let titulo = partes.next()?.to_string();
        Some(Tarea { id, titulo, hecha })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ida_y_vuelta() {
        let mut t = Tarea::new(3, "comprar pan | y leche".to_string());
        t.hecha = true;
        assert_eq!(Tarea::desde_linea(&t.a_linea()), Some(t));
    }

    #[test]
    fn lineas_invalidas() {
        assert_eq!(Tarea::desde_linea(""), None);
        assert_eq!(Tarea::desde_linea("x|0|titulo"), None);
        assert_eq!(Tarea::desde_linea("1|0"), None);
    }
}
