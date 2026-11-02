// cargo run --example option
// Option<T>: la forma de Rust de representar "puede no haber valor" (no existe null).

fn dividir(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

fn buscar(lista: &[&str], clave: &str) -> Option<usize> {
    lista.iter().position(|&x| x == clave)
}

fn main() {
    // match exhaustivo
    match dividir(10.0, 4.0) {
        Some(r) => println!("resultado = {r}"),
        None => println!("no se puede dividir"),
    }

    // if let: cuando solo importa un caso
    if let Some(r) = dividir(1.0, 0.0) {
        println!("{r}");
    } else {
        println!("división entre cero");
    }

    // Métodos útiles
    let a = dividir(9.0, 3.0);
    let b = dividir(9.0, 0.0);
    println!("{:?} {:?}", a, b);
    println!("is_some = {}, is_none = {}", a.is_some(), b.is_none());
    println!("unwrap_or = {}", b.unwrap_or(-1.0));
    println!("unwrap_or_default = {}", b.unwrap_or_default());
    println!("map = {:?}", a.map(|x| x * 2.0));
    println!("and_then = {:?}", a.and_then(|x| dividir(x, 0.0)));
    println!("ok_or = {:?}", b.ok_or("sin resultado"));

    // Option en la biblioteca estándar
    let nombres = vec!["ana", "luis", "eva"];
    println!("{:?} {:?}", nombres.first(), nombres.get(10));
    println!(
        "{:?} {:?}",
        buscar(&nombres, "luis"),
        buscar(&nombres, "zoe")
    );
    println!("{:?}", "42".parse::<i32>().ok());

    // unwrap() hace panic con None: úsalo solo si estás seguro o en pruebas
    // let x: Option<i32> = None;
    // x.unwrap();
}
