// cargo run -p m25_unsafe --example punteros_crudos
// Los cinco "superpoderes" de `unsafe`: punteros crudos, funciones/traits unsafe,
// static mut, uniones. Todo lo demás (borrow checker, tipos…) sigue activo.

use std::ptr;

// ---------- 1) Punteros crudos: *const T y *mut T ----------
fn punteros() {
    let mut x = 10;
    // Crear un puntero crudo es SEGURO; desreferenciarlo no
    let p_const = &x as *const i32;
    let p_mut = &mut x as *mut i32;
    unsafe {
        println!("*p_const = {}", *p_const);
        *p_mut += 5;
        println!("*p_const tras escribir = {}", *p_const);
    }

    // Aritmética de punteros sobre un array
    let arr = [1, 2, 3, 4, 5];
    let base = arr.as_ptr();
    let suma: i32 = (0..arr.len()).map(|i| unsafe { *base.add(i) }).sum();
    println!("suma por punteros = {suma}");

    // Un puntero crudo puede ser nulo o estar colgante: comprobar es TU responsabilidad
    let nulo: *const i32 = ptr::null();
    println!("¿nulo? {}  (as_ref -> {:?})", nulo.is_null(), unsafe {
        nulo.as_ref()
    });

    // Dos &mut al mismo dato son UB en Rust, pero dos *mut al mismo dato no lo son (hasta que los usas mal)
    let mut v = 1;
    let a = &raw mut v;
    let b = &raw mut v;
    unsafe {
        *a += 1;
        *b += 1;
    }
    println!("v = {v}");

    // ptr::swap, ptr::read, ptr::write: operaciones "en bruto"
    let (mut s1, mut s2) = (String::from("uno"), String::from("dos"));
    unsafe { ptr::swap(&mut s1, &mut s2) };
    println!("tras swap: {s1} {s2}");
}

// ---------- 2) unsafe fn: documenta el contrato con `# Safety` ----------
/// Devuelve el elemento `i` sin comprobar límites.
///
/// # Safety
/// `i` debe ser menor que `s.len()`.
unsafe fn elemento_sin_comprobar(s: &[i32], i: usize) -> i32 {
    // SAFETY: el llamante garantiza que i < s.len()
    unsafe { *s.get_unchecked(i) }
}

// ---------- 3) unsafe trait: el implementador promete algo que el compilador no verifica ----------
/// # Safety
/// Los tipos que lo implementen deben tener TODOS sus bits válidos como `u8`s (sin padding ni punteros).
unsafe trait BytesPlanos {
    fn como_bytes(&self) -> &[u8]
    where
        Self: Sized,
    {
        // SAFETY: lo garantiza quien implementa el trait
        unsafe { std::slice::from_raw_parts(self as *const Self as *const u8, size_of::<Self>()) }
    }
}
unsafe impl BytesPlanos for u32 {}
unsafe impl BytesPlanos for [u16; 2] {}

// ---------- 4) static mut: estado global mutable (peligroso, casi siempre hay alternativa) ----------
static mut CONTADOR_GLOBAL: u32 = 0;

fn incrementar_global() {
    // SAFETY: este programa es de un solo hilo, no hay accesos concurrentes
    unsafe { CONTADOR_GLOBAL += 1 };
}
// Alternativa segura: AtomicU32 / Mutex / OnceLock (ver módulo 20 y 30).

// ---------- 5) union: varios tipos en la misma memoria ----------
#[repr(C)]
union EnteroOFloat {
    entero: u32,
    flotante: f32,
}

fn main() {
    punteros();

    let datos = [10, 20, 30];
    // SAFETY: 1 < 3
    println!("elemento 1 = {}", unsafe {
        elemento_sin_comprobar(&datos, 1)
    });
    // Si pasáramos 7 sería comportamiento indefinido (UB), no un panic

    println!(
        "bytes de 0x01020304u32 (little endian) = {:?}",
        0x0102_0304u32.como_bytes()
    );
    println!("bytes de [1u16, 2] = {:?}", [1u16, 2].como_bytes());

    incrementar_global();
    incrementar_global();
    // SAFETY: un solo hilo; leemos por valor con addr_of para no crear una referencia a static mut
    println!("global = {}", unsafe { *ptr::addr_of!(CONTADOR_GLOBAL) });

    let u = EnteroOFloat { flotante: 1.0 };
    // SAFETY: todos los bits de u32 son válidos
    println!("bits de 1.0f32 = {:#x}", unsafe { u.entero });
    // (en código real usa f32::to_bits, que es seguro)
    println!("equivalente seguro: {:#x}", 1.0f32.to_bits());
}
