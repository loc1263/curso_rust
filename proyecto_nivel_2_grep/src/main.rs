use anyhow::{Context, Result};
use clap::Parser;
use rgrep::{Buscador, Opciones, listar_archivos};
use std::path::PathBuf;
use std::process::ExitCode;

/// rgrep: busca un texto en archivos (mini `grep` del curso de Rust)
#[derive(Parser, Debug)]
#[command(name = "rgrep", version, about)]
struct Cli {
    /// Texto a buscar
    patron: String,

    /// Archivos o directorios donde buscar
    #[arg(required = true)]
    rutas: Vec<PathBuf>,

    /// Ignorar mayúsculas y minúsculas
    #[arg(short, long)]
    ignorar_mayusculas: bool,

    /// Mostrar el número de línea
    #[arg(short = 'n', long)]
    numeros: bool,

    /// Mostrar las líneas que NO coinciden
    #[arg(short = 'v', long)]
    invertir: bool,

    /// Coincidir solo con palabras completas
    #[arg(short = 'w', long)]
    palabra: bool,

    /// Mostrar solo el número de coincidencias por archivo
    #[arg(short, long)]
    contar: bool,

    /// Buscar dentro de los directorios de forma recursiva
    #[arg(short, long)]
    recursivo: bool,

    /// Limitar a archivos con esta extensión (p. ej. `rs`)
    #[arg(short, long)]
    ext: Option<String>,
}

fn ejecutar(cli: &Cli) -> Result<bool> {
    let opciones = Opciones {
        ignorar_mayusculas: cli.ignorar_mayusculas,
        invertir: cli.invertir,
        palabra_completa: cli.palabra,
    };
    let buscador = Buscador::nuevo(&cli.patron, opciones)?;

    // Expandir rutas: los directorios solo se recorren con --recursivo
    let mut archivos = Vec::new();
    for ruta in &cli.rutas {
        if ruta.is_dir() {
            if !cli.recursivo {
                eprintln!("rgrep: {} es un directorio (usa -r)", ruta.display());
                continue;
            }
            archivos.extend(
                listar_archivos(ruta, cli.ext.as_deref())
                    .with_context(|| format!("no se pudo recorrer {}", ruta.display()))?,
            );
        } else {
            archivos.push(ruta.clone());
        }
    }

    let varios = archivos.len() > 1;
    let mut hubo_coincidencias = false;
    for archivo in &archivos {
        // Un archivo ilegible (binario, sin permisos...) se avisa y se continúa
        let coincidencias = match buscador.buscar_en_archivo(archivo) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("rgrep: {e}");
                continue;
            }
        };
        hubo_coincidencias |= !coincidencias.is_empty();

        if cli.contar {
            println!("{}:{}", archivo.display(), coincidencias.len());
            continue;
        }
        for c in &coincidencias {
            let mut prefijo = String::new();
            if varios {
                prefijo += &format!("{}:", archivo.display());
            }
            if cli.numeros {
                prefijo += &format!("{}:", c.numero_linea);
            }
            println!("{prefijo}{}", c.linea);
        }
    }
    Ok(hubo_coincidencias)
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    // Convención de grep: 0 = hubo coincidencias, 1 = ninguna, 2 = error
    match ejecutar(&cli) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::from(1),
        Err(e) => {
            eprintln!("rgrep: {e:#}");
            ExitCode::from(2)
        }
    }
}
