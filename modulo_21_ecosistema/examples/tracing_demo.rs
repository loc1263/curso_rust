// cargo run -p m21_ecosistema --example tracing_demo
// RUST_LOG=debug cargo run -p m21_ecosistema --example tracing_demo
//
// `tracing`: logs estructurados con spans. En PowerShell: $env:RUST_LOG="debug"

use tracing::{debug, error, info, instrument, span, warn, Level};
use tracing_subscriber::EnvFilter;

#[instrument] // crea un span automáticamente con los argumentos de la función
fn procesar_pedido(id: u32, importe: f64) -> Result<f64, String> {
    info!("procesando pedido");
    debug!(iva = importe * 0.21, "calculando impuestos");
    if importe < 0.0 {
        error!("importe negativo");
        return Err(format!("pedido {id}: importe negativo"));
    }
    Ok(importe * 1.21)
}

fn main() {
    // Sin RUST_LOG definido, mostramos INFO y superior
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with_target(false)
        .init();

    info!(version = env!("CARGO_PKG_VERSION"), "aplicación iniciada");

    // Campos estructurados: clave = valor (usa ? para Debug y % para Display)
    let usuario = "ana";
    info!(%usuario, intentos = 3, "login correcto");
    warn!(?usuario, "sesión a punto de expirar");

    // Spans manuales: agrupan eventos bajo un contexto
    let span = span!(Level::INFO, "lote", num = 7);
    let _guard = span.enter();

    match procesar_pedido(1, 100.0) {
        Ok(t) => info!(total = t, "pedido OK"),
        Err(e) => error!(%e, "pedido fallido"),
    }
    if let Err(e) = procesar_pedido(2, -5.0) {
        warn!(%e, "se descarta el pedido");
    }
}
