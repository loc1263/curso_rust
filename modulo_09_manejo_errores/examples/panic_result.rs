// cargo run --example panic_result
// cargo run --example panic_result -- --panic     (para ver un panic real)
//
// panic! = error irrecuperable. Result<T, E> = error recuperable.

use std::fs::File;
use std::io::ErrorKind;

fn dividir(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("división entre cero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    if std::env::args().any(|a| a == "--panic") {
        let v = vec![1, 2, 3];
        println!("a punto de fallar...");
        println!("{}", v[99]); // thread 'main' panicked: index out of bounds
    }

    // match sobre Result
    match dividir(10, 2) {
        Ok(r) => println!("10 / 2 = {r}"),
        Err(e) => println!("error: {e}"),
    }
    match dividir(1, 0) {
        Ok(r) => println!("{r}"),
        Err(e) => println!("error: {e}"),
    }

    // Combinadores
    let a = "12".parse::<i32>();
    let b = "doce".parse::<i32>();
    println!("{a:?}");
    println!("{b:?}");
    println!("unwrap_or = {}", b.clone().unwrap_or(0));
    println!("unwrap_or_else = {}", b.clone().unwrap_or_else(|_| -1));
    println!("map = {:?}", a.clone().map(|n| n * 2));
    println!(
        "map_err = {:?}",
        b.clone().map_err(|e| format!("falló: {e}"))
    );
    println!("ok() = {:?}", b.clone().ok());
    println!("is_ok = {}, is_err = {}", a.is_ok(), b.is_err());

    // Distinguir tipos de error de E/S
    match File::open("no_existe.txt") {
        Ok(_) => println!("abierto"),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => println!("el archivo no existe"),
            ErrorKind::PermissionDenied => println!("sin permisos"),
            otro => println!("otro error: {otro:?}"),
        },
    }

    // expect: panic con mensaje propio (útil para invariantes que "no deberían fallar")
    let n: i32 = "7".parse().expect("el literal es un número válido");
    println!("n = {n}");
}
