// cargo run --example strings
// String (propietario, crece) vs &str (vista). Todo es UTF-8.

fn main() {
    // Crear
    let a = String::new();
    let b = String::from("hola");
    let c = "mundo".to_string();
    println!("'{a}' '{b}' '{c}'");

    // Modificar
    let mut s = String::from("Hola");
    s.push(',');
    s.push_str(" Rust");
    println!("{s}");

    // Concatenar
    let s1 = String::from("uno");
    let s2 = String::from("dos");
    let s3 = s1 + "-" + &s2; // s1 se mueve; s2 sigue disponible
    println!("{s3} {s2}");
    let s4 = format!("{s2}/{s3}/{}", 3); // no mueve nada
    println!("{s4}");

    // UTF-8: len() cuenta bytes, no caracteres
    let es = "ñandú";
    println!("bytes = {}, chars = {}", es.len(), es.chars().count());
    // No se puede indexar con es[0]; se recorre:
    for c in es.chars() {
        print!("{c} ");
    }
    println!();
    for (i, c) in es.char_indices() {
        print!("{i}:{c} ");
    }
    println!();
    println!("{:?}", es.bytes().take(3).collect::<Vec<u8>>());

    // Métodos comunes
    let texto = "  Aprender Rust es divertido  ";
    println!("[{}]", texto.trim());
    println!("{}", texto.to_uppercase());
    println!("{}", texto.contains("Rust"));
    println!("{}", texto.trim().starts_with("Aprender"));
    println!("{}", texto.replace("divertido", "genial"));
    println!("{:?}", texto.split_whitespace().collect::<Vec<_>>());
    println!("{:?}", "a,b,c".split(',').collect::<Vec<_>>());
    println!("{}", ["x", "y", "z"].join("-"));
    println!("{:?}", "l1\nl2\nl3".lines().collect::<Vec<_>>());
    println!("{:?}", "texto".find('x'));

    // Conversión desde y hacia números
    let n: i32 = "123".parse().unwrap();
    println!("{}", n + 1);
    println!("{}", 3.5.to_string() + "!");

    // Invertir
    let al_reves: String = "rust".chars().rev().collect();
    println!("{al_reves}");
}
