// cargo run -p m30_concurrencia_avanzada --example atomics_basicos
// Tipos atómicos: operaciones indivisibles sin bloqueos.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, AtomicUsize, Ordering};
use std::thread;
use std::time::Instant;

// Un contador global sin Mutex
// (nota: `try_update` se llamaba `fetch_update` en versiones anteriores de Rust)
static PETICIONES: AtomicUsize = AtomicUsize::new(0);

fn main() {
    // --- load / store / fetch_add ---
    let hilos: Vec<_> = (0..8)
        .map(|_| {
            thread::spawn(|| {
                for _ in 0..10_000 {
                    PETICIONES.fetch_add(1, Ordering::Relaxed); // lee-modifica-escribe INDIVISIBLE
                }
            })
        })
        .collect();
    hilos.into_iter().for_each(|h| h.join().unwrap());
    println!("fetch_add: {} (esperado 80000)", PETICIONES.load(Ordering::Relaxed));

    // Una versión NO atómica (load + store separados) puede perder actualizaciones: condición de carrera lógica.
    // Es no determinista: con hilos que no se solapan en el tiempo, a veces sale justo el total.
    let roto = Arc::new(AtomicUsize::new(0));
    let hilos: Vec<_> = (0..8)
        .map(|_| {
            let r = Arc::clone(&roto);
            thread::spawn(move || {
                for _ in 0..200_000 {
                    let v = r.load(Ordering::Relaxed);
                    r.store(v + 1, Ordering::Relaxed); // otro hilo pudo cambiarlo entre load y store
                }
            })
        })
        .collect();
    hilos.into_iter().for_each(|h| h.join().unwrap());
    let obtenido = roto.load(Ordering::Relaxed);
    println!("load+store separados: {obtenido} de 1600000 (incrementos perdidos: {})", 1_600_000 - obtenido);

    // --- swap, compare_exchange: base de todos los algoritmos sin bloqueo ---
    let a = AtomicI64::new(5);
    println!("swap devuelve el valor anterior: {}", a.swap(10, Ordering::SeqCst));
    // "Si vale 10, ponlo a 20": devuelve Ok(anterior) o Err(valor_actual)
    println!("{:?}", a.compare_exchange(10, 20, Ordering::SeqCst, Ordering::SeqCst));
    println!("{:?}", a.compare_exchange(10, 30, Ordering::SeqCst, Ordering::SeqCst));

    // --- Bucle CAS: aplicar cualquier función de forma atómica (aquí, un máximo) ---
    let maximo = Arc::new(AtomicU64::new(0));
    let hilos: Vec<_> = [17u64, 99, 42, 63]
        .into_iter()
        .map(|candidato| {
            let m = Arc::clone(&maximo);
            thread::spawn(move || {
                let mut actual = m.load(Ordering::Relaxed);
                while candidato > actual {
                    match m.compare_exchange_weak(actual, candidato, Ordering::Relaxed, Ordering::Relaxed) {
                        Ok(_) => break,
                        Err(visto) => actual = visto, // alguien nos ganó: reintentar con el valor nuevo
                    }
                }
            })
        })
        .collect();
    hilos.into_iter().for_each(|h| h.join().unwrap());
    println!("máximo concurrente = {} (también existe fetch_max)", maximo.load(Ordering::Relaxed));

    // try_update encapsula el bucle CAS
    let saldo = AtomicU64::new(100);
    let r = saldo.try_update(Ordering::SeqCst, Ordering::SeqCst, |s| s.checked_sub(150));
    println!("retirar 150 de 100 -> {r:?}");
    let r = saldo.try_update(Ordering::SeqCst, Ordering::SeqCst, |s| s.checked_sub(30));
    println!("retirar 30 -> {r:?}, saldo = {}", saldo.load(Ordering::SeqCst));

    // --- AtomicBool como bandera de parada ---
    let parar = Arc::new(AtomicBool::new(false));
    let vueltas = Arc::new(AtomicU64::new(0));
    let (p, v) = (Arc::clone(&parar), Arc::clone(&vueltas));
    let trabajador = thread::spawn(move || {
        while !p.load(Ordering::Relaxed) {
            v.fetch_add(1, Ordering::Relaxed);
            std::hint::spin_loop();
        }
    });
    thread::sleep(std::time::Duration::from_millis(30));
    parar.store(true, Ordering::Relaxed);
    trabajador.join().unwrap();
    println!("el trabajador paró tras {} vueltas", vueltas.load(Ordering::Relaxed));

    // --- ¿Atómico o Mutex? Medición rápida ---
    let t = Instant::now();
    let c = Arc::new(AtomicU64::new(0));
    let hs: Vec<_> = (0..4)
        .map(|_| {
            let c = Arc::clone(&c);
            thread::spawn(move || (0..500_000).for_each(|_| { c.fetch_add(1, Ordering::Relaxed); }))
        })
        .collect();
    hs.into_iter().for_each(|h| h.join().unwrap());
    let t_atomico = t.elapsed();
    let t = Instant::now();
    let m = Arc::new(std::sync::Mutex::new(0u64));
    let hs: Vec<_> = (0..4)
        .map(|_| {
            let m = Arc::clone(&m);
            thread::spawn(move || (0..500_000).for_each(|_| *m.lock().unwrap() += 1))
        })
        .collect();
    hs.into_iter().for_each(|h| h.join().unwrap());
    println!("4 hilos × 500 000 incrementos: atómico {t_atomico:?} | Mutex {:?}", t.elapsed());
}
