// cargo run --example match_patrones
// Patrones: literales, rangos, guardas, bindings, desestructuración, let else...

struct Punto {
    x: i32,
    y: i32,
}

fn describir(n: i32) -> &'static str {
    match n {
        0 => "cero",
        1 | 2 | 3 => "pequeño",   // varios valores
        4..=9 => "mediano",       // rango inclusivo
        x if x < 0 => "negativo", // guarda
        _ => "grande",            // comodín: match debe ser exhaustivo
    }
}

fn clasificar_edad(edad: u32) -> String {
    match edad {
        n @ 0..=12 => format!("{n}: niño"),     // @ liga el valor
        n @ 13..=17 => format!("{n}: adolescente"),
        n => format!("{n}: adulto"),
    }
}

fn ubicar(p: Punto) -> &'static str {
    match p {
        Punto { x: 0, y: 0 } => "origen",
        Punto { x, y: 0 } if x > 0 => "eje X positivo",
        Punto { x: 0, .. } => "eje Y",
        Punto { .. } => "otro lugar",
    }
}

fn primer_par(v: &[i32]) -> Option<i32> {
    // let else: o se cumple el patrón, o se sale
    let Some(&x) = v.iter().find(|&&n| n % 2 == 0) else {
        return None;
    };
    Some(x)
}

fn main() {
    for n in [0, 2, 7, -5, 100] {
        println!("{n}: {}", describir(n));
    }
    println!("{}", clasificar_edad(8));
    println!("{}", clasificar_edad(30));

    println!("{}", ubicar(Punto { x: 0, y: 0 }));
    println!("{}", ubicar(Punto { x: 5, y: 0 }));
    println!("{}", ubicar(Punto { x: 0, y: 9 }));
    println!("{}", ubicar(Punto { x: 3, y: 3 }));

    // Tuplas y patrones anidados
    let par = (3, -3);
    match par {
        (x, y) if x + y == 0 => println!("suman cero"),
        (x, _) if x % 2 == 1 => println!("x es impar"),
        _ => println!("nada especial"),
    }

    let anidado: Option<Option<i32>> = Some(Some(4));
    if let Some(Some(n)) = anidado {
        println!("anidado: {n}");
    }

    // while let
    let mut pila = vec![1, 2, 3];
    while let Some(tope) = pila.pop() {
        print!("{tope} ");
    }
    println!();

    println!("{:?} {:?}", primer_par(&[1, 3, 4, 5]), primer_par(&[1, 3]));

    // matches! devuelve bool
    println!("{}", matches!('x', 'a'..='z'));
}
