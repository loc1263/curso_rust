// cargo run -p m22_async_await --example select_timeout
// `select!`, timeouts, intervalos y cancelación.

use std::time::Duration;
use tokio::time::{Instant, interval, sleep, timeout};

async fn lenta() -> &'static str {
    sleep(Duration::from_millis(300)).await;
    "lenta"
}

async fn rapida() -> &'static str {
    sleep(Duration::from_millis(50)).await;
    "rápida"
}

// Si un future se descarta antes de terminar, se CANCELA: Drop se ejecuta en cada .await pendiente
struct Aviso(&'static str);
impl Drop for Aviso {
    fn drop(&mut self) {
        println!("  (drop de {})", self.0);
    }
}

async fn con_limpieza() {
    let _aviso = Aviso("con_limpieza");
    sleep(Duration::from_secs(10)).await;
    println!("nunca se imprime");
}

#[tokio::main]
async fn main() {
    // select!: espera a la PRIMERA que termine y cancela las demás
    tokio::select! {
        r = lenta() => println!("ganó {r}"),
        r = rapida() => println!("ganó {r}"),
    }

    // timeout: Err(Elapsed) si no termina a tiempo
    match timeout(Duration::from_millis(100), lenta()).await {
        Ok(v) => println!("terminó: {v}"),
        Err(_) => println!("timeout: lenta() tardó más de 100 ms"),
    }
    println!("{:?}", timeout(Duration::from_millis(100), rapida()).await);

    // Cancelación: al perder el select!, el future se descarta y se ejecuta su Drop
    tokio::select! {
        _ = con_limpieza() => {}
        _ = sleep(Duration::from_millis(30)) => println!("cancelando con_limpieza..."),
    }

    // interval: ticks periódicos; combinado con select! hace un bucle con "tope"
    let mut reloj = interval(Duration::from_millis(40));
    let limite = sleep(Duration::from_millis(170));
    tokio::pin!(limite); // select! sobre un future que se reutiliza necesita estar "pineado"
    let inicio = Instant::now();
    loop {
        tokio::select! {
            _ = reloj.tick() => println!("tick a los {:?}", inicio.elapsed().as_millis() / 10 * 10),
            _ = &mut limite => { println!("fin del bucle"); break; }
        }
    }

    // select! con condiciones (`if`) y rama `else`
    let mut quedan = vec![1, 2, 3].into_iter();
    let mut intervalo = interval(Duration::from_millis(10));
    loop {
        tokio::select! {
            _ = intervalo.tick() => {
                match quedan.next() {
                    Some(n) => println!("procesando {n}"),
                    None => break,
                }
            }
        }
    }

    println!("fin");
}
