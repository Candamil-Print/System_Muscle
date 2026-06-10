//! Tests del módulo de turnos
//! Ejecutar con: cargo test --test turnos_test -- --nocapture

use system_muscle_lib::services::db::connection::get_db_connection;
use system_muscle_lib::commands::turnos::logic::*;
use system_muscle_lib::models::turnos::turno::{NuevoTurno, FiltroTurno};

/// Limpiar turnos de prueba (excepto los que queremos conservar)
fn limpiar_turnos_prueba(conn: &rusqlite::Connection) {
    // Cerrar todos los turnos abiertos primero
    let _ = conn.execute("UPDATE turnos SET estado = 'CERRADO' WHERE estado = 'ABIERTO'", []);
    // Limpiar turnos de prueba
    let _ = conn.execute("DELETE FROM turnos WHERE id_turno > 2", []);
}

/// Obtener un usuario válido
fn obtener_usuario_valido(conn: &rusqlite::Connection) -> i32 {
    conn.query_row("SELECT id_usuario FROM usuarios WHERE estado = 1 LIMIT 1", [], |row| row.get(0))
        .unwrap_or(1)
}

/// Obtener un tipo de turno válido
fn obtener_tipo_turno_valido(conn: &rusqlite::Connection) -> i32 {
    conn.query_row("SELECT id_tipo_turno FROM tipos_turno LIMIT 1", [], |row| row.get(0))
        .unwrap_or(1)
}

// ==============================================
// TESTS DE TURNOS
// ==============================================

#[test]
fn test_abrir_turno() {
    println!("\n🕐 TEST: Abrir turno");
    let conn = get_db_connection().unwrap();
    
    limpiar_turnos_prueba(&conn);
    let id_usuario = obtener_usuario_valido(&conn);
    let id_tipo_turno = obtener_tipo_turno_valido(&conn);
    
    let nuevo = NuevoTurno {
        id_usuario,
        id_tipo_turno,
    };
    
    let resultado = abrir_turno_logic(&conn, &nuevo);
    
    match resultado {
        Ok(id) => {
            println!("   ✅ Turno abierto con ID: {}", id);
            assert!(id > 0);
        }
        Err(e) => panic!("❌ Error: {}", e),
    }
}

#[test]
fn test_obtener_turno_activo() {
    println!("\n🔍 TEST: Obtener turno activo");
    let conn = get_db_connection().unwrap();
    
    // Asegurar que hay al menos un turno abierto
    limpiar_turnos_prueba(&conn);
    let id_usuario = obtener_usuario_valido(&conn);
    let id_tipo_turno = obtener_tipo_turno_valido(&conn);
    
    let nuevo = NuevoTurno {
        id_usuario,
        id_tipo_turno,
    };
    let _ = abrir_turno_logic(&conn, &nuevo).unwrap();
    
    let resultado = obtener_turno_activo_general_logic(&conn);
    
    match resultado {
        Ok(Some(turno)) => {
            println!("   ✅ Turno activo encontrado:");
            println!("      ID: {}", turno.id_turno);
            println!("      Estado: {}", turno.estado);
            assert_eq!(turno.estado, "ABIERTO");
        }
        Ok(None) => println!("   ⚠️ No hay turnos activos"),
        Err(e) => panic!("❌ Error: {}", e),
    }
}

#[test]
fn test_listar_turnos() {
    println!("\n📋 TEST: Listar turnos");
    let conn = get_db_connection().unwrap();
    
    let filtro = FiltroTurno {
        id_usuario: None,
        estado: None,
        fecha_desde: None,
        fecha_hasta: None,
    };
    
    let resultado = listar_turnos_logic(&conn, &filtro);
    
    match resultado {
        Ok(turnos) => {
            println!("   📊 Total turnos: {}", turnos.len());
            for t in &turnos {
                println!("      ID: {} | Estado: {}", t.id_turno, t.estado);
            }
        }
        Err(e) => panic!("❌ Error: {}", e),
    }
}

#[test]
fn test_listar_turnos_detalle() {
    println!("\n📋 TEST: Listar turnos con detalle");
    let conn = get_db_connection().unwrap();
    
    let filtro = FiltroTurno {
        id_usuario: None,
        estado: None,
        fecha_desde: None,
        fecha_hasta: None,
    };
    
    let resultado = listar_turnos_detalle_logic(&conn, &filtro);
    
    match resultado {
        Ok(turnos) => {
            println!("   📊 Total turnos con detalle: {}", turnos.len());
            for t in &turnos {
                println!("      ID: {} | Usuario: {} | Tipo: {} | Estado: {}", 
                    t.id_turno, t.usuario, t.tipo_turno, t.estado);
            }
        }
        Err(e) => panic!("❌ Error: {}", e),
    }
}

#[test]
fn test_cerrar_turno() {
    println!("\n🔒 TEST: Cerrar turno");
    let conn = get_db_connection().unwrap();
    
    // Limpiar y crear un turno nuevo específicamente para este test
    limpiar_turnos_prueba(&conn);
    let id_usuario = obtener_usuario_valido(&conn);
    let id_tipo_turno = obtener_tipo_turno_valido(&conn);
    
    let nuevo = NuevoTurno {
        id_usuario,
        id_tipo_turno,
    };
    let id_turno = abrir_turno_logic(&conn, &nuevo).unwrap();
    println!("   📌 Turno abierto con ID: {}", id_turno);
    
    // Verificar que está abierto
    let turno = obtener_turno_logic(&conn, id_turno).unwrap();
    assert_eq!(turno.estado, "ABIERTO");
    
    // Cerrarlo
    let resultado = cerrar_turno_logic(&conn, id_turno);
    
    match resultado {
        Ok(()) => {
            println!("   ✅ Turno cerrado correctamente");
            let turno_cerrado = obtener_turno_logic(&conn, id_turno).unwrap();
            assert_eq!(turno_cerrado.estado, "CERRADO");
            println!("   ✅ Verificación: Estado = CERRADO");
        }
        Err(e) => panic!("❌ Error: {}", e),
    }
}