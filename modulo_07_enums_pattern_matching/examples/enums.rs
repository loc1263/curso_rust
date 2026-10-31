// cargo run --example enums
// Enums simples y enums con datos en cada variante.

#[derive(Debug, PartialEq)]
enum Direccion {
    Norte,
    Sur,
    Este,
    Oeste,
}

impl Direccion {
    fn opuesta(&self) -> Direccion {
        match self {
            Direccion::Norte => Direccion::Sur,
            Direccion::Sur => Direccion::Norte,
            Direccion::Este => Direccion::Oeste,
            Direccion::Oeste => Direccion::Este,
        }
    }
}

// Cada variante puede guardar datos distintos
#[derive(Debug)]
enum Mensaje {
    Salir,                      // sin datos
    Mover { x: i32, y: i32 },   // como un struct
    Escribir(String),           // como una tupla
    Color(u8, u8, u8),
}

impl Mensaje {
    fn procesar(&self) {
        match self {
            Mensaje::Salir => println!("saliendo"),
            Mensaje::Mover { x, y } => println!("mover a ({x}, {y})"),
            Mensaje::Escribir(texto) => println!("escribir '{texto}'"),
            Mensaje::Color(r, g, b) => println!("color #{r:02x}{g:02x}{b:02x}"),
        }
    }
}

enum Ip {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn mostrar_ip(ip: &Ip) -> String {
    match ip {
        Ip::V4(a, b, c, d) => format!("{a}.{b}.{c}.{d}"),
        Ip::V6(s) => s.clone(),
    }
}

fn main() {
    let d = Direccion::Norte;
    println!("{:?} <-> {:?}", d, d.opuesta());
    println!("{}", Direccion::Este == Direccion::Este.opuesta().opuesta());
    println!("{:?}", Direccion::Oeste);

    let mensajes = [
        Mensaje::Mover { x: 10, y: -3 },
        Mensaje::Escribir(String::from("hola")),
        Mensaje::Color(255, 128, 0),
        Mensaje::Salir,
    ];
    for m in &mensajes {
        m.procesar();
    }

    println!("{}", mostrar_ip(&Ip::V4(127, 0, 0, 1)));
    println!("{}", mostrar_ip(&Ip::V6(String::from("::1"))));
}
