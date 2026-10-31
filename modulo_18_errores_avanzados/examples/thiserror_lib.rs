// cargo run -p m18_errores_avanzados --example thiserror_lib
// `thiserror`: errores tipados para BIBLIOTECAS, sin el boilerplate de Display/Error/From.

use std::collections::HashMap;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
enum ErrorConfig {
    // #[error("...")] genera Display; los campos se interpolan
    #[error("falta la clave '{0}'")]
    Falta(String),

    #[error("valor inválido para '{clave}': {motivo}")]
    Invalido { clave: String, motivo: String },

    // #[from] genera `impl From<ParseIntError>` y `source()`
    #[error("número mal formado")]
    Numero(#[from] ParseIntError),

    #[error("error de E/S")]
    Io(#[from] std::io::Error),
}

type Resultado<T> = Result<T, ErrorConfig>;

fn parsear(texto: &str) -> HashMap<&str, &str> {
    texto
        .lines()
        .filter_map(|l| l.split_once('='))
        .map(|(k, v)| (k.trim(), v.trim()))
        .collect()
}

fn obtener<'a>(cfg: &HashMap<&str, &'a str>, clave: &str) -> Resultado<&'a str> {
    cfg.get(clave).copied().ok_or_else(|| ErrorConfig::Falta(clave.into()))
}

fn puerto(cfg: &HashMap<&str, &str>) -> Resultado<u16> {
    let bruto = obtener(cfg, "puerto")?;
    let n: u32 = bruto.parse()?; // ParseIntError -> ErrorConfig::Numero via #[from]
    u16::try_from(n).map_err(|_| ErrorConfig::Invalido {
        clave: "puerto".into(),
        motivo: format!("{n} excede 65535"),
    })
}

fn leer_archivo(ruta: &str) -> Resultado<String> {
    Ok(std::fs::read_to_string(ruta)?) // io::Error -> ErrorConfig::Io
}

fn main() {
    let casos = ["puerto = 8080", "host = x", "puerto = abc", "puerto = 70000"];
    for texto in casos {
        let cfg = parsear(texto);
        match puerto(&cfg) {
            Ok(p) => println!("'{texto}' -> puerto {p}"),
            Err(e) => {
                print!("'{texto}' -> error: {e}");
                // Recorre la cadena de causas con `source()`
                let mut causa = std::error::Error::source(&e);
                while let Some(c) = causa {
                    print!(" <- {c}");
                    causa = c.source();
                }
                println!();
            }
        }
    }

    match leer_archivo("no_existe.conf") {
        Err(e @ ErrorConfig::Io(_)) => println!("fallo de E/S: {e}"),
        other => println!("{other:?}"),
    }
}
