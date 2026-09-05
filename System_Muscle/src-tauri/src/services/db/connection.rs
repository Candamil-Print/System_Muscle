use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

// Estructura para compartir la conexión a la base de datos entre componentes
pub struct DbState {
    pub conn: Mutex<Connection>,
}

// Ruta de la base de datos en el directorio de datos de la aplicación.
// Vive fuera de src-tauri para que el observador de archivos de `tauri dev`
// no reinicie la aplicación al escribir en la base de datos.
pub fn get_db_path() -> PathBuf {
    let mut dir = match std::env::var("APPDATA") {
        Ok(p) => PathBuf::from(p),
        Err(_) => match std::env::var("HOME") {
            Ok(h) => PathBuf::from(h).join("Library/Application Support"),
            Err(_) => PathBuf::from("."),
        },
    };
    dir.push("com.monsa.system_muscle");
    dir.push("system_muscle.db");
    dir
}

// Inicializa la conexión a la base de datos en la ruta de datos de la app
pub fn init_db() -> Result<Connection> {
    let path = get_db_path();

    if let Some(parent) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            eprintln!("No se pudo crear el directorio de datos: {}", e);
        }
    }

    // Migración: si la nueva DB no existe pero sí la anterior (junto al ejecutable),
    // se copia para no perder la información existente.
    if !path.exists() {
        let legacy = PathBuf::from("system_muscle.db");
        if legacy.exists() {
            match std::fs::copy(&legacy, &path) {
                Ok(_) => println!("Base de datos migrada a {}", path.display()),
                Err(e) => eprintln!("No se pudo migrar la base de datos: {}", e),
            }
        }
    }

    let conn = Connection::open(&path)?;
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    Ok(conn)
}

// Obtiene una conexión a la base de datos
pub fn get_db_connection() -> Result<Connection> {
    let conn = Connection::open("system_muscle.db")?;
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    Ok(conn)
}