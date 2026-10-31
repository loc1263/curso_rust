# Módulo 27 — Macros

## Objetivos

- Escribir macros declarativas con `macro_rules!`.
- Entender los tres tipos de macros procedurales y crear una de cada uno con `syn` y `quote`.
- Depurar macros con `cargo expand`.

## Estructura

```
modulo_27_macros/
├── src/lib.rs               # macro_rules!: mapa!, maximo!, enum_con_nombres!, calcular!, contar!
├── examples/
│   ├── macro_rules.rs       # uso de las declarativas
│   └── procedurales.rs      # uso de las procedurales
└── macros_derive/           # crate aparte (proc-macro = true)
    └── src/lib.rs           # #[derive(Describir)], #[medir_tiempo], mayusculas!
```

Las macros procedurales **deben vivir en su propio crate** (`proc-macro = true`); por eso hay dos paquetes en el workspace.

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m27_macros --example macro_rules` | patrones, repeticiones `$(...),*`, recursión, *tt muncher*, higiene |
| `cargo run -p m27_macros --example procedurales` | derive, attribute y function-like |
| `cargo test -p m27_macros` | tests de las macros declarativas |

## Fragmentos (`$x:tipo`) de `macro_rules!`

| Fragmento | Casa con | Fragmento | Casa con |
|-----------|----------|-----------|----------|
| `expr` | expresión | `ident` | identificador |
| `ty` | tipo | `pat` | patrón |
| `stmt` | sentencia | `block` | `{ ... }` |
| `item` | función, struct… | `path` | ruta `a::b` |
| `literal` | literal | `tt` | un *token tree* (comodín) |
| `vis` | visibilidad (`pub`) | `meta` | contenido de un atributo |

Repeticiones: `$( ... ),*` (0 o más), `$( ... );+` (1 o más), `$( ... )?` (0 o 1).

## Macros procedurales

| Tipo | Declaración | Uso |
|------|-------------|-----|
| derive | `#[proc_macro_derive(Nombre)]` | `#[derive(Nombre)] struct …` |
| attribute | `#[proc_macro_attribute]` | `#[mi_attr(args)] fn …` |
| function-like | `#[proc_macro]` | `mi_macro!(…)` |

Flujo típico: `parse_macro_input!` (syn) → inspeccionar el AST → `quote! { … }` → `.into()`.
En `quote!`, `#variable` interpola; `#(#items),*` repite. Para errores con buena ubicación usa `syn::Error::new_spanned(…).to_compile_error()`.

## Conceptos para recordar

- Las macros operan sobre **tokens**, antes de que el compilador compruebe tipos.
- **Higiene**: las variables locales creadas dentro de `macro_rules!` no chocan con las del usuario.
- Usa `$crate::` para referirte a items de tu crate desde una macro exportada.
- Una función es mejor que una macro siempre que baste con una función. Usa macros para: número variable de argumentos, generar código repetitivo, DSLs, o leer/crear identificadores.
- `cargo expand` (`cargo install cargo-expand`) muestra el código resultante; indispensable para depurar.
- Macros útiles ya hechas: `serde` (derive), `thiserror`, `clap`, `tokio::main`, `tracing::instrument`.

## Ejercicios

1. Escribe `macro_rules! vec_de_strings` que convierta `["a", "b"]` en `Vec<String>`.
2. Implementa `macro_rules! hacer_pub_struct` que genere un struct con campos `pub` y un `Default`.
3. Crea `macro_rules! ejecutar_si` para un DSL sencillo: `cuando!(x > 3 => println!("grande"))`.
4. Añade a `Describir` soporte para listar campos `pub` únicamente.
5. Crea `#[derive(Constructor)]` que genere `fn nuevo(campo1, campo2, …) -> Self`.
6. Crea un atributo `#[reintentar(3)]` que reintente una función `-> Result<_, _>` hasta 3 veces.
