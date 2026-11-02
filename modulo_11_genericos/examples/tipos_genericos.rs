// cargo run -p m11_genericos --example tipos_genericos
// Structs, enums e impl genéricos; const generics básicos.

use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Punto<T> {
    x: T,
    y: T,
}

impl<T> Punto<T> {
    fn nuevo(x: T, y: T) -> Self {
        Punto { x, y }
    }
}

// impl solo para un tipo concreto: `distancia_origen` existe únicamente en Punto<f64>
impl Punto<f64> {
    fn distancia_origen(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// Métodos con sus propios parámetros genéricos
struct Mixto<A, B> {
    a: A,
    b: B,
}

impl<A, B> Mixto<A, B> {
    fn mezclar<C, D>(self, otro: Mixto<C, D>) -> Mixto<A, D> {
        Mixto {
            a: self.a,
            b: otro.b,
        }
    }
}

// Enum genérico propio (Option y Result de la std funcionan igual)
#[derive(Debug)]
enum Caja<T> {
    Vacia,
    Con(T),
}

// Una pila genérica
#[derive(Debug, Default)]
struct Pila<T> {
    elementos: Vec<T>,
}

impl<T> Pila<T> {
    fn nueva() -> Self {
        Pila {
            elementos: Vec::new(),
        }
    }
    fn apilar(&mut self, v: T) {
        self.elementos.push(v);
    }
    fn desapilar(&mut self) -> Option<T> {
        self.elementos.pop()
    }
    fn cima(&self) -> Option<&T> {
        self.elementos.last()
    }
}

// Implementación condicional: `mostrar` solo existe si T: Display
impl<T: Display> Pila<T> {
    fn mostrar(&self) -> String {
        let partes: Vec<String> = self.elementos.iter().map(|e| e.to_string()).collect();
        format!("[{}]", partes.join(" | "))
    }
}

// Const generics: el tamaño forma parte del tipo
fn suma_array<const N: usize>(arr: [i32; N]) -> i32 {
    arr.iter().sum()
}

fn main() {
    let entero = Punto::nuevo(1, 2);
    let real = Punto::nuevo(3.0, 4.0);
    println!("{entero:?} y {real:?}");
    println!(
        "distancia de {real:?} al origen = {}",
        real.distancia_origen()
    );
    // entero.distancia_origen(); // error: solo definido para Punto<f64>

    let m = Mixto { a: 1, b: "x" }.mezclar(Mixto { a: 2.5, b: 'z' });
    println!("mezcla: a = {}, b = {}", m.a, m.b);

    let c1: Caja<i32> = Caja::Vacia;
    let c2 = Caja::Con("dentro");
    println!("{c1:?} {c2:?}");

    let mut p = Pila::nueva();
    p.apilar("a");
    p.apilar("b");
    p.apilar("c");
    println!("pila = {}, cima = {:?}", p.mostrar(), p.cima());
    println!("desapilo {:?}", p.desapilar());
    println!("pila = {}", p.mostrar());

    println!("suma [1,2,3]     = {}", suma_array([1, 2, 3]));
    println!("suma [10; 5]     = {}", suma_array([10; 5]));
}
