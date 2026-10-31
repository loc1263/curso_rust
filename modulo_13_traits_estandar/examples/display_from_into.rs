// cargo run -p m13_traits_estandar --example display_from_into
// Display, From/Into, TryFrom, FromStr, AsRef y manejo manual de comparaciones.

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Rgb(u8, u8, u8);

// Display: formato "para humanos" ({} en println!)
impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }
}

// From: conversión infalible. Al implementar From obtienes Into gratis.
impl From<(u8, u8, u8)> for Rgb {
    fn from(t: (u8, u8, u8)) -> Self {
        Rgb(t.0, t.1, t.2)
    }
}
impl From<Rgb> for u32 {
    fn from(c: Rgb) -> u32 {
        ((c.0 as u32) << 16) | ((c.1 as u32) << 8) | c.2 as u32
    }
}

// TryFrom: conversión que puede fallar
impl TryFrom<[i32; 3]> for Rgb {
    type Error = String;
    fn try_from(a: [i32; 3]) -> Result<Self, String> {
        let canal = |v: i32| u8::try_from(v).map_err(|_| format!("canal fuera de rango: {v}"));
        Ok(Rgb(canal(a[0])?, canal(a[1])?, canal(a[2])?))
    }
}

// FromStr: habilita "...".parse::<Rgb>()
impl FromStr for Rgb {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        let s = s.strip_prefix('#').ok_or("falta el '#'")?;
        if s.len() != 6 {
            return Err(format!("se esperaban 6 dígitos hex, hay {}", s.len()));
        }
        let par = |i: usize| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string());
        Ok(Rgb(par(0)?, par(2)?, par(4)?))
    }
}

// Ord/PartialOrd manual: ordenar por brillo aproximado
#[derive(Debug, PartialEq, Eq)]
struct Pixel(u8, u8, u8);

impl Pixel {
    fn brillo(&self) -> u32 {
        self.0 as u32 + self.1 as u32 + self.2 as u32
    }
}
impl PartialOrd for Pixel {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}
impl Ord for Pixel {
    fn cmp(&self, o: &Self) -> Ordering {
        self.brillo().cmp(&o.brillo())
    }
}

// AsRef: aceptar &str, String, &String... con una sola función
fn longitud<S: AsRef<str>>(s: S) -> usize {
    s.as_ref().len()
}

// `impl Into<String>`: API cómoda que acepta &str y String
fn saludo(nombre: impl Into<String>) -> String {
    let n: String = nombre.into();
    format!("Hola, {n}")
}

fn main() {
    let rojo = Rgb(255, 0, 0);
    println!("{rojo}  /  {rojo:?}");

    let c: Rgb = (10, 20, 30).into();
    let n: u32 = c.into();
    println!("{c} -> {n}");

    println!("{:?}", Rgb::try_from([1, 2, 3]));
    println!("{:?}", Rgb::try_from([1, 300, 3]));

    println!("{:?}", "#ff8000".parse::<Rgb>());
    println!("{:?}", "ff8000".parse::<Rgb>());
    println!("{:?}", "#ff80".parse::<Rgb>());
    println!("{:?}", "#gg0000".parse::<Rgb>());

    let mut pixeles = vec![Pixel(200, 200, 200), Pixel(1, 2, 3), Pixel(100, 0, 0)];
    pixeles.sort();
    println!("por brillo: {pixeles:?}");

    println!("{} {} {}", longitud("abc"), longitud(String::from("hola")), longitud(&String::from("xy")));
    println!("{} / {}", saludo("Ana"), saludo(String::from("Luis")));

    // Conversiones numéricas: From es seguro, TryFrom comprueba el rango
    let g: i64 = i64::from(7_i32);
    let r = u8::try_from(300_i32);
    println!("{g} {r:?}");
}
