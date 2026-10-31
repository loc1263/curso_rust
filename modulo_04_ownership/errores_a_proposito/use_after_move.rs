// NO compila a propósito. Prueba: rustc use_after_move.rs
// Lee el mensaje E0382 y arréglalo de dos maneras: con clone() y con una referencia.

fn consumir(s: String) {
    println!("{s}");
}

fn main() {
    let s = String::from("hola");
    consumir(s);
    println!("{s}");
}
