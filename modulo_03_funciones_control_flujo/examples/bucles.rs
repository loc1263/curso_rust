// cargo run --example bucles
// loop, while, for, rangos, etiquetas, break y continue.

fn main() {
    // loop: infinito hasta `break`; puede devolver un valor
    let mut contador = 0;
    let resultado = loop {
        contador += 1;
        if contador == 10 {
            break contador * 2;
        }
    };
    println!("resultado = {resultado}");

    // while
    let mut n = 3;
    while n > 0 {
        println!("cuenta atrás: {n}");
        n -= 1;
    }

    // for con rangos
    for i in 0..3 {
        print!("{i} "); // 0 1 2
    }
    println!();
    for i in 1..=3 {
        print!("{i} "); // 1 2 3
    }
    println!();
    for i in (1..=3).rev() {
        print!("{i} "); // 3 2 1
    }
    println!();
    for i in (0..10).step_by(3) {
        print!("{i} "); // 0 3 6 9
    }
    println!();

    // for sobre colecciones
    let frutas = ["manzana", "pera", "uva"];
    for fruta in frutas {
        print!("{fruta} ");
    }
    println!();
    for (i, fruta) in frutas.iter().enumerate() {
        println!("{i}: {fruta}");
    }

    // continue
    for i in 1..=6 {
        if i % 2 == 0 {
            continue;
        }
        print!("{i} "); // solo impares
    }
    println!();

    // Etiquetas para salir de bucles anidados
    'externo: for i in 0..5 {
        for j in 0..5 {
            if i * j == 6 {
                println!("encontrado: {i} * {j} = 6");
                break 'externo;
            }
        }
    }
}
