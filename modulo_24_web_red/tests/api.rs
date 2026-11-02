// Tests de la API contra un servidor real en un puerto efímero.

use m24_web_red::{TOKEN, Tarea, app};
use reqwest::{Client, StatusCode};
use serde_json::json;
use tokio::net::TcpListener;

async fn servidor() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    tokio::spawn(async move { axum::serve(listener, app()).await.unwrap() });
    base
}

#[tokio::test]
async fn salud_es_publica() {
    let base = servidor().await;
    let r = reqwest::get(format!("{base}/salud")).await.unwrap();
    assert_eq!(r.status(), StatusCode::OK);
    assert_eq!(r.text().await.unwrap(), "ok");
}

#[tokio::test]
async fn sin_token_da_401_y_con_token_incorrecto_tambien() {
    let base = servidor().await;
    let c = Client::new();
    assert_eq!(
        c.get(format!("{base}/tareas"))
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::UNAUTHORIZED
    );
    let r = c
        .get(format!("{base}/tareas"))
        .bearer_auth("otro")
        .send()
        .await
        .unwrap();
    assert_eq!(r.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn ciclo_completo_crear_completar_borrar() {
    let base = servidor().await;
    let c = Client::new();

    let r = c
        .post(format!("{base}/tareas"))
        .bearer_auth(TOKEN)
        .json(&json!({"titulo": "x"}))
        .send()
        .await
        .unwrap();
    assert_eq!(r.status(), StatusCode::CREATED);
    let t: Tarea = r.json().await.unwrap();
    assert_eq!((t.id, t.hecha), (1, false));

    let t: Tarea = c
        .post(format!("{base}/tareas/1/completar"))
        .bearer_auth(TOKEN)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert!(t.hecha);

    let r = c
        .delete(format!("{base}/tareas/1"))
        .bearer_auth(TOKEN)
        .send()
        .await
        .unwrap();
    assert_eq!(r.status(), StatusCode::NO_CONTENT);
    let r = c
        .get(format!("{base}/tareas/1"))
        .bearer_auth(TOKEN)
        .send()
        .await
        .unwrap();
    assert_eq!(r.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn titulo_vacio_es_422() {
    let base = servidor().await;
    let r = Client::new()
        .post(format!("{base}/tareas"))
        .bearer_auth(TOKEN)
        .json(&json!({"titulo": ""}))
        .send()
        .await
        .unwrap();
    assert_eq!(r.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn filtro_por_hecha_y_limite() {
    let base = servidor().await;
    let c = Client::new();
    for t in ["a", "b", "c"] {
        c.post(format!("{base}/tareas"))
            .bearer_auth(TOKEN)
            .json(&json!({"titulo": t}))
            .send()
            .await
            .unwrap();
    }
    c.post(format!("{base}/tareas/2/completar"))
        .bearer_auth(TOKEN)
        .send()
        .await
        .unwrap();

    let hechas: Vec<Tarea> = c
        .get(format!("{base}/tareas?hecha=true"))
        .bearer_auth(TOKEN)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(hechas.len(), 1);
    let dos: Vec<Tarea> = c
        .get(format!("{base}/tareas?limite=2"))
        .bearer_auth(TOKEN)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(dos.len(), 2);
}
