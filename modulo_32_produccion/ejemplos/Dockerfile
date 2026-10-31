# syntax=docker/dockerfile:1
# Imagen multi-etapa: compila en una imagen grande y entrega un binario en una imagen mínima.
#
#   docker build -t mi-app -f ejemplos/Dockerfile .      (desde la raíz del workspace)
#   docker run --rm -e PUERTO=9000 mi-app

# ---------- Etapa 1: planificar dependencias (cargo-chef cachea las dependencias entre builds) ----------
FROM rust:1-slim AS chef
RUN cargo install cargo-chef --locked
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---------- Etapa 2: compilar ----------
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Esta capa SOLO se invalida si cambian las dependencias, no tu código
RUN cargo chef cook --release -p m32_produccion --recipe-path recipe.json
COPY . .
ARG GIT_SHA=sin-git
ENV GIT_SHA=${GIT_SHA}
RUN cargo build --release --locked -p m32_produccion --features completo

# ---------- Etapa 3: imagen final mínima ----------
# distroless/cc incluye solo glibc y certificados: sin shell, sin gestor de paquetes (menos superficie de ataque)
FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=builder /app/target/release/m32_produccion /usr/local/bin/app
USER nonroot
ENV ENTORNO=produccion
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/app"]

# Variantes:
#  - Binario estático y casi vacío:  rustup target add x86_64-unknown-linux-musl  +  FROM scratch
#  - Alpine con musl: cuidado con asignadores y OpenSSL (prefiere rustls)
