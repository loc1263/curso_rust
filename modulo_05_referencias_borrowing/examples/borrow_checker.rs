// cargo run --example borrow_checker
// Patrones válidos que el borrow checker acepta (NLL: Non-Lexical Lifetimes).

fn primer_elemento(v: &Vec<i32>) -> &i32 {
    &v[0] // la referencia devuelta vive tanto como la entrada (elisión de lifetimes)
}

fn duplicar_todos(v: &mut [i32]) {
    for x in v.iter_mut() {
        *x *= 2;
    }
}

fn main() {
    let mut v = vec![1, 2, 3];

    // Copiar el valor en vez de mantener una referencia
    let primero = v[0];
    v.push(4);
    println!("primero = {primero}, v = {v:?}");

    // Una referencia vive hasta su ÚLTIMO uso, no hasta el final del bloque
    let r = &v;
    println!("{r:?}"); // último uso de r
    v.push(5); // ahora sí se puede mutar
    println!("{v:?}");

    // Un bloque limita la vida del préstamo mutable
    {
        let m = &mut v;
        m.push(6);
    }
    println!("{v:?}");

    // Función que devuelve una referencia
    println!("primer_elemento = {}", primer_elemento(&v));

    // Mutar a través de &mut [T]
    duplicar_todos(&mut v);
    println!("{v:?}");

    // iter_mut y enumerate
    for (i, x) in v.iter_mut().enumerate() {
        *x += i as i32;
    }
    println!("{v:?}");
}
