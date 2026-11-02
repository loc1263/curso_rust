// cargo run -p m28_tipos_avanzados --example asociados_gat_const
// Tipos asociados, constantes asociadas, GATs (Generic Associated Types) y const generics.

use std::fmt::Debug;
use std::ops::{Add, Mul};

// ---------- Tipos asociados vs parámetros genéricos ----------
// Con un tipo asociado cada implementación tiene UN solo Item (no hay que repetirlo en cada uso).
trait Contenedor {
    type Item;
    const NOMBRE: &'static str;
    fn vacio() -> Self;
    fn insertar(&mut self, item: Self::Item);
    fn tamano(&self) -> usize;
}

impl<T> Contenedor for Vec<T> {
    type Item = T;
    const NOMBRE: &'static str = "Vec";
    fn vacio() -> Self {
        Vec::new()
    }
    fn insertar(&mut self, item: T) {
        self.push(item);
    }
    fn tamano(&self) -> usize {
        self.len()
    }
}

// `C::Item` se obtiene del propio C: la función solo necesita UN parámetro genérico
fn llenar<C: Contenedor>(items: impl IntoIterator<Item = C::Item>) -> C {
    let mut c = C::vacio();
    for i in items {
        c.insertar(i);
    }
    println!("  construido un {} con {} elementos", C::NOMBRE, c.tamano());
    c
}

// ---------- GAT: un tipo asociado con parámetros (aquí, un lifetime) ----------
// Permite un "iterador que presta" (lending iterator): cada Item toma prestado del iterador.
trait IteradorPrestado {
    type Item<'a>
    where
        Self: 'a;
    fn siguiente<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

// Ventanas SOLAPADAS y mutables sobre un slice: imposible con Iterator normal
struct VentanasMut<'s, T> {
    datos: &'s mut [T],
    tam: usize,
    pos: usize,
}

impl<'s, T> IteradorPrestado for VentanasMut<'s, T> {
    type Item<'a>
        = &'a mut [T]
    where
        Self: 'a;
    fn siguiente<'a>(&'a mut self) -> Option<&'a mut [T]> {
        let ventana = self.datos.get_mut(self.pos..self.pos + self.tam)?;
        self.pos += 1;
        Some(ventana)
    }
}

// ---------- Const generics ----------
#[derive(Debug, Clone, Copy, PartialEq)]
struct Matriz<T, const F: usize, const C: usize> {
    datos: [[T; C]; F],
}

impl<T: Copy + Default, const F: usize, const C: usize> Matriz<T, F, C> {
    fn nueva(datos: [[T; C]; F]) -> Self {
        Matriz { datos }
    }

    // El tipo de retorno INTERCAMBIA las dimensiones: el compilador lo comprueba
    #[allow(clippy::needless_range_loop)] // índices explícitos: más claro para ver F y C
    fn transponer(&self) -> Matriz<T, C, F> {
        let mut r = [[T::default(); F]; C];
        for i in 0..F {
            for j in 0..C {
                r[j][i] = self.datos[i][j];
            }
        }
        Matriz { datos: r }
    }
}

// (F×C) · (C×K) = (F×K): multiplicar matrices con dimensiones incompatibles NO COMPILA
impl<T, const F: usize, const C: usize, const K: usize> Mul<Matriz<T, C, K>> for Matriz<T, F, C>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T>,
{
    type Output = Matriz<T, F, K>;
    #[allow(clippy::needless_range_loop)]
    fn mul(self, otra: Matriz<T, C, K>) -> Matriz<T, F, K> {
        let mut r = [[T::default(); K]; F];
        for i in 0..F {
            for j in 0..K {
                for k in 0..C {
                    r[i][j] = r[i][j] + self.datos[i][k] * otra.datos[k][j];
                }
            }
        }
        Matriz { datos: r }
    }
}

fn primeros<T: Debug + Copy, const N: usize>(v: &[T]) -> Option<[T; N]> {
    v.get(..N)?.try_into().ok()
}

fn main() {
    println!("-- tipos asociados --");
    let v: Vec<i32> = llenar::<Vec<i32>>([1, 2, 3]);
    println!("  {v:?}");

    println!("\n-- GAT: ventanas mutables solapadas --");
    let mut datos = [1, 2, 3, 4, 5];
    let mut it = VentanasMut {
        datos: &mut datos,
        tam: 3,
        pos: 0,
    };
    while let Some(v) = it.siguiente() {
        v[0] += 10; // modifica y la siguiente ventana verá el cambio
        println!("  {v:?}");
    }

    println!("\n-- const generics --");
    let a = Matriz::nueva([[1, 2, 3], [4, 5, 6]]); // 2×3
    let b = a.transponer(); // 3×2
    println!("a = {a:?}\nbᵀ = {b:?}");
    let producto = a * b; // (2×3)·(3×2) = 2×2
    println!("a·aᵀ = {producto:?}");
    // let mal = a * a; // ERROR de compilación: (2×3)·(2×3) no es válido

    println!("{:?}", primeros::<_, 2>(&[9, 8, 7]));
    println!("{:?}", primeros::<_, 5>(&[9, 8, 7]));
}
