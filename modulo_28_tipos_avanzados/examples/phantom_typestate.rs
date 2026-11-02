// cargo run -p m28_tipos_avanzados --example phantom_typestate
// PhantomData, newtype y typestate: hacer que los ESTADOS INVÁLIDOS no compilen.

use std::marker::PhantomData;
use std::ops::Add;

// ---------- Newtype: tipos distintos con la misma representación ----------
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Id(u64);
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Precio(f64);

fn cobrar(_cliente: Id, _importe: Precio) {}
// cobrar(Precio(9.0), Id(1)) -> ERROR de compilación: no se pueden confundir los argumentos

// ---------- PhantomData: un "tipo fantasma" para llevar información en el tipo ----------
// Unidades de medida comprobadas en compilación, SIN coste en ejecución
#[derive(Debug, Clone, Copy)]
struct Metros;
#[derive(Debug, Clone, Copy)]
struct Pies;

#[derive(Debug, Clone, Copy)]
struct Longitud<U> {
    valor: f64,
    _unidad: PhantomData<U>, // ocupa 0 bytes
}

impl<U> Longitud<U> {
    fn new(valor: f64) -> Self {
        Longitud {
            valor,
            _unidad: PhantomData,
        }
    }
}
impl<U> Add for Longitud<U> {
    type Output = Longitud<U>;
    fn add(self, o: Self) -> Self {
        Longitud::new(self.valor + o.valor)
    }
}
impl From<Longitud<Pies>> for Longitud<Metros> {
    fn from(p: Longitud<Pies>) -> Self {
        Longitud::new(p.valor * 0.3048)
    }
}

// ---------- Typestate: el estado del objeto vive en su tipo ----------
// Una conexión que solo permite enviar cuando está abierta y que no se puede cerrar dos veces.
struct Cerrada;
struct Abierta;

struct Conexion<Estado> {
    destino: String,
    enviados: Vec<String>,
    _estado: PhantomData<Estado>,
}

impl Conexion<Cerrada> {
    fn nueva(destino: &str) -> Self {
        Conexion {
            destino: destino.into(),
            enviados: vec![],
            _estado: PhantomData,
        }
    }
    // `self` por valor: consume la conexión cerrada y devuelve una abierta
    fn abrir(self) -> Conexion<Abierta> {
        println!("  abriendo {}", self.destino);
        Conexion {
            destino: self.destino,
            enviados: self.enviados,
            _estado: PhantomData,
        }
    }
}

impl Conexion<Abierta> {
    fn enviar(&mut self, msg: &str) {
        println!("  -> {msg}");
        self.enviados.push(msg.into());
    }
    fn cerrar(self) -> Conexion<Cerrada> {
        println!("  cerrando tras {} mensajes", self.enviados.len());
        Conexion {
            destino: self.destino,
            enviados: self.enviados,
            _estado: PhantomData,
        }
    }
}

// ---------- Builder con typestate: campos obligatorios comprobados en compilación ----------
struct SinUrl;
struct ConUrl(String);

struct PeticionBuilder<U> {
    url: U,
    metodo: &'static str,
}

impl PeticionBuilder<SinUrl> {
    fn new() -> Self {
        PeticionBuilder {
            url: SinUrl,
            metodo: "GET",
        }
    }
    fn url(self, url: &str) -> PeticionBuilder<ConUrl> {
        PeticionBuilder {
            url: ConUrl(url.into()),
            metodo: self.metodo,
        }
    }
}
impl<U> PeticionBuilder<U> {
    fn metodo(mut self, m: &'static str) -> Self {
        self.metodo = m;
        self
    }
}
impl PeticionBuilder<ConUrl> {
    // `construir` solo existe si ya se puso la URL
    fn construir(self) -> String {
        format!("{} {}", self.metodo, self.url.0)
    }
}

fn main() {
    cobrar(Id(7), Precio(19.99));

    let a: Longitud<Metros> = Longitud::new(100.0);
    let b: Longitud<Pies> = Longitud::new(328.0);
    let suma = a + Longitud::<Metros>::from(b);
    println!("100 m + 328 pies = {:.1} m", suma.valor);
    // let mal = a + b; // ERROR: no se pueden sumar Metros con Pies
    println!(
        "tamaño de Longitud<Metros> = {} bytes (igual que un f64)",
        std::mem::size_of::<Longitud<Metros>>()
    );

    println!("\ntypestate de conexión:");
    let c = Conexion::nueva("servidor:80");
    // c.enviar("hola");     // ERROR: `enviar` no existe en Conexion<Cerrada>
    let mut c = c.abrir();
    c.enviar("hola");
    c.enviar("adiós");
    let c = c.cerrar();
    // c.cerrar();           // ERROR: ya no existe `cerrar` en Conexion<Cerrada> (y `c` se movió)
    let _ = c;

    println!("\nbuilder con typestate:");
    let peticion = PeticionBuilder::new()
        .metodo("POST")
        .url("https://ejemplo.com")
        .construir();
    println!("  {peticion}");
    // PeticionBuilder::new().construir(); // ERROR: falta la URL
}
