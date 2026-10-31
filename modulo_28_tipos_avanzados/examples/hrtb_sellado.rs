// cargo run -p m28_tipos_avanzados --example hrtb_sellado
// HRTB (`for<'a>`), traits sellados, `dyn Any`, tipo `!`, DST y otras piezas del sistema de tipos.

use std::any::Any;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;

// ---------- HRTB: Higher-Rank Trait Bounds ----------
// "F funciona para CUALQUIER lifetime 'a", no para uno concreto.
fn aplicar_a_local<F>(f: F) -> usize
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let local = String::from("  texto local  ");
    // `local` vive menos que cualquier lifetime que pudiera fijar el llamante: por eso hace falta for<'a>
    f(&local).len()
}

// ---------- Trait sellado: nadie fuera de este módulo puede implementarlo ----------
mod sellado {
    mod privado {
        pub trait Sellado {}
    }
    pub trait Formato: privado::Sellado {
        fn extension(&self) -> &'static str;
    }
    pub struct Json;
    pub struct Csv;
    impl privado::Sellado for Json {}
    impl privado::Sellado for Csv {}
    impl Formato for Json {
        fn extension(&self) -> &'static str {
            "json"
        }
    }
    impl Formato for Csv {
        fn extension(&self) -> &'static str {
            "csv"
        }
    }
}
use sellado::{Csv, Formato, Json};

fn exportar(f: &impl Formato) -> String {
    format!("datos.{}", f.extension())
}

// ---------- Marker traits y bounds negativos "por construcción" ----------
trait Persistible: Debug {}
impl<T: Debug + Clone> Persistible for T {}

// ---------- dyn Any: tipos dinámicos con downcast ----------
fn describir(valor: &dyn Any) -> String {
    if let Some(n) = valor.downcast_ref::<i32>() {
        format!("i32 = {n}")
    } else if let Some(s) = valor.downcast_ref::<String>() {
        format!("String = {s:?}")
    } else {
        "tipo desconocido".to_string()
    }
}

// Un "mapa de tipos": un valor por tipo (patrón de ECS / contenedores de dependencias)
#[derive(Default)]
struct MapaDeTipos {
    mapa: HashMap<std::any::TypeId, Box<dyn Any>>,
}
impl MapaDeTipos {
    fn insertar<T: 'static>(&mut self, v: T) {
        self.mapa.insert(std::any::TypeId::of::<T>(), Box::new(v));
    }
    fn obtener<T: 'static>(&self) -> Option<&T> {
        self.mapa.get(&std::any::TypeId::of::<T>())?.downcast_ref()
    }
}

// ---------- Tipo `!` (never): funciones que nunca vuelven ----------
fn fallar(msg: &str) -> ! {
    panic!("{msg}")
}
fn parsear_o_fallar(s: &str) -> i32 {
    match s.parse() {
        Ok(n) => n,
        Err(_) => fallar("no es un número"), // `!` coacciona a cualquier tipo
    }
}

// ---------- Cow: clonar solo si hace falta ----------
fn normalizar(s: &str) -> Cow<'_, str> {
    if s.contains(' ') { Cow::Owned(s.replace(' ', "_")) } else { Cow::Borrowed(s) }
}

// ---------- DST: tipos sin tamaño conocido (str, [T], dyn Trait) siempre detrás de un puntero ----------
fn longitud_de<T: ?Sized + AsRef<[u8]>>(x: &T) -> usize {
    x.as_ref().len() // `?Sized` relaja el bound implícito `T: Sized`
}

fn main() {
    println!("HRTB: {}", aplicar_a_local(|s| s.trim()));

    println!("exportar -> {} / {}", exportar(&Json), exportar(&Csv));
    // struct Xml; impl Formato for Xml {...}  // ERROR: el trait está sellado

    fn requiere_persistible(_: &impl Persistible) {}
    requiere_persistible(&vec![1, 2]);

    println!("{}", describir(&42));
    println!("{}", describir(&String::from("hola")));
    println!("{}", describir(&1.5f32));

    let mut m = MapaDeTipos::default();
    m.insertar(8080u16);
    m.insertar(String::from("localhost"));
    println!("puerto = {:?}, host = {:?}, bool = {:?}", m.obtener::<u16>(), m.obtener::<String>(), m.obtener::<bool>());

    println!("{}", parsear_o_fallar("12"));

    let a = normalizar("sin_espacios");
    let b = normalizar("con espacios aquí");
    println!("{a} ({}) / {b} ({})", matches!(a, Cow::Borrowed(_)), matches!(b, Cow::Owned(_)));

    println!("{} {} {}", longitud_de("abc"), longitud_de(&[1u8, 2][..]), longitud_de(&vec![0u8; 5]));
}
