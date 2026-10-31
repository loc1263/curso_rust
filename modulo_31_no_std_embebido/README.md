# Módulo 31 — `no_std`, embebido y WebAssembly

## Objetivos

- Entender la jerarquía `core` → `alloc` → `std`.
- Escribir código `#![no_std]` reutilizable en microcontroladores, kernels y Wasm.
- Conocer el ecosistema embebido (`embassy`, `embedded-hal`) y WebAssembly (`wasm-bindgen`).

## Contenido

| Archivo | Tema |
|---------|------|
| `src/lib.rs` | biblioteca `#![no_std]`: cola circular `Anillo<T, N>`, `TextoFijo<N>` (`fmt::Write` sin heap), CRC-8, parseo sin `f32` y módulo opcional con `alloc` |
| `examples/sensores.rs` | `cargo run -p m31_no_std_embebido --example sensores [--features alloc]` |
| tests | `cargo test -p m31_no_std_embebido --all-features` |

## Las tres capas de la biblioteca estándar

| Capa | Qué ofrece | Requiere |
|------|-----------|----------|
| `core` | tipos primitivos, `Option`/`Result`, iteradores, `fmt`, atómicos, `slice`, `str`, `mem`, `ptr`… | nada |
| `alloc` | `Box`, `Vec`, `String`, `Rc`, `Arc`, `BTreeMap`, `format!` | un `#[global_allocator]` |
| `std` | hilos, archivos, red, `HashMap`, `println!`, `Instant`, entorno… | un sistema operativo |

`std` reexporta `core` y `alloc`, así que código escrito con `core` funciona en todas partes.

## Cómo se escribe `no_std`

```rust
#![no_std]                        // no enlazar std
#[cfg(feature = "alloc")]
extern crate alloc;               // opcional: Vec, String, Box...
use core::fmt::Write;             // usar core:: en lugar de std::
```

En un **binario** `no_std` además necesitas:

```rust
#![no_std]
#![no_main]

use panic_halt as _;              // un panic handler (halt, reset, imprimir por UART...)

#[cortex_m_rt::entry]             // el punto de entrada lo define el runtime de la plataforma
fn main() -> ! { loop {} }
```

Compilar para un microcontrolador: `rustup target add thumbv7em-none-eabihf` y
`cargo build --target thumbv7em-none-eabihf --release`.

## Buenas prácticas en `no_std`

- Estructuras de **capacidad fija** (`[T; N]`, const generics, crate `heapless`) en lugar de `Vec` cuando no hay heap.
- Evita `f32/f64` si el chip no tiene FPU (son lentos); usa enteros o punto fijo (`fixed`).
- `fmt::Write` / `write!` permite formatear sin heap; `core::fmt::Arguments` para pasar formato sin asignar.
- Deja la **lógica de dominio** en una biblioteca `no_std` testeable en el PC y aísla el hardware tras traits (`embedded-hal`).
- Los tests de la biblioteca corren en el PC con `std` aunque el crate sea `no_std`.
- Tamaño: `opt-level = "z"`, `lto = true`, `panic = "abort"`, `codegen-units = 1`.

## Ecosistema embebido

| Crate | Para qué |
|-------|----------|
| `embedded-hal` | traits comunes (GPIO, SPI, I²C, UART): drivers reutilizables entre chips |
| `cortex-m`, `cortex-m-rt` | acceso a Cortex-M y runtime de arranque |
| `embassy` | framework **async** para embebido (ejecutor sin heap, temporizadores, USB, red) |
| `rtic` | concurrencia por interrupciones con garantías en compilación |
| `heapless` | `Vec`, `String`, colas y mapas de capacidad fija |
| `probe-rs` | flashear y depurar (`cargo embed`, `cargo flash`, `defmt` para logs eficientes) |
| Placas | Raspberry Pi Pico (`rp2040-hal`/`embassy-rp`), STM32 (`embassy-stm32`), ESP32 (`esp-hal`), nRF52 |

Ejemplo `embassy` (parpadear un LED en una Raspberry Pi Pico):

```rust
#![no_std]
#![no_main]
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);
    loop {
        led.toggle();
        Timer::after_millis(500).await;   // async sin heap ni SO
    }
}
```

## WebAssembly

Rust es de los mejores lenguajes para Wasm (sin runtime ni GC).

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
```
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n { (a, b) = (b, a + b); }
    a
}

#[wasm_bindgen]
pub fn saludar(nombre: &str) -> String { format!("Hola, {nombre}, desde Wasm") }
```
```bash
wasm-pack build --target web      # genera pkg/ con .wasm + JS + .d.ts
```
```html
<script type="module">
  import init, { fibonacci, saludar } from "./pkg/mi_crate.js";
  await init();
  console.log(fibonacci(50), saludar("mundo"));
</script>
```

- `wasm32-unknown-unknown`: navegador (con `wasm-bindgen`/`web-sys`/`js-sys`).
- `wasm32-wasip1` / `wasip2`: fuera del navegador (WASI), ejecutable con `wasmtime`. `cargo build --target wasm32-wasip1`.
- Frameworks frontend en Rust: `Leptos`, `Yew`, `Dioxus`. Optimiza tamaño con `wasm-opt` (binaryen).

## Ejercicios

1. Añade a `Anillo` un iterador `iter()` que no use heap y pruébalo.
2. Implementa `parsear_decimas` con soporte para signo `+` y 2 decimales (centésimas).
3. Crea `TextoFijo::push_char` y haz que `write!` trunque en lugar de fallar (documenta la diferencia).
4. Intenta usar `HashMap` o `println!` dentro de la biblioteca y lee el error del compilador.
5. Instala el target `thumbv7em-none-eabihf` y comprueba que la biblioteca compila: `cargo build -p m31_no_std_embebido --target thumbv7em-none-eabihf`.
6. Exporta `crc8` a JavaScript con `wasm-bindgen` y llámala desde una página HTML.
