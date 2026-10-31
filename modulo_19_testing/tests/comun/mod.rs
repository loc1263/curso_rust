// Código auxiliar compartido por los tests de integración.
// Va en `tests/comun/mod.rs` (y no en `tests/comun.rs`) para que Cargo
// NO lo trate como un archivo de tests más.

use m19_testing::Cuenta;

pub fn cuenta_con_saldo(saldo: u64) -> Cuenta {
    Cuenta::nueva(saldo)
}
