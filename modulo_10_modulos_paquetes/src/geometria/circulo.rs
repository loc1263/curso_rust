use super::distancia;
use std::f64::consts::PI;

pub struct Circulo {
    pub centro: (f64, f64),
    pub radio: f64,
}

impl Circulo {
    pub fn area(&self) -> f64 {
        PI * self.radio.powi(2)
    }

    pub fn contiene(&self, punto: (f64, f64)) -> bool {
        distancia(self.centro, punto) <= self.radio
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contiene_puntos() {
        let c = Circulo {
            centro: (0.0, 0.0),
            radio: 5.0,
        };
        assert!(c.contiene((3.0, 4.0))); // justo en el borde
        assert!(!c.contiene((4.0, 4.0)));
    }
}
