//! Servidor TCP async: una tarea por conexión, límites de recursos y apagado ordenado.

use std::future::Future;
use std::io;
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Semaphore, watch};
use tokio::task::JoinSet;
use tokio::time::{interval, sleep, timeout};

use crate::almacen::Almacen;
use crate::protocolo::{Comando, Respuesta, parsear};

/// Tamaño máximo de una línea (protege la memoria frente a clientes hostiles).
pub const MAX_LINEA: u64 = 64 * 1024;
/// Conexiones simultáneas máximas; las demás esperan en la cola de `accept` del SO.
pub const MAX_CONEXIONES: usize = 128;
/// Una conexión inactiva este tiempo se cierra.
pub const INACTIVIDAD: Duration = Duration::from_secs(300);
/// Tiempo máximo para que las conexiones terminen tras pedir el apagado.
pub const GRACIA_APAGADO: Duration = Duration::from_secs(5);

/// Ejecuta un comando contra el almacén. Devuelve la respuesta y si hay que cerrar la conexión.
pub fn despachar(almacen: &Almacen, comando: Comando) -> (Respuesta, bool) {
    almacen.contar_comando();
    let respuesta = match comando {
        Comando::Ping => Respuesta::Pong,
        Comando::Set { clave, valor, ttl } => {
            almacen.set(clave, valor, ttl);
            Respuesta::Ok
        }
        Comando::Get(clave) => almacen
            .get(&clave)
            .map_or(Respuesta::Nulo, Respuesta::Valor),
        Comando::Del(clave) => Respuesta::Entero(i64::from(almacen.del(&clave))),
        Comando::Incr(clave) => almacen
            .incr(&clave)
            .map_or_else(|e| Respuesta::Error(e.to_string()), Respuesta::Entero),
        Comando::Ttl(clave) => Respuesta::Entero(almacen.ttl(&clave)),
        Comando::Keys => Respuesta::Lista(almacen.keys()),
        Comando::Info => Respuesta::Lista(almacen.info()),
        Comando::Quit => return (Respuesta::Adios, true),
    };
    (respuesta, false)
}

/// Procesa una línea de texto y produce la respuesta (útil también para tests sin red).
pub fn procesar_linea(almacen: &Almacen, linea: &str) -> (Respuesta, bool) {
    match parsear(linea) {
        Ok(comando) => despachar(almacen, comando),
        Err(e) => (Respuesta::Error(e.to_string()), false),
    }
}

/// Atiende conexiones hasta que `apagar` se complete.
///
/// Tras el apagado deja de aceptar, avisa a las conexiones abiertas y espera (hasta `GRACIA_APAGADO`)
/// a que terminen. Al volver, ninguna tarea conserva ya un `Almacen`.
pub async fn servir(
    listener: TcpListener,
    almacen: Almacen,
    apagar: impl Future<Output = ()>,
) -> io::Result<()> {
    let (tx_apagar, rx_apagar) = watch::channel(false);
    let limite = Arc::new(Semaphore::new(MAX_CONEXIONES));
    let mut conexiones = JoinSet::new();
    let purga = tokio::spawn(purgar_periodicamente(almacen.clone(), rx_apagar.clone()));
    tokio::pin!(apagar);

    loop {
        // Primero un permiso, luego aceptar: así nunca superamos MAX_CONEXIONES
        let permiso = tokio::select! {
            p = Arc::clone(&limite).acquire_owned() => p.expect("el semáforo nunca se cierra"),
            () = &mut apagar => break,
        };
        tokio::select! {
            aceptada = listener.accept() => match aceptada {
                Ok((socket, _direccion)) => {
                    let almacen = almacen.clone();
                    let rx = rx_apagar.clone();
                    conexiones.spawn(async move {
                        manejar(socket, almacen, rx).await;
                        drop(permiso); // libera el hueco al terminar
                    });
                }
                Err(e) => {
                    // Errores transitorios (p. ej. demasiados archivos abiertos): no tumbar el servidor
                    eprintln!("accept falló: {e}");
                    sleep(Duration::from_millis(100)).await;
                }
            },
            () = &mut apagar => break,
        }
        while conexiones.try_join_next().is_some() {} // recolecta las tareas ya terminadas
    }

    eprintln!("apagando: {} conexiones abiertas", conexiones.len());
    let _ = tx_apagar.send(true);
    let esperar = async { while conexiones.join_next().await.is_some() {} };
    if timeout(GRACIA_APAGADO, esperar).await.is_err() {
        eprintln!("apagado forzado: algunas conexiones no terminaron a tiempo");
        conexiones.abort_all();
        while conexiones.join_next().await.is_some() {}
    }
    let _ = purga.await;
    Ok(())
}

async fn purgar_periodicamente(almacen: Almacen, mut apagar: watch::Receiver<bool>) {
    let mut reloj = interval(Duration::from_secs(1));
    loop {
        tokio::select! {
            _ = reloj.tick() => { almacen.purgar(); }
            _ = apagar.changed() => break,
        }
    }
}

async fn manejar(socket: TcpStream, almacen: Almacen, mut apagar: watch::Receiver<bool>) {
    let _guardia = almacen.conexion_abierta();
    let _ = socket.set_nodelay(true); // respuestas cortas: no esperar a agrupar paquetes
    let (lectura, mut escritura) = socket.into_split();
    let mut lector = BufReader::new(lectura);
    let mut linea = String::new();

    loop {
        linea.clear();
        // `take` limita cuánto puede crecer una línea sin salto de línea
        let mut limitado = (&mut lector).take(MAX_LINEA);
        let leer = limitado.read_line(&mut linea);
        let leidos = tokio::select! {
            resultado = timeout(INACTIVIDAD, leer) => match resultado {
                Ok(Ok(n)) => n,
                Ok(Err(_)) => break, // E/S o UTF-8 inválido
                Err(_) => {
                    let _ = escritura.write_all(b"-ERR cerrada por inactividad\n").await;
                    break;
                }
            },
            _ = apagar.changed() => {
                let _ = escritura.write_all(b"-ERR servidor apagandose\n").await;
                break;
            }
        };
        if leidos == 0 {
            break; // el cliente cerró
        }
        if !linea.ends_with('\n') && leidos as u64 >= MAX_LINEA {
            let _ = escritura.write_all(b"-ERR linea demasiado larga\n").await;
            break;
        }

        let (respuesta, cerrar) = procesar_linea(&almacen, linea.trim_end_matches(['\r', '\n']));
        if escritura
            .write_all(respuesta.serializar().as_bytes())
            .await
            .is_err()
            || cerrar
        {
            break;
        }
    }
}
