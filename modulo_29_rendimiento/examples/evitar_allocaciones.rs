// cargo run --release -p m29_rendimiento --example evitar_allocaciones
// Medir cuántas veces se reserva memoria con un asignador global que cuenta.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

// Envolvemos el asignador del sistema y contamos las llamadas
struct Contador;
static ASIGNACIONES: AtomicUsize = AtomicUsize::new(0);
static BYTES: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Contador {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ASIGNACIONES.fetch_add(1, Relaxed);
        BYTES.fetch_add(layout.size(), Relaxed);
        // SAFETY: delegamos en el asignador del sistema con el mismo layout
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: ptr fue asignado con este layout por System
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static A: Contador = Contador;

/// Ejecuta `f` y devuelve (resultado, asignaciones, bytes).
fn medir<R>(f: impl FnOnce() -> R) -> (R, usize, usize) {
    let (a0, b0) = (ASIGNACIONES.load(Relaxed), BYTES.load(Relaxed));
    let r = f();
    (r, ASIGNACIONES.load(Relaxed) - a0, BYTES.load(Relaxed) - b0)
}

fn informe(titulo: &str, asignaciones: usize, bytes: usize) {
    println!("{titulo:<44} {asignaciones:>6} asignaciones, {bytes:>8} bytes");
}

fn main() {
    // 1) Vec sin capacidad previa: reasigna al crecer (duplicando)
    let (_, n, b) = medir(|| {
        let mut v = Vec::new();
        for i in 0..10_000 {
            v.push(i);
        }
        v
    });
    informe("Vec::new() + 10 000 push", n, b);

    // 2) Con capacidad reservada: una sola asignación
    let (_, n, b) = medir(|| {
        let mut v = Vec::with_capacity(10_000);
        for i in 0..10_000 {
            v.push(i);
        }
        v
    });
    informe("Vec::with_capacity(10 000) + push", n, b);

    // 3) Construir cadenas: format! en bucle vs reutilizar un buffer
    let (_, n, b) = medir(|| {
        let mut total = 0;
        for i in 0..1_000 {
            let s = format!("item-{i}"); // 1 asignación por iteración
            total += s.len();
        }
        total
    });
    informe("format! nuevo en cada iteración", n, b);

    let (_, n, b) = medir(|| {
        use std::fmt::Write;
        let mut buf = String::with_capacity(32);
        let mut total = 0;
        for i in 0..1_000 {
            buf.clear(); // conserva la capacidad
            write!(buf, "item-{i}").unwrap();
            total += buf.len();
        }
        total
    });
    informe("un String reutilizado + write!", n, b);

    // 4) Clonar vs prestar
    let datos: Vec<String> = (0..1_000).map(|i| i.to_string()).collect();
    let (_, n, b) = medir(|| datos.clone().len());
    informe("clonar Vec<String> de 1 000", n, b);
    let (_, n, b) = medir(|| datos.iter().map(|s| s.len()).sum::<usize>());
    informe("iterar prestando (&String)", n, b);

    // 5) Box<dyn Trait> por elemento vs enum
    trait Op {
        fn aplicar(&self, x: i32) -> i32;
    }
    struct Suma(i32);
    struct Doble;
    impl Op for Suma {
        fn aplicar(&self, x: i32) -> i32 {
            x + self.0
        }
    }
    impl Op for Doble {
        fn aplicar(&self, x: i32) -> i32 {
            x * 2
        }
    }
    let (_, n, b) = medir(|| {
        let ops: Vec<Box<dyn Op>> = (0..1_000)
            .map(|i| {
                if i % 2 == 0 {
                    Box::new(Suma(i)) as Box<dyn Op>
                } else {
                    Box::new(Doble)
                }
            })
            .collect();
        ops.iter().fold(0, |acc, o| o.aplicar(acc) % 1000)
    });
    informe("1 000 Box<dyn Op>", n, b);

    enum OpEnum {
        Suma(i32),
        Doble,
    }
    let (_, n, b) = medir(|| {
        let ops: Vec<OpEnum> = (0..1_000)
            .map(|i| {
                if i % 2 == 0 {
                    OpEnum::Suma(i)
                } else {
                    OpEnum::Doble
                }
            })
            .collect();
        ops.iter().fold(0, |acc, o| match o {
            OpEnum::Suma(k) => acc + k,
            OpEnum::Doble => acc * 2,
        } % 1000)
    });
    informe("1 000 enum OpEnum (en un solo Vec)", n, b);

    // 6) Arrays en pila para tamaños pequeños y conocidos
    let (_, n, b) = medir(|| {
        let buf = [0u8; 256]; // pila: 0 asignaciones
        buf.iter().map(|&x| x as u32).sum::<u32>()
    });
    informe("array [u8; 256] en la pila", n, b);

    println!(
        "\nSugerencias: with_capacity, reutilizar buffers (clear), &str/&[T] en vez de clonar,"
    );
    println!(
        "enum en vez de Box<dyn>, SmallVec/ArrayVec para colecciones pequeñas, Cow para clonar solo si hace falta."
    );
}
