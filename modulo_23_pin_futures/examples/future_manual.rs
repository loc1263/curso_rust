// cargo run -p m23_pin_futures --example future_manual
// Implementar `Future` a mano y ejecutarlo con un `block_on` mínimo.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};
use std::time::Duration;

// ---------- Un block_on mínimo: ejecuta UN future hasta terminar ----------

// El Waker avisa al ejecutor de que el future puede progresar: aquí despierta al hilo
struct DespertarHilo(Thread);
impl Wake for DespertarHilo {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

fn block_on<F: Future>(fut: F) -> F::Output {
    let mut fut = std::pin::pin!(fut); // fija el future en la pila
    let waker = Waker::from(Arc::new(DespertarHilo(thread::current())));
    let mut cx = Context::from_waker(&waker);
    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(valor) => return valor,
            Poll::Pending => thread::park(), // duerme hasta que alguien llame a wake()
        }
    }
}

// ---------- Future 1: devuelve Pending N veces y luego Ready ----------

struct CuentaAtras {
    restantes: u32,
}

impl Future for CuentaAtras {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.restantes == 0 {
            Poll::Ready("¡despegue!")
        } else {
            println!("  poll: quedan {}", self.restantes);
            self.restantes -= 1;
            // Contrato de Future: si devuelves Pending DEBES haber registrado el waker
            // para que alguien te vuelva a despertar. Aquí nos autodespertamos.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// ---------- Future 2: un temporizador real con un hilo que llama a wake ----------

struct Temporizador {
    estado: Arc<std::sync::Mutex<EstadoTemp>>,
}
struct EstadoTemp {
    terminado: bool,
    waker: Option<Waker>,
}

impl Temporizador {
    fn nuevo(dur: Duration) -> Self {
        let estado = Arc::new(std::sync::Mutex::new(EstadoTemp {
            terminado: false,
            waker: None,
        }));
        let copia = Arc::clone(&estado);
        thread::spawn(move || {
            thread::sleep(dur);
            let mut e = copia.lock().unwrap();
            e.terminado = true;
            if let Some(w) = e.waker.take() {
                w.wake(); // avisa al ejecutor
            }
        });
        Temporizador { estado }
    }
}

impl Future for Temporizador {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut e = self.estado.lock().unwrap();
        if e.terminado {
            Poll::Ready(())
        } else {
            // Guardamos (o actualizamos) el waker: el ejecutor puede cambiarlo entre polls
            e.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

// ---------- Un async fn es azúcar de una máquina de estados como las anteriores ----------

async fn combinado() -> String {
    println!("combinado: empiezo");
    let msg = CuentaAtras { restantes: 2 }.await;
    println!("combinado: la cuenta atrás dijo {msg:?}; ahora espero 100 ms");
    Temporizador::nuevo(Duration::from_millis(100)).await;
    "listo".to_string()
}

fn main() {
    println!("{}", block_on(CuentaAtras { restantes: 3 }));
    println!("---");
    println!("{}", block_on(combinado()));
}
