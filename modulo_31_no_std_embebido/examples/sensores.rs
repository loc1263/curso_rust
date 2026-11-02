// cargo run -p m31_no_std_embebido --example sensores
// cargo run -p m31_no_std_embebido --example sensores --features alloc
//
// Simulación de un firmware: la LÓGICA es no_std (la biblioteca), y este `main` usa std solo para
// imprimir en el PC. En un microcontrolador, `write_str` iría a un UART y `main` sería `#[entry]`.

use m31_no_std_embebido::{Anillo, TextoFijo, crc8, formatear_lectura, parsear_decimas};
use std::fmt::Write;

// Un "UART" simulado: cualquier tipo con fmt::Write sirve de destino
struct UartSimulado;
impl Write for UartSimulado {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        print!("{s}");
        Ok(())
    }
}

fn main() {
    // Trama de un sensor: "temp:23.5*<crc>"
    let carga = b"temp:23.5";
    let trama_crc = crc8(carga);
    println!(
        "CRC-8 de {:?} = {trama_crc:#04x}",
        core::str::from_utf8(carga).unwrap()
    );

    // Cola de lecturas con capacidad fija (si se llena, descartamos la más antigua: política típica)
    let mut cola: Anillo<i32, 4> = Anillo::new();
    for texto in ["20.1", "20.4", "21.0", "21.5", "22.2", "-1.7"] {
        let decimas = parsear_decimas(texto).expect("lectura válida");
        if let Err(v) = cola.push(decimas) {
            cola.pop(); // descartar la más antigua
            cola.push(v).unwrap();
        }
    }
    println!("en cola ({} de 4):", cola.len());

    // Formateo sin heap: primero a un buffer fijo...
    let mut linea = TextoFijo::<40>::new();
    while let Some(d) = cola.pop() {
        linea.clear();
        formatear_lectura(&mut linea, "temp", d).unwrap();
        println!("  {}", linea.as_str());
    }

    // ...o directo a un destino fmt::Write (UART)
    let mut uart = UartSimulado;
    formatear_lectura(&mut uart, "humedad", 655).unwrap();
    println!();

    #[cfg(feature = "alloc")]
    {
        use m31_no_std_embebido::con_alloc::{parsear_pares, resumen};
        println!(
            "\ncon `alloc`: {}",
            resumen(&parsear_pares("modo=eco; intervalo=500; led=on"))
        );
    }
    #[cfg(not(feature = "alloc"))]
    println!("\n(prueba con --features alloc para ver las colecciones dinámicas)");
}
