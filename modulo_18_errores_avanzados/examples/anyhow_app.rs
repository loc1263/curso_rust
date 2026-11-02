// cargo run -p m18_errores_avanzados --example anyhow_app
// `anyhow`: manejo cómodo de errores en APLICACIONES (binarios).

#![allow(dead_code)]

use anyhow::{Context, Result, anyhow, bail, ensure};
use std::fs;

#[derive(Debug)]
struct Pedido {
    id: u32,
    cantidad: u32,
}

fn leer_pedido(linea: &str) -> Result<Pedido> {
    // `context` añade una explicación a cualquier error
    let (id, cantidad) = linea
        .split_once(',')
        .ok_or_else(|| anyhow!("línea sin coma: '{linea}'"))?;
    let id = id
        .trim()
        .parse::<u32>()
        .with_context(|| format!("id inválido en '{linea}'"))?;
    let cantidad = cantidad
        .trim()
        .parse::<u32>()
        .with_context(|| format!("cantidad inválida en '{linea}'"))?;

    // ensure! = if !cond { bail!(...) }
    ensure!(cantidad > 0, "el pedido {id} tiene cantidad cero");
    if id > 9999 {
        bail!("id {id} fuera de rango");
    }
    Ok(Pedido { id, cantidad })
}

fn cargar(ruta: &str) -> Result<Vec<Pedido>> {
    let texto = fs::read_to_string(ruta).with_context(|| format!("no se pudo abrir '{ruta}'"))?;
    texto.lines().map(leer_pedido).collect()
}

fn main() -> Result<()> {
    for linea in ["7, 3", "x, 3", "7, abc", "sin coma", "5, 0", "12000, 1"] {
        match leer_pedido(linea) {
            Ok(p) => println!("OK     {p:?}"),
            // {:#} muestra la cadena de contextos en una línea; {:?} la muestra detallada
            Err(e) => println!("ERROR  {e:#}"),
        }
    }

    println!("\n-- Cadena completa con {{:?}} --");
    if let Err(e) = cargar("pedidos_inexistentes.csv") {
        println!("{e:?}");
    }

    // `downcast_ref` recupera el error original si hace falta
    let e = cargar("pedidos_inexistentes.csv").unwrap_err();
    if let Some(io) = e.downcast_ref::<std::io::Error>() {
        println!("\nerror original de E/S: {:?}", io.kind());
    }

    // En main, `?` devuelve el error y el proceso termina con código distinto de 0
    let pedidos = cargar("pedidos_inexistentes.csv")?;
    println!("{pedidos:?}");
    Ok(())
}
