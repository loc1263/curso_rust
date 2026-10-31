// cargo run -p m16_iteradores --example perezosos
// Los iteradores son perezosos: nada se ejecuta hasta que un consumidor los pide.

fn main() {
    // Los adaptadores NO hacen nada por sí solos (el compilador incluso avisa con un warning)
    let it = (1..=5).map(|x| {
        println!("  map({x})");
        x * 2
    });
    println!("iterador creado, aún no se ha ejecutado nada");
    let v: Vec<_> = it.collect(); // aquí se ejecuta todo
    println!("{v:?}\n");

    // Evaluación elemento a elemento (no por etapas): se encadena todo por cada elemento
    println!("filter + map, elemento a elemento:");
    let r: Vec<_> = (1..=4)
        .filter(|x| {
            println!("  filter({x})");
            x % 2 == 0
        })
        .map(|x| {
            println!("  map({x})");
            x * 10
        })
        .collect();
    println!("{r:?}\n");

    // Gracias a la pereza se pueden usar secuencias infinitas
    let primeros: Vec<u64> = (1..).map(|n| n * n).filter(|n| n % 3 == 1).take(5).collect();
    println!("primeros cuadrados ≡ 1 (mod 3): {primeros:?}");

    // Cortocircuito: find / any / take paran en cuanto pueden
    let mut visitados = 0;
    let hallado = (1..).inspect(|_| visitados += 1).find(|n| n * n > 200);
    println!("primer n con n² > 200: {hallado:?} (visitó {visitados})");

    // Generadores incorporados
    let potencias: Vec<u32> = std::iter::successors(Some(1u32), |&x| x.checked_mul(10)).collect();
    println!("successors: {potencias:?}");
    let mut c = 0;
    let desde_fn: Vec<_> = std::iter::from_fn(|| {
        c += 1;
        (c <= 3).then_some(c)
    })
    .collect();
    println!("from_fn: {desde_fn:?}");
    let repetido: String = std::iter::repeat("ab").take(3).collect();
    println!("repeat: {repetido}");

    // peekable: mirar el siguiente sin consumirlo
    let mut it = "12+34".chars().peekable();
    let mut numero = String::new();
    while let Some(&c) = it.peek() {
        if c.is_ascii_digit() {
            numero.push(c);
            it.next();
        } else {
            break;
        }
    }
    println!("primer número: {numero}, resto: {:?}", it.collect::<String>());

    // rev, last, nth, cycle
    println!("{:?}", (1..=5).rev().collect::<Vec<_>>());
    println!("{:?}", "hola".chars().nth(2));
    println!("{:?}", [1, 2, 3].iter().cycle().take(7).collect::<Vec<_>>());
}
