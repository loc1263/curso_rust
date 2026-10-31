// cargo run -p m12_traits --example traits_avanzados
// Supertraits, tipos asociados, implementaciones blanket y sobrecarga de operadores.

use std::fmt::{self, Display};
use std::ops::{Add, Mul};

// --- Supertrait: Descripcion exige que el tipo ya implemente Display ---
trait Descripcion: Display {
    fn descripcion(&self) -> String {
        format!("«{self}»")
    }
}

struct Libro(&'static str);
impl Display for Libro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Libro: {}", self.0)
    }
}
impl Descripcion for Libro {}

// --- Tipo asociado: cada implementación decide qué devuelve ---
trait Fuente {
    type Item;
    fn siguiente(&mut self) -> Option<Self::Item>;
}

struct Cuenta {
    actual: u32,
    fin: u32,
}
impl Fuente for Cuenta {
    type Item = u32;
    fn siguiente(&mut self) -> Option<u32> {
        if self.actual < self.fin {
            self.actual += 1;
            Some(self.actual)
        } else {
            None
        }
    }
}

fn consumir<F: Fuente>(mut f: F) -> Vec<F::Item> {
    let mut v = Vec::new();
    while let Some(x) = f.siguiente() {
        v.push(x);
    }
    v
}

// --- Blanket implementation: un trait para TODO tipo que cumpla un bound ---
trait Gritar {
    fn gritar(&self) -> String;
}
impl<T: Display> Gritar for T {
    fn gritar(&self) -> String {
        self.to_string().to_uppercase() + "!"
    }
}

// --- Sobrecarga de operadores con traits de std::ops ---
#[derive(Debug, Clone, Copy, PartialEq)]
struct V2 {
    x: f64,
    y: f64,
}
impl Add for V2 {
    type Output = V2;
    fn add(self, o: V2) -> V2 {
        V2 { x: self.x + o.x, y: self.y + o.y }
    }
}
impl Mul<f64> for V2 {
    type Output = V2;
    fn mul(self, k: f64) -> V2 {
        V2 { x: self.x * k, y: self.y * k }
    }
}

// --- Desambiguar cuando dos traits tienen el mismo método ---
trait Piloto {
    fn volar(&self) -> &'static str;
}
trait Mago {
    fn volar(&self) -> &'static str;
}
struct Humano;
impl Piloto for Humano {
    fn volar(&self) -> &'static str {
        "Capitán al habla"
    }
}
impl Mago for Humano {
    fn volar(&self) -> &'static str {
        "¡Arriba!"
    }
}

fn main() {
    println!("{}", Libro("Rust").descripcion());

    println!("{:?}", consumir(Cuenta { actual: 0, fin: 5 }));

    println!("{} / {}", "hola".gritar(), 42.gritar());

    let a = V2 { x: 1.0, y: 2.0 };
    let b = V2 { x: 0.5, y: 0.5 };
    println!("{:?}", (a + b) * 2.0);

    let h = Humano;
    println!("{} / {}", Piloto::volar(&h), <Humano as Mago>::volar(&h));
}
