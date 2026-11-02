// cargo run -p m21_ecosistema --example clap_cli -- --help
// cargo run -p m21_ecosistema --example clap_cli -- saludar Ana --veces 2 --mayusculas
// cargo run -p m21_ecosistema --example clap_cli -- sumar 1 2 3.5
// cargo run -p m21_ecosistema --example clap_cli -- -v info
//
// `clap` con derive: la estructura ES la definición de la CLI.

use clap::{Parser, Subcommand, ValueEnum};

/// Herramienta de ejemplo del curso de Rust
#[derive(Parser, Debug)]
#[command(name = "demo", version, about, long_about = None)]
struct Cli {
    /// Nivel de detalle (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    #[command(subcommand)]
    comando: Comando,
}

#[derive(Subcommand, Debug)]
enum Comando {
    /// Saluda a alguien
    Saludar {
        /// Nombre de la persona
        nombre: String,

        /// Número de repeticiones
        #[arg(short = 'n', long, default_value_t = 1)]
        veces: u8,

        /// Imprimir en mayúsculas
        #[arg(long)]
        mayusculas: bool,

        /// Idioma del saludo
        #[arg(long, value_enum, default_value_t = Idioma::Es)]
        idioma: Idioma,
    },

    /// Suma números
    Sumar {
        /// Los números a sumar
        #[arg(required = true)]
        numeros: Vec<f64>,
    },

    /// Muestra información del programa
    Info,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum Idioma {
    Es,
    En,
}

fn main() {
    let cli = Cli::parse(); // en caso de error imprime ayuda y sale con código 2

    if cli.verbose > 0 {
        eprintln!("[debug] nivel de detalle = {}, args = {cli:?}", cli.verbose);
    }

    match cli.comando {
        Comando::Saludar {
            nombre,
            veces,
            mayusculas,
            idioma,
        } => {
            let base = match idioma {
                Idioma::Es => format!("Hola, {nombre}"),
                Idioma::En => format!("Hello, {nombre}"),
            };
            let texto = if mayusculas {
                base.to_uppercase()
            } else {
                base
            };
            for _ in 0..veces {
                println!("{texto}");
            }
        }
        Comando::Sumar { numeros } => {
            println!(
                "{} = {}",
                numeros
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join(" + "),
                numeros.iter().sum::<f64>()
            );
        }
        Comando::Info => println!("demo v{}", env!("CARGO_PKG_VERSION")),
    }
}
