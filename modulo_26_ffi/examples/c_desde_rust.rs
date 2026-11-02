// cargo run -p m26_ffi --example c_desde_rust
// Llamar a funciones de la biblioteca estándar de C (libc / CRT) desde Rust.

use std::ffi::{CStr, CString, c_char, c_int, c_void};

// Declaración de funciones externas. En edición 2024 el bloque es `unsafe extern`.
// Las marcadas `safe` pueden llamarse sin bloque unsafe (tú aseguras que lo son).
unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn atoi(s: *const c_char) -> c_int;
    fn qsort(
        base: *mut c_void,
        n: usize,
        tam: usize,
        cmp: extern "C" fn(*const c_void, *const c_void) -> c_int,
    );
    pub safe fn abs(x: c_int) -> c_int;
}

// Callback con ABI de C: Rust lo expone como puntero a función `extern "C"`
extern "C" fn comparar_i32(a: *const c_void, b: *const c_void) -> c_int {
    // SAFETY: qsort nos da punteros a elementos i32 válidos del array
    let (a, b) = unsafe { (*(a as *const i32), *(b as *const i32)) };
    a.cmp(&b) as c_int
}

fn main() {
    // Rust -> C: las cadenas necesitan terminador NUL; CString lo garantiza (y rechaza NUL internos)
    let texto = CString::new("Rust y C").unwrap();
    let longitud = unsafe { strlen(texto.as_ptr()) };
    println!("strlen(\"Rust y C\") = {longitud}");

    let numero = CString::new("1234xyz").unwrap();
    println!("atoi = {}", unsafe { atoi(numero.as_ptr()) });

    println!("abs(-7) = {} (declarada `safe`, sin unsafe)", abs(-7));

    // Pasar un callback a C: qsort ordena nuestro array
    let mut v = [5, 2, 9, 1, 7];
    unsafe {
        qsort(
            v.as_mut_ptr().cast(),
            v.len(),
            size_of::<i32>(),
            comparar_i32,
        );
    }
    println!("qsort -> {v:?}");

    // C -> Rust: CStr envuelve un `char*` recibido; to_str valida UTF-8
    let desde_c: *const c_char = texto.as_ptr();
    let s = unsafe { CStr::from_ptr(desde_c) };
    println!(
        "CStr -> {:?} (bytes con NUL: {})",
        s.to_str().unwrap(),
        s.to_bytes_with_nul().len()
    );

    // CString::new falla si hay un NUL en medio
    println!("{:?}", CString::new("a\0b").is_err());
}
