// cargo run -p m23_pin_futures --example pin_unpin
// ¿Por qué existe `Pin`? Por las estructuras autorreferenciadas.

use std::marker::PhantomPinned;
use std::pin::Pin;

// Una estructura que apunta a SÍ MISMA: `puntero` apunta a `datos`
struct AutoRef {
    datos: String,
    puntero: *const String,
    _fijo: PhantomPinned, // marca el tipo como !Unpin: "no me muevas una vez fijado"
}

impl AutoRef {
    fn nuevo(datos: &str) -> Pin<Box<AutoRef>> {
        let mut caja = Box::pin(AutoRef {
            datos: datos.to_string(),
            puntero: std::ptr::null(),
            _fijo: PhantomPinned,
        });
        // SAFETY: no movemos el valor, solo escribimos en un campo; ya está fijado en el heap
        let ptr: *const String = &caja.datos;
        unsafe { caja.as_mut().get_unchecked_mut().puntero = ptr };
        caja
    }

    fn datos(self: Pin<&Self>) -> &str {
        &self.get_ref().datos
    }

    fn via_puntero(self: Pin<&Self>) -> &str {
        // SAFETY: `puntero` apunta a `datos`, y como estamos fijados no se ha movido
        unsafe { &*self.puntero }
    }
}

fn main() {
    // --- Unpin es el caso normal: casi todos los tipos son Unpin ---
    // Pin<&mut T> con T: Unpin no impide nada (se puede sacar el &mut).
    let mut x = 5;
    let mut pin_x = Pin::new(&mut x);
    *pin_x = 6;
    println!("i32 es Unpin, así que Pin::new funciona: {}", *pin_x);

    // --- Un tipo !Unpin: Pin::new no compila; hay que usar Box::pin / pin! (o unsafe) ---
    let a = AutoRef::nuevo("hola");
    println!("datos       : {}", a.as_ref().datos());
    println!("vía puntero : {}", a.as_ref().via_puntero());
    println!(
        "¿apunta a sí mismo? {}",
        std::ptr::eq(a.as_ref().puntero, &a.datos)
    );

    // Mover el Pin<Box<_>> está permitido: lo que se mueve es el puntero a la caja,
    // los datos siguen en la misma dirección del heap.
    let b = a;
    println!("tras mover la caja: {}", b.as_ref().via_puntero());

    // Lo que NO se puede hacer (no compila): `std::mem::swap(&mut *a1, &mut *a2)`
    // porque requiere `&mut AutoRef` y Pin<Box<AutoRef>> no lo ofrece cuando AutoRef: !Unpin.
    // Intercambiar dos AutoRef copiaría los bytes y `puntero` apuntaría al lugar equivocado.

    // --- Relación con async: un `async fn` genera una máquina de estados que puede guardar
    // referencias a sus propias variables locales a través de un .await. Por eso los
    // futures se "fijan" (Pin) antes de hacer poll.
    async fn ejemplo() -> usize {
        let local = String::from("variable local");
        let referencia = &local; // autorreferencia a través del await
        std::future::ready(()).await;
        referencia.len()
    }
    let fut = ejemplo();
    let fut = Box::pin(fut); // Pin<Box<impl Future>>
    println!("el future ocupa {} bytes", std::mem::size_of_val(&*fut));

    // Resumen:
    //   Pin<P>   = garantía de que el valor apuntado NO se moverá de su sitio.
    //   Unpin    = "da igual si me mueven" (auto trait; la mayoría de tipos).
    //   !Unpin   = tipos que dependen de su dirección (futures async, autorreferencias).
    //   Box::pin / pin!(…) / Pin::new (solo Unpin) = formas de obtener un Pin.
}
