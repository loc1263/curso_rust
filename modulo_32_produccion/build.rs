//! Script de compilación: se ejecuta ANTES de compilar el crate.
//! Aquí exponemos datos de la compilación como variables de entorno disponibles con `env!`.

use std::env;

fn main() {
    // Solo reejecutar si cambia este archivo (evita recompilar siempre)
    println!("cargo:rerun-if-changed=build.rs");

    // Cargo da: PROFILE, TARGET, HOST, OPT_LEVEL, CARGO_PKG_*, CARGO_FEATURE_*...
    let perfil = env::var("PROFILE").unwrap_or_else(|_| "desconocido".into());
    let objetivo = env::var("TARGET").unwrap_or_else(|_| "desconocido".into());
    println!("cargo:rustc-env=M32_PERFIL={perfil}");
    println!("cargo:rustc-env=M32_OBJETIVO={objetivo}");

    // Un commit de git se podría añadir así (en CI con la variable GIT_SHA):
    let sha = env::var("GIT_SHA").unwrap_or_else(|_| "sin-git".into());
    println!("cargo:rustc-env=M32_COMMIT={sha}");
    println!("cargo:rerun-if-env-changed=GIT_SHA");
}
