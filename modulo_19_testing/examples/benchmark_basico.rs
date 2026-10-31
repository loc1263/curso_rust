// cargo run --release -p m19_testing --example benchmark_basico
// Benchmark casero con `Instant` y `black_box`.
// Para mediciones serias (estadística, comparación entre ejecuciones) usa `criterion` (módulo 29).

use std::hint::black_box;
use std::time::Instant;

fn suma_bucle(n: u64) -> u64 {
    let mut s = 0;
    for i in 0..n {
        s += i;
    }
    s
}

fn suma_iter(n: u64) -> u64 {
    (0..n).sum()
}

fn medir<F: FnMut() -> u64>(nombre: &str, repeticiones: u32, mut f: F) {
    // Calentamiento
    for _ in 0..3 {
        black_box(f());
    }
    let inicio = Instant::now();
    for _ in 0..repeticiones {
        // black_box evita que el compilador elimine el cálculo por "no usarse"
        black_box(f());
    }
    let total = inicio.elapsed();
    println!("{nombre:<12} {:>10.2?} por iteración", total / repeticiones);
}

fn main() {
    if cfg!(debug_assertions) {
        println!("AVISO: ejecuta con --release para que la medición tenga sentido.\n");
    }
    let n = black_box(10_000_000);
    medir("bucle for", 50, || suma_bucle(n));
    medir("iter().sum", 50, || suma_iter(n));
}
