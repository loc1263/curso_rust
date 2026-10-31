// cargo run -p m12_traits --example impl_vs_dyn
// Despacho estático (`impl Trait`, genéricos) vs dinámico (`dyn Trait`).

use std::fmt::Display;

trait Figura {
    fn area(&self) -> f64;
    fn nombre(&self) -> &str;
}

struct Circulo(f64);
struct Rectangulo(f64, f64);

impl Figura for Circulo {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.0 * self.0
    }
    fn nombre(&self) -> &str {
        "círculo"
    }
}
impl Figura for Rectangulo {
    fn area(&self) -> f64 {
        self.0 * self.1
    }
    fn nombre(&self) -> &str {
        "rectángulo"
    }
}

// 1) `impl Trait` en argumento: despacho estático (se monomorfiza)
fn imprimir_area(f: &impl Figura) {
    println!("{} -> {:.2}", f.nombre(), f.area());
}

// 2) `impl Trait` en retorno: devuelve UN tipo concreto, pero oculto
fn crear_contador() -> impl Iterator<Item = u32> {
    (1..=3).map(|x| x * 10)
}

fn etiqueta(texto: impl Display) -> String {
    format!("<{texto}>")
}

// 3) `dyn Trait`: despacho dinámico, permite mezclar tipos distintos
fn area_total(figuras: &[Box<dyn Figura>]) -> f64 {
    figuras.iter().map(|f| f.area()).sum()
}

// `impl Trait` NO permite devolver tipos distintos según una condición;
// para eso hace falta Box<dyn Trait>.
fn elegir(circular: bool) -> Box<dyn Figura> {
    if circular {
        Box::new(Circulo(1.0))
    } else {
        Box::new(Rectangulo(2.0, 3.0))
    }
}

// Referencias a trait objects también valen
fn describir(f: &dyn Figura) -> String {
    format!("{} de área {:.1}", f.nombre(), f.area())
}

fn main() {
    imprimir_area(&Circulo(2.0));
    imprimir_area(&Rectangulo(2.0, 5.0));

    let v: Vec<u32> = crear_contador().collect();
    println!("{v:?} {}", etiqueta(42));

    // Colección heterogénea: solo posible con dyn
    let figuras: Vec<Box<dyn Figura>> = vec![
        Box::new(Circulo(1.0)),
        Box::new(Rectangulo(2.0, 3.0)),
        elegir(true),
        elegir(false),
    ];
    for f in &figuras {
        println!("- {}", describir(f.as_ref()));
    }
    println!("área total = {:.2}", area_total(&figuras));

    println!("\nResumen:");
    println!("  genéricos / impl Trait -> rápido, se inlinea, un tipo por instancia");
    println!("  dyn Trait              -> vtable, permite tipos mezclados, pequeño coste en llamadas");
}
