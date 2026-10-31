// cargo run --example operador_pregunta
// El operador `?` propaga el error al llamador en vez de escribir un match.

use std::error::Error;
use std::fs;
use std::num::ParseIntError;

// `?` sobre Result: si es Err, hace `return Err(e.into())`
fn duplicar(s: &str) -> Result<i32, ParseIntError> {
    let n: i32 = s.trim().parse()?;
    Ok(n * 2)
}

fn sumar_cadenas(a: &str, b: &str) -> Result<i32, ParseIntError> {
    Ok(a.parse::<i32>()? + b.parse::<i32>()?)
}

// Box<dyn Error> acepta cualquier tipo de error (io::Error y ParseIntError aquí)
fn leer_numero(ruta: &str) -> Result<i32, Box<dyn Error>> {
    let contenido = fs::read_to_string(ruta)?;
    let n = contenido.trim().parse::<i32>()?;
    Ok(n)
}

// `?` también funciona con Option
fn inicial_mayuscula(s: &str) -> Option<char> {
    let c = s.chars().next()?;
    Some(c.to_ascii_uppercase())
}

fn main() {
    println!("{:?}", duplicar(" 21 "));
    println!("{:?}", duplicar("x"));
    println!("{:?}", sumar_cadenas("2", "3"));
    println!("{:?}", sumar_cadenas("2", "tres"));
    println!("{:?} {:?}", inicial_mayuscula("rust"), inicial_mayuscula(""));

    // Crea un archivo temporal para probar los dos caminos de error
    let ruta = std::env::temp_dir().join("m09_numero.txt");
    fs::write(&ruta, "41\n").unwrap();
    println!("{:?}", leer_numero(ruta.to_str().unwrap()).map(|n| n + 1));
    fs::write(&ruta, "no es número").unwrap();
    match leer_numero(ruta.to_str().unwrap()) {
        Ok(n) => println!("{n}"),
        Err(e) => println!("error de parseo: {e}"),
    }
    fs::remove_file(&ruta).unwrap();
    match leer_numero(ruta.to_str().unwrap()) {
        Ok(n) => println!("{n}"),
        Err(e) => println!("error de E/S: {e}"),
    }
}
