// cargo run --example structs
// Definir y usar structs con campos con nombre.

#[derive(Debug, Clone)]
struct Usuario {
    nombre: String,
    email: String,
    activo: bool,
    visitas: u64,
}

// Field init shorthand: si la variable se llama igual que el campo, basta con el nombre
fn crear_usuario(nombre: String, email: String) -> Usuario {
    Usuario {
        nombre,
        email,
        activo: true,
        visitas: 1,
    }
}

fn main() {
    let mut u1 = crear_usuario("Ana".to_string(), "ana@mail.com".to_string());

    // Para modificar campos, TODA la instancia debe ser `mut`
    u1.email = String::from("ana@nuevo.com");
    u1.visitas += 1;
    println!("{} <{}> activo={} visitas={}", u1.nombre, u1.email, u1.activo, u1.visitas);

    // Sintaxis de actualización: reutiliza los campos restantes de otra instancia
    let u2 = Usuario {
        nombre: String::from("Luis"),
        ..u1.clone()
    };
    println!("{u2:?}");

    // {:#?} imprime en formato legible
    println!("{u2:#?}");

    // Desestructuración
    let Usuario { nombre, visitas, .. } = u2;
    println!("{nombre} tiene {visitas} visitas");
}
