// NO compila a propósito. Prueba: rustc dos_mutables.rs
// error[E0499]: cannot borrow `s` as mutable more than once at a time
// Arréglalo haciendo que el primer préstamo termine antes de crear el segundo.

fn main() {
    let mut s = String::from("hola");
    let r1 = &mut s;
    let r2 = &mut s;
    r1.push('a');
    r2.push('b');
    println!("{s}");
}
