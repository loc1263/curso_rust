// cargo run -p m16_iteradores --example iterador_propio
// Implementar `Iterator` para tus tipos; obtienes map/filter/... gratis.

// --- Fibonacci infinito ---
struct Fibonacci {
    a: u64,
    b: u64,
}
impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}
impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let actual = self.a;
        (self.a, self.b) = (self.b, self.a.checked_add(self.b)?); // None si desborda
        Some(actual)
    }
}

// --- Cuenta atrás finita, con size_hint y DoubleEndedIterator ---
struct CuentaAtras {
    desde: u32,
    hasta: u32, // exclusivo
}
impl Iterator for CuentaAtras {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.desde > self.hasta {
            self.desde -= 1;
            Some(self.desde + 1)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = (self.desde - self.hasta) as usize;
        (n, Some(n))
    }
}
impl ExactSizeIterator for CuentaAtras {}

// --- Una colección propia con iteración por referencia ---
struct Playlist {
    canciones: Vec<String>,
}
struct IterPlaylist<'a> {
    playlist: &'a Playlist,
    indice: usize,
}
impl<'a> Iterator for IterPlaylist<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        let c = self.playlist.canciones.get(self.indice)?;
        self.indice += 1;
        Some(c.as_str())
    }
}
impl Playlist {
    fn iter(&self) -> IterPlaylist<'_> {
        IterPlaylist {
            playlist: self,
            indice: 0,
        }
    }
}
impl<'a> IntoIterator for &'a Playlist {
    type Item = &'a str;
    type IntoIter = IterPlaylist<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// --- Extender Iterator con un trait propio (extension trait) ---
trait IteradorExt: Iterator + Sized {
    fn cada_n(self, n: usize) -> std::iter::StepBy<Self> {
        self.step_by(n)
    }
    fn contar_si<P: FnMut(&Self::Item) -> bool>(self, mut p: P) -> usize {
        self.fold(0, |acc, x| if p(&x) { acc + 1 } else { acc })
    }
}
impl<I: Iterator> IteradorExt for I {}

fn main() {
    let fibs: Vec<_> = Fibonacci::new().take(10).collect();
    println!("fib: {fibs:?}");

    // Todos los adaptadores funcionan sobre tu iterador
    let suma_pares: u64 = Fibonacci::new()
        .take_while(|&x| x < 4_000_000)
        .filter(|x| x % 2 == 0)
        .sum();
    println!("suma de fib pares < 4M = {suma_pares}");
    println!(
        "Fibonacci se detiene por overflow tras {} términos",
        Fibonacci::new().count()
    );

    let ca = CuentaAtras { desde: 5, hasta: 0 };
    println!("len = {}", ca.len());
    println!("{:?}", ca.collect::<Vec<_>>());

    let pl = Playlist {
        canciones: vec!["Uno".into(), "Dos".into(), "Tres".into()],
    };
    for (i, c) in pl.iter().enumerate() {
        println!("{}. {c}", i + 1);
    }
    for c in &pl {
        print!("[{c}] ");
    }
    println!();

    println!("{:?}", (1..=10).cada_n(3).collect::<Vec<_>>());
    println!("{}", (1..=10).contar_si(|x| x % 3 == 0));
}
