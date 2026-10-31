// Tests de integración: cada archivo de `tests/` es un crate aparte
// que usa la biblioteca SOLO a través de su API pública (como lo haría un usuario).

use m19_testing::{Cuenta, dividir, sumar, validar_edad};

mod comun;

#[test]
fn flujo_completo_de_cuenta() {
    let mut cuenta = comun::cuenta_con_saldo(1_000);
    cuenta.depositar(500);
    cuenta.retirar(200);
    assert_eq!(cuenta.saldo(), 1_300);
}

#[test]
fn las_funciones_trabajan_juntas() {
    let total = sumar(dividir(100, 4).unwrap(), 5);
    assert_eq!(total, 30);
    assert!(validar_edad(total).is_ok());
}

#[test]
fn cuenta_por_defecto_esta_vacia() {
    assert_eq!(Cuenta::default().saldo(), 0);
}
