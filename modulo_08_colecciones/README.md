# Módulo 8 — Colecciones

## Objetivos

- Usar `Vec<T>` para listas dinámicas.
- Entender `String` vs `&str` y el manejo de UTF-8.
- Usar `HashMap<K, V>` y la API `entry`.

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m08_colecciones --example vectores` | `push`, `get`, `insert`, `retain`, `dedup`, enums en `Vec` |
| `cargo run -p m08_colecciones --example strings` | `String`, `&str`, `chars`, `split`, `format!`, UTF-8 |
| `cargo run -p m08_colecciones --example hashmap` | `insert`, `get`, `entry`, conteo, agrupación |

## Conceptos para recordar

- `v[i]` hace panic si no existe; `v.get(i)` devuelve `Option`.
- No puedes mantener una referencia a un elemento mientras haces `push` (puede realojar el buffer).
- `String` es un `Vec<u8>` garantizado UTF-8; no se indexa por posición, se itera con `chars()`.
- `+` mueve el `String` de la izquierda; `format!` no mueve nada.
- `HashMap` no garantiza orden. Para orden usa `BTreeMap` o ordena antes de mostrar.
- `entry(k).or_insert(v)` es el patrón idiomático para contar y agrupar.
- Otras colecciones útiles: `VecDeque`, `HashSet`, `BTreeMap`, `BinaryHeap`.

## Ejercicios

1. Dado un `Vec<i32>`, calcula media, mediana y moda (la moda con `HashMap`).
2. Convierte una frase a *pig latin*: la primera consonante va al final seguida de `ay`.
3. Cuenta la frecuencia de cada carácter de un texto e imprime los 3 más comunes.
4. Escribe `fn es_palindromo(s: &str) -> bool` ignorando mayúsculas y espacios.
5. Con un `HashMap<String, Vec<String>>` modela departamentos y empleados, con comandos `Agregar Ana a Ventas` y listado ordenado.
6. Reemplaza el `HashMap` del ejercicio anterior por `BTreeMap` y observa la diferencia al iterar.
