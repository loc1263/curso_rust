// NO compila a propósito. Prueba: rustc mutable_e_inmutable.rs
// error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
// Es el clásico error de guardar una referencia a un elemento y luego hacer push.

fn main() {
    let mut v = vec![1, 2, 3];
    let primero = &v[0];
    v.push(4);
    println!("{primero}");
}
