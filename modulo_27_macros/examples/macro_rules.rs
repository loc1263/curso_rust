// cargo run -p m27_macros --example macro_rules
// Macros declarativas: patrones, repeticiones y recursión. (Definiciones en src/lib.rs)

use m27_macros::{calcular, contar, enum_con_nombres, mapa, maximo, struct_con_nuevo};
use std::collections::HashMap;

enum_con_nombres! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Color { Rojo, Verde, Azul }
}

struct_con_nuevo! {
    pub struct Persona { nombre: String, edad: u8 }
}

// Una macro local, definida justo donde se usa
macro_rules! mostrar {
    ( $( $e:expr ),* ) => {
        $( println!("{:<28} = {:?}", stringify!($e), $e); )*
    };
}

// Macro con fragmentos de distinto tipo: ident, ty, expr, tt, literal, block, pat
macro_rules! funcion_constante {
    ($nombre:ident -> $tipo:ty = $valor:expr) => {
        fn $nombre() -> $tipo {
            $valor
        }
    };
}
funcion_constante!(pi_aprox -> f64 = 3.14159);
funcion_constante!(saludo -> &'static str = "hola");

fn main() {
    let m: HashMap<&str, i32> = mapa! { "uno" => 1, "dos" => 2, "tres" => 3 };
    let vacio: HashMap<i32, i32> = mapa!();
    println!("mapa tiene {} entradas, vacío tiene {}", m.len(), vacio.len());

    mostrar!(maximo!(3), maximo!(3, 9, 5), maximo!(1.5, 0.5));
    mostrar!(contar!(a b c d e), contar!());
    mostrar!(calcular!(2 mas 3), calcular!(2 por 3 mas 4), calcular!(1 mas 2 mas 3 por 4));

    println!("\nenum generado: {:?}", Color::TODAS);
    for c in Color::TODAS {
        print!("{c} ");
    }
    println!();

    let p = Persona::nuevo("Ana".into(), 30);
    println!("{p:?}");

    println!("{} {}", pi_aprox(), saludo());

    // Macros de la std que ya usas: vec!, println!, format!, assert!, matches!, dbg!, todo!...
    println!("{}", matches!(Color::Rojo, Color::Rojo | Color::Verde));
    println!("{}", stringify!(a + b * 2)); // convierte los tokens en texto
    println!("{} línea {}", file!(), line!());

    // Higiene: las variables de la macro no chocan con las del usuario
    let a = 100;
    println!("maximo con variable 'a' del usuario: {}", maximo!(a, 7));
}
