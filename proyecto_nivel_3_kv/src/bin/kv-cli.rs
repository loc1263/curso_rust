//! Cliente interactivo `kv-cli`.
//!
//! ```text
//! cargo run -p kvdb --bin kv-cli -- [HOST:PUERTO]          # REPL
//! echo "PING" | cargo run -p kvdb --bin kv-cli             # una orden por línea desde stdin
//! ```

use anyhow::{Context, Result};
use std::io::{BufRead, Write};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<()> {
    let direccion = std::env::args().nth(1).unwrap_or_else(|| "127.0.0.1:6380".into());
    let socket = TcpStream::connect(&direccion)
        .await
        .with_context(|| format!("no se pudo conectar a {direccion} (¿está corriendo kv-server?)"))?;
    let (lectura, mut escritura) = socket.into_split();
    let mut servidor = BufReader::new(lectura).lines();

    let interactivo = std::io::IsTerminal::is_terminal(&std::io::stdin());
    if interactivo {
        eprintln!("conectado a {direccion}. Escribe comandos (PING, SET, GET, DEL, INCR, TTL, KEYS, INFO, QUIT).");
    }

    // stdin es bloqueante: se lee desde un hilo aparte y se pasa por un canal
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        if interactivo {
            print!("> ");
            let _ = std::io::stdout().flush();
        }
        for linea in stdin.lock().lines().map_while(Result::ok) {
            if tx.send(linea).is_err() {
                break;
            }
        }
    });

    while let Some(orden) = rx.recv().await {
        if orden.trim().is_empty() {
            continue;
        }
        escritura.write_all(orden.as_bytes()).await?;
        escritura.write_all(b"\n").await?;

        // Lee la respuesta: una línea, o `*n` seguido de n líneas
        let Some(primera) = servidor.next_line().await? else {
            eprintln!("el servidor cerró la conexión");
            break;
        };
        if let Some(n) = primera.strip_prefix('*').and_then(|n| n.parse::<usize>().ok()) {
            println!("({n} elementos)");
            for _ in 0..n {
                let item = servidor.next_line().await?.unwrap_or_default();
                println!("  {}", item.strip_prefix('$').unwrap_or(&item));
            }
        } else {
            println!("{}", formatear(&primera));
        }
        if primera == "+BYE" {
            break;
        }
        if interactivo {
            print!("> ");
            let _ = std::io::stdout().flush();
        }
    }
    Ok(())
}

fn formatear(linea: &str) -> String {
    match linea.chars().next() {
        Some('+') => linea[1..].to_string(),
        Some('$') => format!("\"{}\"", &linea[1..]),
        Some(':') => format!("(entero) {}", &linea[1..]),
        Some('_') => "(nil)".to_string(),
        Some('-') => format!("(error) {}", linea[1..].trim_start_matches("ERR ")),
        _ => linea.to_string(),
    }
}
