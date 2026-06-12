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
// TESTS BÁSICOS DE TURNOS
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

// ==============================================
// TESTS DE DETECCIÓN DE TURNO ACTUAL
// ==============================================

#[test]
fn test_obtener_tipo_turno_actual() {
    println!("\n⏰ TEST: Obtener tipo de turno según día y hora actual");
    let conn = get_db_connection().unwrap();
    
    let resultado = obtener_tipo_turno_actual_logic(&conn);
    
    match resultado {
        Ok(id_tipo_turno) => {
            println!("   ✅ Tipo de turno detectado: ID = {}", id_tipo_turno);
            
            // Obtener detalles del tipo de turno
            let mut stmt = conn.prepare("SELECT nombre, hora_inicio, hora_fin FROM tipos_turno WHERE id_tipo_turno = ?1")
                .unwrap();
            let detalles = stmt.query_row([id_tipo_turno], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))
            }).unwrap();
            
            println!("      📛 Nombre: {}", detalles.0);
            println!("      🕐 Horario: {} - {}", detalles.1, detalles.2);
        }
        Err(e) => {
            println!("   ⚠️ No se pudo determinar el turno: {}", e);
            println!("   (Puede que no haya turno definido para este día/hora)");
        }
    }
}

#[test]
fn test_abrir_turno_automatico() {
    println!("\n🚀 TEST: Abrir turno automático");
    let conn = get_db_connection().unwrap();
    
    // Limpiar turnos abiertos del usuario 1
    let _ = conn.execute("UPDATE turnos SET estado = 'CERRADO' WHERE id_usuario = 1 AND estado = 'ABIERTO'", []);
    
    let id_usuario = obtener_usuario_valido(&conn);
    
    let resultado = abrir_turno_automatico_logic(&conn, id_usuario);
    
    match resultado {
        Ok(id_turno) => {
            println!("   ✅ Turno abierto automáticamente con ID: {}", id_turno);
            
            // Obtener detalles del turno creado
            let turno = obtener_turno_logic(&conn, id_turno).unwrap();
            println!("      👤 Usuario ID: {}", turno.id_usuario);
            println!("      📅 Fecha inicio: {}", turno.fecha_inicio);
            println!("      📌 Estado: {}", turno.estado);
            
            // Obtener el tipo de turno asignado
            let mut stmt = conn.prepare("SELECT tt.nombre FROM tipos_turno tt JOIN turnos t ON t.id_tipo_turno = tt.id_tipo_turno WHERE t.id_turno = ?1")
                .unwrap();
            let tipo_nombre: String = stmt.query_row([id_turno], |row| row.get(0)).unwrap();
            println!("      🔄 Tipo de turno asignado: {}", tipo_nombre);
            
            assert_eq!(turno.estado, "ABIERTO");
            
            // Limpiar
            let _ = conn.execute("DELETE FROM turnos WHERE id_turno = ?1", [id_turno]);
        }
        Err(e) => {
            println!("   ⚠️ No se pudo abrir el turno: {}", e);
            println!("   (Puede que no haya un tipo de turno definido para este día/hora)");
        }
    }
}

#[test]
fn test_abrir_turno_automatico_usuario_inactivo() {
    println!("\n🚫 TEST: Abrir turno automático con usuario inexistente");
    let conn = get_db_connection().unwrap();
    
    // Usar un ID de usuario que no existe (ej: 9999)
    let resultado = abrir_turno_automatico_logic(&conn, 9999);
    
    match resultado {
        Ok(_) => panic!("❌ No debería abrir turno con usuario inexistente"),
        Err(e) => {
            println!("   ✅ Error esperado: {}", e);
            assert!(e.contains("no existe") || e.contains("inactivo"));
        }
    }
}

#[test]
fn test_abrir_turno_automatico_con_turno_abierto() {
    println!("\n🔁 TEST: Abrir turno cuando ya hay uno abierto");
    let conn = get_db_connection().unwrap();
    
    let id_usuario = obtener_usuario_valido(&conn);
    
    // Cerrar turnos abiertos anteriores
    let _ = conn.execute("UPDATE turnos SET estado = 'CERRADO' WHERE id_usuario = ?1 AND estado = 'ABIERTO'", [id_usuario]);
    
    // Abrir primer turno manualmente
    let id_tipo = obtener_tipo_turno_valido(&conn);
    let nuevo = NuevoTurno {
        id_usuario,
        id_tipo_turno: id_tipo,
    };
    let primer_turno = abrir_turno_logic(&conn, &nuevo).unwrap();
    println!("   📌 Primer turno abierto con ID: {}", primer_turno);
    
    // Intentar abrir otro turno automáticamente
    let resultado = abrir_turno_automatico_logic(&conn, id_usuario);
    
    match resultado {
        Ok(_) => panic!("❌ No debería abrir segundo turno"),
        Err(e) => {
            println!("   ✅ Error esperado: {}", e);
            assert!(e.contains("ya tiene un turno abierto"));
        }
    }
    
    // Limpiar
    let _ = conn.execute("DELETE FROM turnos WHERE id_turno = ?1", [primer_turno]);
}