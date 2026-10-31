// cargo run -p m11_genericos --example monomorfizacion
// Los genéricos no cuestan nada en ejecución: el compilador genera una copia por tipo.

use std::any::type_name;
use std::mem::size_of;

fn info<T>() {
    println!("{:<24} ocupa {} bytes", type_name::<T>(), size_of::<T>());
}

// El compilador genera `duplicar::<i32>` y `duplicar::<f64>` como funciones distintas,
// exactamente como si las hubieras escrito a mano.
fn duplicar<T: std::ops::Add<Output = T> + Copy>(x: T) -> T {
    x + x
}

fn nombre_de<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn main() {
    println!("-- Tamaños de distintos tipos --");
    info::<u8>();
    info::<i32>();
    info::<f64>();
    info::<(u8, u32)>();
    info::<[u16; 4]>();
    info::<Option<Box<i32>>>(); // niche optimization: Option<Box<T>> == Box<T>
    info::<Box<i32>>();

    println!("\n-- Una función, varias instancias --");
    println!("{} -> {}", nombre_de(&2_i32), duplicar(2_i32));
    println!("{} -> {}", nombre_de(&2.5_f64), duplicar(2.5_f64));
    println!("{} -> {}", nombre_de(&7_u8), duplicar(7_u8));

    println!("\nCada llamada con un tipo distinto genera código especializado (monomorfización).");
    println!("Ventaja: velocidad igual a la del código escrito a mano.");
    println!("Coste: binarios más grandes y más tiempo de compilación.");
}
