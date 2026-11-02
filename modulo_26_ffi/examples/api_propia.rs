// cargo run -p m26_ffi --example api_propia
// Usamos nuestra propia API C (exportada en lib.rs) desde Rust, tal como lo haría un programa en C.
// El equivalente en C está en `c/demo.c` y el header en `include/m26_ffi.h`.

use m26_ffi::*;
use std::ffi::{CStr, CString, c_int, c_void};

extern "C" fn acumular(valor: c_int, datos: *mut c_void) {
    // SAFETY: `datos` es el &mut i32 que pasamos más abajo
    let total = unsafe { &mut *(datos as *mut i32) };
    *total += valor;
}

fn main() {
    println!("m26_sumar(2, 3)          = {}", m26_sumar(2, 3));
    println!("m26_sumar(MAX, 1)        = {}", m26_sumar(i32::MAX, 1));

    let a = Punto { x: 0.0, y: 0.0 };
    let b = Punto { x: 3.0, y: 4.0 };
    println!("distancia                = {}", m26_distancia(a, b));

    let mut p = Punto { x: 1.5, y: 2.0 };
    let rc = unsafe { m26_escalar(&mut p, 2.0) };
    println!("escalar -> rc = {rc}, p = {p:?}");
    println!("escalar(NULL) -> rc = {}", unsafe {
        m26_escalar(std::ptr::null_mut(), 2.0)
    });

    // Cadenas: Rust reserva, nosotros (como "C") debemos liberar con la función de Rust
    let nombre = CString::new("Ana").unwrap();
    let saludo = unsafe { m26_saludar(nombre.as_ptr()) };
    println!(
        "saludo                   = {}",
        unsafe { CStr::from_ptr(saludo) }.to_str().unwrap()
    );
    unsafe { m26_liberar_cadena(saludo) }; // ¡NO usar free() de C: distinto asignador!

    // Handle opaco
    let c = m26_contador_nuevo(10);
    let mut ultimo = 0;
    for _ in 0..3 {
        ultimo = unsafe { m26_contador_incrementar(c) };
    }
    println!("contador tras 3 pasos    = {ultimo}");
    unsafe { m26_contador_liberar(c) };

    // Callback con datos de usuario
    let mut total = 0i32;
    let rc = m26_para_cada(10, acumular, &mut total as *mut i32 as *mut c_void);
    println!("para_cada(10) -> rc = {rc}, total = {total}");
}
