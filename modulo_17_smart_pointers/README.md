# Módulo 17 — Smart pointers

## Objetivos

- Elegir el puntero adecuado: `Box`, `Rc`, `Arc`, `Cell`, `RefCell`, `Weak`.
- Entender `Deref` y `Drop`.
- Evitar ciclos de referencias.

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m17_smart_pointers --example box_rc_arc` | `Box` recursivo, árbol, `Rc`, `Arc` |
| `cargo run -p m17_smart_pointers --example refcell_weak` | `Cell`, `RefCell`, `Rc<RefCell<T>>`, `Weak` |
| `cargo run -p m17_smart_pointers --example mutex_y_deref` | smart pointer propio, `Arc<Mutex<T>>` |

## Cuándo usar cada uno

| Tipo | Propietarios | Mutación | Hilos | Uso típico |
|------|--------------|----------|-------|------------|
| `Box<T>` | 1 | sí (si eres dueño) | sí | tipos recursivos, `dyn Trait`, datos grandes |
| `Rc<T>` | varios | no | **no** | grafos / datos compartidos en un hilo |
| `Arc<T>` | varios | no | sí | datos compartidos entre hilos |
| `Cell<T>` | 1 | sí (`Copy`) | no | contadores simples |
| `RefCell<T>` | 1 | sí (en ejecución) | no | mutabilidad interior |
| `Mutex<T>` | 1 | sí (con bloqueo) | sí | `Arc<Mutex<T>>` entre hilos |
| `Weak<T>` | — | — | — | referencias no propietarias (padre, caché) |

## Conceptos para recordar

- `Rc::clone` copia el puntero, no los datos; el valor se libera cuando el contador llega a 0.
- `RefCell` mueve las reglas del borrow checker a ejecución: violarlas provoca `panic`.
- Dos `Rc` que se apuntan mutuamente nunca se liberan (fuga). Usa `Weak` en una de las direcciones.
- `Deref` habilita la *deref coercion*; `Drop` ejecuta código al liberar.
- `Arc<Mutex<T>>` es el patrón estándar para estado mutable compartido entre hilos.

## Ejercicios

1. Implementa una lista enlazada simple con `Option<Box<Nodo>>` (`push`, `pop`, `len`).
2. Modela un grafo con `Rc<RefCell<Nodo>>` y recórrelo.
3. Haz un árbol con enlace al padre usando `Weak` y calcula la profundidad de un nodo.
4. Implementa un patrón *Observer* con `Rc<RefCell<Vec<Box<dyn Fn(&str)>>>>`.
5. Provoca un `panic` de `RefCell` (doble `borrow_mut`) y arréglalo.
6. Cambia `Rc` por `Arc` en un programa con hilos y comprueba el error de `Send` al usar `Rc`.
