// cargo run -p m11_genericos --example funciones_genericas
// Funciones genéricas, trait bounds, `where` y turbofish.

use std::fmt::{Debug, Display};

// `T` puede ser cualquier tipo que se pueda comparar con `>=`
fn mayor<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

// Copy permite sacar valores del slice sin moverlos
fn maximo<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    let mut it = items.iter().copied();
    let mut max = it.next()?;
    for x in it {
        if x > max {
            max = x;
        }
    }
    Some(max)
}

// Con muchos bounds, `where` es más legible
fn describir<T, U>(a: T, b: U) -> String
where
    T: Display,
    U: Debug + Clone,
{
    format!("a = {a}, b = {:?}", b.clone())
}

fn intercambiar<A, B>(par: (A, B)) -> (B, A) {
    (par.1, par.0)
}

// Un parámetro genérico que no se puede inferir: hay que indicarlo con turbofish
fn valor_por_defecto<T: Default>() -> T {
    T::default()
}

fn main() {
    println!("mayor(3, 7)        = {}", mayor(3, 7));
    println!("mayor(2.5, 1.5)    = {}", mayor(2.5, 1.5));
    println!("mayor(\"a\", \"b\")    = {}", mayor("a", "b"));

    println!("maximo([4, 9, 2])  = {:?}", maximo(&[4, 9, 2]));
    println!("maximo([] de i32)  = {:?}", maximo::<i32>(&[]));
    println!("maximo(chars)      = {:?}", maximo(&['x', 'z', 'a']));

    println!("{}", describir("hola", vec![1, 2, 3]));
    println!("{:?}", intercambiar(("uno", 1)));

    // Turbofish: `::<T>` elige el tipo cuando el compilador no puede deducirlo
    let n = "42".parse::<i32>().unwrap();
    let cero: f64 = valor_por_defecto();
    let vacio = valor_por_defecto::<String>();
    println!("n = {n}, cero = {cero}, vacio = {vacio:?}");

    // Esto NO compila: mayor(1, "uno") -> ambos argumentos deben ser el mismo T
}
