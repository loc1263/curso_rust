// cargo run --example vectores
// Vec<T>: lista dinámica en el heap.

#[derive(Debug)]
enum Celda {
    Entero(i32),
    Texto(String),
}

fn main() {
    // Crear
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(1);
    v.push(8);
    let w = vec![3, 3, 1, 2];
    println!("{v:?} {w:?}");

    // Acceder
    println!("v[0] = {}", v[0]); // panic si el índice no existe
    println!("get(1) = {:?}, get(9) = {:?}", v.get(1), v.get(9));
    println!("first = {:?}, last = {:?}", v.first(), v.last());

    // Modificar
    v.insert(1, 100);
    println!("tras insert: {v:?}");
    let quitado = v.remove(0);
    println!("remove(0) = {quitado}, ahora {v:?}");
    println!("pop = {:?}", v.pop());
    v.extend([7, 7, 9]);
    println!("extend: {v:?}");

    // Recorrer
    for x in &v {
        print!("{x} ");
    }
    println!();
    for x in &mut v {
        *x += 1;
    }
    println!("tras sumar 1: {v:?}");

    // Ordenar, eliminar duplicados, filtrar
    let mut n = vec![3, 1, 3, 2, 1];
    n.sort();
    println!("sort: {n:?}");
    n.dedup();
    println!("dedup: {n:?}");
    n.retain(|&x| x != 2);
    println!("retain: {n:?}");
    println!("contains(3) = {}", n.contains(&3));

    // Consultas
    println!("len = {}, suma = {}", w.len(), w.iter().sum::<i32>());
    println!("max = {:?}, min = {:?}", w.iter().max(), w.iter().min());
    println!("vacío = {}", Vec::<i32>::new().is_empty());

    // Un Vec guarda un solo tipo, pero un enum permite mezclar variantes
    let fila = vec![
        Celda::Entero(3),
        Celda::Texto(String::from("azul")),
        Celda::Entero(10),
    ];
    for celda in &fila {
        match celda {
            Celda::Entero(n) => println!("entero {n}"),
            Celda::Texto(t) => println!("texto {t}"),
        }
    }
}
