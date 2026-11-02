// cargo run -p m24_web_red --example servidor
// Levanta la API en http://127.0.0.1:3000 (Ctrl+C para detener).
//
// Prueba con curl (en PowerShell usa `curl.exe`):
//   curl http://127.0.0.1:3000/salud
//   curl -H "Authorization: Bearer secreto-del-curso" -H "Content-Type: application/json" \
//        -d '{"titulo":"aprender axum"}' http://127.0.0.1:3000/tareas
//   curl -H "Authorization: Bearer secreto-del-curso" http://127.0.0.1:3000/tareas

use m24_web_red::app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let direccion = std::env::var("DIRECCION").unwrap_or_else(|_| "127.0.0.1:3000".into());
    let listener = TcpListener::bind(&direccion).await?;
    println!("escuchando en http://{}", listener.local_addr()?);

    axum::serve(listener, app())
        // Apagado ordenado: deja terminar las peticiones en curso al recibir Ctrl+C
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("no se pudo escuchar Ctrl+C");
            println!("\napagando...");
        })
        .await
}
