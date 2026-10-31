//! Crate de demostración de módulos, visibilidad y paths.
//!
//! La raíz de una biblioteca es `src/lib.rs`. Aquí se declaran los módulos:
//! - `pub mod geometria;` -> el código está en `src/geometria.rs`
//! - `pub mod restaurante { ... }` -> módulo en línea

pub mod geometria;

pub mod restaurante {
    pub mod cocina {
        /// Visible para todos.
        pub fn preparar(plato: &str) -> String {
            format!("Preparando {plato} con {}", guarnicion())
        }

        /// Visible solo dentro de este crate.
        pub(crate) fn ingredientes() -> Vec<&'static str> {
            vec!["tomate", "cebolla", "ajo"]
        }

        // Privada: solo se puede usar dentro de `cocina`
        fn guarnicion() -> &'static str {
            "patatas"
        }
    }

    pub mod sala {
        use super::cocina; // `super` = el módulo padre (`restaurante`)

        pub fn servir(plato: &str) -> String {
            format!("{} -> servido", cocina::preparar(plato))
        }

        pub fn menu() -> String {
            cocina::ingredientes().join(", ")
        }
    }

    /// Un struct público con un campo privado: se construye con `new`.
    pub struct Cuenta {
        pub mesa: u32,
        total: f64,
    }

    impl Cuenta {
        pub fn new(mesa: u32) -> Self {
            Self { mesa, total: 0.0 }
        }

        pub fn agregar(&mut self, precio: f64) {
            self.total += precio;
        }

        pub fn total(&self) -> f64 {
            self.total
        }
    }
}

// Re-exportación: `modulos_demo::servir` en lugar de `modulos_demo::restaurante::sala::servir`
pub use restaurante::sala::servir;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sirve_un_plato() {
        assert_eq!(servir("paella"), "Preparando paella con patatas -> servido");
    }

    #[test]
    fn la_cuenta_suma() {
        let mut c = restaurante::Cuenta::new(3);
        c.agregar(10.5);
        c.agregar(4.5);
        assert_eq!(c.total(), 15.0);
    }
}
