// cargo run -p m16_iteradores --example adaptadores
// Adaptadores (perezosos) y consumidores de iteradores.

use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Debug)]
struct Empleado {
    nombre: &'static str,
    depto: &'static str,
    sueldo: u32,
}

fn main() {
    let numeros = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // iter() -> &T, iter_mut() -> &mut T, into_iter() -> T
    let cuadrados_pares: Vec<i32> = numeros.iter().filter(|&&n| n % 2 == 0).map(|n| n * n).collect();
    println!("cuadrados de pares: {cuadrados_pares:?}");

    // Consumidores: sum, product, count, min, max, fold
    println!("suma = {}", numeros.iter().sum::<i32>());
    println!("producto(1..=5) = {}", (1..=5).product::<u64>());
    println!("max = {:?}, min = {:?}", numeros.iter().max(), numeros.iter().min());
    println!("fold concatenado = {}", numeros.iter().take(4).fold(String::new(), |acc, n| format!("{acc}{n}-")));

    // Búsqueda
    println!("primero > 6: {:?}", numeros.iter().find(|&&n| n > 6));
    println!("posición del 5: {:?}", numeros.iter().position(|&n| n == 5));
    println!("any > 9: {}, all > 0: {}", numeros.iter().any(|&n| n > 9), numeros.iter().all(|&n| n > 0));

    // Recorte
    let v: Vec<_> = numeros.iter().skip(2).step_by(3).collect();
    println!("skip(2).step_by(3): {v:?}");
    let v: Vec<_> = numeros.iter().take_while(|&&n| n < 4).collect();
    println!("take_while < 4: {v:?}");
    let v: Vec<_> = numeros.iter().skip_while(|&&n| n < 8).collect();
    println!("skip_while < 8: {v:?}");

    // Combinación
    for (i, letra) in "abc".chars().enumerate() {
        print!("{i}:{letra} ");
    }
    println!();
    let nombres = ["Ana", "Luis", "Eva"];
    let edades = [30, 25, 41];
    let pares: Vec<_> = nombres.iter().zip(edades.iter()).collect();
    println!("zip: {pares:?}");
    let (pares_n, impares_n): (Vec<i32>, Vec<i32>) = numeros.iter().partition(|&&n| n % 2 == 0);
    println!("partition: {pares_n:?} / {impares_n:?}");
    let encadenado: Vec<_> = (1..3).chain(7..9).collect();
    println!("chain: {encadenado:?}");
    let aplanado: Vec<_> = vec![vec![1, 2], vec![3], vec![]].into_iter().flatten().collect();
    println!("flatten: {aplanado:?}");
    let palabras: Vec<_> = ["hola mundo", "rust es genial"].iter().flat_map(|s| s.split(' ')).collect();
    println!("flat_map: {palabras:?}");
    let ventanas: Vec<_> = numeros.windows(3).map(|w| w.iter().sum::<i32>()).take(3).collect();
    println!("windows(3) sumas: {ventanas:?}");
    let trozos: Vec<_> = numeros.chunks(4).collect();
    println!("chunks(4): {trozos:?}");

    // filter_map + parse: descarta lo que falla
    let entradas = ["1", "dos", "3", "x", "5"];
    let validos: Vec<i32> = entradas.iter().filter_map(|s| s.parse().ok()).collect();
    println!("filter_map: {validos:?}");

    // collect en distintas colecciones
    let set: HashSet<_> = [1, 2, 2, 3, 3, 3].into_iter().collect();
    println!("set tiene {} elementos", set.len());
    let mapa: HashMap<_, _> = nombres.iter().cloned().zip(edades).collect();
    println!("edad de Eva = {}", mapa["Eva"]);
    let texto: String = vec!["a", "b", "c"].into_iter().collect();
    println!("String: {texto}");

    // collect a Result<Vec<_>, _>: se detiene en el primer error
    let ok: Result<Vec<i32>, _> = ["1", "2", "3"].iter().map(|s| s.parse::<i32>()).collect();
    let mal: Result<Vec<i32>, _> = ["1", "x", "3"].iter().map(|s| s.parse::<i32>()).collect();
    println!("{ok:?}\n{mal:?}");

    // Ejemplo realista: agrupar y agregar
    let empleados = [
        Empleado { nombre: "Ana", depto: "Ingeniería", sueldo: 3000 },
        Empleado { nombre: "Luis", depto: "Ventas", sueldo: 2000 },
        Empleado { nombre: "Eva", depto: "Ingeniería", sueldo: 3500 },
        Empleado { nombre: "Raúl", depto: "Ventas", sueldo: 2200 },
    ];
    let mut por_depto: BTreeMap<&str, Vec<&Empleado>> = BTreeMap::new();
    for e in &empleados {
        por_depto.entry(e.depto).or_default().push(e);
    }
    for (depto, lista) in &por_depto {
        let total: u32 = lista.iter().map(|e| e.sueldo).sum();
        let nombres: Vec<_> = lista.iter().map(|e| e.nombre).collect();
        println!("{depto}: {nombres:?} media = {}", total / lista.len() as u32);
    }
    let mejor = empleados.iter().max_by_key(|e| e.sueldo).unwrap();
    println!("mejor pagado: {}", mejor.nombre);
}
