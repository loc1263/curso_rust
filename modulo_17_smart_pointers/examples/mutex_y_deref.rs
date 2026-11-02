// cargo run -p m17_smart_pointers --example mutex_y_deref
// Tu propio smart pointer (Deref + Drop) y `Arc<Mutex<T>>` para compartir entre hilos.

use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use std::thread;

// Un "Box" casero: Deref + Drop son los dos traits que definen un smart pointer
struct MiBox<T: std::fmt::Debug> {
    valor: T,
}
impl<T: std::fmt::Debug> MiBox<T> {
    fn new(valor: T) -> Self {
        MiBox { valor }
    }
}
impl<T: std::fmt::Debug> Deref for MiBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.valor
    }
}
impl<T: std::fmt::Debug> DerefMut for MiBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.valor
    }
}
impl<T: std::fmt::Debug> Drop for MiBox<T> {
    fn drop(&mut self) {
        println!("MiBox liberado con {:?}", self.valor);
    }
}

fn saluda(nombre: &str) {
    println!("Hola, {nombre}");
}

fn main() {
    let mut m = MiBox::new(String::from("Rust"));
    saluda(&m); // deref coercion: &MiBox<String> -> &String -> &str
    m.push_str("acean"); // DerefMut
    println!("*m = {}", *m); // `*m` es `*m.deref()`
    drop(m);

    // --- Arc<Mutex<T>>: estado mutable compartido entre hilos ---
    let contador = Arc::new(Mutex::new(0));
    let mut hilos = vec![];
    for _ in 0..8 {
        let contador = Arc::clone(&contador);
        hilos.push(thread::spawn(move || {
            for _ in 0..1000 {
                // lock() devuelve un guard; al salir del ámbito se libera el bloqueo
                let mut guard = contador.lock().unwrap();
                *guard += 1;
            }
        }));
    }
    for h in hilos {
        h.join().unwrap();
    }
    println!(
        "contador final = {} (esperado 8000)",
        *contador.lock().unwrap()
    );

    // El guard se libera al terminar el ámbito; acortarlo evita bloqueos largos
    let datos = Arc::new(Mutex::new(vec![1, 2, 3]));
    {
        let mut v = datos.lock().unwrap();
        v.push(4);
    } // aquí se libera
    println!("{:?}", datos.lock().unwrap());
}
