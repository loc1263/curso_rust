// cargo run -p m24_web_red --example cliente
// Arranca la API en un puerto libre y la consume con `reqwest`, todo en el mismo proceso.

use m24_web_red::{TOKEN, Tarea, app};
use reqwest::{Client, StatusCode};
use serde_json::json;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Puerto 0 = "el que el SO tenga libre"
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let base = format!("http://{}", listener.local_addr()?);
    tokio::spawn(async move { axum::serve(listener, app()).await.unwrap() });

    let http = Client::new();

    println!(
        "GET /salud -> {}",
        http.get(format!("{base}/salud"))
            .send()
            .await?
            .text()
            .await?
    );

    let sin_token = http.get(format!("{base}/tareas")).send().await?;
    println!("GET /tareas sin token -> {}", sin_token.status());

    // Crear varias tareas
    for titulo in ["aprender axum", "escribir tests", "desplegar"] {
        let r = http
            .post(format!("{base}/tareas"))
            .bearer_auth(TOKEN)
            .json(&json!({ "titulo": titulo }))
            .send()
            .await?;
        println!("POST /tareas {titulo:?} -> {}", r.status());
    }

    // Validación: título vacío
    let r = http
        .post(format!("{base}/tareas"))
        .bearer_auth(TOKEN)
        .json(&json!({"titulo": " "}))
        .send()
        .await?;
    println!("POST título vacío -> {} {}", r.status(), r.text().await?);

    // Completar y consultar
    http.post(format!("{base}/tareas/2/completar"))
        .bearer_auth(TOKEN)
        .send()
        .await?;
    let t: Tarea = http
        .get(format!("{base}/tareas/2"))
        .bearer_auth(TOKEN)
        .send()
        .await?
        .json()
        .await?;
    println!("GET /tareas/2 -> {t:?}");

    // Filtros con query string
    let hechas: Vec<Tarea> = http
        .get(format!("{base}/tareas?hecha=true"))
        .bearer_auth(TOKEN)
        .send()
        .await?
        .json()
        .await?;
    println!("hechas: {hechas:?}");

    // Borrar y comprobar el 404
    let r = http
        .delete(format!("{base}/tareas/1"))
        .bearer_auth(TOKEN)
        .send()
        .await?;
    println!("DELETE /tareas/1 -> {}", r.status());
    let r = http
        .get(format!("{base}/tareas/1"))
        .bearer_auth(TOKEN)
        .send()
        .await?;
    assert_eq!(r.status(), StatusCode::NOT_FOUND);
    println!("GET /tareas/1 -> {} {}", r.status(), r.text().await?);

    Ok(())
}
