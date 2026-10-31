// cargo run -p m21_ecosistema --example serde_json_demo
// `serde` + `serde_json`: serializar y deserializar estructuras Rust.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Usuario {
    id: u32,
    nombre: String,
    // Cambia el nombre del campo en el JSON
    #[serde(rename = "correoElectronico")]
    correo: String,
    // Si falta en el JSON, usa Default
    #[serde(default)]
    activo: bool,
    // No se escribe si es None
    #[serde(skip_serializing_if = "Option::is_none")]
    telefono: Option<String>,
    roles: Vec<Rol>,
    #[serde(default)]
    preferencias: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Rol {
    Admin,
    Editor,
    Lector,
}

// Enums con datos: representación "etiquetada" por un campo
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "tipo", content = "datos")]
enum Evento {
    Login { usuario: String },
    Compra { importe: f64, items: u32 },
    Logout,
}

fn main() -> Result<(), serde_json::Error> {
    let mut prefs = HashMap::new();
    prefs.insert("tema".to_string(), "oscuro".to_string());

    let u = Usuario {
        id: 1,
        nombre: "Ana".into(),
        correo: "ana@ejemplo.com".into(),
        activo: true,
        telefono: None,
        roles: vec![Rol::Admin, Rol::Editor],
        preferencias: prefs,
    };

    // Struct -> JSON
    let json_compacto = serde_json::to_string(&u)?;
    println!("{json_compacto}\n");
    println!("{}\n", serde_json::to_string_pretty(&u)?);

    // JSON -> struct (ida y vuelta)
    let vuelta: Usuario = serde_json::from_str(&json_compacto)?;
    println!("roundtrip igual: {}", vuelta == u);

    // Campos opcionales y con default ausentes
    let minimo = r#"{"id": 2, "nombre": "Luis", "correoElectronico": "l@x.com", "roles": ["lector"]}"#;
    let l: Usuario = serde_json::from_str(minimo)?;
    println!("{l:?}\n");

    // Los errores indican línea y columna
    let malo = r#"{"id": "no-es-numero", "nombre": "X"}"#;
    match serde_json::from_str::<Usuario>(malo) {
        Ok(_) => println!("inesperado"),
        Err(e) => println!("error de deserialización: {e}\n"),
    }

    // Enum etiquetado
    let eventos = vec![
        Evento::Login { usuario: "ana".into() },
        Evento::Compra { importe: 19.95, items: 3 },
        Evento::Logout,
    ];
    let texto = serde_json::to_string(&eventos)?;
    println!("{texto}");
    let eventos: Vec<Evento> = serde_json::from_str(&texto)?;
    println!("{eventos:?}\n");

    // JSON dinámico (sin struct) con `Value` y la macro json!
    let v: Value = json!({
        "nombre": "Rust",
        "version": 1.85,
        "tags": ["rápido", "seguro"],
        "autor": { "equipo": "core" }
    });
    println!("nombre = {}", v["nombre"]);
    println!("primer tag = {}", v["tags"][0]);
    println!("equipo = {:?}", v["autor"]["equipo"].as_str());
    println!("campo inexistente = {}", v["nada"]); // null, no hace panic
    Ok(())
}
