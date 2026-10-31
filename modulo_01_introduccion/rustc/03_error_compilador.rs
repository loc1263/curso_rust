// Este archivo NO compila a propósito: sirve para leer mensajes del compilador.
// Prueba: rustc 03_error_compilador.rs
// Luego corrige el error y vuelve a compilar.

fn main() {
    let x: i32 = "texto"; // error[E0308]: mismatched types
    println!("{x}");
}
