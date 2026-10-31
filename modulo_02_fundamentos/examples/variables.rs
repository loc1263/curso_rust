// cargo run --example variables
// Variables, mutabilidad, shadowing y constantes.

const MAX_PUNTOS: u32 = 100_000; // siempre inmutable, tipo obligatorio, valor conocido en compilación
static SALUDO: &str = "hola"; // vive durante todo el programa

fn main() {
    // Inmutable por defecto
    let x = 5;
    println!("x = {x}");
    // x = 6; // error[E0384]: cannot assign twice to immutable variable

    // Mutable con `mut`
    let mut contador = 0;
    contador += 1;
    contador *= 10;
    println!("contador = {contador}");

    // Shadowing: se crea una variable NUEVA con el mismo nombre
    let z = 5;
    let z = z * 2;
    {
        let z = z + 1; // solo vive dentro del bloque
        println!("z interno = {z}");
    }
    println!("z externo = {z}");

    // El shadowing permite cambiar el tipo (mut no)
    let espacios = "   ";
    let espacios = espacios.len();
    println!("espacios = {espacios}");

    println!("{MAX_PUNTOS} {SALUDO}");

    // Anotación de tipo explícita
    let ratio: f64 = 0.75;
    println!("ratio = {ratio}");

    // Variables que empiezan con _ no generan aviso si no se usan
    let _sin_usar = 42;
}
