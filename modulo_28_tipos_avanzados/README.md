# Módulo 28 — Sistema de tipos avanzado

## Objetivos

- Usar el sistema de tipos para que los **estados inválidos no se puedan representar**.
- Dominar tipos asociados, GATs, const generics, `PhantomData`, newtype y typestate.
- Entender HRTB, traits sellados y `dyn Any`.

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m28_tipos_avanzados --example asociados_gat_const` | tipos/constantes asociadas, GAT (iterador que presta), matrices con const generics |
| `cargo run -p m28_tipos_avanzados --example phantom_typestate` | newtype, `PhantomData` (unidades), typestate (conexión, builder) |
| `cargo run -p m28_tipos_avanzados --example hrtb_sellado` | `for<'a>`, trait sellado, `dyn Any`, `!`, `Cow`, `?Sized` |

## Patrones y cuándo usarlos

| Patrón | Idea | Ejemplo |
|--------|------|---------|
| **Newtype** | envolver un tipo para darle identidad propia | `struct Id(u64)` ≠ `struct Precio(f64)` |
| **PhantomData** | parámetro de tipo sin dato real | `Longitud<Metros>` |
| **Typestate** | el estado vive en el tipo; las transiciones consumen `self` | `Conexion<Cerrada>` → `Conexion<Abierta>` |
| **Builder tipado** | campos obligatorios verificados al compilar | `PeticionBuilder<ConUrl>::construir()` |
| **Trait sellado** | impedir implementaciones externas | `Formato` |
| **Tipo asociado** | un tipo determinado por la implementación | `Iterator::Item` |
| **GAT** | tipo asociado con lifetime/genéricos propios | iteradores que prestan |
| **const generics** | tamaños como parte del tipo | `Matriz<T, F, C>` |

## Conceptos para recordar

- Tipo asociado vs genérico: si para un tipo hay **una sola** implementación natural, usa asociado (`Iterator`); si puede haber **varias** (`From<A>`, `Add<Rhs>`), usa parámetro.
- Los tipos con `PhantomData` no ocupan espacio: el coste en ejecución es cero.
- Las transiciones por valor (`fn abrir(self) -> Otro`) hacen imposible usar el estado antiguo.
- HRTB (`for<'a>`) aparece al aceptar closures que reciben referencias de cualquier duración.
- `?Sized` relaja el requisito implícito de que `T` tenga tamaño conocido (permite `str`, `[T]`, `dyn Trait`).
- `dyn Any` + `downcast_ref` permite polimorfismo dinámico; úsalo con moderación.
- Variance: `&'a T` es covariante en `'a`; `&mut T` es invariante en `T`; `PhantomData<fn(T)>` cambia la varianza de tu tipo.

## Ejercicios

1. Añade a `Conexion` un estado `Autenticada` entre `Abierta` y `enviar`; `enviar` solo debe existir tras autenticar.
2. Implementa `Mul<f64>` para `Longitud<U>` y `Add` entre `Longitud<Metros>` y `Longitud<Pies>` con conversión.
3. Crea `struct Vector<const N: usize>([f64; N])` con producto escalar solo entre vectores del mismo `N`.
4. Escribe un `LendingIterator` que devuelva líneas de un buffer reutilizando la misma memoria.
5. Diseña un trait sellado `Moneda` con `Euro` y `Dolar` e implementa `convertir::<A, B>()`.
6. Explica por qué `fn f(x: &mut Vec<&'static str>)` no acepta `&mut Vec<&'a str>` (invarianza).
