// cargo run -p m17_smart_pointers --example refcell_weak
// Mutabilidad interior (Cell, RefCell), Rc<RefCell<T>> y Weak para evitar ciclos.

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

// Cell: mutabilidad interior para tipos Copy (sin referencias)
struct Contador {
    visitas: Cell<u32>,
}
impl Contador {
    fn visitar(&self) -> u32 {
        self.visitas.set(self.visitas.get() + 1); // &self, pero modifica
        self.visitas.get()
    }
}

// Rc<RefCell<T>>: varios dueños que pueden mutar (un solo hilo)
#[derive(Debug)]
struct Cuenta {
    saldo: i32,
}

// Árbol con referencia al padre: hijos fuertes (Rc), padre débil (Weak)
#[derive(Debug)]
struct Nodo {
    nombre: String,
    padre: RefCell<Weak<Nodo>>,
    hijos: RefCell<Vec<Rc<Nodo>>>,
}
impl Nodo {
    fn nuevo(nombre: &str) -> Rc<Nodo> {
        Rc::new(Nodo {
            nombre: nombre.into(),
            padre: RefCell::new(Weak::new()),
            hijos: RefCell::new(vec![]),
        })
    }
    fn agregar_hijo(padre: &Rc<Nodo>, hijo: Rc<Nodo>) {
        *hijo.padre.borrow_mut() = Rc::downgrade(padre);
        padre.hijos.borrow_mut().push(hijo);
    }
}

fn main() {
    let c = Contador {
        visitas: Cell::new(0),
    };
    c.visitar();
    c.visitar();
    println!("visitas = {}", c.visitar());

    // --- RefCell: las reglas del borrow checker se comprueban en EJECUCIÓN ---
    let dato = RefCell::new(vec![1, 2, 3]);
    dato.borrow_mut().push(4);
    println!("{:?} (len = {})", dato.borrow(), dato.borrow().len());

    // Varias lecturas a la vez: OK. Lectura + escritura: ¡panic!
    {
        let _r1 = dato.borrow();
        let _r2 = dato.borrow();
        println!(
            "try_borrow_mut con lecturas activas: {:?}",
            dato.try_borrow_mut().is_err()
        );
    }

    // --- Rc<RefCell<T>> ---
    let cuenta = Rc::new(RefCell::new(Cuenta { saldo: 100 }));
    let titular_a = Rc::clone(&cuenta);
    let titular_b = Rc::clone(&cuenta);
    titular_a.borrow_mut().saldo -= 30;
    titular_b.borrow_mut().saldo += 5;
    println!("saldo compartido = {}", cuenta.borrow().saldo);

    // --- Weak: evita ciclos de referencias (fugas de memoria) ---
    let raiz = Nodo::nuevo("raíz");
    let hoja = Nodo::nuevo("hoja");
    Nodo::agregar_hijo(&raiz, hoja.clone());

    println!(
        "padre de hoja = {:?}",
        hoja.padre.borrow().upgrade().map(|p| p.nombre.clone())
    );
    println!(
        "raíz: fuertes = {}, débiles = {}",
        Rc::strong_count(&raiz),
        Rc::weak_count(&raiz)
    );
    println!(
        "hoja: fuertes = {}, débiles = {}",
        Rc::strong_count(&hoja),
        Rc::weak_count(&hoja)
    );

    drop(raiz); // el padre se libera (nadie lo mantiene fuerte)
    println!(
        "tras soltar la raíz, padre de hoja = {:?}",
        hoja.padre.borrow().upgrade().map(|p| p.nombre.clone())
    );

    // Si el hijo tuviera Rc (fuerte) hacia el padre, habría un ciclo y nunca se liberarían.

    // ¡Cuidado! Esto compila pero hace panic en ejecución:
    // let x = RefCell::new(1);
    // let _a = x.borrow_mut();
    // let _b = x.borrow_mut(); // panic: already borrowed
}
