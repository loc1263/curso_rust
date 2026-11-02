// cargo run --release -p m29_rendimiento --example layout_memoria
// Cómo se organiza la memoria: tamaño, alineación, orden de campos, y por qué importa la caché.

#![allow(dead_code)]

use std::hint::black_box;
use std::mem::{align_of, size_of};
use std::time::Instant;

// Rust puede REORDENAR los campos para minimizar el padding
struct Reordenable {
    a: u8,
    b: u64,
    c: u8,
    d: u32,
}

// repr(C): orden de declaración, como en C -> más padding
#[repr(C)]
struct ComoC {
    a: u8,
    b: u64,
    c: u8,
    d: u32,
}

#[repr(C, packed)] // sin padding, pero leer campos desalineados es más lento y no se pueden tomar referencias
struct Empaquetado {
    a: u8,
    b: u64,
    c: u8,
    d: u32,
}

enum Forma {
    Circulo(f32),
    Rectangulo(f32, f32),
    Nada,
}

// ---------- Array de estructuras (AoS) vs estructura de arrays (SoA) ----------
#[derive(Clone)]
struct ParticulaAoS {
    x: f32,
    y: f32,
    z: f32,
    // campos "fríos" que no necesitamos en el bucle caliente
    id: u64,
    nombre: [u8; 32],
    masa: f64,
}

struct ParticulasSoA {
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<f32>,
}

fn main() {
    println!("-- tamaños y alineación --");
    println!(
        "Reordenable : {:>2} bytes, align {}",
        size_of::<Reordenable>(),
        align_of::<Reordenable>()
    );
    println!(
        "ComoC       : {:>2} bytes, align {}",
        size_of::<ComoC>(),
        align_of::<ComoC>()
    );
    println!(
        "Empaquetado : {:>2} bytes, align {}",
        size_of::<Empaquetado>(),
        align_of::<Empaquetado>()
    );
    println!(
        "Forma (enum): {:>2} bytes (tag + mayor variante)",
        size_of::<Forma>()
    );
    println!(
        "&str {} | String {} | Vec<u8> {} | Box<i32> {} | Option<Box<i32>> {}",
        size_of::<&str>(),
        size_of::<String>(),
        size_of::<Vec<u8>>(),
        size_of::<Box<i32>>(),
        size_of::<Option<Box<i32>>>()
    );
    println!(
        "&dyn Trait {} (puntero gordo: dato + vtable)",
        size_of::<&dyn std::fmt::Debug>()
    );
    println!(
        "() {} | [u8; 0] {} | PhantomData {}",
        size_of::<()>(),
        size_of::<[u8; 0]>(),
        size_of::<std::marker::PhantomData<u64>>()
    );
    println!(
        "bool {} | Option<bool> {} (nicho en el valor inválido)",
        size_of::<bool>(),
        size_of::<Option<bool>>()
    );
    println!(
        "u32 {} | Option<u32> {} | Option<NonZeroU32> {}",
        size_of::<u32>(),
        size_of::<Option<u32>>(),
        size_of::<Option<std::num::NonZeroU32>>()
    );

    println!("\n-- AoS vs SoA: sumar solo x, y, z de 2M partículas --");
    let n = 2_000_000;
    let aos: Vec<ParticulaAoS> = (0..n)
        .map(|i| ParticulaAoS {
            x: i as f32,
            y: 1.0,
            z: 2.0,
            id: i as u64,
            nombre: [0; 32],
            masa: 1.0,
        })
        .collect();
    let soa = ParticulasSoA {
        x: (0..n).map(|i| i as f32).collect(),
        y: vec![1.0; n],
        z: vec![2.0; n],
    };
    println!(
        "tamaño de ParticulaAoS = {} bytes (solo usamos 12)",
        size_of::<ParticulaAoS>()
    );

    let repeticiones = 20;
    let t = Instant::now();
    let mut r1 = 0.0;
    for _ in 0..repeticiones {
        r1 += black_box(&aos).iter().map(|p| p.x + p.y + p.z).sum::<f32>();
    }
    let t_aos = t.elapsed();

    let t = Instant::now();
    let mut r2 = 0.0;
    for _ in 0..repeticiones {
        let s = black_box(&soa);
        r2 +=
            s.x.iter()
                .zip(&s.y)
                .zip(&s.z)
                .map(|((x, y), z)| x + y + z)
                .sum::<f32>();
    }
    let t_soa = t.elapsed();

    println!("AoS: {t_aos:>9.2?}  (resultado {r1:e})");
    println!("SoA: {t_soa:>9.2?}  (resultado {r2:e})");
    println!(
        "SoA es ~{:.1}x más rápido: lee menos memoria y se vectoriza mejor.",
        t_aos.as_secs_f64() / t_soa.as_secs_f64()
    );
    if cfg!(debug_assertions) {
        println!("(AVISO: estás en debug; usa --release)");
    }

    println!("\n-- recorrer por filas vs por columnas (caché) --");
    let dim = 4_000;
    let m = vec![1u32; dim * dim];
    let t = Instant::now();
    let mut s = 0u64;
    for i in 0..dim {
        for j in 0..dim {
            s += black_box(m[i * dim + j]) as u64; // acceso contiguo
        }
    }
    let filas = t.elapsed();
    let t = Instant::now();
    let mut s2 = 0u64;
    for j in 0..dim {
        for i in 0..dim {
            s2 += black_box(m[i * dim + j]) as u64; // saltos de `dim` elementos
        }
    }
    let columnas = t.elapsed();
    assert_eq!(s, s2);
    println!("por filas: {filas:>9.2?} | por columnas: {columnas:>9.2?}");
}
