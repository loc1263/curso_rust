// cargo run --example tuplas_arrays
// Tipos compuestos: tuplas y arrays.

fn main() {
    // Tuplas: tamaño fijo, tipos mezclados
    let t: (i32, f64, &str) = (500, 6.4, "hola");
    let (x, y, z) = t; // desestructuración
    println!("{x} {y} {z}");
    println!("t.0 = {}, t.2 = {}", t.0, t.2);

    let unidad = (); // la tupla vacía es el tipo "unit"
    println!("{unidad:?}");

    // Arrays: tamaño fijo, un solo tipo, viven en el stack
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 3] = [10, 20, 30];
    let ceros = [0; 5]; // [0, 0, 0, 0, 0]
    println!("{a:?} {b:?} {ceros:?}");
    println!("len = {}, primero = {}, último = {}", a.len(), a[0], a[a.len() - 1]);

    // Acceso seguro: get devuelve Option
    println!("{:?} {:?}", a.get(2), a.get(10));
    // a[10] -> panic en tiempo de ejecución: index out of bounds

    // Recorrer
    for n in a.iter() {
        print!("{n} ");
    }
    println!();

    // Arrays multidimensionales
    let matriz = [[1, 2, 3], [4, 5, 6]];
    println!("matriz[1][2] = {}", matriz[1][2]);

    // Comparación
    println!("{}", [1, 2, 3] == [1, 2, 3]);
}
