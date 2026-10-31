//! Plantilla de crate listo para producción.
//!
//! Demuestra: features aditivas, metadatos de compilación (`build.rs`), versión, lints estrictos,
//! documentación con doc tests y una API pequeña y estable.
//!
//! # Ejemplo
//!
//! ```
//! use m32_produccion::{InfoBuild, redondear};
//! assert_eq!(redondear(2.567, 2), 2.57);
//! assert!(!InfoBuild::actual().version.is_empty());
//! ```

/// Información de la compilación, incrustada por `build.rs` y por Cargo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfoBuild {
    /// Versión del paquete (`Cargo.toml`).
    pub version: &'static str,
    /// Perfil de compilación (`debug` o `release`).
    pub perfil: &'static str,
    /// Triple del objetivo, p. ej. `x86_64-pc-windows-msvc`.
    pub objetivo: &'static str,
    /// Commit de git (si CI definió `GIT_SHA`).
    pub commit: &'static str,
    /// Features activadas en esta compilación.
    pub features: Vec<&'static str>,
}

impl InfoBuild {
    /// Datos de la compilación actual.
    #[must_use]
    pub fn actual() -> Self {
        let mut features = Vec::new();
        if cfg!(feature = "estadisticas") {
            features.push("estadisticas");
        }
        if cfg!(feature = "colores") {
            features.push("colores");
        }
        InfoBuild {
            version: env!("CARGO_PKG_VERSION"),
            perfil: env!("M32_PERFIL"),
            objetivo: env!("M32_OBJETIVO"),
            commit: env!("M32_COMMIT"),
            features,
        }
    }
}

impl std::fmt::Display for InfoBuild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "v{} [{}] {} commit={} features=[{}]",
            self.version,
            self.perfil,
            self.objetivo,
            self.commit,
            self.features.join(",")
        )
    }
}

/// Redondea `x` a `decimales` decimales.
#[must_use]
pub fn redondear(x: f64, decimales: u32) -> f64 {
    let factor = 10f64.powi(i32::try_from(decimales).unwrap_or(i32::MAX));
    (x * factor).round() / factor
}

/// Resumen estadístico. Solo existe con la feature `estadisticas`.
#[cfg(feature = "estadisticas")]
pub mod estadisticas {
    /// Media, mínimo y máximo de una serie.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Resumen {
        /// Media aritmética.
        pub media: f64,
        /// Valor mínimo.
        pub min: f64,
        /// Valor máximo.
        pub max: f64,
    }

    /// Calcula el resumen; `None` si la serie está vacía.
    ///
    /// ```
    /// # #[cfg(feature = "estadisticas")] {
    /// use m32_produccion::estadisticas::resumir;
    /// let r = resumir(&[1.0, 2.0, 6.0]).unwrap();
    /// assert_eq!((r.media, r.min, r.max), (3.0, 1.0, 6.0));
    /// # }
    /// ```
    #[must_use]
    pub fn resumir(datos: &[f64]) -> Option<Resumen> {
        if datos.is_empty() {
            return None;
        }
        let suma: f64 = datos.iter().sum();
        Some(Resumen {
            #[allow(clippy::cast_precision_loss)]
            media: suma / datos.len() as f64,
            min: datos.iter().copied().fold(f64::INFINITY, f64::min),
            max: datos.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        })
    }
}

/// Utilidades de color ANSI. Solo existe con la feature `colores`.
#[cfg(feature = "colores")]
pub mod colores {
    /// Texto en verde.
    #[must_use]
    pub fn verde(s: &str) -> String {
        format!("\x1b[32m{s}\x1b[0m")
    }
    /// Texto en rojo.
    #[must_use]
    pub fn rojo(s: &str) -> String {
        format!("\x1b[31m{s}\x1b[0m")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redondeo() {
        assert_eq!(redondear(1.005_f64 * 1000.0, 0), 1005.0);
        assert_eq!(redondear(2.567_89, 3), 2.568);
    }

    #[test]
    fn la_info_de_build_esta_rellena() {
        let i = InfoBuild::actual();
        assert_ne!(i.objetivo, "");
        assert!(i.to_string().starts_with('v'));
    }

    #[cfg(feature = "estadisticas")]
    #[test]
    fn estadisticas_serie_vacia() {
        assert!(estadisticas::resumir(&[]).is_none());
    }
}
