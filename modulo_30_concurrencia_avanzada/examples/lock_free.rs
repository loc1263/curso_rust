// cargo run --release -p m30_concurrencia_avanzada --example lock_free
// Estructuras sin (o con pocos) bloqueos: spinlock, cola SPSC de anillo y falso compartido (false sharing).

use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::thread;
use std::time::Instant;

// =====================================================================
// 1) Spinlock: un Mutex mínimo que "gira" esperando. Solo para secciones MUY cortas.
// =====================================================================
pub struct Spinlock<T> {
    bloqueado: AtomicBool,
    dato: UnsafeCell<T>,
}

// SAFETY: el spinlock garantiza acceso exclusivo a `dato`
unsafe impl<T: Send> Sync for Spinlock<T> {}

pub struct Guardia<'a, T> {
    lock: &'a Spinlock<T>,
}

impl<T> Spinlock<T> {
    pub const fn new(dato: T) -> Self {
        Spinlock { bloqueado: AtomicBool::new(false), dato: UnsafeCell::new(dato) }
    }

    pub fn lock(&self) -> Guardia<'_, T> {
        // Acquire al tomar el lock: vemos todo lo que escribió quien lo soltó (Release)
        while self.bloqueado.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            // "test-and-test-and-set": esperamos con lecturas baratas hasta que parezca libre
            while self.bloqueado.load(Ordering::Relaxed) {
                std::hint::spin_loop();
            }
        }
        Guardia { lock: self }
    }
}

impl<T> Deref for Guardia<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: tenemos el lock
        unsafe { &*self.lock.dato.get() }
    }
}
impl<T> DerefMut for Guardia<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: tenemos el lock en exclusiva
        unsafe { &mut *self.lock.dato.get() }
    }
}
impl<T> Drop for Guardia<'_, T> {
    fn drop(&mut self) {
        // Release al soltar: publica todo lo escrito dentro de la sección crítica
        self.lock.bloqueado.store(false, Ordering::Release);
    }
}

// =====================================================================
// 2) Cola SPSC (un productor, un consumidor) de anillo, sin bloqueos ni esperas por lock
// =====================================================================
struct Anillo<T, const N: usize> {
    buffer: [UnsafeCell<MaybeUninit<T>>; N],
    cabeza: AtomicUsize, // próxima posición a LEER (la modifica el consumidor)
    cola: AtomicUsize,   // próxima posición a ESCRIBIR (la modifica el productor)
}

// SAFETY: con un único productor y un único consumidor, cada celda tiene un solo escritor a la vez,
// y los índices Release/Acquire ordenan el acceso a las celdas.
unsafe impl<T: Send, const N: usize> Sync for Anillo<T, N> {}

pub struct Productor<T, const N: usize>(Arc<Anillo<T, N>>);
pub struct Consumidor<T, const N: usize>(Arc<Anillo<T, N>>);

pub fn cola_spsc<T, const N: usize>() -> (Productor<T, N>, Consumidor<T, N>) {
    let a = Arc::new(Anillo {
        buffer: std::array::from_fn(|_| UnsafeCell::new(MaybeUninit::uninit())),
        cabeza: AtomicUsize::new(0),
        cola: AtomicUsize::new(0),
    });
    (Productor(Arc::clone(&a)), Consumidor(a))
}

impl<T, const N: usize> Productor<T, N> {
    /// Devuelve `Err(v)` si la cola está llena.
    pub fn enviar(&self, v: T) -> Result<(), T> {
        let a = &*self.0;
        let cola = a.cola.load(Ordering::Relaxed); // solo nosotros la modificamos
        let cabeza = a.cabeza.load(Ordering::Acquire);
        if cola - cabeza == N {
            return Err(v); // llena
        }
        // SAFETY: la celda `cola % N` no está ocupada (cola - cabeza < N) y solo nosotros escribimos
        unsafe { (*a.buffer[cola % N].get()).write(v) };
        a.cola.store(cola + 1, Ordering::Release); // publica la escritura al consumidor
        Ok(())
    }
}

impl<T, const N: usize> Consumidor<T, N> {
    pub fn recibir(&self) -> Option<T> {
        let a = &*self.0;
        let cabeza = a.cabeza.load(Ordering::Relaxed);
        let cola = a.cola.load(Ordering::Acquire); // sincroniza con el Release del productor
        if cabeza == cola {
            return None; // vacía
        }
        // SAFETY: la celda fue inicializada por el productor (visible por Acquire) y la leemos una sola vez
        let v = unsafe { (*a.buffer[cabeza % N].get()).assume_init_read() };
        a.cabeza.store(cabeza + 1, Ordering::Release); // libera la celda al productor
        Some(v)
    }
}

impl<T, const N: usize> Drop for Anillo<T, N> {
    fn drop(&mut self) {
        // Liberar los elementos que quedaron sin consumir
        let (mut i, fin) = (*self.cabeza.get_mut(), *self.cola.get_mut());
        while i != fin {
            // SAFETY: las celdas entre cabeza y cola están inicializadas
            unsafe { self.buffer[i % N].get_mut().assume_init_drop() };
            i += 1;
        }
    }
}

// =====================================================================
// 3) False sharing: dos contadores en la misma línea de caché se "pelean" aunque sean independientes
// =====================================================================
#[repr(align(64))] // una línea de caché por contador
struct Separado(AtomicU64);

fn medir_false_sharing() {
    const VUELTAS: u64 = 20_000_000;
    let juntos = Arc::new([AtomicU64::new(0), AtomicU64::new(0)]);
    let t = Instant::now();
    let hs: Vec<_> = (0..2)
        .map(|i| {
            let j = Arc::clone(&juntos);
            thread::spawn(move || (0..VUELTAS).for_each(|_| { j[i].fetch_add(1, Ordering::Relaxed); }))
        })
        .collect();
    hs.into_iter().for_each(|h| h.join().unwrap());
    let t_juntos = t.elapsed();

    let separados = Arc::new([Separado(AtomicU64::new(0)), Separado(AtomicU64::new(0))]);
    let t = Instant::now();
    let hs: Vec<_> = (0..2)
        .map(|i| {
            let s = Arc::clone(&separados);
            thread::spawn(move || (0..VUELTAS).for_each(|_| { s[i].0.fetch_add(1, Ordering::Relaxed); }))
        })
        .collect();
    hs.into_iter().for_each(|h| h.join().unwrap());
    println!("contadores adyacentes: {t_juntos:>9.2?} | en líneas de caché distintas: {:>9.2?}", t.elapsed());
}

fn main() {
    // --- Spinlock ---
    let lock = Arc::new(Spinlock::new(0u64));
    let hs: Vec<_> = (0..4)
        .map(|_| {
            let l = Arc::clone(&lock);
            thread::spawn(move || {
                for _ in 0..100_000 {
                    *l.lock() += 1;
                }
            })
        })
        .collect();
    hs.into_iter().for_each(|h| h.join().unwrap());
    println!("Spinlock: {} (esperado 400000)", *lock.lock());

    // --- Cola SPSC ---
    let (tx, rx) = cola_spsc::<u64, 1024>();
    let total = 1_000_000u64;
    let t = Instant::now();
    let prod = thread::spawn(move || {
        for i in 1..=total {
            let mut v = i;
            while let Err(devuelto) = tx.enviar(v) {
                v = devuelto; // llena: reintentar
                std::hint::spin_loop();
            }
        }
    });
    let cons = thread::spawn(move || {
        let (mut suma, mut recibidos) = (0u64, 0u64);
        while recibidos < total {
            if let Some(v) = rx.recibir() {
                suma += v;
                recibidos += 1;
            } else {
                std::hint::spin_loop();
            }
        }
        suma
    });
    prod.join().unwrap();
    let suma = cons.join().unwrap();
    println!("SPSC: {total} mensajes en {:?}, suma = {suma} (esperado {})", t.elapsed(), total * (total + 1) / 2);

    // --- False sharing ---
    medir_false_sharing();

    println!("\nSobre estructuras lock-free generales (pilas Treiber, colas Michael-Scott):");
    println!("  - El difícil problema es la RECLAMACIÓN de memoria (ABA, use-after-free).");
    println!("  - No las escribas tú en producción: usa `crossbeam` (epoch GC, SegQueue, ArrayQueue, deque).");
}
