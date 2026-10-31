# Módulo 12 — Traits

## Objetivos

- Definir comportamiento compartido con traits.
- Elegir entre despacho estático (`impl Trait`) y dinámico (`dyn Trait`).
- Usar supertraits, tipos asociados y *blanket impls*.

## Ejemplos

| Comando | Tema |
|---------|------|
| `cargo run -p m12_traits --example definir_traits` | definición, métodos por defecto, bounds, constantes asociadas |
| `cargo run -p m12_traits --example impl_vs_dyn` | `impl Trait` vs `Box<dyn Trait>` |
| `cargo run -p m12_traits --example traits_avanzados` | supertraits, tipos asociados, blanket impls, operadores |

## Conceptos para recordar

- Un trait es un contrato: un conjunto de métodos que un tipo promete implementar.
- Regla huérfana: solo puedes implementar un trait si el trait **o** el tipo son tuyos.
- `impl Trait` (estático) es rápido; `dyn Trait` (dinámico) permite colecciones heterogéneas.
- Para ser *object safe*, un trait no puede tener métodos genéricos ni devolver `Self` (salvo con `where Self: Sized`).
- Los tipos asociados (`type Item`) evitan parámetros genéricos extra en el trait.
- Una *blanket implementation* (`impl<T: Display> Gritar for T`) implementa el trait para todos los tipos que cumplan el bound.

## Ejercicios

1. Define `trait Volador { fn volar(&self) -> String; }` e impleméntalo para `Pajaro` y `Avion`.
2. Crea una `Vec<Box<dyn Volador>>` y recórrela.
3. Escribe `fn mas_rapido(a: &impl Velocidad, b: &impl Velocidad)`. ¿Por qué puedes pasar tipos distintos aquí pero no con `fn mas_rapido<T: Velocidad>(a: &T, b: &T)`?
4. Implementa `Add` y `Neg` para una estructura `Complejo`.
5. Define `trait Contenedor { type Elem; fn obtener(&self, i: usize) -> Option<&Self::Elem>; }` e impleméntalo para una pila.
6. Intenta hacer `dyn` un trait con un método `fn clonar(&self) -> Self` y lee el error del compilador.
