// Compilar: rustc 02_macros_println.rs
// `println!` es una macro (lleva `!`), no una función.

fn main() {
    let nombre = "Ferris";
    let edad = 10;

    println!("Hola");
    println!("Me llamo {}", nombre);
    println!("{} tiene {} años", nombre, edad);
    println!("{nombre} tiene {edad} años"); // variables en línea
    println!("{0} {1} {0}", "a", "b"); // argumentos posicionales
    println!("{:?}", (1, "dos", 3.0)); // formato Debug
    print!("sin salto de línea");
    println!();
    eprintln!("esto va a stderr");
}
