//! Protocolo de texto, una orden por línea (estilo Redis simplificado).
//!
//! ```text
//! PING                              -> +PONG
//! SET clave valor [EX seg | PX ms]  -> +OK
//! GET clave                         -> $valor      (o _ si no existe)
//! DEL clave                         -> :1 | :0
//! INCR clave                        -> :n
//! TTL clave                         -> :segundos | :-1 (sin caducidad) | :-2 (no existe)
//! KEYS                              -> *n  y n líneas  $clave
//! INFO                              -> *n  y n líneas  $nombre:valor
//! QUIT                              -> +BYE (y cierra la conexión)
//! ```
//!
//! Los valores con espacios van entre comillas dobles: `SET saludo "hola mundo"`
//! (dentro se permiten `\"` y `\\`). Los errores se responden como `-ERR mensaje`.

use std::time::Duration;
use thiserror::Error;

/// Orden ya analizada.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Comando {
    Ping,
    Set {
        clave: String,
        valor: String,
        ttl: Option<Duration>,
    },
    Get(String),
    Del(String),
    Incr(String),
    Ttl(String),
    Keys,
    Info,
    Quit,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ErrorProtocolo {
    #[error("comando vacío")]
    Vacio,
    #[error("comando desconocido '{0}'")]
    Desconocido(String),
    #[error("número de argumentos incorrecto para {0}")]
    Argumentos(&'static str),
    #[error("comillas sin cerrar")]
    ComillasSinCerrar,
    #[error("caracteres de control no permitidos")]
    CaracterDeControl,
    #[error("opción inválida: {0}")]
    Opcion(String),
}

/// Divide una línea en tokens respetando las comillas dobles.
pub fn dividir(linea: &str) -> Result<Vec<String>, ErrorProtocolo> {
    let mut tokens = Vec::new();
    let mut actual = String::new();
    let mut hay_token = false; // distingue "" (token vacío) de "no hay token"
    let mut entre_comillas = false;
    let mut chars = linea.chars();

    while let Some(c) = chars.next() {
        if c.is_control() && c != '\t' {
            return Err(ErrorProtocolo::CaracterDeControl);
        }
        match (c, entre_comillas) {
            ('"', false) => {
                entre_comillas = true;
                hay_token = true;
            }
            ('"', true) => entre_comillas = false,
            ('\\', true) => match chars.next() {
                Some(e @ ('"' | '\\')) => actual.push(e),
                Some(otro) => {
                    actual.push('\\');
                    actual.push(otro);
                }
                None => return Err(ErrorProtocolo::ComillasSinCerrar),
            },
            (c, false) if c.is_whitespace() => {
                if hay_token {
                    tokens.push(std::mem::take(&mut actual));
                    hay_token = false;
                }
            }
            (c, _) => {
                actual.push(c);
                hay_token = true;
            }
        }
    }
    if entre_comillas {
        return Err(ErrorProtocolo::ComillasSinCerrar);
    }
    if hay_token {
        tokens.push(actual);
    }
    Ok(tokens)
}

/// Inversa de `dividir` para un token: añade comillas solo si hacen falta.
pub fn citar(s: &str) -> String {
    if !s.is_empty()
        && !s
            .chars()
            .any(|c| c.is_whitespace() || c == '"' || c == '\\')
    {
        return s.to_string();
    }
    let mut r = String::with_capacity(s.len() + 2);
    r.push('"');
    for c in s.chars() {
        if c == '"' || c == '\\' {
            r.push('\\');
        }
        r.push(c);
    }
    r.push('"');
    r
}

/// Analiza una línea completa.
pub fn parsear(linea: &str) -> Result<Comando, ErrorProtocolo> {
    let tokens = dividir(linea)?;
    let (nombre, args) = tokens.split_first().ok_or(ErrorProtocolo::Vacio)?;
    let nombre = nombre.to_ascii_uppercase();

    // Cada comando valida su número de argumentos
    let uno = |cmd: &'static str, args: &[String]| -> Result<String, ErrorProtocolo> {
        match args {
            [a] => Ok(a.clone()),
            _ => Err(ErrorProtocolo::Argumentos(cmd)),
        }
    };
    match nombre.as_str() {
        "PING" if args.is_empty() => Ok(Comando::Ping),
        "PING" => Err(ErrorProtocolo::Argumentos("PING")),
        "KEYS" if args.is_empty() => Ok(Comando::Keys),
        "KEYS" => Err(ErrorProtocolo::Argumentos("KEYS")),
        "INFO" if args.is_empty() => Ok(Comando::Info),
        "INFO" => Err(ErrorProtocolo::Argumentos("INFO")),
        "QUIT" if args.is_empty() => Ok(Comando::Quit),
        "QUIT" => Err(ErrorProtocolo::Argumentos("QUIT")),
        "GET" => Ok(Comando::Get(uno("GET", args)?)),
        "DEL" => Ok(Comando::Del(uno("DEL", args)?)),
        "INCR" => Ok(Comando::Incr(uno("INCR", args)?)),
        "TTL" => Ok(Comando::Ttl(uno("TTL", args)?)),
        "SET" => parsear_set(args),
        otro => Err(ErrorProtocolo::Desconocido(otro.to_string())),
    }
}

fn parsear_set(args: &[String]) -> Result<Comando, ErrorProtocolo> {
    let [clave, valor, opciones @ ..] = args else {
        return Err(ErrorProtocolo::Argumentos("SET"));
    };
    let ttl = match opciones {
        [] => None,
        [opcion, cantidad] => {
            let n: u64 = cantidad
                .parse()
                .map_err(|_| ErrorProtocolo::Opcion(format!("'{cantidad}' no es un número")))?;
            if n == 0 {
                return Err(ErrorProtocolo::Opcion(
                    "la caducidad debe ser mayor que 0".into(),
                ));
            }
            match opcion.to_ascii_uppercase().as_str() {
                "EX" => Some(Duration::from_secs(n)),
                "PX" => Some(Duration::from_millis(n)),
                otra => return Err(ErrorProtocolo::Opcion(otra.to_string())),
            }
        }
        _ => return Err(ErrorProtocolo::Argumentos("SET")),
    };
    Ok(Comando::Set {
        clave: clave.clone(),
        valor: valor.clone(),
        ttl,
    })
}

/// Respuesta del servidor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Respuesta {
    Ok,
    Pong,
    Adios,
    Valor(String),
    Nulo,
    Entero(i64),
    Lista(Vec<String>),
    Error(String),
}

impl Respuesta {
    /// Serializa la respuesta (siempre termina en `\n`).
    pub fn serializar(&self) -> String {
        match self {
            Respuesta::Ok => "+OK\n".into(),
            Respuesta::Pong => "+PONG\n".into(),
            Respuesta::Adios => "+BYE\n".into(),
            Respuesta::Valor(v) => format!("${v}\n"),
            Respuesta::Nulo => "_\n".into(),
            Respuesta::Entero(n) => format!(":{n}\n"),
            Respuesta::Lista(items) => {
                let mut s = format!("*{}\n", items.len());
                for i in items {
                    s.push('$');
                    s.push_str(i);
                    s.push('\n');
                }
                s
            }
            Respuesta::Error(m) => format!("-ERR {m}\n"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_con_comillas_y_escapes() {
        assert_eq!(
            dividir(r#"SET k "hola mundo""#).unwrap(),
            ["SET", "k", "hola mundo"]
        );
        assert_eq!(
            dividir(r#"SET k "a \"b\" \\ c""#).unwrap(),
            ["SET", "k", r#"a "b" \ c"#]
        );
        assert_eq!(dividir(r#"SET k """#).unwrap(), ["SET", "k", ""]);
        assert_eq!(dividir("  a   b  ").unwrap(), ["a", "b"]);
        assert_eq!(
            dividir(r#"SET k "sin cerrar"#),
            Err(ErrorProtocolo::ComillasSinCerrar)
        );
        assert_eq!(dividir("a\u{7}b"), Err(ErrorProtocolo::CaracterDeControl));
    }

    #[test]
    fn citar_y_dividir_son_inversas() {
        for s in [
            "simple",
            "con espacio",
            "",
            r#"comi"lla"#,
            r"barra\inversa",
            "  ",
        ] {
            assert_eq!(dividir(&citar(s)).unwrap(), [s], "falló con {s:?}");
        }
    }

    #[test]
    fn parsea_comandos() {
        assert_eq!(parsear("ping"), Ok(Comando::Ping));
        assert_eq!(parsear("get a"), Ok(Comando::Get("a".into())));
        assert_eq!(
            parsear("SET a 1 EX 10"),
            Ok(Comando::Set {
                clave: "a".into(),
                valor: "1".into(),
                ttl: Some(Duration::from_secs(10))
            })
        );
        assert_eq!(
            parsear("set a 1 px 250"),
            Ok(Comando::Set {
                clave: "a".into(),
                valor: "1".into(),
                ttl: Some(Duration::from_millis(250))
            })
        );
    }

    #[test]
    fn errores_de_parseo() {
        assert_eq!(parsear("   "), Err(ErrorProtocolo::Vacio));
        assert_eq!(
            parsear("FOO"),
            Err(ErrorProtocolo::Desconocido("FOO".into()))
        );
        assert_eq!(parsear("GET"), Err(ErrorProtocolo::Argumentos("GET")));
        assert_eq!(parsear("GET a b"), Err(ErrorProtocolo::Argumentos("GET")));
        assert_eq!(parsear("SET a"), Err(ErrorProtocolo::Argumentos("SET")));
        assert!(matches!(
            parsear("SET a 1 EX 0"),
            Err(ErrorProtocolo::Opcion(_))
        ));
        assert!(matches!(
            parsear("SET a 1 EX x"),
            Err(ErrorProtocolo::Opcion(_))
        ));
        assert!(matches!(
            parsear("SET a 1 ZZ 5"),
            Err(ErrorProtocolo::Opcion(_))
        ));
    }

    #[test]
    fn serializa_respuestas() {
        assert_eq!(Respuesta::Ok.serializar(), "+OK\n");
        assert_eq!(Respuesta::Entero(-2).serializar(), ":-2\n");
        assert_eq!(
            Respuesta::Lista(vec!["a".into(), "b".into()]).serializar(),
            "*2\n$a\n$b\n"
        );
        assert_eq!(Respuesta::Error("x".into()).serializar(), "-ERR x\n");
    }
}
