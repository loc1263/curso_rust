// cargo run --release -p m30_concurrencia_avanzada --example orderings
// Orderings y el modelo de memoria: qué garantiza cada uno.
//
// Resumen:
//   Relaxed  - solo atomicidad de ESA variable; sin orden respecto a otras operaciones.
//   Release  - (en stores) todo lo escrito ANTES queda visible para quien haga Acquire de este valor.
//   Acquire  - (en loads) lo que veas tras este load incluye todo lo anterior al Release correspondiente.
//   AcqRel   - ambos (para read-modify-write: fetch_add, compare_exchange...).
//   SeqCst   - además, todos los SeqCst comparten UN orden total global. Lo más simple y más caro.

use std::cell::UnsafeCell;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering, fence};
use std::thread;

// ---------- 1) Release / Acquire: publicar datos ----------
// Un dato NO atómico protegido por una bandera atómica.
struct Mensaje {
    dato: UnsafeCell<String>,
    listo: AtomicBool,
}
// SAFETY: el acceso a `dato` está sincronizado por `listo` (Release/Acquire)
unsafe impl Sync for Mensaje {}

fn publicar_con_release_acquire() {
    let m = Arc::new(Mensaje {
        dato: UnsafeCell::new(String::new()),
        listo: AtomicBool::new(false),
    });

    let productor = {
        let m = Arc::clone(&m);
        thread::spawn(move || {
            // SAFETY: nadie lee `dato` hasta que `listo` sea true
            unsafe { *m.dato.get() = "datos importantes".to_string() };
            // Release: la escritura anterior queda visible para quien vea `listo == true` con Acquire
            m.listo.store(true, Ordering::Release);
        })
    };

    let consumidor = {
        let m = Arc::clone(&m);
        thread::spawn(move || {
            // Acquire: si vemos true, vemos también el String escrito antes del Release
            while !m.listo.load(Ordering::Acquire) {
                std::hint::spin_loop();
            }
            // SAFETY: el productor ya terminó de escribir (happens-before por Release/Acquire)
            unsafe { (*m.dato.get()).clone() }
        })
    };
    productor.join().unwrap();
    println!(
        "Release/Acquire: el consumidor leyó {:?}",
        consumidor.join().unwrap()
    );
    // Con Relaxed en ambos lados esto sería una data race (UB): no hay garantía de ver el String.
}

// ---------- 2) Relaxed: contadores/estadísticas independientes ----------
fn contador_relaxed() {
    let c = Arc::new(AtomicUsize::new(0));
    let hs: Vec<_> = (0..4)
        .map(|_| {
            let c = Arc::clone(&c);
            thread::spawn(move || {
                (0..1000).for_each(|_| {
                    c.fetch_add(1, Ordering::Relaxed);
                })
            })
        })
        .collect();
    hs.into_iter().for_each(|h| h.join().unwrap());
    // join() ya sincroniza: tras él vemos el valor final aunque todo fuera Relaxed
    println!(
        "Relaxed: contador = {} (no publica ningún otro dato, por eso basta)",
        c.load(Ordering::Relaxed)
    );
}

// ---------- 3) Fences: separar la sincronización de la operación atómica ----------
fn con_fences() {
    static DATO: AtomicU32 = AtomicU32::new(0);
    static BANDERA: AtomicBool = AtomicBool::new(false);
    let w = thread::spawn(|| {
        DATO.store(42, Ordering::Relaxed);
        fence(Ordering::Release);
        BANDERA.store(true, Ordering::Relaxed);
    });
    let r = thread::spawn(|| {
        while !BANDERA.load(Ordering::Relaxed) {
            std::hint::spin_loop();
        }
        fence(Ordering::Acquire);
        DATO.load(Ordering::Relaxed)
    });
    w.join().unwrap();
    println!("fence Release/Acquire: dato = {}", r.join().unwrap());
}

// ---------- 4) SeqCst: el patrón donde Release/Acquire NO basta (Dekker / "store buffering") ----------
// Dos hilos: cada uno pone su bandera y luego mira la del otro. Con SeqCst es IMPOSIBLE que ambos
// vean `false`; con Release/Acquire sí podría pasar (en hardware débil, como ARM, o por reordenación).
fn dekker_seqcst() {
    let mut ambos_false = 0;
    for _ in 0..2_000 {
        let (x, y) = (
            Arc::new(AtomicBool::new(false)),
            Arc::new(AtomicBool::new(false)),
        );
        let (x1, y1, x2, y2) = (
            Arc::clone(&x),
            Arc::clone(&y),
            Arc::clone(&x),
            Arc::clone(&y),
        );
        let a = thread::spawn(move || {
            x1.store(true, Ordering::SeqCst);
            y1.load(Ordering::SeqCst)
        });
        let b = thread::spawn(move || {
            y2.store(true, Ordering::SeqCst);
            x2.load(Ordering::SeqCst)
        });
        if !a.join().unwrap() && !b.join().unwrap() {
            ambos_false += 1;
        }
    }
    println!("SeqCst (Dekker): veces que ambos vieron false = {ambos_false} (siempre 0)");
}

fn main() {
    publicar_con_release_acquire();
    contador_relaxed();
    con_fences();
    dekker_seqcst();

    println!("\nGuía rápida:");
    println!("  - ¿Solo un contador o estadística?              -> Relaxed");
    println!(
        "  - ¿Publicar datos con una bandera?              -> Release (escritor) + Acquire (lector)"
    );
    println!(
        "  - ¿Leer-modificar-escribir que publica datos?   -> AcqRel (o Acquire/Release en el CAS)"
    );
    println!("  - ¿No estás seguro / razonar con varias variables? -> SeqCst");
    println!(
        "  - x86 es un modelo fuerte: errores de ordering pueden pasar desapercibidos allí y romper en ARM."
    );
    println!("  - Verifica con `loom` (explora intercalados) y Miri (detecta data races).");
}
