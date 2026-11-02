// cargo run -p m20_concurrencia --example canales_mpsc
// Comunicación por mensajes: "no comuniques compartiendo memoria; comparte memoria comunicando".

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

enum Mensaje {
    Trabajo(u32),
    Terminar,
}

fn main() {
    // mpsc = multiple producer, single consumer
    let (tx, rx) = mpsc::channel();
    let h = thread::spawn(move || {
        for s in ["uno", "dos", "tres"] {
            tx.send(s.to_string()).unwrap(); // send mueve el valor al canal
            thread::sleep(Duration::from_millis(5));
        }
    }); // tx se destruye aquí -> el canal se cierra
    for recibido in rx {
        // iterar sobre rx termina cuando todos los emisores se destruyen
        println!("recibido: {recibido}");
    }
    h.join().unwrap();

    // Varios productores: clonar el emisor
    let (tx, rx) = mpsc::channel();
    let mut hs = vec![];
    for id in 0..3 {
        let tx = tx.clone();
        hs.push(thread::spawn(move || {
            for n in 0..2 {
                tx.send((id, n)).unwrap();
            }
        }));
    }
    drop(tx); // ¡importante! si no, el `for` de abajo no terminaría nunca
    let mut todos: Vec<(i32, i32)> = rx.iter().collect();
    todos.sort();
    println!("{todos:?}");
    for h in hs {
        h.join().unwrap();
    }

    // Patrón worker: un hilo trabajador que procesa órdenes hasta recibir Terminar
    let (tx_trabajo, rx_trabajo) = mpsc::channel();
    let (tx_resultado, rx_resultado) = mpsc::channel();
    let worker = thread::spawn(move || {
        while let Ok(msg) = rx_trabajo.recv() {
            match msg {
                Mensaje::Trabajo(n) => tx_resultado.send(n * n).unwrap(),
                Mensaje::Terminar => break,
            }
        }
    });
    for n in 1..=4 {
        tx_trabajo.send(Mensaje::Trabajo(n)).unwrap();
    }
    tx_trabajo.send(Mensaje::Terminar).unwrap();
    worker.join().unwrap();
    let cuadrados: Vec<u32> = rx_resultado.try_iter().collect();
    println!("cuadrados calculados por el worker: {cuadrados:?}");

    // try_recv / recv_timeout: no bloquean indefinidamente
    let (_tx, rx) = mpsc::channel::<i32>();
    println!("try_recv: {:?}", rx.try_recv());
    println!(
        "recv_timeout: {:?}",
        rx.recv_timeout(Duration::from_millis(20))
    );

    // sync_channel: canal con capacidad limitada (aplica contrapresión)
    let (tx, rx) = mpsc::sync_channel(2);
    let prod = thread::spawn(move || {
        for i in 0..5 {
            tx.send(i).unwrap(); // se bloquea si hay 2 pendientes
        }
    });
    thread::sleep(Duration::from_millis(20));
    println!("sync_channel: {:?}", rx.iter().collect::<Vec<_>>());
    prod.join().unwrap();
}
