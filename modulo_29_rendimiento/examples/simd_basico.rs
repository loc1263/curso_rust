// cargo run --release -p m29_rendimiento --example simd_basico
// SIMD: una instrucción, varios datos. Autovectorización vs intrínsecos de `std::arch`.

use std::hint::black_box;
use std::time::Instant;

// 1) Autovectorización: escribe bucles simples sobre slices y el compilador usa SIMD (en --release)
fn suma_escalar(v: &[f32]) -> f32 {
    v.iter().sum() // los f32 NO se reordenan por asociatividad -> no se vectoriza la reducción
}

// Con acumuladores independientes damos libertad al compilador para vectorizar
fn suma_autovectorizable(v: &[f32]) -> f32 {
    let mut acc = [0.0f32; 8];
    let mut trozos = v.chunks_exact(8);
    for t in &mut trozos {
        for i in 0..8 {
            acc[i] += t[i];
        }
    }
    acc.iter().sum::<f32>() + trozos.remainder().iter().sum::<f32>()
}

// 2) Intrínsecos x86_64 con detección en tiempo de ejecución
#[cfg(target_arch = "x86_64")]
fn suma_avx(v: &[f32]) -> Option<f32> {
    if !is_x86_feature_detected!("avx") {
        return None;
    }
    // SAFETY: acabamos de comprobar que la CPU soporta AVX
    Some(unsafe { suma_avx_impl(v) })
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx")]
unsafe fn suma_avx_impl(v: &[f32]) -> f32 {
    use std::arch::x86_64::*;
    // SAFETY: AVX habilitado por target_feature; los accesos están dentro de los límites
    unsafe {
        let mut acumulador = _mm256_setzero_ps();
        let mut trozos = v.chunks_exact(8);
        for t in &mut trozos {
            // carga 8 f32 (sin exigir alineación) y suma en paralelo
            acumulador = _mm256_add_ps(acumulador, _mm256_loadu_ps(t.as_ptr()));
        }
        let mut lanes = [0.0f32; 8];
        _mm256_storeu_ps(lanes.as_mut_ptr(), acumulador);
        lanes.iter().sum::<f32>() + trozos.remainder().iter().sum::<f32>()
    }
}

fn cronometrar(nombre: &str, mut f: impl FnMut() -> f32) {
    let t = Instant::now();
    let mut r = 0.0;
    for _ in 0..50 {
        r += black_box(f());
    }
    println!("{nombre:<26} {:>9.2?}  (suma acumulada = {r:e})", t.elapsed());
}

fn main() {
    let datos: Vec<f32> = (0..4_000_000).map(|i| (i % 100) as f32 * 0.5).collect();
    let v = black_box(&datos);

    cronometrar("iter().sum()", || suma_escalar(v));
    cronometrar("acumuladores independientes", || suma_autovectorizable(v));
    #[cfg(target_arch = "x86_64")]
    match suma_avx(v) {
        Some(_) => cronometrar("AVX (std::arch)", || suma_avx(v).unwrap()),
        None => println!("esta CPU no soporta AVX"),
    }
    #[cfg(not(target_arch = "x86_64"))]
    println!("intrínsecos x86_64 no disponibles en esta arquitectura");

    println!("\nNotas:");
    println!(" - `std::simd` (portable SIMD) sigue siendo inestable (nightly); crates estables: `wide`, `pulp`, `memchr`.");
    println!(" - Para activar instrucciones de tu CPU en TODO el binario: RUSTFLAGS=\"-C target-cpu=native\".");
    println!(" - Comprueba qué se vectoriza con `cargo asm` o Compiler Explorer (godbolt.org).");
    if cfg!(debug_assertions) {
        println!(" - AVISO: en debug no hay vectorización; usa --release.");
    }
}
