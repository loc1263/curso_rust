// NO compila a propósito. Prueba: rustc referencia_colgante.rs
// error[E0106]: missing lifetime specifier
// Devolver una referencia a algo que se libera al terminar la función.
// Solución: devolver el String por valor (sin &).

fn colgante() -> &String {
    let s = String::from("hola");
    &s
}

fn main() {
    let r = colgante();
    println!("{r}");
}
