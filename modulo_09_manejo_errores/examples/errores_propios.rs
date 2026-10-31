// cargo run --example errores_propios
// Tipos de error propios: enum + Display + Error + From.

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum ErrorEdad {
    NoNumerico(ParseIntError),
    Negativa(i32),
    Irreal(i32),
}

impl fmt::Display for ErrorEdad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorEdad::NoNumerico(e) => write!(f, "no es un número: {e}"),
            ErrorEdad::Negativa(n) => write!(f, "la edad no puede ser negativa ({n})"),
            ErrorEdad::Irreal(n) => write!(f, "{n} años no es una edad realista"),
        }
    }
}

impl Error for ErrorEdad {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ErrorEdad::NoNumerico(e) => Some(e),
            _ => None,
        }
    }
}

// Con From, `?` convierte ParseIntError en ErrorEdad automáticamente
impl From<ParseIntError> for ErrorEdad {
    fn from(e: ParseIntError) -> Self {
        ErrorEdad::NoNumerico(e)
    }
}

fn validar_edad(s: &str) -> Result<u8, ErrorEdad> {
    let n: i32 = s.trim().parse()?;
    if n < 0 {
        return Err(ErrorEdad::Negativa(n));
    }
    if n > 150 {
        return Err(ErrorEdad::Irreal(n));
    }
    Ok(n as u8)
}

fn main() -> Result<(), Box<dyn Error>> {
    for entrada in ["25", "-3", "abc", "200"] {
        match validar_edad(entrada) {
            Ok(edad) => println!("'{entrada}' -> edad válida: {edad}"),
            Err(e) => {
                println!("'{entrada}' -> error: {e}");
                if let Some(causa) = e.source() {
                    println!("   causa: {causa}");
                }
            }
        }
    }

    // main puede devolver Result: si es Err, el programa termina con código 1
    let edad = validar_edad("30")?;
    println!("edad final = {edad}");
    Ok(())
}
