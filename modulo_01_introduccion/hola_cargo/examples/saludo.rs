// Ejemplo ejecutable: cargo run --example saludo
// Los archivos en `examples/` son binarios extra que no forman parte del paquete principal.

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        println!("Uso: cargo run --example saludo -- <nombre>");
    } else {
        println!("¡Hola, {}!", args.join(" "));
    }
}
