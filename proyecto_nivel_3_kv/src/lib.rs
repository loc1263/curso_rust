//! `kvdb`: una mini base de datos clave-valor async, con TTL y persistencia AOF.
//!
//! Módulos:
//! - [`protocolo`]: comandos y respuestas (parseo, serialización, comillas).
//! - [`almacen`]: estado compartido, caducidad y registro de solo-añadir.
//! - [`servidor`]: servidor TCP con tokio (una tarea por conexión, límites y apagado ordenado).

pub mod almacen;
pub mod protocolo;
pub mod servidor;

use std::future::Future;
use std::path::Path;
use tokio::net::TcpListener;

/// Arranca el servidor completo: abre (y reproduce) el AOF, atiende conexiones y, al apagarse,
/// espera a que el log quede escrito en disco.
pub async fn ejecutar(
    listener: TcpListener,
    aof: Option<&Path>,
    apagar: impl Future<Output = ()>,
) -> anyhow::Result<()> {
    let (almacen, tarea_aof) = almacen::Almacen::abrir(aof).await?;
    servidor::servir(listener, almacen, apagar).await?;
    // `servir` consumió el último `Almacen`: el canal del AOF se cerró y el escritor termina solo
    if let Some(tarea) = tarea_aof {
        tarea.await??;
    }
    Ok(())
}
