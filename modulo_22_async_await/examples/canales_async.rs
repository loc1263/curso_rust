// cargo run -p m22_async_await --example canales_async
// Canales y primitivas de sincronización async de tokio.

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, Semaphore, broadcast, mpsc, oneshot, watch};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    // --- mpsc: muchos productores, un consumidor (con capacidad = contrapresión) ---
    let (tx, mut rx) = mpsc::channel::<String>(4);
    for id in 0..3 {
        let tx = tx.clone();
        tokio::spawn(async move {
            for n in 0..2 {
                tx.send(format!("productor {id}, mensaje {n}"))
                    .await
                    .unwrap();
                sleep(Duration::from_millis(10)).await;
            }
        });
    }
    drop(tx); // sin esto `recv` nunca devolvería None
    let mut recibidos = Vec::new();
    while let Some(m) = rx.recv().await {
        recibidos.push(m);
    }
    recibidos.sort();
    println!(
        "mpsc: {} mensajes, primero = {:?}\n",
        recibidos.len(),
        recibidos[0]
    );

    // --- oneshot: una sola respuesta (patrón petición/respuesta) ---
    let (resp_tx, resp_rx) = oneshot::channel();
    tokio::spawn(async move {
        sleep(Duration::from_millis(20)).await;
        let _ = resp_tx.send(42);
    });
    println!("oneshot: respuesta = {:?}\n", resp_rx.await);

    // --- broadcast: cada receptor recibe TODOS los mensajes ---
    let (btx, mut rx1) = broadcast::channel(16);
    let mut rx2 = btx.subscribe();
    btx.send("noticia 1").unwrap();
    btx.send("noticia 2").unwrap();
    println!(
        "broadcast rx1: {:?} {:?}",
        rx1.recv().await,
        rx1.recv().await
    );
    println!("broadcast rx2: {:?}\n", rx2.recv().await);

    // --- watch: un valor que cambia; los receptores ven siempre el ÚLTIMO ---
    let (wtx, mut wrx) = watch::channel("config v1".to_string());
    let observador = tokio::spawn(async move {
        while wrx.changed().await.is_ok() {
            println!("watch: nueva configuración -> {}", *wrx.borrow());
        }
    });
    wtx.send("config v2".into()).unwrap();
    sleep(Duration::from_millis(10)).await;
    wtx.send("config v3".into()).unwrap();
    sleep(Duration::from_millis(10)).await;
    drop(wtx);
    observador.await.unwrap();
    println!();

    // --- Mutex async: se puede mantener bloqueado a través de un .await ---
    let contador = Arc::new(Mutex::new(0));
    let hs: Vec<_> = (0..5)
        .map(|_| {
            let c = Arc::clone(&contador);
            tokio::spawn(async move {
                let mut g = c.lock().await;
                *g += 1;
                sleep(Duration::from_millis(5)).await; // con std::sync::Mutex esto sería problemático
            })
        })
        .collect();
    for h in hs {
        h.await.unwrap();
    }
    println!("Mutex async: {}\n", *contador.lock().await);

    // --- Semaphore: limitar la concurrencia (p. ej. 2 peticiones a la vez) ---
    let sem = Arc::new(Semaphore::new(2));
    let activas = Arc::new(Mutex::new((0u32, 0u32))); // (actuales, máximo visto)
    let hs: Vec<_> = (0..6)
        .map(|_| {
            let (sem, activas) = (Arc::clone(&sem), Arc::clone(&activas));
            tokio::spawn(async move {
                let _permiso = sem.acquire().await.unwrap();
                {
                    let mut a = activas.lock().await;
                    a.0 += 1;
                    a.1 = a.1.max(a.0);
                }
                sleep(Duration::from_millis(20)).await;
                activas.lock().await.0 -= 1;
            })
        })
        .collect();
    for h in hs {
        h.await.unwrap();
    }
    println!(
        "Semaphore: máximo de tareas simultáneas = {}",
        activas.lock().await.1
    );
}
