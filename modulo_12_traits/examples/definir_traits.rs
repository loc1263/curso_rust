// cargo run -p m12_traits --example definir_traits
// Definir un trait, implementarlo, métodos por defecto y trait bounds.

trait Animal {
    // Método obligatorio
    fn nombre(&self) -> String;
    fn sonido(&self) -> String;

    // Método con implementación por defecto (se puede sobrescribir)
    fn presentarse(&self) -> String {
        format!("Soy {} y hago {}", self.nombre(), self.sonido())
    }
}

struct Perro;
struct Gato {
    nombre: String,
}

impl Animal for Perro {
    fn nombre(&self) -> String {
        "un perro".into()
    }
    fn sonido(&self) -> String {
        "guau".into()
    }
}

impl Animal for Gato {
    fn nombre(&self) -> String {
        self.nombre.clone()
    }
    fn sonido(&self) -> String {
        "miau".into()
    }
    // Sobrescribe el valor por defecto
    fn presentarse(&self) -> String {
        format!("{} te mira con desdén. ¡{}!", self.nombre, self.sonido())
    }
}

// Trait bound: acepta cualquier tipo que implemente Animal
fn saludar<A: Animal>(a: &A) {
    println!("{}", a.presentarse());
}

// Varios bounds con `+`
fn saludar_y_clonar<A: Animal + Clone>(a: &A) -> A {
    saludar(a);
    a.clone()
}

// Un trait con función asociada y constante
trait Forma {
    const LADOS: u32;
    fn area(&self) -> f64;
    fn nuevo_unitario() -> Self
    where
        Self: Sized;
}

struct Cuadrado(f64);
struct Triangulo(f64, f64);

impl Forma for Cuadrado {
    const LADOS: u32 = 4;
    fn area(&self) -> f64 {
        self.0 * self.0
    }
    fn nuevo_unitario() -> Self {
        Cuadrado(1.0)
    }
}

impl Forma for Triangulo {
    const LADOS: u32 = 3;
    fn area(&self) -> f64 {
        self.0 * self.1 / 2.0
    }
    fn nuevo_unitario() -> Self {
        Triangulo(1.0, 1.0)
    }
}

fn resumen<F: Forma>(f: &F) -> String {
    format!("{} lados, área {:.2}", F::LADOS, f.area())
}

// Implementar un trait propio para un tipo ajeno (permitido: el trait es nuestro)
trait Doble {
    fn doble(&self) -> Self;
}
impl Doble for i32 {
    fn doble(&self) -> i32 {
        self * 2
    }
}
impl Doble for String {
    fn doble(&self) -> String {
        format!("{self}{self}")
    }
}

#[derive(Clone)]
struct Loro;
impl Animal for Loro {
    fn nombre(&self) -> String {
        "un loro".into()
    }
    fn sonido(&self) -> String {
        "hola".into()
    }
}

fn main() {
    saludar(&Perro);
    saludar(&Gato { nombre: "Misu".into() });
    let _otro_loro = saludar_y_clonar(&Loro);

    println!("{}", resumen(&Cuadrado(3.0)));
    println!("{}", resumen(&Triangulo(3.0, 4.0)));
    println!("unitario: {}", resumen(&Cuadrado::nuevo_unitario()));

    println!("{} {}", 21.doble(), String::from("ab").doble());

    // Regla huérfana: no se puede `impl Display for Vec<i32>` (ni trait ni tipo son nuestros).
}
