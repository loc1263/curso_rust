// cargo run -p m18_errores_avanzados --example box_dyn_error
// `Box<dyn Error>` a mano: lo que `anyhow` automatiza; conversión entre errores y `downcast`.

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ErrorNegocio {
    codigo: u32,
}
impl fmt::Display for ErrorNegocio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error de negocio #{}", self.codigo)
    }
}
impl Error for ErrorNegocio {}

fn paso_1(n: i32) -> Result<i32, Box<dyn Error>> {
    if n < 0 {
        // &str / String -> Box<dyn Error> automáticamente
        return Err("n no puede ser negativo".into());
    }
    Ok(n * 2)
}

fn paso_2(s: &str) -> Result<i32, Box<dyn Error>> {
    let n: i32 = s.parse()?; // ParseIntError -> Box<dyn Error>
    if n == 13 {
        return Err(Box::new(ErrorNegocio { codigo: 13 }));
    }
    paso_1(n)
}

// Error que envuelve a otro: `source()` conserva la causa
#[derive(Debug)]
struct ErrorServicio {
    contexto: &'static str,
    causa: Box<dyn Error + Send + Sync>,
}
impl fmt::Display for ErrorServicio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "falló {}", self.contexto)
    }
}
impl Error for ErrorServicio {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.causa.as_ref())
    }
}

fn servicio(entrada: &str) -> Result<i32, ErrorServicio> {
    entrada.parse::<i32>().map_err(|e| ErrorServicio {
        contexto: "leer la entrada",
        causa: Box::new(e),
    })
}

fn main() {
    for s in ["21", "-4", "abc", "13"] {
        match paso_2(s) {
            Ok(v) => println!("'{s}' -> {v}"),
            Err(e) => {
                // downcast_ref: averiguar qué tipo concreto hay dentro
                if let Some(neg) = e.downcast_ref::<ErrorNegocio>() {
                    println!("'{s}' -> error de negocio con código {}", neg.codigo);
                } else {
                    println!("'{s}' -> error genérico: {e}");
                }
            }
        }
    }

    if let Err(e) = servicio("xyz") {
        println!("\n{e}");
        let mut fuente = e.source();
        while let Some(s) = fuente {
            println!("  causado por: {s}");
            fuente = s.source();
        }
    }
}
