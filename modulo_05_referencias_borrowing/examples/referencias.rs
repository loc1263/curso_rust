// cargo run --example referencias
// Préstamos inmutables (&) y mutables (&mut).

fn longitud(s: &String) -> usize {
    s.len()
} // `s` es solo un préstamo: no se libera nada aquí

fn agregar(s: &mut String) {
    s.push_str(" mundo");
}

fn main() {
    let mut s = String::from("hola");

    // Préstamo inmutable: el dueño conserva la propiedad
    let l = longitud(&s);
    println!("'{s}' mide {l}");

    // Préstamo mutable
    agregar(&mut s);
    println!("{s}");

    // Muchas referencias inmutables a la vez
    let r1 = &s;
    let r2 = &s;
    println!("{r1} {r2}");

    // O UNA mutable (cuando las inmutables ya no se usan: NLL)
    let r3 = &mut s;
    r3.push('!');
    println!("{r3}");

    // Desreferenciar con *
    let n = 5;
    let rn = &n;
    println!("*rn + 1 = {}", *rn + 1);

    let mut m = 1;
    let rm = &mut m;
    *rm += 1;
    println!("m = {m}");
}
