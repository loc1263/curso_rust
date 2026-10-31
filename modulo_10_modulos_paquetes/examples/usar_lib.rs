// cargo run --example usar_lib
// Usar un crate (aquí, la biblioteca de este mismo paquete) desde fuera.

use modulos_demo::geometria::circulo::Circulo;
use modulos_demo::restaurante::{self, Cuenta};
use modulos_demo::{geometria, servir}; // `servir` viene de la re-exportación

fn main() {
    println!("{}", servir("tortilla"));
    println!("{}", restaurante::sala::menu());
    // restaurante::cocina::guarnicion(); // error: función privada
    // restaurante::cocina::ingredientes(); // error: pub(crate) no es visible fuera del crate

    let mut cuenta = Cuenta::new(7);
    cuenta.agregar(12.0);
    cuenta.agregar(3.5);
    println!("mesa {} -> {:.2} €", cuenta.mesa, cuenta.total());
    // cuenta.total = 0.0; // error: el campo es privado

    let c = Circulo { centro: (0.0, 0.0), radio: 2.0 };
    println!("área = {:.3}", c.area());
    println!("contiene (1,1): {}", c.contiene((1.0, 1.0)));
    println!("distancia = {}", geometria::distancia((0.0, 0.0), (3.0, 4.0)));
}
