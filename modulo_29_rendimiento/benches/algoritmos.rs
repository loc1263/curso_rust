// cargo bench -p m29_rendimiento
// Benchmarks estadísticos con criterion (informes HTML en target/criterion si activas la feature `html_reports`).

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use m29_rendimiento::*;
use std::hint::black_box;

fn sumas(c: &mut Criterion) {
    let mut grupo = c.benchmark_group("suma_cuadrados");
    for n in [1_000usize, 100_000, 1_000_000] {
        let datos: Vec<u64> = (0..n as u64).collect();
        // bench_with_input mide la misma función con distintos tamaños de entrada
        grupo.bench_with_input(BenchmarkId::new("indices", n), &datos, |b, d| {
            b.iter(|| suma_cuadrados_indices(black_box(d)))
        });
        grupo.bench_with_input(BenchmarkId::new("iter", n), &datos, |b, d| {
            b.iter(|| suma_cuadrados_iter(black_box(d)))
        });
        grupo.bench_with_input(BenchmarkId::new("rayon", n), &datos, |b, d| {
            b.iter(|| suma_cuadrados_par(black_box(d)))
        });
    }
    grupo.finish();
}

fn ordenacion(c: &mut Criterion) {
    let base = datos_pseudoaleatorios(2_000, 42);
    c.bench_function("sort_std_2000", |b| {
        b.iter_batched(
            || base.clone(),
            |mut v| v.sort(),
            criterion::BatchSize::SmallInput,
        )
    });
    c.bench_function("sort_unstable_2000", |b| {
        b.iter_batched(
            || base.clone(),
            |mut v| v.sort_unstable(),
            criterion::BatchSize::SmallInput,
        )
    });
    c.bench_function("insercion_2000", |b| {
        b.iter_batched(
            || base.clone(),
            |mut v| ordenar_insercion(&mut v),
            criterion::BatchSize::SmallInput,
        )
    });
}

fn palabras(c: &mut Criterion) {
    let texto = "Rust es un lenguaje rápido seguro y expresivo ".repeat(2_000);
    c.bench_function("largas_con_alloc", |b| {
        b.iter(|| contar_largas_con_alloc(black_box(&texto), 5))
    });
    c.bench_function("largas_sin_alloc", |b| {
        b.iter(|| contar_largas_sin_alloc(black_box(&texto), 5))
    });
}

criterion_group!(benches, sumas, ordenacion, palabras);
criterion_main!(benches);
