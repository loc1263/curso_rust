// cargo run --example funciones
// Funciones, parámetros, retorno y la diferencia entre expresiones y sentencias.

fn saludar() {
    println!("¡Hola!");
}

fn sumar(a: i32, b: i32) -> i32 {
    a + b // sin `;` -> es la expresión que se devuelve
}

fn cuadrado(x: i32) -> i32 {
    return x * x; // `return` explícito también es válido
}

fn dividir(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        return f64::NAN; // retorno anticipado
    }
    a / b
}

// Varios valores de retorno con una tupla
fn min_max(v: &[i32]) -> (i32, i32) {
    let mut min = v[0];
    let mut max = v[0];
    for &n in v {
        if n < min {
            min = n;
        }
        if n > max {
            max = n;
        }
    }
    (min, max)
}

fn main() {
    saludar();
    println!("sumar(2, 3) = {}", sumar(2, 3));
    println!("cuadrado(7) = {}", cuadrado(7));
    println!("dividir(1, 0) = {}", dividir(1.0, 0.0));

    let (min, max) = min_max(&[4, 9, -2, 7]);
    println!("min = {min}, max = {max}");

    // Un bloque `{}` es una expresión: su último valor (sin `;`) es el resultado
    let y = {
        let x = 3;
        x + 1
    };
    println!("y = {y}");

    // Con `;` el valor se descarta y el bloque vale `()`
    let nada = {
        let _x = 3;
    };
    println!("{nada:?}");
}
