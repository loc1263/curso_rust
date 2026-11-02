# Módulo 11 — Genéricos

## La idea en 2 minutos

Imagina una caja con etiqueta "contiene: ___". Puedes usar la misma caja para tornillos o para libros sin fabricar una caja distinta por cada cosa. Los **genéricos** son eso: escribes el código una vez y el compilador lo reutiliza con muchos tipos.

```rust
fn mayor<T: PartialOrd>(a: T, b: T) -> T { if a >= b { a } else { b } }

mayor(3, 7);       // T = i32
mayor("a", "b");   // T = &str
```

`T: PartialOrd` es una **promesa**: "solo acepto tipos que se puedan comparar". Sin ella el compilador no te dejaría usar `>=`, porque no sabe qué es `T`.
Lo sorprendente: no hay coste en ejecución. Rust genera una copia especializada por cada tipo (*monomorfización*), igual que si la hubieras escrito a mano.

**Orden recomendado:** `funciones_genericas` → `tipos_genericos` → `monomorfizacion`.

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
