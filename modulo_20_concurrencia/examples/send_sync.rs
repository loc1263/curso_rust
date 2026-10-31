// cargo run -p m20_concurrencia --example send_sync
// `Send` y `Sync`: cómo el compilador impide las data races.

#![allow(dead_code)]

use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

// Funciones auxiliares que solo compilan si T cumple el trait: comprobación en tiempo de compilación
fn es_send<T: Send>() {}
fn es_sync<T: Sync>() {}

// Un puntero crudo no es Send ni Sync por defecto
struct ConPuntero(*mut i32);

// Un wrapper que promete (con `unsafe`) que SÍ es seguro enviarlo entre hilos.
// El compilador confía en ti: aquí asumes la responsabilidad.
struct Seguro(*mut i32);
unsafe impl Send for Seguro {}

// Un tipo que opta explícitamente por NO ser Send con PhantomData<Rc<()>>
struct NoEnviable {
    _marca: PhantomData<Rc<()>>,
}

fn main() {
    // Send: el valor puede MOVERSE a otro hilo.
    // Sync: &T puede COMPARTIRSE entre hilos (equivale a: &T es Send).
    es_send::<i32>();
    es_sync::<i32>();
    es_send::<String>();
    es_send::<Arc<Mutex<i32>>>();
    es_sync::<Arc<Mutex<i32>>>();
    es_send::<Mutex<RefCell<i32>>>(); // Mutex arregla la falta de Sync de RefCell para compartir
    es_send::<Seguro>();

    // Estos NO compilan (descomenta uno para ver el error):
    // es_send::<Rc<i32>>();            // Rc usa contador no atómico
    // es_sync::<RefCell<i32>>();        // RefCell no es seguro entre hilos
    // es_send::<ConPuntero>();          // los punteros crudos no son Send
    // es_send::<NoEnviable>();

    let _ = (ConPuntero(std::ptr::null_mut()), NoEnviable { _marca: PhantomData });

    // Ejemplo de error clásico que Rust detecta en compilación:
    //   let rc = Rc::new(5);
    //   thread::spawn(move || println!("{rc}"));
    //   -> error: `Rc<i32>` cannot be sent between threads safely

    // Una data race necesita: acceso concurrente + al menos una escritura + sin sincronización.
    // Rust lo evita: &mut es exclusivo, y para compartir mutando necesitas Mutex/atómicos.
    let compartido = Arc::new(Mutex::new(Vec::new()));
    let hs: Vec<_> = (0..4)
        .map(|i| {
            let v = Arc::clone(&compartido);
            thread::spawn(move || v.lock().unwrap().push(i))
        })
        .collect();
    hs.into_iter().for_each(|h| h.join().unwrap());
    let mut v = compartido.lock().unwrap().clone();
    v.sort();
    println!("{v:?}");

    println!("Todas las comprobaciones Send/Sync pasaron en tiempo de compilación.");
}
