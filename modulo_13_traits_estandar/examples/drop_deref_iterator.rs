// cargo run -p m13_traits_estandar --example drop_deref_iterator
// Otros traits muy usados: Drop, Deref, Index, IntoIterator y PartialEq entre tipos.

use std::ops::{Deref, DerefMut, Index};

// Drop: código que se ejecuta al salir del ámbito (RAII)
struct Guardia(&'static str);
impl Drop for Guardia {
    fn drop(&mut self) {
        println!("  [drop] liberando {}", self.0);
    }
}

// Deref: hace que un wrapper se comporte como el tipo que envuelve
struct Texto(String);
impl Deref for Texto {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl DerefMut for Texto {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

// Index: sobrecarga de `[]`
struct Tabla {
    datos: Vec<i32>,
}
impl Index<usize> for Tabla {
    type Output = i32;
    fn index(&self, i: usize) -> &i32 {
        &self.datos[i]
    }
}

// IntoIterator: permite usar tu tipo en `for`
struct Equipo {
    miembros: Vec<String>,
}
impl IntoIterator for Equipo {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;
    fn into_iter(self) -> Self::IntoIter {
        self.miembros.into_iter()
    }
}
impl<'a> IntoIterator for &'a Equipo {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.miembros.iter()
    }
}

// PartialEq entre tipos distintos
struct Metros(f64);
struct Centimetros(f64);
impl PartialEq<Centimetros> for Metros {
    fn eq(&self, o: &Centimetros) -> bool {
        (self.0 * 100.0 - o.0).abs() < 1e-9
    }
}

fn longitud_de(s: &str) -> usize {
    s.len()
}

fn main() {
    println!("Entrando al bloque");
    {
        let _a = Guardia("A");
        let _b = Guardia("B");
        println!("  trabajando...");
    } // se liberan en orden inverso: B y luego A
    let c = Guardia("C");
    drop(c); // liberación anticipada
    println!("Fuera del bloque\n");

    let mut t = Texto("hola".into());
    t.push_str(" mundo"); // DerefMut -> String::push_str
    println!("len = {}, mayúsculas = {}", t.len(), t.to_uppercase());
    println!("deref coercion: {}", longitud_de(&t)); // &Texto -> &String -> &str

    let tabla = Tabla {
        datos: vec![10, 20, 30],
    };
    println!("tabla[1] = {}", tabla[1]);

    let eq = Equipo {
        miembros: vec!["Ana".into(), "Luis".into()],
    };
    for m in &eq {
        println!("miembro: {m}");
    }
    for m in eq {
        println!("consumido: {m}");
    }

    println!("1 m == 100 cm ? {}", Metros(1.0) == Centimetros(100.0));
}
