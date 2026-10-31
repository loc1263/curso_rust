// cargo run --example tipos
// Tipos escalares: enteros, flotantes, bool y char.

fn main() {
    // Enteros: i8 i16 i32 i64 i128 isize / u8 u16 u32 u64 u128 usize
    let a: i32 = -42;
    let b: u8 = 255;
    let c = 1_000_000u64;
    let hex = 0xff;
    let bin = 0b1010;
    let byte = b'A';
    println!("{a} {b} {c} {hex} {bin} {byte}");
    println!("i32: {}..={}", i32::MIN, i32::MAX);
    println!("u8 max: {}", u8::MAX);

    // Desbordamiento: en debug hace panic; hay métodos explícitos
    let m = u8::MAX;
    println!("checked:    {:?}", m.checked_add(1)); // None
    println!("wrapping:   {}", m.wrapping_add(1)); // 0
    println!("saturating: {}", m.saturating_add(1)); // 255
    println!("overflowing:{:?}", m.overflowing_add(1)); // (0, true)

    // Flotantes: f32 y f64 (por defecto)
    let f = 2.5;
    let g: f32 = 1.0 / 3.0;
    println!("{f} {g}");
    println!("7 / 2 = {}", 7 / 2); // división entera
    println!("7.0 / 2.0 = {}", 7.0 / 2.0);
    println!("-7 % 3 = {}", -7 % 3);
    println!("0.1 + 0.2 = {}", 0.1 + 0.2); // ojo con la precisión

    // bool
    let activo = true;
    let inactivo = !activo;
    println!("{activo} {inactivo}");

    // char: 4 bytes, Unicode
    let letra = 'ñ';
    let emoji = '🦀';
    println!("{letra} {emoji} ({} bytes en UTF-8)", emoji.len_utf8());

    // Conversiones con `as`
    let x = 3.99_f64;
    println!("3.99 as i32 = {}", x as i32); // trunca
    println!("-1i32 as u8 = {}", -1i32 as u8); // 255
    println!("300i32 as u8 = {}", 300i32 as u8); // 44
    println!("'A' as u32 = {}", 'A' as u32);
    println!("97u8 as char = {}", 97u8 as char);

    // Conversión segura con From/TryFrom
    let pequeño: u8 = 200;
    let grande: u32 = u32::from(pequeño);
    println!("{grande}");
    println!("{:?}", u8::try_from(300i32)); // Err

    // parse
    let n: i32 = "42".parse().unwrap();
    println!("{}", n + 1);
    println!("{:?}", "abc".parse::<i32>());
}
