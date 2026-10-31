//! Funciones con varias implementaciones para comparar rendimiento (ejemplos y benchmarks).

use rayon::prelude::*;

/// Versión ingenua: bucle con índice (comprobación de límites en cada acceso, salvo que el compilador la elimine).
pub fn suma_cuadrados_indices(v: &[u64]) -> u64 {
    let mut total = 0;
    for i in 0..v.len() {
        total += v[i] * v[i];
    }
    total
}

/// Versión idiomática: iteradores (se compilan a un bucle sin comprobaciones y vectorizable).
pub fn suma_cuadrados_iter(v: &[u64]) -> u64 {
    v.iter().map(|x| x * x).sum()
}

/// Versión paralela con rayon: solo compensa para datos grandes.
pub fn suma_cuadrados_par(v: &[u64]) -> u64 {
    v.par_iter().map(|x| x * x).sum()
}

/// Cuenta palabras ASIGNANDO un `String` por palabra (lo que NO hay que hacer en un camino caliente).
pub fn contar_largas_con_alloc(texto: &str, min: usize) -> usize {
    texto
        .split_whitespace()
        .map(|p| p.to_lowercase()) // una asignación por palabra
        .filter(|p| p.len() >= min)
        .count()
}

/// Misma lógica sin asignar: trabaja con `&str` prestados.
pub fn contar_largas_sin_alloc(texto: &str, min: usize) -> usize {
    texto.split_whitespace().filter(|p| p.len() >= min).count()
}

/// Ordenación por inserción: O(n²), solo para comparar con `sort`.
pub fn ordenar_insercion(v: &mut [u32]) {
    for i in 1..v.len() {
        let mut j = i;
        while j > 0 && v[j - 1] > v[j] {
            v.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Generador pseudoaleatorio determinista (xorshift) para no depender de `rand`.
pub fn datos_pseudoaleatorios(n: usize, semilla: u64) -> Vec<u32> {
    let mut x = semilla.max(1);
    (0..n)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            (x >> 32) as u32
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn las_tres_sumas_coinciden() {
        let v: Vec<u64> = (1..=1000).collect();
        let a = suma_cuadrados_indices(&v);
        assert_eq!(a, suma_cuadrados_iter(&v));
        assert_eq!(a, suma_cuadrados_par(&v));
    }

    #[test]
    fn ordenacion_correcta() {
        let mut v = datos_pseudoaleatorios(200, 7);
        let mut esperado = v.clone();
        esperado.sort();
        ordenar_insercion(&mut v);
        assert_eq!(v, esperado);
    }

    #[test]
    fn contar_largas() {
        assert_eq!(contar_largas_con_alloc("Hola mundo cruel", 5), 2);
        assert_eq!(contar_largas_sin_alloc("Hola mundo cruel", 5), 2);
    }
}
