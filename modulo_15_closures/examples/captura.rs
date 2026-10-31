// cargo run -p m15_closures --example captura
// Cómo capturan los closures su entorno: Fn, FnMut, FnOnce y `move`.

fn main() {
    // Sintaxis: |parámetros| expresión
    let suma = |a: i32, b: i32| a + b;
    let doble = |x| x * 2; // los tipos se infieren de su primer uso
    println!("{} {}", suma(2, 3), doble(21));

    // --- Fn: solo LEE el entorno (captura por referencia &) ---
    let saludo = String::from("hola");
    let imprimir = || println!("{saludo}");
    imprimir();
    imprimir();
    println!("saludo sigue siendo mío: {saludo}");

    // --- FnMut: MODIFICA el entorno (captura por &mut) ---
    let mut contador = 0;
    let mut incrementar = || {
        contador += 1;
        contador
    };
    incrementar();
    incrementar();
    println!("contador = {}", incrementar());
    // println!("{contador}"); // no se puede mientras `incrementar` siga en uso

    // --- FnOnce: CONSUME algo del entorno (captura por valor) ---
    let datos = vec![1, 2, 3];
    let consumir = move || {
        let suma: i32 = datos.iter().sum();
        drop(datos); // mover `datos` fuera: solo se puede ejecutar una vez
        suma
    };
    println!("suma consumida = {}", consumir());
    // consumir(); // ERROR: `consumir` ya fue movido al llamarlo

    // --- `move`: fuerza la captura por valor (imprescindible con hilos) ---
    let nombre = String::from("Ana");
    let saluda = move || format!("Hola, {nombre}");
    println!("{}", saluda());
    // println!("{nombre}"); // ERROR: `nombre` se movió dentro del closure

    // Los tipos Copy se copian: el original sigue disponible
    let n = 10;
    let suma_n = move |x| x + n;
    println!("{} (n sigue = {n})", suma_n(5));

    // Jerarquía: Fn ⊂ FnMut ⊂ FnOnce.
    // Un closure Fn se puede usar donde se pida FnMut o FnOnce.
}
