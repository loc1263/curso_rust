// cargo run -p m25_unsafe --example maybe_uninit
// `MaybeUninit`, `ManuallyDrop` y una pila de capacidad fija escrita con `unsafe` (con su `Drop`).
// Recomendación: pasa estos ejemplos por Miri: `cargo +nightly miri run -p m25_unsafe --example maybe_uninit`

use std::mem::{ManuallyDrop, MaybeUninit};

/// Pila de capacidad `N` sin asignaciones en el heap.
/// Invariante: los elementos `0..len` están inicializados; el resto NO.
pub struct PilaFija<T, const N: usize> {
    datos: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> PilaFija<T, N> {
    pub fn nueva() -> Self {
        PilaFija {
            // SAFETY: un array de MaybeUninit no necesita inicialización
            datos: unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() },
            len: 0,
        }
    }

    /// Devuelve `Err(valor)` si está llena.
    pub fn apilar(&mut self, valor: T) -> Result<(), T> {
        if self.len == N {
            return Err(valor);
        }
        self.datos[self.len].write(valor); // inicializa el hueco
        self.len += 1;
        Ok(())
    }

    pub fn desapilar(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        // SAFETY: el índice `len` estaba inicializado (invariante) y ahora lo damos de baja,
        // así que nadie volverá a leerlo.
        Some(unsafe { self.datos[self.len].assume_init_read() })
    }

    pub fn cima(&self) -> Option<&T> {
        // SAFETY: si len > 0, datos[len-1] está inicializado
        (self.len > 0).then(|| unsafe { self.datos[self.len - 1].assume_init_ref() })
    }

    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

// MaybeUninit NUNCA ejecuta Drop por sí mismo: hay que liberar a mano lo inicializado
impl<T, const N: usize> Drop for PilaFija<T, N> {
    fn drop(&mut self) {
        for i in 0..self.len {
            // SAFETY: los `len` primeros están inicializados y se liberan una sola vez
            unsafe { self.datos[i].assume_init_drop() };
        }
    }
}

struct Ruidoso(&'static str);
impl Drop for Ruidoso {
    fn drop(&mut self) {
        println!("  drop({})", self.0);
    }
}

fn main() {
    // --- MaybeUninit para inicializar un array elemento a elemento ---
    let mut arr: [MaybeUninit<String>; 3] = [const { MaybeUninit::uninit() }; 3];
    for (i, hueco) in arr.iter_mut().enumerate() {
        hueco.write(format!("elemento {i}"));
    }
    // SAFETY: los 3 huecos acaban de inicializarse; MaybeUninit<T> y T tienen el mismo layout
    let arr: [String; 3] = unsafe { std::mem::transmute(arr) };
    println!("{arr:?}");

    // --- PilaFija ---
    let mut p: PilaFija<Ruidoso, 3> = PilaFija::nueva();
    p.apilar(Ruidoso("a")).ok();
    p.apilar(Ruidoso("b")).ok();
    p.apilar(Ruidoso("c")).ok();
    match p.apilar(Ruidoso("d")) {
        Ok(()) => unreachable!(),
        Err(rechazado) => println!("llena, rechazo {}", rechazado.0),
    } // `rechazado` se libera aquí
    println!("cima = {:?}, len = {}", p.cima().map(|r| r.0), p.len());
    println!("desapilo {:?}", p.desapilar().map(|r| r.0));
    println!("al salir de main se liberan los restantes:");
    drop(p);

    // --- ManuallyDrop: evita el Drop automático ---
    let md = ManuallyDrop::new(Ruidoso("nunca se libera (fuga a propósito)"));
    println!("ManuallyDrop contiene: {}", md.0);

    let mut md2 = ManuallyDrop::new(Ruidoso("liberado a mano"));
    // SAFETY: no volvemos a usar md2 después
    unsafe { ManuallyDrop::drop(&mut md2) };

    // mem::forget: tragarse un valor sin ejecutar Drop (es SEGURO: las fugas no son UB)
    std::mem::forget(Ruidoso("olvidado"));

    // mem::replace / take / swap: mover sin unsafe
    let mut s = String::from("viejo");
    let anterior = std::mem::replace(&mut s, "nuevo".into());
    let tomado = std::mem::take(&mut s);
    println!("{anterior} -> {tomado}, s = {s:?}");
}
