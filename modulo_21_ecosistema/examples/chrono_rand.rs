// cargo run -p m21_ecosistema --example chrono_rand
// `chrono` para fechas y horas, `rand` para números aleatorios.

use chrono::{Datelike, Duration, Local, NaiveDate, TimeZone, Utc, Weekday};
use rand::rngs::StdRng;
use rand::seq::{IndexedRandom, SliceRandom};
use rand::{RngExt, SeedableRng};

fn main() {
    // ---------- chrono ----------
    let ahora = Utc::now();
    println!("UTC ahora   : {}", ahora.format("%Y-%m-%d %H:%M:%S"));
    println!("Hora local  : {}", Local::now().format("%d/%m/%Y %H:%M"));

    // Fechas "ingenuas" (sin zona horaria)
    let nacimiento = NaiveDate::from_ymd_opt(1990, 5, 17).unwrap();
    let hoy = ahora.date_naive();
    let dias = (hoy - nacimiento).num_days();
    println!(
        "{nacimiento} fue un {:?}; han pasado {dias} días",
        nacimiento.weekday()
    );

    // Aritmética con Duration
    let entrega = hoy + Duration::days(30);
    println!(
        "entrega en 30 días: {entrega} ({})",
        entrega.format("%A %d de %B")
    );

    // Con zona horaria fija: formatos RFC 3339 y parseo
    let fecha = Utc.with_ymd_and_hms(2025, 12, 31, 23, 59, 0).unwrap();
    println!("RFC 3339: {}", fecha.to_rfc3339());
    println!(
        "un minuto después: {}",
        (fecha + Duration::minutes(1)).format("%Y-%m-%d %H:%M")
    );
    let parseada = chrono::DateTime::parse_from_rfc3339("2024-02-29T12:30:00+02:00").unwrap();
    println!(
        "parseada: {parseada} (año bisiesto: {})",
        NaiveDate::from_ymd_opt(2024, 2, 29).is_some()
    );

    // Cuántos viernes 13 hay en 2026
    let viernes_13 = (1..=12)
        .filter_map(|m| NaiveDate::from_ymd_opt(2026, m, 13))
        .filter(|d| d.weekday() == Weekday::Fri)
        .count();
    println!("viernes 13 en 2026: {viernes_13}");

    // ---------- rand ----------
    let mut rng = rand::rng(); // generador del hilo actual, sembrado por el SO
    let dado: u32 = rng.random_range(1..=6);
    let probabilidad: f64 = rng.random(); // [0, 1)
    let moneda = rng.random_bool(0.5);
    println!(
        "\ndado = {dado}, p = {probabilidad:.3}, moneda = {}",
        if moneda { "cara" } else { "cruz" }
    );

    // Elegir y mezclar
    let frutas = ["manzana", "pera", "kiwi", "fresa"];
    println!("fruta aleatoria: {:?}", frutas.choose(&mut rng));
    let mut mazo: Vec<u8> = (1..=10).collect();
    mazo.shuffle(&mut rng);
    println!("mazo mezclado: {mazo:?}");

    // Generador con semilla: resultados reproducibles (útil en tests y simulaciones)
    let mut a = StdRng::seed_from_u64(42);
    let mut b = StdRng::seed_from_u64(42);
    let sa: Vec<u32> = (0..5).map(|_| a.random_range(0..100)).collect();
    let sb: Vec<u32> = (0..5).map(|_| b.random_range(0..100)).collect();
    println!(
        "misma semilla -> misma secuencia: {sa:?} == {sb:?} -> {}",
        sa == sb
    );

    // Monte Carlo: estimar π
    let mut rng = StdRng::seed_from_u64(1);
    let n = 1_000_000;
    let dentro = (0..n)
        .filter(|_| {
            let (x, y): (f64, f64) = (rng.random(), rng.random());
            x * x + y * y <= 1.0
        })
        .count();
    println!("π ≈ {:.4} (con {n} puntos)", 4.0 * dentro as f64 / n as f64);
}
