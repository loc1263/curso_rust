// cargo run -p m20_concurrencia --example estado_compartido
// Memoria compartida: Arc, Mutex, RwLock, Condvar, Once/OnceLock.

use std::collections::HashMap;
use std::sync::{Arc, Condvar, Mutex, OnceLock, RwLock};
use std::thread;
use std::time::Duration;

static CONFIG: OnceLock<String> = OnceLock::new();

fn main() {
    // --- Mutex: exclusión mutua ---
    let cuenta = Arc::new(Mutex::new(0u64));
    let hilos: Vec<_> = (0..10)
        .map(|_| {
            let cuenta = Arc::clone(&cuenta);
            thread::spawn(move || {
                for _ in 0..1_000 {
                    *cuenta.lock().unwrap() += 1;
                }
            })
        })
        .collect();
    hilos.into_iter().for_each(|h| h.join().unwrap());
    println!("Mutex: {} (esperado 10000)", cuenta.lock().unwrap());

    // --- RwLock: muchos lectores o UN escritor ---
    let tabla = Arc::new(RwLock::new(HashMap::from([("rust", 1)])));
    {
        let lectores: Vec<_> = (0..3)
            .map(|i| {
                let t = Arc::clone(&tabla);
                thread::spawn(move || {
                    let r = t.read().unwrap(); // varios `read` simultáneos
                    println!("  lector {i} ve {:?}", r.get("rust"));
                })
            })
            .collect();
        lectores.into_iter().for_each(|h| h.join().unwrap());
    }
    tabla.write().unwrap().insert("go", 2); // acceso exclusivo
    println!("RwLock: {} entradas", tabla.read().unwrap().len());

    // --- Poisoning: si un hilo hace panic con el lock tomado, queda "envenenado" ---
    let m = Arc::new(Mutex::new(1));
    let m2 = Arc::clone(&m);
    let _ = thread::spawn(move || {
        let _g = m2.lock().unwrap();
        panic!("muero con el lock");
    })
    .join();
    println!("¿envenenado? {}", m.is_poisoned());
    let valor = m.lock().unwrap_or_else(|e| e.into_inner()); // recuperar el dato igualmente
    println!("dato recuperado = {}", *valor);
    drop(valor);

    // --- Condvar: esperar a que se cumpla una condición ---
    let par = Arc::new((Mutex::new(false), Condvar::new()));
    let par2 = Arc::clone(&par);
    let h = thread::spawn(move || {
        thread::sleep(Duration::from_millis(30));
        let (lock, cv) = &*par2;
        *lock.lock().unwrap() = true;
        cv.notify_one();
    });
    let (lock, cv) = &*par;
    let mut listo = lock.lock().unwrap();
    while !*listo {
        listo = cv.wait(listo).unwrap(); // libera el lock mientras duerme
    }
    println!("Condvar: la condición se cumplió");
    drop(listo);
    h.join().unwrap();

    // --- OnceLock: inicialización global perezosa y segura ---
    let c = CONFIG.get_or_init(|| {
        println!("inicializando CONFIG (una sola vez)");
        "modo=produccion".to_string()
    });
    println!("{c} / {}", CONFIG.get().unwrap());

    // Evitar deadlocks: toma los locks SIEMPRE en el mismo orden y no los mantengas
    // más de lo necesario.
}
