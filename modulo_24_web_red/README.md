# Módulo 24 — Programación web y red

## Objetivos

- Construir una API REST con `axum`: rutas, extractores, estado, errores y middleware.
- Consumir HTTP con `reqwest`.
- Saber cómo se integra una base de datos con `sqlx`.

## Contenido

| Archivo | Tema |
|---------|------|
| `src/lib.rs` | API de tareas: `Router`, `State`, `Json`, `Path`, `Query`, `IntoResponse`, middleware (log + autenticación) |
| `examples/servidor.rs` | `cargo run -p m24_web_red --example servidor` — servidor en `127.0.0.1:3000` con apagado ordenado |
| `examples/cliente.rs` | `cargo run -p m24_web_red --example cliente` — servidor + cliente `reqwest` en el mismo proceso |
| `tests/api.rs` | `cargo test -p m24_web_red` — tests de extremo a extremo |

## Endpoints

| Método | Ruta | Auth | Descripción |
|--------|------|------|-------------|
| GET | `/salud` | no | comprobación de vida |
| GET | `/tareas?hecha=&limite=` | Bearer | lista (filtros opcionales) |
| POST | `/tareas` | Bearer | crea `{ "titulo": "…" }` → 201 |
| GET | `/tareas/{id}` | Bearer | detalle o 404 |
| POST | `/tareas/{id}/completar` | Bearer | marca como hecha |
| DELETE | `/tareas/{id}` | Bearer | borra → 204 |

Token de ejemplo: `secreto-del-curso` (cabecera `Authorization: Bearer secreto-del-curso`).

## Conceptos para recordar

- Un *handler* es una `async fn` cuyos argumentos son **extractores** (`State`, `Path`, `Query`, `Json`, `HeaderMap`…) y cuyo retorno implementa `IntoResponse`.
- El **último** extractor puede consumir el cuerpo (`Json`); los anteriores no.
- `Result<T, E>` es una respuesta válida si `E: IntoResponse`: así se centraliza el manejo de errores (`ErrorApi`).
- Estado compartido: `Router::with_state(estado)`; debe ser `Clone` (normalmente un `Arc<…>` por dentro).
- **Middleware** con `middleware::from_fn`: recibe la petición y `Next`; puede cortar la cadena (401) o medir tiempos.
- `axum` se apoya en `tower`/`hyper`: puedes añadir capas de `tower-http` (CORS, compresión, trazas, timeouts).
- Mantén `app()` separado de `main` para poder probar el router sin red o con un puerto efímero (`:0`).
- Nunca mantengas un `std::sync::MutexGuard` a través de un `.await` (aquí los handlers no esperan con el lock tomado).

## Bases de datos con `sqlx` (guía, no incluida para no exigir un servidor externo)

```toml
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "macros"] }
```

```rust
let pool = sqlx::SqlitePool::connect("sqlite://tareas.db?mode=rwc").await?;
sqlx::query("CREATE TABLE IF NOT EXISTS tareas (id INTEGER PRIMARY KEY, titulo TEXT NOT NULL, hecha BOOLEAN NOT NULL DEFAULT 0)")
    .execute(&pool).await?;

// Consultas con parámetros enlazados (¡nunca concatenes strings: inyección SQL!)
let tarea = sqlx::query_as::<_, Tarea>("SELECT id, titulo, hecha FROM tareas WHERE id = ?")
    .bind(id)
    .fetch_optional(&pool)
    .await?;
```

Para integrarlo: guarda el `SqlitePool` (es `Clone`) en el estado en lugar del `BTreeMap` y convierte `sqlx::Error` en `ErrorApi` con `impl From`.
`sqlx::query!` (con macros) verifica las consultas **en compilación** contra la base de datos.

## Autenticación real

- Contraseñas: `argon2` (hash con sal), nunca en claro.
- Sesiones o JWT (`jsonwebtoken`) con caducidad; extrae el usuario con un extractor propio (`FromRequestParts`).
- HTTPS en el proxy inverso (nginx/Caddy) o con `rustls`.

## Ejercicios

1. Añade `PUT /tareas/{id}` para cambiar el título.
2. Cambia el almacenamiento a SQLite con `sqlx` y conserva los tests.
3. Crea un extractor `Usuario` que lea el token y reemplace el middleware `autenticar`.
4. Añade `tower_http::trace::TraceLayer` y `CorsLayer`.
5. Limita a 10 peticiones por segundo por IP (`tower::limit` o `tower_governor`).
6. Escribe un cliente `reqwest` con reintentos y timeout (`Client::builder().timeout(..)`).
