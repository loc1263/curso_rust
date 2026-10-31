// cargo run --example modulos_inline
// Módulos en línea, `use`, `as`, `self`, `super`, imports anidados y glob.

use std::collections::{HashMap, HashSet}; // import anidado
use std::fmt::Result as FmtResult; // alias con `as`

mod banco {
    pub mod cuentas {
        #[derive(Debug)]
        pub struct Cuenta {
            pub titular: String,
            saldo: f64, // privado fuera de `cuentas`
        }

        impl Cuenta {
            pub fn new(titular: &str, saldo: f64) -> Self {
                Self { titular: titular.to_string(), saldo }
            }

            pub fn saldo(&self) -> f64 {
                self.saldo
            }
        }

        pub fn con_comision(c: &Cuenta) -> f64 {
            c.saldo() - super::comision() // `super::` accede al módulo padre
        }
    }

    // Privada para el exterior, pero visible para los hijos de `banco`
    fn comision() -> f64 {
        1.5
    }

    pub fn resumen(c: &cuentas::Cuenta) -> String {
        format!("{} tiene {:.2}", c.titular, c.saldo())
    }
}

use banco::cuentas::{self, Cuenta}; // `self` importa el propio módulo `cuentas`

mod prelude {
    pub use super::banco::resumen;
    pub const MONEDA: &str = "EUR";
}
use prelude::*; // glob: importa todo lo público (úsalo con moderación)

fn _alias_de_tipo() -> FmtResult {
    Ok(())
}

fn main() {
    let c = Cuenta::new("Ana", 100.0);
    println!("{}", resumen(&c)); // llega por el glob `prelude::*`
    println!("con comisión: {:.2} {MONEDA}", cuentas::con_comision(&c));

    let mut mapa: HashMap<&str, i32> = HashMap::new();
    mapa.insert("a", 1);
    let conjunto: HashSet<i32> = [1, 2, 2, 3].into_iter().collect();
    println!("{mapa:?} {}", conjunto.len());

    // Paths absolutos dentro del crate: crate::banco::resumen(...)
    println!("{}", crate::banco::resumen(&c));
}
