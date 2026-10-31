// cargo run --example hashmap
// HashMap<K, V>: pares clave-valor sin orden garantizado.

use std::collections::HashMap;

fn main() {
    // Crear e insertar
    let mut puntos: HashMap<String, u32> = HashMap::new();
    puntos.insert(String::from("azul"), 10);
    puntos.insert(String::from("rojo"), 50);
    println!("{} elementos", puntos.len());

    // Crear desde un array de pares
    let capitales = HashMap::from([("España", "Madrid"), ("Francia", "París")]);

    // Consultar: get devuelve Option<&V>
    println!("{:?}", puntos.get("azul"));
    println!("{:?}", puntos.get("verde"));
    println!("{}", capitales["España"]); // panic si no existe
    println!("{}", puntos.contains_key("rojo"));

    // Sobrescribir
    puntos.insert(String::from("azul"), 25);

    // entry: insertar solo si no existe
    puntos.entry(String::from("verde")).or_insert(5);
    puntos.entry(String::from("azul")).or_insert(0); // no cambia

    // Actualizar según el valor anterior
    *puntos.entry(String::from("rojo")).or_insert(0) += 1;

    // Quitar
    let quitado = puntos.remove("verde");
    println!("quitado = {quitado:?}");

    // Recorrer (el orden es arbitrario: se ordena para mostrar siempre igual)
    let mut pares: Vec<_> = puntos.iter().collect();
    pares.sort();
    for (clave, valor) in pares {
        println!("{clave}: {valor}");
    }

    // Caso clásico: contar palabras
    let texto = "la casa de la playa y la montaña de mi casa";
    let mut conteo: HashMap<&str, u32> = HashMap::new();
    for palabra in texto.split_whitespace() {
        *conteo.entry(palabra).or_insert(0) += 1;
    }
    let mut ordenado: Vec<_> = conteo.into_iter().collect();
    ordenado.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    println!("{ordenado:?}");

    // Agrupar: clave -> Vec
    let mut por_inicial: HashMap<char, Vec<&str>> = HashMap::new();
    for nombre in ["ana", "alba", "bea", "bruno", "carla"] {
        let inicial = nombre.chars().next().unwrap();
        por_inicial.entry(inicial).or_default().push(nombre);
    }
    let mut claves: Vec<_> = por_inicial.keys().copied().collect();
    claves.sort();
    for k in claves {
        println!("{k}: {:?}", por_inicial[&k]);
    }
}
