//! Binario de ejemplo: lee configuración del entorno, escribe logs a stderr y sale con códigos claros.
//!
//! ```text
//! cargo run -p m32_produccion
//! cargo run -p m32_produccion --features completo
//! cargo run -p m32_produccion --no-default-features
//! PUERTO=9000 cargo run -p m32_produccion       (PowerShell: $env:PUERTO=9000)
//! ```

use std::process::ExitCode;

use m32_produccion::InfoBuild;

/// Configuración de 12-factor: todo viene del entorno, con valores por defecto sensatos.
struct Config {
    puerto: u16,
    entorno: String,
}

impl Config {
    fn desde_entorno() -> Result<Self, String> {
        let puerto = match std::env::var("PUERTO") {
            Ok(v) => v
                .parse()
                .map_err(|e| format!("PUERTO='{v}' inválido: {e}"))?,
            Err(_) => 8080,
        };
        let entorno = std::env::var("ENTORNO").unwrap_or_else(|_| "desarrollo".into());
        Ok(Config { puerto, entorno })
    }
}

fn main() -> ExitCode {
    // Los logs van a stderr (stdout queda para datos); en producción usa `tracing` en formato JSON
    eprintln!("arrancando {}", InfoBuild::actual());

    let config = match Config::desde_entorno() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error de configuración: {e}");
            return ExitCode::from(2); // 2 = uso/configuración incorrectos
        }
    };
    eprintln!("entorno={} puerto={}", config.entorno, config.puerto);

    #[cfg(feature = "estadisticas")]
    if let Some(r) = m32_produccion::estadisticas::resumir(&[3.5, 1.0, 8.25]) {
        println!("resumen: media={:.2} min={} max={}", r.media, r.min, r.max);
    }

    #[cfg(feature = "colores")]
    println!("{}", m32_produccion::colores::verde("todo OK"));
    #[cfg(not(feature = "colores"))]
    println!("todo OK");

    ExitCode::SUCCESS
}
