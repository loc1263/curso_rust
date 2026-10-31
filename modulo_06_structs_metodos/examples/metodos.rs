// cargo run --example metodos
// Métodos (&self, &mut self, self) y funciones asociadas.

#[derive(Debug)]
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    // Función asociada (sin self): se llama con `Rectangulo::new(...)`
    fn new(ancho: u32, alto: u32) -> Self {
        Self { ancho, alto }
    }

    fn cuadrado(lado: u32) -> Self {
        Self::new(lado, lado)
    }

    // &self: lee sin tomar propiedad
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }

    fn perimetro(&self) -> u32 {
        2 * (self.ancho + self.alto)
    }

    fn puede_contener(&self, otro: &Rectangulo) -> bool {
        self.ancho > otro.ancho && self.alto > otro.alto
    }

    // &mut self: modifica la instancia
    fn escalar(&mut self, factor: u32) {
        self.ancho *= factor;
        self.alto *= factor;
    }

    // self: consume la instancia
    fn into_tupla(self) -> (u32, u32) {
        (self.ancho, self.alto)
    }
}

// Se pueden tener varios bloques impl
impl Rectangulo {
    fn es_cuadrado(&self) -> bool {
        self.ancho == self.alto
    }
}

fn main() {
    let mut r = Rectangulo::new(30, 50);
    println!("{r:?}");
    println!("área = {}, perímetro = {}", r.area(), r.perimetro());

    let pequeño = Rectangulo::cuadrado(10);
    println!("¿r contiene a pequeño? {}", r.puede_contener(&pequeño));
    println!("¿pequeño es cuadrado? {}", pequeño.es_cuadrado());

    r.escalar(2);
    println!("escalado: {r:?}");

    let (a, b) = r.into_tupla();
    println!("{a} x {b}");
    // println!("{r:?}"); // error: r fue consumido por into_tupla
}
