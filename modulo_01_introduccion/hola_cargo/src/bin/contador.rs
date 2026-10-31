// Segundo binario del mismo paquete: cargo run --bin contador
// Cada archivo en `src/bin/` genera su propio ejecutable.

fn main() {
    for i in 1..=5 {
        println!("Contando: {i}");
    }
}
