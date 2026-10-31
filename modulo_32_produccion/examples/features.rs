// cargo run -p m32_produccion --example features
// cargo run -p m32_produccion --example features --features colores
// cargo run -p m32_produccion --example features --no-default-features
//
//! Compilación condicional: el código de una feature desactivada ni siquiera se compila.

use m32_produccion::{InfoBuild, redondear};

fn main() {
    println!("{}\n", InfoBuild::actual());

    println!("redondear(7.45678, 2) = {}", redondear(7.45678, 2));

    #[cfg(feature = "estadisticas")]
    {
        let r = m32_produccion::estadisticas::resumir(&[4.0, 8.0, 15.0, 16.0, 23.0, 42.0]).unwrap();
        println!("estadísticas: {r:?}");
    }
    #[cfg(not(feature = "estadisticas"))]
    println!("(feature `estadisticas` desactivada: no hay módulo `estadisticas`)");

    #[cfg(feature = "colores")]
    println!("{} / {}", m32_produccion::colores::verde("verde"), m32_produccion::colores::rojo("rojo"));
    #[cfg(not(feature = "colores"))]
    println!("(feature `colores` desactivada)");

    // `cfg!` también existe como expresión booleana (ambos lados se compilan)
    println!("\nmodo depuración: {}", cfg!(debug_assertions));
    println!("sistema operativo: {}", std::env::consts::OS);
    println!("windows: {} | unix: {}", cfg!(windows), cfg!(unix));
}
