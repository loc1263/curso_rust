// cargo run -p m23_pin_futures --example ejecutor_mini
// Un ejecutor multitarea mínimo (estilo "build your own executor" del Async Book).
// Muchas tareas conviven en UN solo hilo; cada una se vuelve a encolar cuando su waker se activa.

use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread;
use std::time::Duration;

// Una tarea = un future "en caja" + un canal para volver a encolarse
struct Tarea {
    nombre: String,
    future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send>>>>,
    cola: SyncSender<Arc<Tarea>>,
}

// Despertar una tarea = ponerla de nuevo en la cola del ejecutor
impl Wake for Tarea {
    fn wake(self: Arc<Self>) {
        let copia = Arc::clone(&self);
        self.cola
            .send(copia)
            .expect("la cola del ejecutor está llena o cerrada");
    }
}

struct Ejecutor {
    cola: Receiver<Arc<Tarea>>,
}

#[derive(Clone)]
struct Lanzador {
    cola: SyncSender<Arc<Tarea>>,
}

fn nuevo_ejecutor() -> (Ejecutor, Lanzador) {
    let (tx, rx) = sync_channel(1000);
    (Ejecutor { cola: rx }, Lanzador { cola: tx })
}

impl Lanzador {
    fn lanzar(&self, nombre: &str, fut: impl Future<Output = ()> + Send + 'static) {
        let tarea = Arc::new(Tarea {
            nombre: nombre.to_string(),
            future: Mutex::new(Some(Box::pin(fut))),
            cola: self.cola.clone(),
        });
        self.cola.send(tarea).unwrap();
    }
}

impl Ejecutor {
    // Termina cuando TODOS los Lanzador (y las tareas con copia de la cola) se destruyen
    fn ejecutar(self) {
        while let Ok(tarea) = self.cola.recv() {
            let mut slot = tarea.future.lock().unwrap();
            if let Some(mut fut) = slot.take() {
                let waker = Waker::from(Arc::clone(&tarea));
                let mut cx = Context::from_waker(&waker);
                if fut.as_mut().poll(&mut cx).is_pending() {
                    *slot = Some(fut); // aún no ha terminado: la guardamos para el próximo poll
                } else {
                    println!("  ✔ tarea '{}' completada", tarea.nombre);
                }
            }
        }
    }
}

// ---------- Un temporizador async (como `tokio::time::sleep`) ----------
struct Dormir {
    compartido: Arc<Mutex<(bool, Option<Waker>)>>,
}
impl Dormir {
    fn new(ms: u64) -> Self {
        let compartido = Arc::new(Mutex::new((false, None::<Waker>)));
        let c = Arc::clone(&compartido);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(ms));
            let mut g = c.lock().unwrap();
            g.0 = true;
            if let Some(w) = g.1.take() {
                w.wake();
            }
        });
        Dormir { compartido }
    }
}
impl Future for Dormir {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut g = self.compartido.lock().unwrap();
        if g.0 {
            Poll::Ready(())
        } else {
            g.1 = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

async fn trabajo(nombre: &'static str, pasos: u32, ms: u64) {
    for i in 1..=pasos {
        println!("{nombre}: paso {i}/{pasos}");
        Dormir::new(ms).await;
    }
}

fn main() {
    let (ejecutor, lanzador) = nuevo_ejecutor();
    lanzador.lanzar("rápida", trabajo("rápida", 3, 40));
    lanzador.lanzar("lenta", trabajo("lenta ", 2, 90));
    lanzador.lanzar("instantánea", async { println!("instantánea: hecho") });
    drop(lanzador); // ya no lanzaremos más tareas desde main

    // Todo se ejecuta intercalado en ESTE hilo
    ejecutor.ejecutar();
    println!("el ejecutor terminó: no quedan tareas");
}
