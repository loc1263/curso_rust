// Tests de extremo a extremo: servidor real en un puerto efímero y clientes TCP reales.

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream, tcp::OwnedReadHalf, tcp::OwnedWriteHalf};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

/// Servidor de pruebas: se apaga con `detener()` y devuelve el resultado de `ejecutar`.
struct Servidor {
    direccion: SocketAddr,
    apagar: Option<oneshot::Sender<()>>,
    tarea: JoinHandle<anyhow::Result<()>>,
}

impl Servidor {
    async fn arrancar(aof: Option<&Path>) -> Servidor {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let direccion = listener.local_addr().unwrap();
        let (tx, rx) = oneshot::channel::<()>();
        let aof = aof.map(Path::to_path_buf);
        let tarea = tokio::spawn(async move {
            kvdb::ejecutar(listener, aof.as_deref(), async {
                let _ = rx.await;
            })
            .await
        });
        Servidor {
            direccion,
            apagar: Some(tx),
            tarea,
        }
    }

    async fn cliente(&self) -> Cliente {
        Cliente::conectar(self.direccion).await
    }

    async fn detener(mut self) {
        let _ = self.apagar.take().unwrap().send(());
        tokio::time::timeout(Duration::from_secs(10), self.tarea)
            .await
            .expect("el servidor no se apagó a tiempo")
            .unwrap()
            .expect("error al apagar");
    }
}

struct Cliente {
    lector: BufReader<OwnedReadHalf>,
    escritor: OwnedWriteHalf,
}

impl Cliente {
    async fn conectar(direccion: SocketAddr) -> Cliente {
        let (r, w) = TcpStream::connect(direccion).await.unwrap().into_split();
        Cliente {
            lector: BufReader::new(r),
            escritor: w,
        }
    }

    async fn linea(&mut self) -> String {
        let mut s = String::new();
        self.lector.read_line(&mut s).await.unwrap();
        s.trim_end().to_string()
    }

    /// Envía una orden y devuelve la primera línea de la respuesta.
    async fn cmd(&mut self, orden: &str) -> String {
        self.escritor
            .write_all(format!("{orden}\n").as_bytes())
            .await
            .unwrap();
        self.linea().await
    }

    /// Para respuestas tipo lista: `*n` + n líneas.
    async fn lista(&mut self, orden: &str) -> Vec<String> {
        let cabecera = self.cmd(orden).await;
        let n: usize = cabecera
            .strip_prefix('*')
            .unwrap_or_else(|| panic!("no es una lista: {cabecera}"))
            .parse()
            .unwrap();
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(self.linea().await.trim_start_matches('$').to_string());
        }
        v
    }
}

fn archivo_temporal(nombre: &str) -> PathBuf {
    let ruta = std::env::temp_dir().join(format!("kvdb_{nombre}_{}.aof", std::process::id()));
    let _ = std::fs::remove_file(&ruta);
    ruta
}

#[tokio::test]
async fn ping_set_get_del() {
    let s = Servidor::arrancar(None).await;
    let mut c = s.cliente().await;
    assert_eq!(c.cmd("PING").await, "+PONG");
    assert_eq!(c.cmd("GET a").await, "_");
    assert_eq!(c.cmd("SET a 1").await, "+OK");
    assert_eq!(c.cmd("GET a").await, "$1");
    assert_eq!(c.cmd("DEL a").await, ":1");
    assert_eq!(c.cmd("DEL a").await, ":0");
    assert_eq!(c.cmd("GET a").await, "_");
    s.detener().await;
}

#[tokio::test]
async fn valores_con_espacios_y_comillas() {
    let s = Servidor::arrancar(None).await;
    let mut c = s.cliente().await;
    assert_eq!(c.cmd(r#"SET saludo "hola   mundo""#).await, "+OK");
    assert_eq!(c.cmd("GET saludo").await, "$hola   mundo");
    assert_eq!(c.cmd(r#"SET q "dice \"hola\"""#).await, "+OK");
    assert_eq!(c.cmd("GET q").await, r#"$dice "hola""#);
    s.detener().await;
}

#[tokio::test]
async fn errores_no_cierran_la_conexion() {
    let s = Servidor::arrancar(None).await;
    let mut c = s.cliente().await;
    assert!(c.cmd("FOO").await.starts_with("-ERR comando desconocido"));
    assert!(c.cmd("GET").await.starts_with("-ERR número de argumentos"));
    assert!(
        c.cmd(r#"SET a "sin cerrar"#)
            .await
            .starts_with("-ERR comillas")
    );
    assert_eq!(c.cmd("SET texto abc").await, "+OK");
    assert!(
        c.cmd("INCR texto")
            .await
            .starts_with("-ERR el valor no es un entero")
    );
    assert_eq!(c.cmd("PING").await, "+PONG"); // sigue viva
    s.detener().await;
}

#[tokio::test]
async fn incr_keys_e_info() {
    let s = Servidor::arrancar(None).await;
    let mut c = s.cliente().await;
    assert_eq!(c.cmd("INCR visitas").await, ":1");
    assert_eq!(c.cmd("INCR visitas").await, ":2");
    c.cmd("SET b x").await;
    c.cmd("SET a x").await;
    assert_eq!(c.lista("KEYS").await, ["a", "b", "visitas"]);
    let info = c.lista("INFO").await;
    assert!(info.contains(&"claves:3".to_string()), "{info:?}");
    assert!(info.contains(&"conexiones:1".to_string()), "{info:?}");
    s.detener().await;
}

#[tokio::test]
async fn caducidad_ttl() {
    let s = Servidor::arrancar(None).await;
    let mut c = s.cliente().await;
    assert_eq!(c.cmd("TTL nada").await, ":-2");
    c.cmd("SET fijo v").await;
    assert_eq!(c.cmd("TTL fijo").await, ":-1");
    assert_eq!(c.cmd("SET temp v PX 80").await, "+OK");
    assert_eq!(c.cmd("TTL temp").await, ":1");
    assert_eq!(c.cmd("GET temp").await, "$v");
    tokio::time::sleep(Duration::from_millis(150)).await;
    assert_eq!(c.cmd("GET temp").await, "_");
    assert_eq!(c.cmd("TTL temp").await, ":-2");
    s.detener().await;
}

#[tokio::test]
async fn clientes_concurrentes_no_pierden_incrementos() {
    let s = Servidor::arrancar(None).await;
    let mut tareas = Vec::new();
    for _ in 0..20 {
        let direccion = s.direccion;
        tareas.push(tokio::spawn(async move {
            let mut c = Cliente::conectar(direccion).await;
            for _ in 0..50 {
                assert!(c.cmd("INCR contador").await.starts_with(':'));
            }
        }));
    }
    for t in tareas {
        t.await.unwrap();
    }
    let mut c = s.cliente().await;
    assert_eq!(c.cmd("GET contador").await, "$1000");
    s.detener().await;
}

#[tokio::test]
async fn quit_cierra_la_conexion() {
    let s = Servidor::arrancar(None).await;
    let mut c = s.cliente().await;
    assert_eq!(c.cmd("QUIT").await, "+BYE");
    let mut resto = Vec::new();
    assert_eq!(
        c.lector.read_to_end(&mut resto).await.unwrap(),
        0,
        "el servidor debe cerrar"
    );
    s.detener().await;
}

#[tokio::test]
async fn linea_demasiado_larga_se_rechaza() {
    let s = Servidor::arrancar(None).await;
    let mut c = s.cliente().await;
    let enorme = "x".repeat(kvdb::servidor::MAX_LINEA as usize + 10);
    c.escritor.write_all(enorme.as_bytes()).await.unwrap();
    assert!(c.linea().await.starts_with("-ERR linea demasiado larga"));
    s.detener().await;
}

#[tokio::test]
async fn el_apagado_avisa_a_los_clientes_y_termina() {
    let s = Servidor::arrancar(None).await;
    let mut c = s.cliente().await;
    assert_eq!(c.cmd("PING").await, "+PONG");
    let mut c2 = s.cliente().await; // ocioso: debe recibir el aviso
    c2.cmd("PING").await;
    s.detener().await; // falla si el servidor no termina en 10 s
    assert!(c.linea().await.contains("apagandose"));
}

#[tokio::test]
async fn persistencia_aof_sobrevive_a_un_reinicio() {
    let ruta = archivo_temporal("persistencia");

    let s = Servidor::arrancar(Some(&ruta)).await;
    let mut c = s.cliente().await;
    c.cmd(r#"SET nombre "Ana María""#).await;
    c.cmd("SET borrar x").await;
    c.cmd("DEL borrar").await;
    c.cmd("INCR n").await;
    c.cmd("INCR n").await;
    c.cmd("SET efimera x PX 30").await;
    c.cmd("SET larga v EX 3600").await;
    drop(c);
    s.detener().await; // el apagado ordenado vuelca el AOF

    tokio::time::sleep(Duration::from_millis(60)).await; // deja caducar `efimera`

    let s = Servidor::arrancar(Some(&ruta)).await;
    let mut c = s.cliente().await;
    assert_eq!(c.cmd("GET nombre").await, "$Ana María");
    assert_eq!(c.cmd("GET borrar").await, "_");
    assert_eq!(c.cmd("GET n").await, "$2");
    assert_eq!(
        c.cmd("GET efimera").await,
        "_",
        "una clave caducada no debe resucitar"
    );
    assert_eq!(c.cmd("GET larga").await, "$v");
    let ttl: i64 = c
        .cmd("TTL larga")
        .await
        .trim_start_matches(':')
        .parse()
        .unwrap();
    assert!(
        (3590..=3600).contains(&ttl),
        "el TTL se conserva tras reiniciar: {ttl}"
    );
    drop(c);
    s.detener().await;

    let _ = std::fs::remove_file(ruta);
}
