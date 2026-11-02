# Módulo 14 — Lifetimes

## La idea en 2 minutos

Una referencia es como un préstamo de un libro: no puedes quedártelo más tiempo del que vive su dueño. Rust lo comprueba en compilación para que **nunca** exista una referencia a algo que ya no existe (adiós a los "punteros colgantes").

Normalmente no escribes nada: el compilador lo deduce. Solo cuando una función devuelve una referencia y hay varias de entrada debes decirle **de cuál depende**:

```rust
// "el resultado vive como máximo lo que vivan a y b"
fn mas_larga<'a>(a: &'a str, b: &'a str) -> &'a str { if a.len() >= b.len() { a } else { b } }
```

Un lifetime (`'a`) **no alarga** la vida de nada: solo **describe** una relación para que el compilador la verifique.
Si te atascas, pregúntate: *¿de qué dato sale la referencia que devuelvo?* Y recuerda que siempre puedes devolver un valor propio (`String`) en lugar de una referencia.

**Orden recomendado:** `lifetimes_funciones` → `lifetimes_structs`.

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
