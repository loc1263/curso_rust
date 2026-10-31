// cargo run --example condicionales
// `if` como sentencia y como expresión.

fn clasificar(nota: u32) -> &'static str {
    if nota >= 90 {
        "A"
    } else if nota >= 80 {
        "B"
    } else if nota >= 70 {
        "C"
    } else {
        "F"
    }
}

fn main() {
    let n = 7;

    if n < 5 {
        println!("{n} es menor que 5");
    } else if n < 10 {
        println!("{n} está entre 5 y 9");
    } else {
        println!("{n} es 10 o más");
    }

    // `if` es una expresión: ambas ramas deben tener el mismo tipo
    let paridad = if n % 2 == 0 { "par" } else { "impar" };
    println!("{n} es {paridad}");

    // La condición debe ser bool (no existe "truthy" como en otros lenguajes)
    // if n { }  // error: expected `bool`, found integer

    // Operadores lógicos
    let edad = 20;
    let tiene_carnet = true;
    if edad >= 18 && tiene_carnet {
        println!("Puede conducir");
    }
    if !(edad < 18 || !tiene_carnet) {
        println!("Mismo resultado, otra forma");
    }

    for nota in [95, 83, 71, 40] {
        println!("{nota} -> {}", clasificar(nota));
    }
}
