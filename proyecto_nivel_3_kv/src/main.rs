//! Servidor `kv-server`.
//!
//! ```text
//! cargo run -p kvdb --bin kv-server -- [--addr 127.0.0.1:6380] [--aof datos.aof]
//! ```
//! Variables de entorno equivalentes: `KV_ADDR`, `KV_AOF`.

use anyhow::{Context, Result, bail};
use std::path::PathBuf;
use tokio::net::TcpListener;

struct Config {
    direccion: String,
    aof: Option<PathBuf>,
}

fn leer_config() -> Result<Config> {
    let mut direccion = std::env::var("KV_ADDR").unwrap_or_else(|_| "127.0.0.1:6380".into());
    let mut aof = std::env::var_os("KV_AOF").map(PathBuf::from);

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--addr" => direccion = args.next().context("--addr necesita un valor")?,
            "--aof" => aof = Some(args.next().context("--aof necesita una ruta")?.into()),
            "-h" | "--help" => {
                println!("uso: kv-server [--addr HOST:PUERTO] [--aof RUTA]");
                std::process::exit(0);
            }
            otro => bail!("argumento desconocido '{otro}' (usa --help)"),
        }
    }
    Ok(Config { direccion, aof })
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = leer_config()?;
    let listener = TcpListener::bind(&config.direccion)
        .await
        .with_context(|| format!("no se pudo escuchar en {}", config.direccion))?;
    eprintln!("kvdb {} escuchando en {}", env!("CARGO_PKG_VERSION"), listener.local_addr()?);
    match &config.aof {
        Some(ruta) => eprintln!("persistencia AOF en {}", ruta.display()),
        None => eprintln!("sin persistencia (usa --aof RUTA para activarla)"),
    }

    kvdb::ejecutar(listener, config.aof.as_deref(), async {
        let _ = tokio::signal::ctrl_c().await;
        eprintln!("\nCtrl+C recibido");
    })
    .await?;
    eprintln!("adiós");
    Ok(())
}
