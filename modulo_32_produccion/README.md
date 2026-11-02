# Módulo 32 — Producción

## La idea en 2 minutos

Escribir código que funciona en tu máquina es la mitad del trabajo. **Producción** es todo lo que lo rodea para que otros lo usen sin sorpresas:

```
código ──▶ calidad (fmt, clippy, tests) ──▶ seguridad (audit, deny) ──▶ empaquetado (release, Docker) ──▶ observabilidad (logs, métricas)
```

Las piezas de este módulo y su pregunta asociada:

| Pieza | Responde a... |
|---|---|
| Workspaces | ¿cómo organizo varios crates sin repetir versiones? |
| Features | ¿cómo hago partes opcionales sin compilar lo que no uso? |
| CI (`ci.yml`) | ¿cómo me aseguro de que nada se rompe antes de fusionar? |
| `cargo-deny` / `audit` | ¿mis dependencias tienen vulnerabilidades o licencias problemáticas? |
| Dockerfile | ¿cómo entrego un binario pequeño y seguro? |
| SemVer + crates.io | ¿cómo publico sin romper a quien me usa? |

**Cómo estudiarlo:** prueba la plantilla con y sin features (`--features completo`, `--no-default-features`) y luego abre los archivos de `ejemplos/` como si fueran recetas para copiar a tu propio repositorio.

## Objetivos

- Organizar proyectos grandes con **workspaces** y **features**.
- Automatizar calidad y seguridad en **CI/CD**.
- **Publicar** un crate, empaquetar en **Docker** y añadir **observabilidad**.

## Contenido

| Archivo | Tema |
|---------|------|
| `Cargo.toml` | metadatos de publicación, `rust-version` (MSRV), features aditivas, `[lints]` |
| `build.rs` | script de compilación que incrusta perfil, objetivo y commit con `cargo:rustc-env` |
| `src/lib.rs`, `src/main.rs` | API documentada con doc tests; binario configurado por entorno (12-factor) con códigos de salida |
| `examples/features.rs` | `cargo run -p m32_produccion --example features [--features colores] [--no-default-features]` |
| `ejemplos/workspace_raiz.toml` | workspace con `[workspace.package]`, `[workspace.dependencies]` y lints compartidos |
| `ejemplos/.github/workflows/ci.yml` | CI: fmt, clippy, tests multi-SO, MSRV, `cargo-deny`, `cargo-audit`, cobertura, publicación |
| `ejemplos/deny.toml` | política de licencias, avisos y fuentes de dependencias |
| `ejemplos/Dockerfile` | imagen multi-etapa con `cargo-chef` y `distroless` (usuario no root) |

La carpeta `ejemplos/` contiene plantillas para copiar a la **raíz de tu repositorio**; aquí están inertes a propósito.

## Probar la plantilla

```bash
cargo run  -p m32_produccion                          # features por defecto (estadisticas)
cargo run  -p m32_produccion --features completo      # + colores
cargo run  -p m32_produccion --no-default-features    # sin nada opcional
cargo test -p m32_produccion --all-features
cargo test -p m32_produccion --no-default-features    # ¡las combinaciones deben compilar!
cargo clippy -p m32_produccion --all-features
cargo doc  -p m32_produccion --open
```

## Workspaces

Un workspace agrupa varios crates con un único `Cargo.lock` y un único directorio `target/`.
Estructura típica: `crates/nucleo` (lógica, sin I/O), `crates/cli`, `crates/servidor`, `crates/macros`…

- `[workspace.package]` y `[workspace.dependencies]` centralizan versiones (los miembros usan `x.workspace = true`).
- `cargo build -p nombre`, `cargo test --workspace`, `cargo test --workspace --exclude lento`.
- Los `[profile.*]` **solo** se leen en el `Cargo.toml` raíz.
- Separar un crate de macros, un núcleo sin dependencias y los binarios reduce los tiempos de compilación.

## Features

- Deben ser **aditivas**: si dos crates dependen del tuyo con features distintas, Cargo las **unifica**; activar una no puede romper otra.
- Evita features que "excluyan" a otras. Para elegir implementaciones usa traits o módulos, no `#[cfg(not(feature))]` contradictorios.
- Marca dependencias opcionales: `serde = { version = "1", optional = true }` crea la feature implícita `serde`.
- Prueba combinaciones: `--no-default-features`, `--all-features`, y cada una con `cargo hack --feature-powerset`.

## Calidad y seguridad (qué hace cada herramienta)

| Herramienta | Función |
|-------------|---------|
| `cargo fmt --check` | estilo uniforme |
| `cargo clippy -- -D warnings` | errores comunes y estilo idiomático |
| `cargo test` + `cargo llvm-cov` | pruebas y cobertura |
| `cargo audit` | vulnerabilidades conocidas (RustSec) en `Cargo.lock` |
| `cargo deny` | licencias, duplicados, fuentes no permitidas, crates prohibidos |
| `cargo vet` / `cargo crev` | confianza en el código de las dependencias |
| `cargo semver-checks` | detecta cambios que rompen la API pública antes de publicar |
| `cargo udeps` / `cargo machete` | dependencias sin usar |
| `cargo outdated` / Dependabot / Renovate | actualizaciones |
| `cargo +nightly miri test` | UB en código `unsafe` |

## Publicar en crates.io

1. Completa `description`, `license`, `repository`, `readme`, `keywords` (≤ 5), `categories`; quita `publish = false`.
2. Documenta la API (`missing_docs`), añade ejemplos en doc tests y un `CHANGELOG.md`.
3. `cargo publish --dry-run` y revisa el contenido con `cargo package --list`.
4. Etiqueta la versión (`git tag v0.3.1`) y `cargo publish` (requiere `cargo login <token>`).
5. **Publicar es permanente**: solo se puede *yank* (`cargo yank --version 0.3.1`), no borrar.
6. Sigue **SemVer**: romper la API pública ⇒ subir el major (en `0.x`, el minor). Declara y prueba la **MSRV**.
7. Para crates internos de una empresa: registro privado (Kellnr, Artifactory) con `[registries]`.

## Observabilidad

| Pilar | Herramienta Rust |
|-------|------------------|
| **Logs** estructurados | `tracing` + `tracing-subscriber` (formato JSON en producción con `.json()`) |
| **Trazas** distribuidas | `tracing-opentelemetry` + `opentelemetry-otlp` → Jaeger / Tempo / Honeycomb |
| **Métricas** | `metrics` + exportador Prometheus (`metrics-exporter-prometheus`) o `opentelemetry` |
| **Salud** | endpoints `/salud` (liveness) y `/listo` (readiness) |
| **Errores** | `sentry`, informes de panics con `std::panic::set_hook` |

Recomendaciones: un `request_id` en cada span; no registres secretos ni datos personales; niveles
configurables con `RUST_LOG`; cierre ordenado (`ctrl_c` + drenar peticiones) como en el módulo 24.

## Docker y despliegue

- **Multi-etapa**: compila con la imagen `rust`, ejecuta en `distroless`/`scratch` (decenas de MB, sin shell).
- **Caché de dependencias** con `cargo-chef` para builds de CI rápidos.
- Ejecuta como **usuario no root**, sin capacidades extra, con `readOnlyRootFilesystem` si es posible.
- Configuración por **variables de entorno**; secretos por el orquestador, nunca en la imagen.
- Compila con `--locked` para builds reproducibles; fija la imagen base por *digest* en producción.
- Binarios estáticos: target `x86_64-unknown-linux-musl` (usa `rustls` en vez de OpenSSL).
- Perfil `release` recomendado: `lto = "thin"`, `codegen-units = 1`, `strip = true`, `panic = "abort"` (si no usas `catch_unwind`).
- Cross-compilation: `cross` (usa Docker) o `cargo zigbuild`.

## Ejercicios

1. Convierte el repositorio del curso a un workspace con `[workspace.package]` y `[workspace.dependencies]`.
2. Copia `ejemplos/.github/workflows/ci.yml` a un repositorio de prueba y haz que `cargo clippy -D warnings` pase.
3. Instala `cargo-deny` y `cargo-audit` y ejecútalos sobre `modulo_24_web_red`.
4. Construye la imagen Docker y compara su tamaño con `rust:1` (usa `docker images`).
5. Añade `tracing` con salida JSON a `src/main.rs` y un campo `request_id` por operación.
6. Publica un crate de prueba (`cargo publish --dry-run`) y practica `cargo semver-checks` tras cambiar una firma pública.
