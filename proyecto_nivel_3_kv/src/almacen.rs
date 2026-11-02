//! Almacén en memoria con caducidad (TTL) y persistencia opcional en un registro de solo-añadir (AOF).
//!
//! Decisiones de diseño:
//! - `Mutex` de la std con secciones críticas MUY cortas (nunca se mantiene a través de un `.await`).
//! - El AOF se escribe desde una tarea aparte que recibe líneas por un canal: los clientes no esperan al disco.
//!   El envío al canal se hace con el lock tomado, así el orden del log coincide con el de la memoria.
//! - Las claves caducadas se eliminan de forma perezosa al accederlas y por una purga periódica.
//!   Cada eliminación se registra como `D` para que la reproducción del log sea determinista.
//!
//! Formato del AOF (una operación por línea, tokens con las reglas de `protocolo::dividir`):
//! ```text
//! S <clave> <valor> <caduca_unix_ms|0>
//! D <clave>
//! I <clave>
//! ```

use std::collections::HashMap;
use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use thiserror::Error;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;

use crate::protocolo::{citar, dividir};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ErrorAlmacen {
    #[error("el valor no es un entero")]
    NoEsEntero,
    #[error("el resultado desborda un entero de 64 bits")]
    Desbordamiento,
}

struct Entrada {
    valor: String,
    expira: Option<Instant>,
}

impl Entrada {
    fn caducada(&self, ahora: Instant) -> bool {
        self.expira.is_some_and(|e| e <= ahora)
    }
}

type Mapa = HashMap<String, Entrada>;

struct Interior {
    datos: Mutex<Mapa>,
    aof: Option<UnboundedSender<String>>,
    comandos: AtomicU64,
    conexiones: AtomicUsize,
}

/// Manejador barato de clonar (`Arc` por dentro) que se comparte entre todas las conexiones.
#[derive(Clone)]
pub struct Almacen {
    interior: Arc<Interior>,
}

/// Tarea que escribe el AOF; hay que esperarla tras soltar todos los `Almacen` para no perder datos.
pub type TareaAof = JoinHandle<io::Result<()>>;

/// Mantiene el contador de conexiones activas mientras viva.
pub struct GuardiaConexion(Almacen);

impl Drop for GuardiaConexion {
    fn drop(&mut self) {
        self.0.interior.conexiones.fetch_sub(1, Ordering::Relaxed);
    }
}

fn ahora_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as u64)
}

impl Almacen {
    fn nuevo(datos: Mapa, aof: Option<UnboundedSender<String>>) -> Self {
        Almacen {
            interior: Arc::new(Interior {
                datos: Mutex::new(datos),
                aof,
                comandos: AtomicU64::new(0),
                conexiones: AtomicUsize::new(0),
            }),
        }
    }

    /// Almacén solo en memoria (sin persistencia).
    pub fn en_memoria() -> Self {
        Self::nuevo(HashMap::new(), None)
    }

    /// Abre el almacén. Con `ruta`, reproduce el AOF existente y sigue añadiendo operaciones.
    pub async fn abrir(ruta: Option<&Path>) -> io::Result<(Self, Option<TareaAof>)> {
        let Some(ruta) = ruta else {
            return Ok((Self::en_memoria(), None));
        };
        let datos = match tokio::fs::read_to_string(ruta).await {
            Ok(texto) => reproducir(&texto),
            Err(e) if e.kind() == io::ErrorKind::NotFound => HashMap::new(),
            Err(e) => return Err(e),
        };
        let archivo = OpenOptions::new()
            .create(true)
            .append(true)
            .open(ruta)
            .await?;
        let (tx, rx) = mpsc::unbounded_channel();
        let tarea = tokio::spawn(escritor_aof(archivo, rx));
        Ok((Self::nuevo(datos, Some(tx)), Some(tarea)))
    }

    fn datos(&self) -> MutexGuard<'_, Mapa> {
        // Si un hilo hizo panic con el lock tomado, los datos siguen siendo utilizables
        self.interior
            .datos
            .lock()
            .unwrap_or_else(|e| e.into_inner())
    }

    fn registrar(&self, linea: String) {
        if let Some(tx) = &self.interior.aof {
            let _ = tx.send(linea); // solo falla si el escritor murió; el error se verá al cerrar
        }
    }

    /// Elimina `clave` si está caducada (y lo anota en el log).
    fn limpiar(&self, datos: &mut Mapa, clave: &str, ahora: Instant) {
        if datos.get(clave).is_some_and(|e| e.caducada(ahora)) {
            datos.remove(clave);
            self.registrar(format!("D {}\n", citar(clave)));
        }
    }

    pub fn set(&self, clave: String, valor: String, ttl: Option<Duration>) {
        let ahora = Instant::now();
        let caduca_ms = ttl.map_or(0, |d| ahora_unix_ms() + d.as_millis() as u64);
        let mut datos = self.datos();
        self.registrar(format!(
            "S {} {} {caduca_ms}\n",
            citar(&clave),
            citar(&valor)
        ));
        datos.insert(
            clave,
            Entrada {
                valor,
                expira: ttl.map(|d| ahora + d),
            },
        );
    }

    pub fn get(&self, clave: &str) -> Option<String> {
        let mut datos = self.datos();
        self.limpiar(&mut datos, clave, Instant::now());
        datos.get(clave).map(|e| e.valor.clone())
    }

    pub fn del(&self, clave: &str) -> bool {
        let mut datos = self.datos();
        self.limpiar(&mut datos, clave, Instant::now());
        let existia = datos.remove(clave).is_some();
        if existia {
            self.registrar(format!("D {}\n", citar(clave)));
        }
        existia
    }

    pub fn incr(&self, clave: &str) -> Result<i64, ErrorAlmacen> {
        let mut datos = self.datos();
        self.limpiar(&mut datos, clave, Instant::now());
        let nuevo = match datos.get_mut(clave) {
            Some(e) => {
                let n: i64 = e.valor.parse().map_err(|_| ErrorAlmacen::NoEsEntero)?;
                let n = n.checked_add(1).ok_or(ErrorAlmacen::Desbordamiento)?;
                e.valor = n.to_string();
                n
            }
            None => {
                datos.insert(
                    clave.to_string(),
                    Entrada {
                        valor: "1".into(),
                        expira: None,
                    },
                );
                1
            }
        };
        self.registrar(format!("I {}\n", citar(clave)));
        Ok(nuevo)
    }

    /// Segundos restantes (redondeando hacia arriba), -1 si no caduca, -2 si no existe.
    pub fn ttl(&self, clave: &str) -> i64 {
        let mut datos = self.datos();
        let ahora = Instant::now();
        self.limpiar(&mut datos, clave, ahora);
        match datos.get(clave) {
            None => -2,
            Some(Entrada { expira: None, .. }) => -1,
            Some(Entrada {
                expira: Some(e), ..
            }) => e
                .saturating_duration_since(ahora)
                .as_millis()
                .div_ceil(1000) as i64,
        }
    }

    /// Claves vivas, ordenadas.
    pub fn keys(&self) -> Vec<String> {
        let ahora = Instant::now();
        let mut claves: Vec<String> = self
            .datos()
            .iter()
            .filter(|(_, e)| !e.caducada(ahora))
            .map(|(k, _)| k.clone())
            .collect();
        claves.sort();
        claves
    }

    /// Elimina las claves caducadas; devuelve cuántas.
    pub fn purgar(&self) -> usize {
        let mut datos = self.datos();
        let ahora = Instant::now();
        let caducadas: Vec<String> = datos
            .iter()
            .filter(|(_, e)| e.caducada(ahora))
            .map(|(k, _)| k.clone())
            .collect();
        for k in &caducadas {
            datos.remove(k);
            self.registrar(format!("D {}\n", citar(k)));
        }
        caducadas.len()
    }

    // ---------- estadísticas ----------

    pub fn contar_comando(&self) {
        self.interior.comandos.fetch_add(1, Ordering::Relaxed);
    }

    pub fn conexion_abierta(&self) -> GuardiaConexion {
        self.interior.conexiones.fetch_add(1, Ordering::Relaxed);
        GuardiaConexion(self.clone())
    }

    pub fn info(&self) -> Vec<String> {
        vec![
            format!("version:{}", env!("CARGO_PKG_VERSION")),
            format!("claves:{}", self.keys().len()),
            format!(
                "conexiones:{}",
                self.interior.conexiones.load(Ordering::Relaxed)
            ),
            format!(
                "comandos:{}",
                self.interior.comandos.load(Ordering::Relaxed)
            ),
            format!(
                "persistencia:{}",
                if self.interior.aof.is_some() {
                    "aof"
                } else {
                    "ninguna"
                }
            ),
        ]
    }
}

/// Reconstruye el estado aplicando el log tal cual (sin mirar la hora), y al final descarta lo caducado.
fn reproducir(texto: &str) -> Mapa {
    let mut datos = Mapa::new();
    let ahora_ms = ahora_unix_ms();
    let ahora = Instant::now();

    for (n, linea) in texto.lines().enumerate() {
        let tokens = match dividir(linea) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("aof: línea {} ilegible ({e}); se ignora", n + 1);
                continue;
            }
        };
        let tokens: Vec<&str> = tokens.iter().map(String::as_str).collect();
        match tokens.as_slice() {
            ["S", clave, valor, caduca] => {
                let Ok(caduca_ms) = caduca.parse::<u64>() else {
                    eprintln!("aof: línea {} con caducidad inválida; se ignora", n + 1);
                    continue;
                };
                let expira = (caduca_ms != 0)
                    .then(|| ahora + Duration::from_millis(caduca_ms.saturating_sub(ahora_ms)));
                datos.insert(
                    (*clave).to_string(),
                    Entrada {
                        valor: (*valor).to_string(),
                        expira,
                    },
                );
            }
            ["D", clave] => {
                datos.remove(*clave);
            }
            ["I", clave] => {
                let e = datos
                    .entry((*clave).to_string())
                    .or_insert_with(|| Entrada {
                        valor: "0".into(),
                        expira: None,
                    });
                let n: i64 = e.valor.parse().unwrap_or(0);
                e.valor = n.saturating_add(1).to_string();
            }
            _ => eprintln!("aof: línea {} desconocida; se ignora", n + 1),
        }
    }
    datos.retain(|_, e| !e.caducada(Instant::now()));
    datos
}

async fn escritor_aof(archivo: File, mut rx: UnboundedReceiver<String>) -> io::Result<()> {
    let mut salida = BufWriter::new(archivo);
    while let Some(linea) = rx.recv().await {
        salida.write_all(linea.as_bytes()).await?;
        // Agrupa todo lo que ya esté pendiente y vuelca de una vez (menos llamadas al SO)
        while let Ok(otra) = rx.try_recv() {
            salida.write_all(otra.as_bytes()).await?;
        }
        salida.flush().await?;
    }
    // Todos los emisores se destruyeron: cierre ordenado
    salida.flush().await?;
    salida.get_mut().sync_all().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get_del() {
        let a = Almacen::en_memoria();
        assert_eq!(a.get("x"), None);
        a.set("x".into(), "1".into(), None);
        assert_eq!(a.get("x").as_deref(), Some("1"));
        assert!(a.del("x"));
        assert!(!a.del("x"));
    }

    #[test]
    fn incr_crea_suma_y_valida() {
        let a = Almacen::en_memoria();
        assert_eq!(a.incr("n"), Ok(1));
        assert_eq!(a.incr("n"), Ok(2));
        a.set("t".into(), "texto".into(), None);
        assert_eq!(a.incr("t"), Err(ErrorAlmacen::NoEsEntero));
        a.set("max".into(), i64::MAX.to_string(), None);
        assert_eq!(a.incr("max"), Err(ErrorAlmacen::Desbordamiento));
    }

    #[test]
    fn ttl_y_caducidad() {
        let a = Almacen::en_memoria();
        assert_eq!(a.ttl("no"), -2);
        a.set("p".into(), "v".into(), None);
        assert_eq!(a.ttl("p"), -1);
        a.set("c".into(), "v".into(), Some(Duration::from_millis(30)));
        assert_eq!(a.ttl("c"), 1); // redondeo hacia arriba
        std::thread::sleep(Duration::from_millis(50));
        assert_eq!(a.get("c"), None);
        assert_eq!(a.ttl("c"), -2);
    }

    #[test]
    fn purgar_elimina_solo_las_caducadas() {
        let a = Almacen::en_memoria();
        a.set("a".into(), "1".into(), Some(Duration::from_millis(10)));
        a.set("b".into(), "2".into(), None);
        std::thread::sleep(Duration::from_millis(30));
        assert_eq!(a.keys(), ["b"]); // keys ya ignora las caducadas
        assert_eq!(a.purgar(), 1);
        assert_eq!(a.purgar(), 0);
    }

    #[test]
    fn reproducir_aplica_el_log() {
        let log = "S a 1 0\nS b \"dos palabras\" 0\nI a\nI c\nD b\nS z 9 1\nbasura\nS q\n";
        let datos = reproducir(log);
        assert_eq!(datos["a"].valor, "2");
        assert_eq!(datos["c"].valor, "1");
        assert!(!datos.contains_key("b"));
        assert!(!datos.contains_key("z"), "caducó en 1970");
    }
}
