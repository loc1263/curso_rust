//! API C expuesta desde Rust. El header correspondiente está en `include/m26_ffi.h`
//! (escrito a mano; `cbindgen` puede generarlo automáticamente, ver README).
//!
//! Reglas de una frontera FFI sólida:
//! - Solo tipos `repr(C)` o primitivos a través de la frontera.
//! - Nunca dejar que un `panic` cruce hacia C (UB): se captura con `catch_unwind`.
//! - Quien reserva la memoria es quien la libera: si Rust crea algo, Rust ofrece la función para liberarlo.
//! - Punteros recibidos de C: comprobar nulos y documentar el contrato.

use std::ffi::{CStr, CString, c_char, c_int};
use std::panic::{AssertUnwindSafe, catch_unwind};

// ---------------- Funciones simples ----------------

/// Suma con aritmética saturada (no hay overflow ni panic).
#[unsafe(no_mangle)] // conserva el nombre del símbolo tal cual
pub extern "C" fn m26_sumar(a: c_int, b: c_int) -> c_int {
    a.saturating_add(b)
}

// ---------------- Structs repr(C) ----------------

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Punto {
    pub x: f64,
    pub y: f64,
}

#[unsafe(no_mangle)]
pub extern "C" fn m26_distancia(a: Punto, b: Punto) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

/// Modifica el punto en el sitio (C pasa un puntero).
///
/// # Safety
/// `p` debe ser nulo o apuntar a un `Punto` válido y modificable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m26_escalar(p: *mut Punto, factor: f64) -> c_int {
    // SAFETY: el contrato exige que p sea nulo o válido
    match unsafe { p.as_mut() } {
        Some(p) => {
            p.x *= factor;
            p.y *= factor;
            0
        }
        None => -1, // códigos de error en vez de panics
    }
}

// ---------------- Cadenas ----------------

/// Devuelve un saludo reservado por Rust. Hay que liberarlo con `m26_liberar_cadena`.
/// Devuelve NULL si `nombre` es nulo o no es UTF-8.
///
/// # Safety
/// `nombre` debe ser nulo o una cadena C terminada en NUL válida.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m26_saludar(nombre: *const c_char) -> *mut c_char {
    if nombre.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: no nulo y, por contrato, terminado en NUL
    let nombre = match unsafe { CStr::from_ptr(nombre) }.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    CString::new(format!("¡Hola, {nombre}, desde Rust!"))
        .map(CString::into_raw) // cede la propiedad a C
        .unwrap_or(std::ptr::null_mut())
}

/// # Safety
/// `s` debe ser nulo o un puntero devuelto por `m26_saludar` que no se haya liberado antes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m26_liberar_cadena(s: *mut c_char) {
    if !s.is_null() {
        // SAFETY: fue creado con CString::into_raw
        drop(unsafe { CString::from_raw(s) });
    }
}

// ---------------- Tipo opaco (handle) ----------------
// C solo ve un puntero a un tipo incompleto; el contenido es privado de Rust.

pub struct Contador {
    valor: i64,
    paso: i64,
}

#[unsafe(no_mangle)]
pub extern "C" fn m26_contador_nuevo(paso: i64) -> *mut Contador {
    Box::into_raw(Box::new(Contador { valor: 0, paso }))
}

/// # Safety
/// `c` debe ser un puntero válido devuelto por `m26_contador_nuevo`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m26_contador_incrementar(c: *mut Contador) -> i64 {
    // SAFETY: contrato del llamante
    match unsafe { c.as_mut() } {
        Some(c) => {
            c.valor += c.paso;
            c.valor
        }
        None => i64::MIN,
    }
}

/// # Safety
/// `c` debe ser nulo o un puntero de `m26_contador_nuevo` no liberado antes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn m26_contador_liberar(c: *mut Contador) {
    if !c.is_null() {
        // SAFETY: lo creó Box::into_raw
        drop(unsafe { Box::from_raw(c) });
    }
}

// ---------------- Callbacks y protección frente a panics ----------------

pub type Callback = extern "C" fn(valor: c_int, datos: *mut std::ffi::c_void);

/// Llama a `cb` para cada número de 1 a n. Devuelve -1 si algo hizo panic.
#[unsafe(no_mangle)]
pub extern "C" fn m26_para_cada(n: c_int, cb: Callback, datos: *mut std::ffi::c_void) -> c_int {
    // Un panic que cruce una frontera extern "C" aborta el proceso; mejor capturarlo y devolver un error.
    let r = catch_unwind(AssertUnwindSafe(|| {
        for i in 1..=n {
            cb(i, datos);
        }
    }));
    if r.is_ok() { 0 } else { -1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suma_satura() {
        assert_eq!(m26_sumar(i32::MAX, 1), i32::MAX);
    }

    #[test]
    fn nulos_no_explotan() {
        unsafe {
            assert_eq!(m26_escalar(std::ptr::null_mut(), 2.0), -1);
            assert!(m26_saludar(std::ptr::null()).is_null());
            m26_liberar_cadena(std::ptr::null_mut());
            m26_contador_liberar(std::ptr::null_mut());
        }
    }

    #[test]
    fn ciclo_de_vida_del_contador() {
        let c = m26_contador_nuevo(5);
        unsafe {
            m26_contador_incrementar(c);
            assert_eq!(m26_contador_incrementar(c), 10);
            m26_contador_liberar(c);
        }
    }
}
