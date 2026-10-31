// cargo run --example tuple_structs
// Tuple structs, unit structs, newtypes y derives comunes.

#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Punto(f64, f64);

// Newtype: un tipo distinto aunque envuelva lo mismo
#[derive(Debug)]
struct Metros(f64);

#[derive(Debug)]
struct Kilometros(f64);

// Unit struct: sin campos
#[derive(Debug)]
struct Marcador;

fn a_kilometros(m: Metros) -> Kilometros {
    Kilometros(m.0 / 1000.0)
}

fn main() {
    let rojo = Color(255, 0, 0);
    println!("{rojo:?} -> r={}", rojo.0);

    let Color(r, g, b) = rojo;
    println!("{r},{g},{b}");

    let p1 = Punto(1.0, 2.0);
    let p2 = p1; // Copy: p1 sigue siendo válido
    println!("{p1:?} {p2:?} iguales={}", p1 == p2);
    println!("{:?}", Punto::default());

    // Con newtypes el compilador evita mezclar unidades
    let distancia = Metros(2500.0);
    let km = a_kilometros(distancia);
    println!("{:?} -> {} km", km, km.0);
    // a_kilometros(Kilometros(2.5)); // error: tipos distintos

    println!("{:?}", Marcador);
}
