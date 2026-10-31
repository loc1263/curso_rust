# Módulo 11 — Genéricos

## Objetivos

- Escribir funciones y tipos que funcionen con muchos tipos a la vez.
- Restringir los genéricos con *trait bounds* y `where`.
- Entender la monomorfización y su coste.

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m11_genericos --example funciones_genericas` | `<T>`, bounds, `where`, turbofish |
| `cargo run -p m11_genericos --example tipos_genericos` | structs/enums/`impl` genéricos, `impl` condicional, const generics |
| `cargo run -p m11_genericos --example monomorfizacion` | cómo el compilador especializa el código |

## Conceptos para recordar

- `fn f<T: Trait>(x: T)` dice "T debe implementar `Trait`"; sin bounds no puedes hacer casi nada con `T`.
- `where` es azúcar sintáctico para bounds largos.
- Puedes implementar métodos solo para ciertos tipos (`impl Punto<f64>`) o bajo ciertos bounds (`impl<T: Display> ...`).
- Los const generics (`[T; N]`) permiten parametrizar por valores.
- Monomorfización: una copia por tipo concreto. Cero coste en ejecución, más tamaño de binario.

## Ejercicios

1. Escribe `fn contar<T: PartialEq>(items: &[T], buscado: &T) -> usize`.
2. Crea `struct Par<T> { a: T, b: T }` con un método `mayor(&self) -> &T` disponible solo si `T: PartialOrd`.
3. Implementa `Cola<T>` con `encolar`, `desencolar` y `longitud`.
4. Escribe `fn primero_y_ultimo<T: Clone>(v: &[T]) -> Option<(T, T)>`.
5. Crea `struct Matriz<const F: usize, const C: usize>` y un método `transponer` que devuelva `Matriz<C, F>`.
6. Usa `std::mem::size_of` para comparar `Option<u32>`, `Option<&u32>` y `Option<Box<u32>>`.
