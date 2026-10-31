// cargo run -p m27_macros --example procedurales
// Macros procedurales: derive, attribute y function-like. (Código en macros_derive/src/lib.rs)

use m27_macros::{Describir, mayusculas, medir_tiempo};
use std::thread::sleep;
use std::time::Duration;

#[derive(Describir)]
#[allow(dead_code)]
struct Usuario {
    id: u32,
    nombre: String,
    activo: bool,
}

#[derive(Describir)]
#[allow(dead_code)]
enum Estado {
    Nuevo,
    EnCurso,
    Cerrado,
}

#[derive(Describir)]
#[allow(dead_code)]
struct Metros(f64);

#[derive(Describir)]
#[allow(dead_code)]
struct Caja<T> {
    contenido: T,
}

#[medir_tiempo]
fn trabajo_lento(n: u64) -> u64 {
    sleep(Duration::from_millis(30));
    if n == 0 {
        return 0; // `return` funciona gracias al closure interno
    }
    (1..=n).sum()
}

fn main() {
    println!("{}", Usuario::describir());
    println!("{}", Estado::describir());
    println!("{}", Metros::describir());
    println!("{}", Caja::<i32>::describir());

    println!("resultado = {}", trabajo_lento(100));
    println!("resultado = {}", trabajo_lento(0));

    // Se evalúa en compilación: en el binario ya está el literal "RUST EN MAYÚSCULAS"
    const T: &str = mayusculas!("rust en mayúsculas");
    println!("{T}");

    // Para ver el código que genera una macro:  cargo install cargo-expand && cargo expand --example procedurales
}
