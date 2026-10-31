// cargo run -p m17_smart_pointers --example box_rc_arc
// Box (propiedad única en el heap), Rc (propiedad compartida) y Arc (Rc para hilos).

use std::rc::Rc;
use std::sync::Arc;
use std::thread;

// Box permite tipos recursivos: sin él, el tamaño sería infinito
#[derive(Debug)]
enum Lista {
    Cons(i32, Box<Lista>),
    Nil,
}
use Lista::{Cons, Nil};

impl Lista {
    fn suma(&self) -> i32 {
        match self {
            Cons(v, resto) => v + resto.suma(),
            Nil => 0,
        }
    }
}

// Árbol binario con Box
#[derive(Debug)]
struct Nodo {
    valor: i32,
    izq: Option<Box<Nodo>>,
    der: Option<Box<Nodo>>,
}
impl Nodo {
    fn nuevo(valor: i32) -> Self {
        Nodo { valor, izq: None, der: None }
    }
    fn insertar(&mut self, v: i32) {
        let rama = if v < self.valor { &mut self.izq } else { &mut self.der };
        match rama {
            Some(n) => n.insertar(v),
            None => *rama = Some(Box::new(Nodo::nuevo(v))),
        }
    }
    fn en_orden(&self, salida: &mut Vec<i32>) {
        if let Some(i) = &self.izq {
            i.en_orden(salida);
        }
        salida.push(self.valor);
        if let Some(d) = &self.der {
            d.en_orden(salida);
        }
    }
}

trait Saluda {
    fn saluda(&self) -> String;
}
struct Es;
struct En;
impl Saluda for Es {
    fn saluda(&self) -> String {
        "hola".into()
    }
}
impl Saluda for En {
    fn saluda(&self) -> String {
        "hello".into()
    }
}

fn main() {
    // --- Box ---
    let lista = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{lista:?} suma = {}", lista.suma());

    let mut raiz = Nodo::nuevo(50);
    for v in [30, 70, 20, 40, 60, 80] {
        raiz.insertar(v);
    }
    let mut orden = Vec::new();
    raiz.en_orden(&mut orden);
    println!("árbol en orden: {orden:?}");

    // Box<dyn Trait>: trait objects en el heap
    let saludos: Vec<Box<dyn Saluda>> = vec![Box::new(Es), Box::new(En)];
    println!("{:?}", saludos.iter().map(|s| s.saluda()).collect::<Vec<_>>());

    // Mover un valor grande al heap
    let grande = Box::new([0u8; 1024]);
    println!("Box de {} bytes en el heap", grande.len());

    // --- Rc: varios dueños (un solo hilo) ---
    let compartido = Rc::new(String::from("datos compartidos"));
    println!("referencias: {}", Rc::strong_count(&compartido));
    let a = Rc::clone(&compartido); // clona el puntero, NO los datos
    {
        let b = compartido.clone();
        println!("referencias: {} ({a} / {b})", Rc::strong_count(&compartido));
    }
    println!("referencias tras el bloque: {}", Rc::strong_count(&compartido));

    // Grafo compartido: dos listas comparten la cola
    let cola = Rc::new(vec![3, 4]);
    let l1 = (vec![1], Rc::clone(&cola));
    let l2 = (vec![2], Rc::clone(&cola));
    println!("{:?} y {:?} comparten {} dueños", l1.0, l2.0, Rc::strong_count(&cola));

    // --- Arc: como Rc pero con contador atómico (Send + Sync) ---
    let numeros = Arc::new(vec![1, 2, 3, 4]);
    let mut hilos = Vec::new();
    for id in 0..3 {
        let n = Arc::clone(&numeros);
        hilos.push(thread::spawn(move || {
            let suma: i32 = n.iter().sum();
            println!("hilo {id}: suma = {suma}");
            suma
        }));
    }
    let total: i32 = hilos.into_iter().map(|h| h.join().unwrap()).sum();
    println!("total = {total}, referencias finales = {}", Arc::strong_count(&numeros));

    // Rc NO es Send: `thread::spawn(move || rc)` no compilaría.
    // Rc/Arc dan acceso de SOLO LECTURA; para mutar hace falta RefCell/Mutex (siguiente ejemplo).
}
