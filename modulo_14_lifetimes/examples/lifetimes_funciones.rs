// cargo run -p m14_lifetimes --example lifetimes_funciones
// Anotaciones de lifetime en funciones y reglas de elisión.

// Sin anotación el compilador no sabe de cuál de los dos argumentos
// depende la referencia devuelta. `'a` lo declara: el resultado vive
// como máximo lo que viva el MÁS CORTO de los dos.
fn mas_larga<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// Aquí el resultado solo depende de `a`; `b` puede tener otro lifetime
fn primera<'a, 'b>(a: &'a str, _b: &'b str) -> &'a str {
    a
}

// --- Elisión: el compilador deduce los lifetimes en estos casos ---

// Regla 1: cada referencia de entrada recibe su propio lifetime.
// Regla 2: si hay UNA sola entrada, ese lifetime se asigna a la salida.
fn primera_palabra(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
// equivale a: fn primera_palabra<'a>(s: &'a str) -> &'a str

// Regla 3: en métodos, el lifetime de `&self` se asigna a la salida.
struct Texto {
    contenido: String,
}
impl Texto {
    fn inicio(&self, n: usize) -> &str {
        &self.contenido[..n.min(self.contenido.len())]
    }
}

// `'static`: vive durante todo el programa (p. ej. los literales)
fn mensaje() -> &'static str {
    "soy un literal"
}

fn main() {
    let a = String::from("larga larga");
    let resultado;
    {
        let b = String::from("corta");
        resultado = mas_larga(a.as_str(), b.as_str());
        println!("dentro: {resultado}"); // b sigue vivo aquí
    }
    // println!("{resultado}"); // ERROR: `b` ya no existe y `resultado` podría apuntar a él

    println!("{}", primera("uno", "dos"));
    println!("{}", primera_palabra("hola mundo cruel"));

    let t = Texto {
        contenido: "Lifetimes en Rust".into(),
    };
    println!("{}", t.inicio(9));

    println!("{}", mensaje());

    // Un lifetime no cambia cuánto vive un valor: solo DESCRIBE relaciones
    // para que el compilador pueda comprobar que no hay referencias colgantes.
}
