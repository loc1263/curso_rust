// cargo run --example stack_heap
// Stack vs heap, tamaños y el trait Drop (liberación automática).

use std::mem::size_of;

struct Recurso(&'static str);

impl Drop for Recurso {
    fn drop(&mut self) {
        println!("  liberando {}", self.0);
    }
}

fn main() {
    // Tamaños en el stack
    println!("i32:    {} bytes", size_of::<i32>());
    println!("f64:    {} bytes", size_of::<f64>());
    println!("char:   {} bytes", size_of::<char>());
    println!("&str:   {} bytes (puntero + longitud)", size_of::<&str>());
    println!("String: {} bytes (puntero + longitud + capacidad)", size_of::<String>());

    // Datos en el heap
    let mut s = String::new();
    println!("len={} cap={}", s.len(), s.capacity());
    s.push_str("hola mundo");
    println!("len={} cap={}", s.len(), s.capacity());

    let caja = Box::new(42); // el entero vive en el heap
    println!("caja = {caja}");

    // Drop: al salir de ámbito se libera, en orden inverso a la creación
    println!("--- ámbitos ---");
    let _a = Recurso("a");
    {
        let _b = Recurso("b");
        let _c = Recurso("c");
        println!("  fin del bloque interno");
    }
    let d = Recurso("d");
    drop(d); // liberación anticipada
    println!("  después de drop(d)");
    println!("  fin de main");
}
