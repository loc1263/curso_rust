// cargo run -p m20_concurrencia --example hilos
// Crear hilos, `join`, `move`, hilos con ámbito (`thread::scope`) y paniqueo en un hilo.

use std::thread;
use std::time::Duration;

fn main() {
    // spawn devuelve un JoinHandle; `join` espera y recoge el valor de retorno
    let h = thread::spawn(|| {
        for i in 1..=3 {
            println!("  [hilo] paso {i}");
            thread::sleep(Duration::from_millis(10));
        }
        "terminé"
    });
    for i in 1..=3 {
        println!("[main] paso {i}");
        thread::sleep(Duration::from_millis(10));
    }
    println!("el hilo dice: {}", h.join().unwrap());

    // `move` transfiere la propiedad al hilo (el hilo puede vivir más que la función)
    let datos = vec![1, 2, 3];
    let h = thread::spawn(move || datos.iter().sum::<i32>());
    println!("suma en hilo = {}", h.join().unwrap());

    // Varios hilos: cada uno calcula una parte
    let trabajos: Vec<_> = (0..4)
        .map(|id| thread::spawn(move || (id, (id * 100..(id + 1) * 100).sum::<u32>())))
        .collect();
    for t in trabajos {
        let (id, parcial) = t.join().unwrap();
        println!("trozo {id}: {parcial}");
    }

    // thread::scope: los hilos pueden PRESTAR datos locales (sin Arc ni 'static)
    let mut numeros = vec![1, 2, 3, 4, 5, 6];
    let (izq, der) = numeros.split_at_mut(3);
    thread::scope(|s| {
        s.spawn(|| izq.iter_mut().for_each(|x| *x *= 10));
        s.spawn(|| der.iter_mut().for_each(|x| *x += 1));
    }); // todos los hilos terminan aquí
    println!("{numeros:?}");

    // Un pánico en un hilo no tumba el programa: `join` devuelve Err
    let h = thread::spawn(|| {
        if true {
            panic!("fallo en el hilo");
        }
    });
    match h.join() {
        Ok(_) => println!("sin pánico"),
        Err(_) => println!("el hilo hizo panic (capturado con join)"),
    }

    println!("hilos disponibles: {:?}", thread::available_parallelism().map(|n| n.get()));
}
