# Módulo 3 — Funciones y control de flujo

## Objetivos

- Definir funciones con parámetros y valores de retorno.
- Distinguir **expresiones** de **sentencias**.
- Controlar el flujo con `if`, `loop`, `while` y `for`.

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m03_funciones_control_flujo --example funciones` | parámetros, retorno, tuplas, bloques como expresión |
| `cargo run -p m03_funciones_control_flujo --example condicionales` | `if`/`else if`, `if` como expresión, operadores lógicos |
| `cargo run -p m03_funciones_control_flujo --example bucles` | `loop`, `while`, `for`, rangos, etiquetas, `continue` |

## Conceptos para recordar

- Los parámetros siempre llevan tipo; el retorno se indica con `->`.
- La última expresión **sin `;`** es el valor de retorno.
- `if` es una expresión; sus ramas deben devolver el mismo tipo.
- La condición debe ser `bool` de forma estricta.
- `loop` puede devolver un valor con `break valor`.
- `for` sobre un rango o iterador es la forma preferida de iterar (sin índices manuales).

## Ejercicios

1. Escribe `fn es_primo(n: u32) -> bool`.
2. Implementa FizzBuzz del 1 al 100.
3. Escribe `fn factorial(n: u64) -> u64` primero con `for` y después de forma recursiva.
4. Adivina el número: fija un secreto y usa `loop` con una lista de intentos hasta acertar.
5. Imprime una tabla de multiplicar del 1 al 10 con bucles anidados.
6. Escribe `fn collatz(n: u64) -> u32` que devuelva cuántos pasos hacen falta para llegar a 1.
