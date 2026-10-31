# Módulo 10 — Módulos y paquetes

## Objetivos

- Organizar código con `mod`, `pub` y `use`.
- Entender paquetes, crates y la raíz de cada crate.
- Usar dependencias de crates.io.

## Vocabulario

| Término | Qué es |
|---------|--------|
| **Paquete** | Lo que define un `Cargo.toml`; contiene uno o más crates |
| **Crate** | Una unidad de compilación: binario (`main.rs`) o biblioteca (`lib.rs`) |
| **Módulo** | Una unidad de organización y privacidad dentro de un crate |
| **Path** | Ruta a un elemento: `crate::a::b`, `super::x`, `self::y` |

## Estructura del ejemplo

```
modulo_10_modulos_paquetes/
├── Cargo.toml
├── src/
│   ├── lib.rs               # raíz: declara `geometria` y `restaurante`
│   ├── geometria.rs         # módulo en su propio archivo
│   └── geometria/
│       └── circulo.rs       # submódulo: geometria::circulo
└── examples/
    ├── modulos_inline.rs    # mod en línea, use, as, self, super, glob
    └── usar_lib.rs          # usar la biblioteca desde fuera
```

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p modulos_demo --example modulos_inline` | módulos en línea y formas de `use` |
| `cargo run -p modulos_demo --example usar_lib` | usar una biblioteca, visibilidad, re-exports |
| `cargo test -p modulos_demo` | tests dentro de los módulos |
| `cargo doc -p modulos_demo --open` | documentación generada a partir de los `///` y `//!` |

## Conceptos para recordar

- Todo es **privado por defecto**; `pub` lo expone, `pub(crate)` lo limita al crate.
- Los campos de un struct tienen su propia visibilidad; un campo privado obliga a usar un constructor.
- `mod nombre;` busca `nombre.rs` o `nombre/mod.rs`; los submódulos van en `nombre/`.
- `pub use` re-exporta y permite diseñar una API más cómoda que la estructura interna.
- Un paquete puede tener varios binarios (`src/bin/`) y como mucho una biblioteca.

## Dependencias de crates.io

```bash
cargo add rand            # añade la última versión a [dependencies]
cargo add serde --features derive
cargo add --dev pretty_assertions
cargo update              # actualiza Cargo.lock
cargo tree                # muestra el árbol de dependencias
```

Ejemplo de uso tras `cargo add rand`:

```rust
use rand::Rng;

fn main() {
    let n: u8 = rand::rng().random_range(1..=100);
    println!("{n}");
}
```

## Ejercicios

1. Crea un módulo `matematicas` con submódulos `basicas` (suma/resta) y `avanzadas` (potencia/factorial) en archivos separados.
2. Añade un campo privado a un struct y fuerza su creación con `new` validando el valor.
3. Re-exporta con `pub use` las funciones más usadas para llamarlas como `modulos_demo::factorial`.
4. Añade `rand` con `cargo add` y haz que `usar_lib` imprima un número aleatorio.
5. Crea un paquete con una biblioteca y dos binarios en `src/bin/` que la usen.
6. Ejecuta `cargo doc --open` tras añadir comentarios `///` a tus funciones públicas.
