//! Utilidades `#![no_std]` de ejemplo.
//!
//! - Solo se usa `core` (el subconjunto de la biblioteca estándar que no necesita sistema operativo).
//! - Con la feature `alloc` se habilita `alloc` (colecciones en heap), que sí requiere un asignador global.
//! - Todo lo demás (hilos, archivos, red, `HashMap`, `println!`…) vive en `std` y NO está disponible.

#![no_std]
#![forbid(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::fmt::{self, Write};

// =====================================================================
// Estructuras de tamaño fijo: sin heap, todo en la pila (o en un `static`)
// =====================================================================

/// Cola circular de capacidad `N` sin asignaciones.
#[derive(Debug)]
pub struct Anillo<T, const N: usize> {
    datos: [Option<T>; N],
    cabeza: usize,
    len: usize,
}

impl<T, const N: usize> Anillo<T, N> {
    pub const fn new() -> Self {
        Anillo {
            datos: [const { None }; N],
            cabeza: 0,
            len: 0,
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub const fn is_full(&self) -> bool {
        self.len == N
    }

    /// Devuelve `Err(v)` si está llena (en embebido no hay "reservar más": se decide qué hacer).
    pub fn push(&mut self, v: T) -> Result<(), T> {
        if self.is_full() {
            return Err(v);
        }
        let fin = (self.cabeza + self.len) % N;
        self.datos[fin] = Some(v);
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let v = self.datos[self.cabeza].take();
        self.cabeza = (self.cabeza + 1) % N;
        self.len -= 1;
        v
    }
}

impl<T, const N: usize> Default for Anillo<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Buffer de texto de tamaño fijo que implementa `core::fmt::Write`:
/// permite usar `write!` sin heap (equivalente a `format!` en entornos sin alloc).
#[derive(Debug)]
pub struct TextoFijo<const N: usize> {
    bytes: [u8; N],
    len: usize,
}

impl<const N: usize> TextoFijo<N> {
    pub const fn new() -> Self {
        TextoFijo {
            bytes: [0; N],
            len: 0,
        }
    }

    pub fn as_str(&self) -> &str {
        // Solo escribimos &str completos (UTF-8 válido) en write_str, así que esto nunca falla
        core::str::from_utf8(&self.bytes[..self.len]).unwrap_or("")
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }
}

impl<const N: usize> Default for TextoFijo<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> Write for TextoFijo<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        if self.len + bytes.len() > N {
            return Err(fmt::Error); // no cabe: se informa en vez de truncar a mitad de un carácter
        }
        self.bytes[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();
        Ok(())
    }
}

// =====================================================================
// Algoritmos con core puro
// =====================================================================

/// CRC-8 (polinomio 0x07), típico en protocolos de sensores. Sin tablas ni heap.
pub fn crc8(datos: &[u8]) -> u8 {
    let mut crc = 0u8;
    for &b in datos {
        crc ^= b;
        for _ in 0..8 {
            crc = if crc & 0x80 != 0 {
                (crc << 1) ^ 0x07
            } else {
                crc << 1
            };
        }
    }
    crc
}

/// Convierte `"23.5"` en décimas de grado (235) sin usar `f32` (muchos MCU no tienen FPU).
pub fn parsear_decimas(s: &str) -> Option<i32> {
    let (negativo, s) = match s.strip_prefix('-') {
        Some(resto) => (true, resto),
        None => (false, s),
    };
    let (entera, frac) = s.split_once('.').unwrap_or((s, "0"));
    let entera: i32 = entera.parse().ok()?;
    let decima: i32 = frac.get(..1)?.parse().ok()?;
    let valor = entera * 10 + decima;
    Some(if negativo { -valor } else { valor })
}

/// Escribe una lectura en formato legible en cualquier destino `fmt::Write` (UART, buffer, pantalla…).
pub fn formatear_lectura<W: Write>(destino: &mut W, sensor: &str, decimas: i32) -> fmt::Result {
    let signo = if decimas < 0 { "-" } else { "" };
    let abs = decimas.unsigned_abs();
    write!(destino, "{sensor}: {signo}{}.{} C", abs / 10, abs % 10)
}

// =====================================================================
// Con `alloc` (opcional): colecciones dinámicas sin std
// =====================================================================
#[cfg(feature = "alloc")]
pub mod con_alloc {
    use alloc::format;
    use alloc::string::String;
    use alloc::vec::Vec;

    /// Divide un mensaje `"clave=valor;clave=valor"` en pares.
    pub fn parsear_pares(s: &str) -> Vec<(String, String)> {
        s.split(';')
            .filter_map(|par| par.split_once('='))
            .map(|(k, v)| (String::from(k.trim()), String::from(v.trim())))
            .collect()
    }

    pub fn resumen(pares: &[(String, String)]) -> String {
        let partes: Vec<String> = pares.iter().map(|(k, v)| format!("{k} -> {v}")).collect();
        partes.join(", ")
    }
}

#[cfg(test)]
mod tests {
    // Los tests se ejecutan en el PC con std; la biblioteca en sí sigue siendo no_std
    use super::*;

    #[test]
    fn anillo_fifo_y_envuelve() {
        let mut a: Anillo<u8, 3> = Anillo::new();
        assert_eq!(a.push(1), Ok(()));
        a.push(2).unwrap();
        a.push(3).unwrap();
        assert_eq!(a.push(4), Err(4)); // llena
        assert_eq!(a.pop(), Some(1));
        a.push(5).unwrap(); // reutiliza el hueco: la posición envuelve
        assert_eq!(
            [a.pop(), a.pop(), a.pop(), a.pop()],
            [Some(2), Some(3), Some(5), None]
        );
    }

    #[test]
    fn texto_fijo_formatea_y_rechaza_desborde() {
        let mut t = TextoFijo::<16>::new();
        write!(t, "x={}", 42).unwrap();
        assert_eq!(t.as_str(), "x=42");
        assert!(write!(t, "una cadena demasiado larga").is_err());
    }

    #[test]
    fn crc8_conocido() {
        assert_eq!(crc8(b"123456789"), 0xF4); // vector de prueba estándar del CRC-8
        assert_eq!(crc8(&[]), 0);
    }

    #[test]
    fn decimas() {
        assert_eq!(parsear_decimas("23.5"), Some(235));
        assert_eq!(parsear_decimas("-4.2"), Some(-42));
        assert_eq!(parsear_decimas("7"), Some(70));
        assert_eq!(parsear_decimas("x"), None);
    }

    #[test]
    fn lectura_formateada() {
        let mut t = TextoFijo::<32>::new();
        formatear_lectura(&mut t, "temp", -42).unwrap();
        assert_eq!(t.as_str(), "temp: -4.2 C");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn pares() {
        let p = con_alloc::parsear_pares("a=1; b = 2;roto");
        assert_eq!(p.len(), 2);
        assert_eq!(con_alloc::resumen(&p), "a -> 1, b -> 2");
    }
}
