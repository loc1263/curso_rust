// cargo run -p m22_async_await --example basico
// async/await con tokio: tareas, concurrencia vs secuencial, `spawn_blocking`.

use std::time::{Duration, Instant};
use tokio::time::sleep;

// Una `async fn` devuelve un Future: NO hace nada hasta que se le hace `.await`
async fn tarea(nombre: &str, ms: u64) -> String {
    println!("  [{nombre}] empieza");
    sleep(Duration::from_millis(ms)).await; // cede el hilo mientras espera
    println!("  [{nombre}] termina");
    format!("{nombre} ({ms} ms)")
}

#[tokio::main] // crea el runtime y ejecuta main dentro de él
async fn main() {
    // 1) Secuencial: la segunda no empieza hasta que acaba la primera
    let t = Instant::now();
    println!("Secuencial:");
    tarea("A", 100).await;
    tarea("B", 100).await;
    println!("  total: {:?}\n", t.elapsed());

    // 2) Concurrente con join!: ambas avanzan a la vez en la MISMA tarea
    let t = Instant::now();
    println!("Concurrente con join!:");
    let (a, b) = tokio::join!(tarea("A", 100), tarea("B", 100));
    println!("  resultados: {a} / {b}");
    println!("  total: {:?}\n", t.elapsed());

    // 3) tokio::spawn: cada tarea es independiente y puede ejecutarse en paralelo
    let t = Instant::now();
    println!("Con spawn:");
    let handles: Vec<_> = (1..=3)
        .map(|i| tokio::spawn(async move { tarea(&format!("T{i}"), 50 * i).await }))
        .collect();
    for h in handles {
        println!("  -> {}", h.await.unwrap()); // JoinHandle devuelve Result (por si la tarea hace panic)
    }
    println!("  total: {:?}\n", t.elapsed());

    // 4) Un async block es un Future anónimo; `async move` toma posesión de lo que captura
    let saludo = String::from("hola desde un async block");
    let fut = async move { saludo.len() };
    println!("longitud = {}", fut.await);

    // 5) Las tareas que hacen panic no tumban el runtime
    let h = tokio::spawn(async { panic!("¡boom!") });
    match h.await {
        Err(e) if e.is_panic() => println!("la tarea hizo panic, el programa sigue\n"),
        _ => unreachable!(),
    }

    // 6) CPU intensivo o E/S bloqueante: NO bloquees el runtime con std::thread::sleep;
    //    muévelo a un hilo aparte con spawn_blocking
    let t = Instant::now();
    let pesado = tokio::task::spawn_blocking(|| {
        std::thread::sleep(Duration::from_millis(100)); // bloqueante, pero en el pool de bloqueo
        (1..=1_000_000u64).sum::<u64>()
    });
    let (suma, _) = tokio::join!(async { pesado.await.unwrap() }, tarea("mientras tanto", 50));
    println!("suma = {suma} ({:?})", t.elapsed());
}
