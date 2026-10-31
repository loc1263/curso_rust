// Tests de integración: ejecutan el binario real `rgrep` sobre archivos temporales.

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

/// Directorio temporal único por test (se limpia al terminar).
struct Temporal(PathBuf);

impl Temporal {
    fn nuevo(nombre: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("rgrep_test_{nombre}_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        Temporal(dir)
    }
    fn archivo(&self, nombre: &str, contenido: &str) -> PathBuf {
        let ruta = self.0.join(nombre);
        fs::create_dir_all(ruta.parent().unwrap()).unwrap();
        fs::write(&ruta, contenido).unwrap();
        ruta
    }
}
impl Drop for Temporal {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn rgrep(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rgrep")).args(args).output().unwrap()
}

fn stdout(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).replace("\r\n", "\n")
}

#[test]
fn busca_en_un_archivo_con_numeros_de_linea() {
    let t = Temporal::nuevo("basico");
    let f = t.archivo("a.txt", "uno\ndos\ntres dos\n");
    let o = rgrep(&["-n", "dos", f.to_str().unwrap()]);
    assert_eq!(o.status.code(), Some(0));
    assert_eq!(stdout(&o), "2:dos\n3:tres dos\n");
}

#[test]
fn sin_coincidencias_devuelve_codigo_1() {
    let t = Temporal::nuevo("sin");
    let f = t.archivo("a.txt", "hola\n");
    let o = rgrep(&["zzz", f.to_str().unwrap()]);
    assert_eq!(o.status.code(), Some(1));
    assert_eq!(stdout(&o), "");
}

#[test]
fn recursivo_con_extension() {
    let t = Temporal::nuevo("rec");
    t.archivo("src/main.rs", "fn main() { todo }\n");
    t.archivo("src/notas.txt", "todo lo demás\n");
    t.archivo("lib.rs", "// todo\n");
    let o = rgrep(&["-r", "-c", "--ext", "rs", "todo", t.0.to_str().unwrap()]);
    let salida = stdout(&o);
    assert_eq!(salida.lines().count(), 2, "solo deben aparecer los .rs: {salida}");
    assert!(salida.lines().all(|l| l.contains(".rs:1")));
}

#[test]
fn directorio_sin_recursivo_se_avisa() {
    let t = Temporal::nuevo("dir");
    t.archivo("a.txt", "x\n");
    let o = rgrep(&["x", t.0.to_str().unwrap()]);
    assert!(String::from_utf8_lossy(&o.stderr).contains("es un directorio"));
}

#[test]
fn invertir_e_ignorar_mayusculas() {
    let t = Temporal::nuevo("inv");
    let f = t.archivo("a.txt", "Rust\nPython\nrust\n");
    let o = rgrep(&["-v", "-i", "RUST", f.to_str().unwrap()]);
    assert_eq!(stdout(&o), "Python\n");
}
