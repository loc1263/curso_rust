// cargo run -p m13_traits_estandar --example derive_comparaciones
// `derive` de Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash y Default.

#![allow(dead_code)]

use std::collections::{BTreeSet, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct Version {
    mayor: u32,
    menor: u32,
    parche: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Nivel {
    Bajo,
    Medio,
    Alto,
}

#[derive(Debug, Default, Clone)]
struct Config {
    nombre: String,
    puerto: u16,
    activo: bool,
    etiquetas: Vec<String>,
    version: Version,
}

fn main() {
    let a = Version {
        mayor: 1,
        menor: 2,
        parche: 3,
    };
    let b = Version {
        mayor: 1,
        menor: 10,
        parche: 0,
    };

    // Debug: {:?} compacto y {:#?} bonito
    println!("{a:?}");
    println!("{b:#?}");

    // PartialEq / Ord: comparación lexicográfica campo por campo, en orden de declaración
    println!("a == b ? {}", a == b);
    println!("a < b  ? {}", a < b);
    println!("max    = {:?}", a.max(b));

    // Enums: se ordenan por posición de variante
    println!("Bajo < Alto ? {}", Nivel::Bajo < Nivel::Alto);

    // Copy: `a` sigue siendo válido tras la asignación
    let c = a;
    println!("a = {a:?}, c = {c:?}");

    // Hash + Eq: pueden ir en HashSet / claves de HashMap
    let mut set = HashSet::new();
    set.insert(a);
    set.insert(c); // duplicado, se ignora
    set.insert(b);
    println!("HashSet con {} elementos", set.len());

    // Ord: pueden ir en BTreeSet (ordenado)
    let ordenado: BTreeSet<_> = [b, a, Version::default()].into_iter().collect();
    println!("ordenados: {ordenado:?}");

    // Default: valores "cero" automáticos y sintaxis de actualización
    let cfg = Config {
        nombre: "demo".into(),
        puerto: 8080,
        ..Default::default()
    };
    println!("{cfg:#?}");

    let copia = cfg.clone();
    println!("clon con mismo nombre: {}", copia.nombre == cfg.nombre);

    // f64 solo implementa PartialEq/PartialOrd (por NaN), no Eq/Ord/Hash
    let nan = f64::NAN;
    #[allow(clippy::eq_op)] // a propósito: demuestra que NaN no es igual a sí mismo
    let iguales = nan == nan;
    println!("NaN == NaN ? {iguales}");
}
