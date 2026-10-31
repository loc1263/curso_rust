// cargo run --example funciones_ownership
// Pasar valores a funciones también mueve o copia.

fn toma_propiedad(s: String) {
    println!("tomé: {s}");
} // `s` sale de ámbito y se libera aquí

fn copia_entero(n: i32) {
    println!("copié: {n}");
}

fn crea() -> String {
    String::from("recién creada") // se devuelve y se mueve al llamador
}

fn pasa_y_devuelve(s: String) -> String {
    s
}

fn longitud(s: String) -> (String, usize) {
    let largo = s.len();
    (s, largo) // devolver ambos es incómodo: por eso existen las referencias (módulo 5)
}

fn main() {
    let s = String::from("hola");
    toma_propiedad(s);
    // println!("{s}"); // error: s fue movida

    let n = 5;
    copia_entero(n);
    println!("n sigue viva: {n}");

    let a = crea();
    println!("{a}");

    let b = pasa_y_devuelve(a);
    println!("{b}");

    let (c, largo) = longitud(b);
    println!("'{c}' mide {largo}");
}
