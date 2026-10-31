# Módulo 14 — Lifetimes

## Objetivos

- Entender qué problema resuelven los lifetimes (referencias colgantes).
- Anotar funciones y structs con `'a`.
- Saber cuándo el compilador los deduce solo (elisión) y qué significa `'static`.

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m14_lifetimes --example lifetimes_funciones` | `fn f<'a>`, reglas de elisión, `'static` |
| `cargo run -p m14_lifetimes --example lifetimes_structs` | structs con referencias, iteradores que prestan datos, varios lifetimes |

## Conceptos para recordar

- Un lifetime **describe** relaciones entre referencias; no alarga la vida de ningún valor.
- Reglas de elisión: (1) cada entrada tiene su lifetime, (2) si hay una sola entrada se copia a la salida, (3) en métodos, la salida toma el de `&self`.
- Si el compilador pide `'a`, pregúntate: "¿de qué entrada depende lo que devuelvo?".
- Un struct con `&'a T` no puede sobrevivir al dato al que apunta.
- `'static` significa "puede vivir todo el programa": literales de cadena o datos propios (`String` cumple `T: 'static`).
- Si peleas con lifetimes, considera devolver un valor propio (`String`, `clone`) en vez de una referencia.

## Ejercicios

1. Escribe `fn mas_corta<'a>(a: &'a str, b: &'a str) -> &'a str`.
2. Crea `struct Resaltador<'a> { texto: &'a str }` con un método que devuelva la primera línea.
3. Provoca el error "borrowed value does not live long enough" y arréglalo moviendo una declaración.
4. Escribe una función que devuelva `&'static str` según un `enum` (p. ej. nombre del día).
5. Implementa un iterador `Lineas<'a>` sobre `&'a str` sin usar `str::lines`.
6. Explica por qué `fn f() -> &str` no compila y propone dos soluciones.
