// cargo run -p m25_unsafe --example abstraccion_segura
// La idea central: encapsular `unsafe` dentro de una API SEGURA cuyos invariantes se mantienen.
// Si el código seguro que rodea al bloque unsafe no puede romper la garantía, la abstracción es sólida.

use std::slice;

// ---------- split_at_mut: lo que el borrow checker no puede probar, pero nosotros sí ----------
// El compilador rechaza dos &mut al mismo slice, aunque sean zonas disjuntas.
fn dividir_mut<T>(s: &mut [T], medio: usize) -> (&mut [T], &mut [T]) {
    let len = s.len();
    let ptr = s.as_mut_ptr();
    assert!(medio <= len, "medio fuera de rango"); // invariante comprobado en código SEGURO
    // SAFETY: las dos mitades [0, medio) y [medio, len) no se solapan y ambas están
    // dentro del slice original, que estamos tomando prestado en exclusiva.
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, medio),
            slice::from_raw_parts_mut(ptr.add(medio), len - medio),
        )
    }
}

// ---------- Un buffer de tamaño fijo con acceso rápido ----------
pub struct Buffer {
    datos: Vec<u8>,
}

impl Buffer {
    pub fn nuevo(n: usize) -> Self {
        Buffer { datos: vec![0; n] }
    }

    /// API segura: comprueba límites.
    pub fn get(&self, i: usize) -> Option<u8> {
        self.datos.get(i).copied()
    }

    /// Variante rápida: el llamante asume el contrato.
    ///
    /// # Safety
    /// `i < self.len()`.
    pub unsafe fn get_sin_comprobar(&self, i: usize) -> u8 {
        debug_assert!(i < self.datos.len()); // en debug detecta el error
        // SAFETY: lo garantiza el llamante
        unsafe { *self.datos.get_unchecked(i) }
    }

    pub fn len(&self) -> usize {
        self.datos.len()
    }
    pub fn is_empty(&self) -> bool {
        self.datos.is_empty()
    }
}

// ---------- Un tipo que garantiza un invariante en su constructor ----------
/// Cadena que SIEMPRE es ASCII válido: permite saltarse la validación UTF-8 al leerla.
pub struct Ascii(Vec<u8>);

impl Ascii {
    pub fn nueva(s: &str) -> Option<Ascii> {
        s.is_ascii().then(|| Ascii(s.as_bytes().to_vec()))
    }
    pub fn como_str(&self) -> &str {
        // SAFETY: el único constructor (`nueva`) verifica que todo es ASCII, y ASCII ⊂ UTF-8.
        // El campo es privado: nadie fuera del módulo puede romper el invariante.
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

// ---------- Swap de dos elementos distintos usando punteros ----------
fn intercambiar_indices<T>(s: &mut [T], a: usize, b: usize) {
    assert!(a < s.len() && b < s.len(), "índice fuera de rango");
    if a == b {
        return;
    }
    let p = s.as_mut_ptr();
    // SAFETY: a != b, ambos en rango; los punteros apuntan a elementos distintos y válidos
    unsafe { std::ptr::swap(p.add(a), p.add(b)) };
}

fn main() {
    let mut v = [1, 2, 3, 4, 5, 6];
    let (izq, der) = dividir_mut(&mut v, 2);
    izq[0] = 100;
    der[0] = 300;
    izq[1] += der[1];
    println!("{v:?}");

    let b = Buffer::nuevo(4);
    println!("get(2) = {:?}, get(9) = {:?}", b.get(2), b.get(9));
    // SAFETY: 3 < 4
    println!("sin comprobar(3) = {}", unsafe { b.get_sin_comprobar(3) });
    println!("len = {} vacío = {}", b.len(), b.is_empty());

    println!(
        "{:?}",
        Ascii::nueva("hola").map(|a| a.como_str().to_uppercase())
    );
    println!("{:?}", Ascii::nueva("ñandú").is_some());

    let mut letras = ['a', 'b', 'c', 'd'];
    intercambiar_indices(&mut letras, 0, 3);
    println!("{letras:?}");
}
