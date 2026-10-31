// cargo run --example move_clone
// Move, Clone y Copy.

fn main() {
    // String vive en el heap: asignar MUEVE la propiedad
    let s1 = String::from("hola");
    let s2 = s1; // s1 ya no es válida
    // println!("{s1}"); // error[E0382]: borrow of moved value: `s1`
    println!("{s2}");

    // clone() hace una copia profunda explícita
    let s3 = s2.clone();
    println!("{s2} {s3}");

    // Los tipos simples implementan Copy: se copian, no se mueven
    let x = 5;
    let y = x;
    println!("{x} {y}");

    // Lo mismo con tuplas formadas solo por tipos Copy
    let t = (1, 2.0);
    let u = t;
    println!("{t:?} {u:?}");

    // Vec también se mueve
    let v = vec![1, 2, 3];
    let w = v;
    println!("{w:?}");

    // Se puede volver a usar una variable movida si se le asigna un valor nuevo
    let mut texto = String::from("uno");
    let otro = texto;
    texto = String::from("dos");
    println!("{otro} {texto}");
}
