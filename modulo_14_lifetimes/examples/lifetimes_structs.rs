// cargo run -p m14_lifetimes --example lifetimes_structs
// Structs que contienen referencias, iteradores que prestan datos y bounds de lifetime.

#![allow(dead_code)]

use std::fmt::Display;

// Un struct con una referencia necesita un lifetime: no puede vivir más que el dato prestado
#[derive(Debug)]
struct Extracto<'a> {
    parte: &'a str,
}

struct Parser<'a> {
    entrada: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn nuevo(entrada: &'a str) -> Self {
        Parser { entrada, pos: 0 }
    }

    // Devuelve un trozo de `entrada` (lifetime 'a), no de `&self`:
    // así el resultado puede sobrevivir al propio parser.
    fn siguiente_palabra(&mut self) -> Option<&'a str> {
        let resto = &self.entrada[self.pos..];
        let resto_trim = resto.trim_start();
        if resto_trim.is_empty() {
            return None;
        }
        let inicio = self.pos + (resto.len() - resto_trim.len());
        let fin = resto_trim
            .find(char::is_whitespace)
            .map_or(self.entrada.len(), |i| inicio + i);
        self.pos = fin;
        Some(&self.entrada[inicio..fin])
    }
}

// Iterator que presta datos: Item = &'a str
impl<'a> Iterator for Parser<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        self.siguiente_palabra()
    }
}

// Lifetime + trait bound: T debe cumplir Display y la referencia vive 'a
fn anunciar<'a, T>(titulo: &'a str, valor: T) -> String
where
    T: Display + 'a,
{
    format!("{titulo}: {valor}")
}

// Struct con dos lifetimes independientes
struct Par<'a, 'b> {
    a: &'a str,
    b: &'b str,
}
impl<'a, 'b> Par<'a, 'b> {
    fn solo_a(&self) -> &'a str {
        self.a
    }
}

fn main() {
    let texto = String::from("Llamadme Ismael. Hace algunos años...");
    let primera_frase = texto.split('.').next().expect("hay al menos un trozo");
    let e = Extracto {
        parte: primera_frase,
    };
    println!("{e:?}");

    let palabras: Vec<&str>;
    {
        let mut p = Parser::nuevo(&texto);
        println!("primera palabra: {:?}", p.siguiente_palabra());
        // Las palabras viven tanto como `texto`, no como `p`
        palabras = Parser::nuevo(&texto).collect();
    }
    println!("{} palabras: {:?}", palabras.len(), palabras);

    println!("{}", anunciar("respuesta", 42));

    let largo = String::from("vida larga");
    let a_vivir;
    {
        let corto = String::from("x");
        let par = Par {
            a: &largo,
            b: &corto,
        };
        a_vivir = par.solo_a(); // solo depende de 'a, así que sobrevive a `corto`
    }
    println!("{a_vivir}");
}
