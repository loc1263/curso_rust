// cargo run -p m15_closures --example parametros_retornos
// Closures como parámetros, como valores devueltos y almacenados en structs.

use std::collections::HashMap;

// Parámetro genérico: despacho estático, sin coste
fn aplicar<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

// `impl Fn`: igual, más corto
fn aplicar_dos_veces(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

// FnMut: el closure puede cambiar su estado interno
fn repetir(mut f: impl FnMut(), veces: u32) {
    for _ in 0..veces {
        f();
    }
}

// FnOnce: se ejecuta como mucho una vez (puede consumir sus capturas)
fn ejecutar_una_vez<F: FnOnce() -> String>(f: F) -> String {
    f()
}

// Devolver un closure: `impl Fn` si siempre es el mismo closure...
fn sumador(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

// ...y `Box<dyn Fn>` si pueden ser distintos
fn operacion(nombre: &str) -> Box<dyn Fn(f64, f64) -> f64> {
    match nombre {
        "suma" => Box::new(|a, b| a + b),
        "mult" => Box::new(|a, b| a * b),
        _ => Box::new(|_, _| f64::NAN),
    }
}

// Closures dentro de un struct
struct Cache<F: Fn(u64) -> u64> {
    calculo: F,
    valores: HashMap<u64, u64>,
}
impl<F: Fn(u64) -> u64> Cache<F> {
    fn nueva(calculo: F) -> Self {
        Cache { calculo, valores: HashMap::new() }
    }
    fn obtener(&mut self, k: u64) -> u64 {
        if let Some(v) = self.valores.get(&k) {
            println!("  (cache) {k}");
            return *v;
        }
        let v = (self.calculo)(k);
        self.valores.insert(k, v);
        v
    }
}

// Tabla de funciones: Vec de closures con distinto cuerpo pero misma firma
fn pipeline() -> Vec<Box<dyn Fn(i32) -> i32>> {
    vec![Box::new(|x| x + 1), Box::new(|x| x * 10), Box::new(|x| x - 3)]
}

fn main() {
    println!("{}", aplicar(|x| x * x, 7));
    println!("{}", aplicar_dos_veces(|x| x + 3, 10));
    println!("{}", aplicar(i32::abs, -5)); // una función normal también vale

    let mut llamadas = 0;
    repetir(|| llamadas += 1, 5);
    println!("llamadas = {llamadas}");

    let texto = String::from("consumido");
    println!("{}", ejecutar_una_vez(move || texto));

    let suma5 = sumador(5);
    println!("{} {}", suma5(1), suma5(10));

    println!("{} {}", operacion("suma")(2.0, 3.0), operacion("mult")(2.0, 3.0));

    let mut c = Cache::nueva(|n| {
        println!("  calculando {n}²...");
        n * n
    });
    println!("{}", c.obtener(4));
    println!("{}", c.obtener(4));

    let resultado = pipeline().iter().fold(1, |acc, f| f(acc));
    println!("pipeline(1) = {resultado}");

    // Ordenar con closures
    let mut palabras = vec!["manzana", "pera", "kiwi", "fresa"];
    palabras.sort_by_key(|p| p.len());
    println!("{palabras:?}");
    palabras.sort_by(|a, b| b.cmp(a));
    println!("{palabras:?}");
}
