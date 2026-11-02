# Módulo 18 — Errores avanzados

## La idea en 2 minutos

En el Nivel 1 viste `Result` y `?`. Aquí resolvemos una duda práctica: *¿qué tipo de error devuelvo?* La respuesta depende de **quién va a leer el error**:

- **Una biblioteca** → quien la usa quiere distinguir casos (`match`): usa un `enum` con **`thiserror`**.
- **Una aplicación** → solo quieres informar al usuario con contexto: usa **`anyhow`**.

```rust
// biblioteca
#[derive(Debug, thiserror::Error)]
enum ErrorConfig {
    #[error("falta la clave '{0}'")] Falta(String),
}

// aplicación
let texto = std::fs::read_to_string(ruta).with_context(|| format!("no se pudo abrir {ruta}"))?;
```

**Orden recomendado:** `box_dyn_error` (lo que hay "por debajo") → `thiserror_lib` → `anyhow_app`.
(`anyhow_app` termina con un error a propósito para mostrar cómo `main` puede devolver `Result`.)

## Objetivos

- Diseñar tipos de error propios y componibles.
- Usar `thiserror` en bibliotecas y `anyhow` en aplicaciones.
- Entender `Box<dyn Error>`, `source()` y `downcast`.

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m18_errores_avanzados --example box_dyn_error` | `Box<dyn Error>`, `source`, `downcast_ref` |
| `cargo run -p m18_errores_avanzados --example thiserror_lib` | enum de errores con `#[derive(Error)]`, `#[from]` |
| `cargo run -p m18_errores_avanzados --example anyhow_app` | `anyhow::Result`, `context`, `bail!`, `ensure!` |

## Regla práctica

| Escribes… | Usa… | Por qué |
|-----------|------|---------|
| una biblioteca | `thiserror` (enum tipado) | quien llama puede hacer `match` sobre las variantes |
| una aplicación | `anyhow` | solo te importa informar con contexto, no distinguir variantes |

## Conceptos para recordar

- `#[from]` implementa `From<E>` y `source()`; así `?` convierte errores sin código extra.
- `anyhow::Error` es un `Box<dyn Error>` mejorado: con contexto, backtrace opcional y `downcast`.
- `.context("...")` envuelve el error original; `{:#}` imprime la cadena en una línea y `{:?}` la detallada.
- `source()` permite recorrer la cadena de causas.
- Evita `unwrap()` en código de biblioteca; propaga el error con `?`.

## Ejercicios

1. Define un `enum ErrorBanco` con `thiserror` (saldo insuficiente, cuenta inexistente, E/S).
2. Escribe una función con `anyhow::Result` que lea un JSON/CSV añadiendo contexto con el nombre del archivo.
3. Recorre la cadena de `source()` de un error y numera cada nivel.
4. Convierte un error de `thiserror` en `anyhow::Error` con `?` y recupéralo con `downcast_ref`.
5. Usa `bail!` y `ensure!` para validar los argumentos de una función.
6. Decide si un parser de formato propio debe devolver `anyhow::Error` o un enum tipado, y justifica.
