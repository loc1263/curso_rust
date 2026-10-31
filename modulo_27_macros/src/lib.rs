//! Macros declarativas (`macro_rules!`) del módulo 27 + reexportación de las procedurales.

pub use m27_macros_derive::{Describir, mayusculas, medir_tiempo};

/// `mapa!{ "a" => 1, "b" => 2 }` crea un HashMap (con coma final opcional).
#[macro_export]
macro_rules! mapa {
    // Caso vacío
    () => { ::std::collections::HashMap::new() };
    // Repetición: $( ... ),* = cero o más separados por coma; $(,)? = coma final opcional
    ( $( $clave:expr => $valor:expr ),+ $(,)? ) => {{
        let mut m = ::std::collections::HashMap::new();
        $( m.insert($clave, $valor); )+
        m
    }};
}

/// `maximo!(a, b, c, ...)`: macro recursiva.
#[macro_export]
macro_rules! maximo {
    ($x:expr) => { $x };
    ($x:expr, $( $resto:expr ),+) => {{
        let a = $x;
        let b = $crate::maximo!($( $resto ),+);
        if a > b { a } else { b }
    }};
}

/// Genera un enum con `Display` y una lista de todas sus variantes.
#[macro_export]
macro_rules! enum_con_nombres {
    ( $(#[$meta:meta])* $vis:vis enum $nombre:ident { $( $variante:ident ),+ $(,)? } ) => {
        $(#[$meta])*
        $vis enum $nombre { $( $variante ),+ }

        impl $nombre {
            pub const TODAS: &'static [$nombre] = &[ $( $nombre::$variante ),+ ];

            pub fn nombre(&self) -> &'static str {
                match self { $( $nombre::$variante => stringify!($variante) ),+ }
            }
        }

        impl ::std::fmt::Display for $nombre {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(self.nombre())
            }
        }
    };
}

/// Crea un struct con un constructor `nuevo` que recibe todos los campos.
#[macro_export]
macro_rules! struct_con_nuevo {
    ( $vis:vis struct $nombre:ident { $( $campo:ident : $tipo:ty ),* $(,)? } ) => {
        #[derive(Debug, Clone)]
        $vis struct $nombre { $( pub $campo: $tipo ),* }
        impl $nombre {
            pub fn nuevo( $( $campo: $tipo ),* ) -> Self { Self { $( $campo ),* } }
        }
    };
}

/// Mini-DSL con palabras: `calcular!(2 por 3 mas 4)` = 10 ("tt muncher"). `por` liga más fuerte que `mas`.
#[macro_export]
macro_rules! calcular {
    ($n:literal) => { $n };
    ($n:literal mas $($resto:tt)+) => { $n + $crate::calcular!($($resto)+) };
    ($n:literal por $m:literal $($resto:tt)*) => { $crate::calcular!(($n * $m) $($resto)*) };
    (($e:expr)) => { $e };
    (($e:expr) mas $($resto:tt)+) => { $e + $crate::calcular!($($resto)+) };
    (($e:expr) por $m:literal $($resto:tt)*) => { $crate::calcular!(($e * $m) $($resto)*) };
}

/// Cuenta los argumentos en compilación.
#[macro_export]
macro_rules! contar {
    () => { 0usize };
    ($cabeza:tt $($cola:tt)*) => { 1usize + $crate::contar!($($cola)*) };
}

#[cfg(test)]
mod tests {
    #[test]
    fn mapa_y_maximo() {
        let m = mapa! { "a" => 1, "b" => 2, };
        assert_eq!(m["b"], 2);
        assert_eq!(maximo!(3, 9, 4), 9);
        assert_eq!(contar!(a b c d), 4);
    }

    #[test]
    fn calculadora() {
        assert_eq!(calcular!(2 mas 3), 5);
        assert_eq!(calcular!(2 por 3 mas 4), 10);
    }
}
