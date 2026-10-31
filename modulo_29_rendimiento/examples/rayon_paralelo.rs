// cargo run --release -p m29_rendimiento --example rayon_paralelo
// Paralelismo de datos con rayon: cambiar `iter()` por `par_iter()`.

use m29_rendimiento::*;
use rayon::prelude::*;
use std::time::Instant;

fn cronometrar<R>(nombre: &str, f: impl FnOnce() -> R) -> R {
    let t = Instant::now();
    let r = f();
    println!("{nombre:<34} {:>10.2?}", t.elapsed());
    r
}

fn es_primo(n: u64) -> bool {
    n >= 2 && (2..).take_while(|d| d * d <= n).all(|d| n % d != 0)
}

fn main() {
    if cfg!(debug_assertions) {
        println!("AVISO: usa --release para obtener cifras significativas.\n");
    }
    println!("hilos de rayon: {}\n", rayon::current_num_threads());

    // 1) Carga ligera por elemento: el coste de repartir el trabajo puede superar la ganancia
    let datos: Vec<u64> = (0..20_000_000).collect();
    let a = cronometrar("suma cuadrados (secuencial)", || suma_cuadrados_iter(&datos));
    let b = cronometrar("suma cuadrados (rayon)", || suma_cuadrados_par(&datos));
    assert_eq!(a, b);

    // 2) Carga pesada por elemento: aquí rayon brilla
    let limite = 2_000_000;
    let seq = cronometrar("primos hasta 2M (secuencial)", || (0..limite).filter(|&n| es_primo(n)).count());
    let par = cronometrar("primos hasta 2M (rayon)", || (0..limite).into_par_iter().filter(|&n| es_primo(n)).count());
    assert_eq!(seq, par);
    println!("  -> {par} primos\n");

    // 3) Ordenación paralela
    let base = datos_pseudoaleatorios(5_000_000, 1);
    let mut v1 = base.clone();
    let mut v2 = base.clone();
    cronometrar("sort_unstable (secuencial)", || v1.sort_unstable());
    cronometrar("par_sort_unstable (rayon)", || v2.par_sort_unstable());
    assert_eq!(v1, v2);

    // 4) Operaciones típicas: map + reduce, chunks, join (divide y vencerás)
    let total: u64 = (1..=1_000u64).into_par_iter().map(|x| x * x).reduce(|| 0, |a, b| a + b);
    println!("\nmap + reduce = {total}");
    let (izq, der) = rayon::join(|| (1..=10_000u64).sum::<u64>(), || (10_001..=20_000u64).sum::<u64>());
    println!("rayon::join   = {izq} + {der}");

    // 5) Rayon garantiza ausencia de data races en compilación: no puedes mutar un contador
    //    compartido desde varias hebras sin Mutex/atómicos. Prefiere reducciones a estado compartido.
    println!("\nRegla: mide antes de paralelizar; reparte trabajo GRANDE por elemento o datos MUY grandes.");
}
