# Módulo 26 — FFI (interoperar con C y otros lenguajes)

## Objetivos

- Llamar a C desde Rust (`extern "C"`) y exponer Rust a C.
- Gestionar cadenas, structs, punteros opacos y callbacks de forma segura.
- Conocer las herramientas para generar bindings (`bindgen`, `cbindgen`) y exponer Rust a Python/Node.

## Contenido

| Archivo | Tema |
|---------|------|
| `src/lib.rs` | API C exportada: `#[unsafe(no_mangle)] extern "C"`, `repr(C)`, handle opaco, callbacks, `catch_unwind` (compila como `cdylib` + `rlib`) |
| `examples/c_desde_rust.rs` | `cargo run -p m26_ffi --example c_desde_rust` — llamar a `strlen`, `atoi`, `qsort` con callback |
| `examples/api_propia.rs` | `cargo run -p m26_ffi --example api_propia` — usar nuestra API como lo haría C |
| `include/m26_ffi.h` | header C de la API |
| `c/demo.c` | programa C real que enlaza con la biblioteca (instrucciones de compilación dentro) |

`cargo build -p m26_ffi` genera `target/debug/m26_ffi.dll` (Windows), `libm26_ffi.so` (Linux) o `libm26_ffi.dylib` (macOS).

## Reglas de la frontera FFI

| Regla | Por qué |
|-------|---------|
| `extern "C"` + `#[unsafe(no_mangle)]` | ABI estable y nombre de símbolo sin decorar |
| `#[repr(C)]` en structs/enums compartidos | el layout de Rust no está garantizado |
| `CString` / `CStr` para cadenas | C usa `char*` terminado en NUL; Rust usa `&str` UTF-8 sin NUL |
| Quien reserva, libera | los asignadores pueden ser distintos: ofrece `xxx_liberar()` |
| Handles opacos (`Box::into_raw` / `Box::from_raw`) | C no necesita conocer el contenido |
| Nunca dejes escapar un `panic` hacia C | es UB / aborto; usa `catch_unwind` y códigos de error |
| Valida punteros nulos y documenta `# Safety` | C no te da garantías |
| No uses tipos Rust (`String`, `Vec`, `Option<Box<T>>` con datos…) en la firma | no tienen ABI C |

## Generación automática de bindings

**`bindgen`** (C → Rust): lee headers `.h` y genera declaraciones `extern "C"`.

```toml
# build.rs
[build-dependencies]
bindgen = "0.71"
```
```rust
// build.rs
fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate().unwrap();
    bindings.write_to_file(std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs")).unwrap();
}
// lib.rs: include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
```

**`cbindgen`** (Rust → C): genera el `.h` desde tu API `extern "C"`.

```bash
cargo install cbindgen
cbindgen --lang c --output include/m26_ffi.h
```

Para compilar código C junto con tu crate usa el crate `cc` en `build.rs`.

## Rust desde Python: PyO3 + maturin

```toml
[lib]
name = "mi_modulo"
crate-type = ["cdylib"]

[dependencies]
pyo3 = { version = "0.25", features = ["extension-module"] }
```
```rust
use pyo3::prelude::*;

#[pyfunction]
fn suma_cuadrados(v: Vec<i64>) -> i64 { v.iter().map(|x| x * x).sum() }

#[pymodule]
fn mi_modulo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(suma_cuadrados, m)?)
}
```
```bash
pip install maturin
maturin develop --release        # compila e instala en el venv
python -c "import mi_modulo; print(mi_modulo.suma_cuadrados([1,2,3]))"
```

## Rust desde Node.js: napi-rs

```rust
use napi_derive::napi;

#[napi]
pub fn fibonacci(n: u32) -> u32 { if n < 2 { n } else { fibonacci(n - 1) + fibonacci(n - 2) } }
```
```bash
npm i -g @napi-rs/cli && napi new mi-addon    # plantilla con CI y paquetes por plataforma
```

Alternativas: `uniffi` (Kotlin/Swift/Python), `wasm-bindgen` (navegador, módulo 31), `cxx` (C++ seguro).

## Ejercicios

1. Compila `c/demo.c` contra la biblioteca y ejecútalo.
2. Añade `m26_ordenar(int *v, size_t n)` que ordene un array recibido de C usando `slice::from_raw_parts_mut`.
3. Expón una función que devuelva un `struct { int codigo; char *mensaje; }` con su función de liberación.
4. Genera el header con `cbindgen` y compáralo con el escrito a mano.
5. Llama desde Rust a `GetTickCount64` (Windows) o `getpid` (Unix) con un bloque `extern`.
6. Crea un módulo Python con PyO3 que exponga `suma_cuadrados` y mide la diferencia de velocidad frente a Python puro.
