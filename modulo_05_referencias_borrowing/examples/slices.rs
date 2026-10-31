// cargo run --example slices
// Slices: vistas a una porción contigua de datos, sin copiar.

fn primera_palabra(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    s
}

fn suma(v: &[i32]) -> i32 {
    v.iter().sum()
}

fn main() {
    let frase = String::from("hola mundo cruel");

    // &str: slice de texto
    println!("{}", &frase[0..4]);
    println!("{}", &frase[5..10]);
    println!("{}", primera_palabra(&frase));

    // Un literal ya es un &str
    let literal: &str = "literal";
    println!("{}", primera_palabra(literal));

    // Con una variable String, `&frase` se convierte solo a &str (deref coercion)
    // Por eso las funciones deben recibir &str y no &String.

    // &[T]: slice de arrays y vectores
    let a = [1, 2, 3, 4, 5];
    println!("{:?}", &a[1..3]); // [2, 3]
    println!("{:?}", &a[..2]); // [1, 2]
    println!("{:?}", &a[3..]); // [4, 5]
    println!("{}", suma(&a));
    println!("{}", suma(&a[1..4]));

    let v = vec![10, 20, 30];
    println!("{}", suma(&v));

    // Cuidado: los índices de un &str son bytes, no caracteres.
    // &"ñandú"[0..1] -> panic: no cae en un límite de carácter UTF-8
}
