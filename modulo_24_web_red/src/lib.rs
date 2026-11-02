//! API REST de tareas con `axum`: rutas, extractores, estado compartido, middleware y autenticación.
//!
//! El almacenamiento es en memoria para centrarnos en HTTP; en el README se explica
//! cómo sustituirlo por una base de datos con `sqlx`.

use axum::{
    Json, Router,
    extract::{Path, Query, Request, State},
    http::{StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Token de ejemplo. En producción: JWT/sesiones, nunca un valor fijo en el código.
pub const TOKEN: &str = "secreto-del-curso";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tarea {
    pub id: u64,
    pub titulo: String,
    pub hecha: bool,
}

#[derive(Debug, Deserialize)]
pub struct NuevaTarea {
    pub titulo: String,
}

#[derive(Debug, Deserialize)]
pub struct Filtro {
    pub hecha: Option<bool>,
    pub limite: Option<usize>,
}

#[derive(Default)]
struct Datos {
    siguiente_id: u64,
    tareas: BTreeMap<u64, Tarea>,
}

/// Estado compartido entre todos los handlers. `Clone` es barato: solo clona el `Arc`.
#[derive(Clone, Default)]
pub struct Estado {
    datos: Arc<Mutex<Datos>>,
}

/// Error de la API: se convierte en una respuesta HTTP con cuerpo JSON.
#[derive(Debug)]
pub enum ErrorApi {
    NoEncontrada(u64),
    Invalida(&'static str),
}

impl IntoResponse for ErrorApi {
    fn into_response(self) -> Response {
        let (codigo, mensaje) = match self {
            ErrorApi::NoEncontrada(id) => {
                (StatusCode::NOT_FOUND, format!("la tarea {id} no existe"))
            }
            ErrorApi::Invalida(m) => (StatusCode::UNPROCESSABLE_ENTITY, m.to_string()),
        };
        (codigo, Json(serde_json::json!({ "error": mensaje }))).into_response()
    }
}

// ---------------------------- handlers ----------------------------

async fn salud() -> &'static str {
    "ok"
}

// Query<T> extrae ?hecha=true&limite=5; State<T> el estado compartido
async fn listar(State(estado): State<Estado>, Query(f): Query<Filtro>) -> Json<Vec<Tarea>> {
    let datos = estado.datos.lock().unwrap();
    let tareas = datos
        .tareas
        .values()
        .filter(|t| f.hecha.is_none_or(|h| t.hecha == h))
        .take(f.limite.unwrap_or(usize::MAX))
        .cloned()
        .collect();
    Json(tareas)
}

// Json<T> como extractor del cuerpo y como respuesta. Se devuelve (status, body).
async fn crear(
    State(estado): State<Estado>,
    Json(entrada): Json<NuevaTarea>,
) -> Result<(StatusCode, Json<Tarea>), ErrorApi> {
    if entrada.titulo.trim().is_empty() {
        return Err(ErrorApi::Invalida("el título no puede estar vacío"));
    }
    let mut datos = estado.datos.lock().unwrap();
    datos.siguiente_id += 1;
    let tarea = Tarea {
        id: datos.siguiente_id,
        titulo: entrada.titulo,
        hecha: false,
    };
    datos.tareas.insert(tarea.id, tarea.clone());
    Ok((StatusCode::CREATED, Json(tarea)))
}

// Path<T> extrae segmentos de la URL: /tareas/{id}
async fn obtener(
    State(estado): State<Estado>,
    Path(id): Path<u64>,
) -> Result<Json<Tarea>, ErrorApi> {
    let datos = estado.datos.lock().unwrap();
    datos
        .tareas
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(ErrorApi::NoEncontrada(id))
}

async fn completar(
    State(estado): State<Estado>,
    Path(id): Path<u64>,
) -> Result<Json<Tarea>, ErrorApi> {
    let mut datos = estado.datos.lock().unwrap();
    let tarea = datos
        .tareas
        .get_mut(&id)
        .ok_or(ErrorApi::NoEncontrada(id))?;
    tarea.hecha = true;
    Ok(Json(tarea.clone()))
}

async fn borrar(State(estado): State<Estado>, Path(id): Path<u64>) -> Result<StatusCode, ErrorApi> {
    let mut datos = estado.datos.lock().unwrap();
    datos
        .tareas
        .remove(&id)
        .map(|_| StatusCode::NO_CONTENT)
        .ok_or(ErrorApi::NoEncontrada(id))
}

// ---------------------------- middleware ----------------------------

/// Registra método, ruta, código y duración de cada petición.
async fn registrar(req: Request, next: Next) -> Response {
    let (metodo, ruta) = (req.method().clone(), req.uri().path().to_string());
    let inicio = Instant::now();
    let respuesta = next.run(req).await;
    println!(
        "  [log] {metodo} {ruta} -> {} ({:?})",
        respuesta.status().as_u16(),
        inicio.elapsed()
    );
    respuesta
}

/// Exige `Authorization: Bearer <TOKEN>`; si no, responde 401 sin llamar al handler.
async fn autenticar(req: Request, next: Next) -> Result<Response, StatusCode> {
    let valido = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .is_some_and(|t| t == TOKEN);
    if valido {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

/// Construye el router completo. Separarlo de `main` permite probarlo sin abrir la red.
pub fn app() -> Router {
    // Rutas protegidas por el middleware de autenticación
    let protegidas = Router::new()
        .route("/tareas", get(listar).post(crear))
        .route("/tareas/{id}", get(obtener).delete(borrar))
        .route("/tareas/{id}/completar", post(completar))
        .layer(middleware::from_fn(autenticar));

    Router::new()
        .route("/salud", get(salud)) // pública
        .merge(protegidas)
        .layer(middleware::from_fn(registrar)) // aplica a todas
        .with_state(Estado::default())
}
