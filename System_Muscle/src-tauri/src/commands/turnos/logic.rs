use rusqlite::Connection;
use crate::models::turnos::turno::{Turno, NuevoTurno, TurnoDetalle, FiltroTurno};

// ==============================================
// FUNCIONES LÓGICAS
// ==============================================

pub fn abrir_turno_logic(conn: &Connection, nuevo: &NuevoTurno) -> Result<i32, String> {
    let mut stmt = conn.prepare("SELECT 1 FROM usuarios WHERE id_usuario = ?1 AND estado = 1")
        .map_err(|e| e.to_string())?;
    let usuario_existe = stmt.exists([nuevo.id_usuario]).map_err(|e| e.to_string())?;
    
    if !usuario_existe {
        return Err("El usuario no existe o está inactivo".to_string());
    }
    
    let mut stmt = conn.prepare("SELECT 1 FROM tipos_turno WHERE id_tipo_turno = ?1")
        .map_err(|e| e.to_string())?;
    let tipo_existe = stmt.exists([nuevo.id_tipo_turno]).map_err(|e| e.to_string())?;
    
    if !tipo_existe {
        return Err("El tipo de turno no existe".to_string());
    }
    
    let mut stmt = conn.prepare("SELECT 1 FROM turnos WHERE id_usuario = ?1 AND estado = 'ABIERTO'")
        .map_err(|e| e.to_string())?;
    let turno_abierto = stmt.exists([nuevo.id_usuario]).map_err(|e| e.to_string())?;
    
    if turno_abierto {
        return Err("El usuario ya tiene un turno abierto".to_string());
    }
    
    conn.execute(
        "INSERT INTO turnos (id_usuario, id_tipo_turno, fecha_inicio, estado)
         VALUES (?1, ?2, CURRENT_TIMESTAMP, 'ABIERTO')",
        [&nuevo.id_usuario, &nuevo.id_tipo_turno]
    ).map_err(|e| e.to_string())?;
    
    Ok(conn.last_insert_rowid() as i32)
}

pub fn cerrar_turno_logic(conn: &Connection, id_turno: i32) -> Result<(), String> {
    let estado_result: Result<String, rusqlite::Error> = conn.query_row(
        "SELECT estado FROM turnos WHERE id_turno = ?1",
        [id_turno],
        |row| row.get(0)
    );
    
    let estado = match estado_result {
        Ok(e) => e,
        Err(e) => return Err(e.to_string()),
    };
    
    if estado != "ABIERTO" {
        return Err("El turno no está abierto o no existe".to_string());
    }
    
    conn.execute(
        "UPDATE turnos SET fecha_fin = CURRENT_TIMESTAMP, estado = 'CERRADO' WHERE id_turno = ?1",
        [id_turno]
    ).map_err(|e| e.to_string())?;
    
    Ok(())
}

pub fn obtener_turno_logic(conn: &Connection, id: i32) -> Result<Turno, String> {
    let mut stmt = conn.prepare(
        "SELECT id_turno, id_usuario, id_tipo_turno, fecha_inicio, fecha_fin, estado
         FROM turnos WHERE id_turno = ?1"
    ).map_err(|e| e.to_string())?;
    
    let turno_result = stmt.query_row([id], |row| {
        let id_turno = match row.get(0) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let id_usuario = match row.get(1) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let id_tipo_turno = match row.get(2) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let fecha_inicio = match row.get(3) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let fecha_fin = match row.get(4) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let estado = match row.get(5) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        
        Ok(Turno {
            id_turno,
            id_usuario,
            id_tipo_turno,
            fecha_inicio,
            fecha_fin,
            estado,
        })
    });
    
    match turno_result {
        Ok(turno) => Ok(turno),
        Err(e) => Err(e.to_string()),
    }
}

pub fn obtener_turno_activo_logic(conn: &Connection, id_usuario: i32) -> Result<Option<Turno>, String> {
    let mut stmt = conn.prepare(
        "SELECT id_turno, id_usuario, id_tipo_turno, fecha_inicio, fecha_fin, estado
         FROM turnos WHERE id_usuario = ?1 AND estado = 'ABIERTO' LIMIT 1"
    ).map_err(|e| e.to_string())?;
    
    let mut rows = match stmt.query([id_usuario]) {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };
    
    match rows.next() {
        Ok(Some(row)) => {
            let id_turno = match row.get(0) {
                Ok(v) => v,
                Err(e) => return Err(e.to_string()),
            };
            let id_usuario_val = match row.get(1) {
                Ok(v) => v,
                Err(e) => return Err(e.to_string()),
            };
            let id_tipo_turno = match row.get(2) {
                Ok(v) => v,
                Err(e) => return Err(e.to_string()),
            };
            let fecha_inicio = match row.get(3) {
                Ok(v) => v,
                Err(e) => return Err(e.to_string()),
            };
            let fecha_fin = match row.get(4) {
                Ok(v) => v,
                Err(e) => return Err(e.to_string()),
            };
            let estado = match row.get(5) {
                Ok(v) => v,
                Err(e) => return Err(e.to_string()),
            };
            
            Ok(Some(Turno {
                id_turno,
                id_usuario: id_usuario_val,
                id_tipo_turno,
                fecha_inicio,
                fecha_fin,
                estado,
            }))
        }
        Ok(None) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

pub fn obtener_turno_activo_general_logic(conn: &Connection) -> Result<Option<Turno>, String> {
    let mut stmt = conn.prepare(
        "SELECT id_turno, id_usuario, id_tipo_turno, fecha_inicio, fecha_fin, estado
         FROM turnos WHERE estado = 'ABIERTO' LIMIT 1"
    ).map_err(|e| e.to_string())?;
    
    let mut rows = match stmt.query([]) {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };
    
    match rows.next() {
        Ok(Some(row)) => {
            let id_turno = match row.get(0) {
                Ok(v) => v,
                Err(e) => return Err(e.to_string()),
            };
            let id_usuario = match row.get(1) {
                Ok(v) => v,
                Err(e) => return Err(e.to_string()),
            };
            let id_tipo_turno = match row.get(2) {
                Ok(v) => v,
                Err(e) => return Err(e.to_string()),
            };
            let fecha_inicio = match row.get(3) {
                Ok(v) => v,
                Err(e) => return Err(e.to_string()),
            };
            let fecha_fin = match row.get(4) {
                Ok(v) => v,
                Err(e) => return Err(e.to_string()),
            };
            let estado = match row.get(5) {
                Ok(v) => v,
                Err(e) => return Err(e.to_string()),
            };
            
            Ok(Some(Turno {
                id_turno,
                id_usuario,
                id_tipo_turno,
                fecha_inicio,
                fecha_fin,
                estado,
            }))
        }
        Ok(None) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

pub fn listar_turnos_logic(conn: &Connection, filtro: &FiltroTurno) -> Result<Vec<Turno>, String> {
    let mut conditions = Vec::new();
    let mut params: Vec<String> = Vec::new();
    
    if let Some(id_usuario) = filtro.id_usuario {
        conditions.push("id_usuario = ?".to_string());
        params.push(id_usuario.to_string());
    }
    if let Some(ref estado) = filtro.estado {
        conditions.push("estado = ?".to_string());
        params.push(estado.clone());
    }
    if let Some(ref fecha_desde) = filtro.fecha_desde {
        conditions.push("fecha_inicio >= ?".to_string());
        params.push(fecha_desde.clone());
    }
    if let Some(ref fecha_hasta) = filtro.fecha_hasta {
        conditions.push("fecha_inicio <= ?".to_string());
        params.push(fecha_hasta.clone());
    }
    
    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };
    
    let query = format!(
        "SELECT id_turno, id_usuario, id_tipo_turno, fecha_inicio, fecha_fin, estado
         FROM turnos
         {} 
         ORDER BY fecha_inicio DESC",
        where_clause
    );
    
    let mut stmt = match conn.prepare(&query) {
        Ok(s) => s,
        Err(e) => return Err(e.to_string()),
    };
    
    let params_slice: Vec<&str> = params.iter().map(|s| s.as_str()).collect();
    let rows = match stmt.query_map(rusqlite::params_from_iter(params_slice), |row| {
        let id_turno = match row.get(0) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let id_usuario = match row.get(1) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let id_tipo_turno = match row.get(2) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let fecha_inicio = match row.get(3) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let fecha_fin = match row.get(4) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let estado = match row.get(5) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        
        Ok(Turno {
            id_turno,
            id_usuario,
            id_tipo_turno,
            fecha_inicio,
            fecha_fin,
            estado,
        })
    }) {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };
    
    let mut turnos = Vec::new();
    for turno_result in rows {
        match turno_result {
            Ok(turno) => turnos.push(turno),
            Err(e) => return Err(e.to_string()),
        }
    }
    
    Ok(turnos)
}

pub fn listar_turnos_detalle_logic(conn: &Connection, filtro: &FiltroTurno) -> Result<Vec<TurnoDetalle>, String> {
    let mut conditions = Vec::new();
    let mut params: Vec<String> = Vec::new();
    
    if let Some(id_usuario) = filtro.id_usuario {
        conditions.push("t.id_usuario = ?".to_string());
        params.push(id_usuario.to_string());
    }
    if let Some(ref estado) = filtro.estado {
        conditions.push("t.estado = ?".to_string());
        params.push(estado.clone());
    }
    if let Some(ref fecha_desde) = filtro.fecha_desde {
        conditions.push("t.fecha_inicio >= ?".to_string());
        params.push(fecha_desde.clone());
    }
    if let Some(ref fecha_hasta) = filtro.fecha_hasta {
        conditions.push("t.fecha_inicio <= ?".to_string());
        params.push(fecha_hasta.clone());
    }
    
    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };
    
    let query = format!(
        r#"SELECT 
            t.id_turno, 
            u.nombre_completo as usuario,
            tt.nombre as tipo_turno,
            tt.hora_inicio,
            tt.hora_fin,
            t.fecha_inicio, 
            t.fecha_fin, 
            t.estado
         FROM turnos t
         LEFT JOIN usuarios u ON t.id_usuario = u.id_usuario
         LEFT JOIN tipos_turno tt ON t.id_tipo_turno = tt.id_tipo_turno
         {} 
         ORDER BY t.fecha_inicio DESC"#,
        where_clause
    );
    
    let mut stmt = match conn.prepare(&query) {
        Ok(s) => s,
        Err(e) => return Err(e.to_string()),
    };
    
    let params_slice: Vec<&str> = params.iter().map(|s| s.as_str()).collect();
    let rows = match stmt.query_map(rusqlite::params_from_iter(params_slice), |row| {
        let id_turno = match row.get(0) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let usuario = match row.get(1) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let tipo_turno = match row.get(2) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let hora_inicio = match row.get(3) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let hora_fin = match row.get(4) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let fecha_inicio = match row.get(5) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let fecha_fin = match row.get(6) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let estado = match row.get(7) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        
        Ok(TurnoDetalle {
            id_turno,
            usuario,
            tipo_turno,
            hora_inicio,
            hora_fin,
            fecha_inicio,
            fecha_fin,
            estado,
        })
    }) {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };
    
    let mut turnos = Vec::new();
    for turno_result in rows {
        match turno_result {
            Ok(turno) => turnos.push(turno),
            Err(e) => return Err(e.to_string()),
        }
    }
    
    Ok(turnos)
}

pub fn turnos_por_usuario_logic(conn: &Connection, id_usuario: i32) -> Result<Vec<Turno>, String> {
    let filtro = FiltroTurno {
        id_usuario: Some(id_usuario),
        estado: None,
        fecha_desde: None,
        fecha_hasta: None,
    };
    listar_turnos_logic(conn, &filtro)
}

pub fn turnos_por_estado_logic(conn: &Connection, estado: &str) -> Result<Vec<Turno>, String> {
    let filtro = FiltroTurno {
        id_usuario: None,
        estado: Some(estado.to_string()),
        fecha_desde: None,
        fecha_hasta: None,
    };
    listar_turnos_logic(conn, &filtro)
}