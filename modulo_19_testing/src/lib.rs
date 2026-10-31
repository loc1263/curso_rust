//! Biblioteca de ejemplo para practicar testing.
//!
//! Ejecuta todo con `cargo test -p m19_testing`:
//! - tests unitarios (`#[cfg(test)]` al final de este archivo),
//! - tests de integración (carpeta `tests/`),
//! - doc tests (los bloques de código de los comentarios `///`).

use std::fmt;

/// Suma dos números.
///
/// # Ejemplos
///
/// ```
/// use m19_testing::sumar;
/// assert_eq!(sumar(2, 3), 5);
/// ```
pub fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

/// Divide `a` entre `b`, devolviendo `None` si `b` es cero.
///
/// ```
/// use m19_testing::dividir;
/// assert_eq!(dividir(10, 2), Some(5));
/// assert_eq!(dividir(1, 0), None);
/// ```
pub fn dividir(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

/// Comprueba si una cadena es un palíndromo (ignora mayúsculas y no alfanuméricos).
///
/// ```
/// use m19_testing::es_palindromo;
/// assert!(es_palindromo("Anita lava la tina"));
/// assert!(!es_palindromo("Rust"));
/// ```
pub fn es_palindromo(texto: &str) -> bool {
    let limpio: Vec<char> = texto
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();
    limpio.iter().eq(limpio.iter().rev())
}

/// Error al validar una edad.
#[derive(Debug, PartialEq)]
pub enum ErrorEdad {
    Negativa,
    Irreal,
}

impl fmt::Display for ErrorEdad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorEdad::Negativa => write!(f, "la edad no puede ser negativa"),
            ErrorEdad::Irreal => write!(f, "la edad no es realista"),
        }
    }
}

/// Valida una edad.
///
/// ```
/// use m19_testing::{validar_edad, ErrorEdad};
/// assert_eq!(validar_edad(30), Ok(30));
/// assert_eq!(validar_edad(-1), Err(ErrorEdad::Negativa));
/// ```
pub fn validar_edad(edad: i32) -> Result<u8, ErrorEdad> {
    match edad {
        e if e < 0 => Err(ErrorEdad::Negativa),
        e if e > 150 => Err(ErrorEdad::Irreal),
        e => Ok(e as u8),
    }
}

/// Una cuenta bancaria con invariantes que merece la pena probar.
#[derive(Debug, Default)]
pub struct Cuenta {
    saldo: u64,
}

impl Cuenta {
    pub fn nueva(saldo_inicial: u64) -> Self {
        Cuenta { saldo: saldo_inicial }
    }

    pub fn saldo(&self) -> u64 {
        self.saldo
    }

    pub fn depositar(&mut self, monto: u64) {
        self.saldo += monto;
    }

    /// # Panics
    /// Entra en pánico si el saldo no alcanza.
    pub fn retirar(&mut self, monto: u64) {
        assert!(monto <= self.saldo, "saldo insuficiente");
        self.saldo -= monto;
    }
}

// Los tests unitarios viven en el mismo archivo y pueden probar cosas privadas.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suma_basica() {
        assert_eq!(sumar(2, 2), 4);
        assert_ne!(sumar(2, 2), 5);
    }

    #[test]
    fn suma_con_mensaje_propio() {
        let r = sumar(-1, 1);
        assert!(r == 0, "se esperaba 0 pero salió {r}");
    }

    #[test]
    fn division_por_cero_es_none() {
        assert_eq!(dividir(5, 0), None);
    }

    #[test]
    fn palindromos() {
        assert!(es_palindromo("Reconocer"));
        assert!(es_palindromo("A man, a plan, a canal: Panama"));
        assert!(!es_palindromo("hola"));
        assert!(es_palindromo("")); // caso límite
    }

    // Tests que devuelven Result permiten usar `?`
    #[test]
    fn edad_valida() -> Result<(), ErrorEdad> {
        let e = validar_edad(40)?;
        assert_eq!(e, 40);
        Ok(())
    }

    #[test]
    fn edades_invalidas() {
        assert_eq!(validar_edad(-5), Err(ErrorEdad::Negativa));
        assert_eq!(validar_edad(200), Err(ErrorEdad::Irreal));
    }

    #[test]
    #[should_panic(expected = "saldo insuficiente")]
    fn retirar_de_mas_hace_panic() {
        let mut c = Cuenta::nueva(10);
        c.retirar(50);
    }

    #[test]
    fn deposito_y_retiro() {
        let mut c = Cuenta::nueva(100);
        c.depositar(50);
        c.retirar(30);
        assert_eq!(c.saldo(), 120);
    }

    #[test]
    #[ignore = "ejemplo de test lento; ejecútalo con `cargo test -- --ignored`"]
    fn test_lento() {
        let suma: u64 = (0..50_000_000u64).sum();
        assert!(suma > 0);
    }
}
