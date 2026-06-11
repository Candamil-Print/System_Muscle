use rusqlite::Connection;
use crate::models::reportes_entrada::reporte_entrada::{
    DashboardEntradas, EntradasPorDia, EntradasPorTipoProducto, EntradasPorUsuario,
    ResumenEntradasProducto, TotalesEntradas,
};

fn validar_rango_fechas(fecha_inicio: &str, fecha_fin: &str) -> Result<(), String> {
    if fecha_inicio.is_empty() || fecha_fin.is_empty() {
        return Err("Las fechas de inicio y fin son obligatorias".to_string());
    }
    Ok(())
}

/// Resumen de entradas agrupado por producto en un rango de fechas (YYYY-MM-DD).
pub fn resumen_entradas_por_producto_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<ResumenEntradasProducto>, String> {
    validar_rango_fechas(fecha_inicio, fecha_fin)?;

    let mut stmt = conn
        .prepare(
            r#"SELECT
                me.id_producto,
                p.nombre,
                p.tipo_producto,
                COUNT(me.id_movimiento)    AS numero_movimientos,
                COALESCE(SUM(me.cantidad), 0) AS cantidad_total_ingresada,
                MIN(DATE(me.fecha))        AS primera_entrada,
                MAX(DATE(me.fecha))        AS ultima_entrada
            FROM movimientos_entrada me
            INNER JOIN productos p ON me.id_producto = p.id_producto
            WHERE DATE(me.fecha) BETWEEN DATE(?1) AND DATE(?2)
            GROUP BY me.id_producto
            ORDER BY cantidad_total_ingresada DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(rusqlite::params![fecha_inicio, fecha_fin], |row| {
            Ok(ResumenEntradasProducto {
                id_producto: row.get(0)?,
                nombre_producto: row.get(1)?,
                tipo_producto: row.get(2)?,
                numero_movimientos: row.get(3)?,
                cantidad_total_ingresada: row.get(4)?,
                primera_entrada: row.get(5)?,
                ultima_entrada: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Totales globales de entradas en un rango de fechas.
pub fn totales_entradas_rango_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<TotalesEntradas, String> {
    validar_rango_fechas(fecha_inicio, fecha_fin)?;

    conn.query_row(
        r#"SELECT
            ?1 AS fecha_inicio,
            ?2 AS fecha_fin,
            COUNT(id_movimiento)          AS numero_movimientos,
            COALESCE(SUM(cantidad), 0)    AS cantidad_total_ingresada,
            COUNT(DISTINCT id_producto)   AS productos_distintos
        FROM movimientos_entrada
        WHERE DATE(fecha) BETWEEN DATE(?1) AND DATE(?2)"#,
        rusqlite::params![fecha_inicio, fecha_fin],
        |row| {
            Ok(TotalesEntradas {
                fecha_inicio: row.get(0)?,
                fecha_fin: row.get(1)?,
                numero_movimientos: row.get(2)?,
                cantidad_total_ingresada: row.get(3)?,
                productos_distintos: row.get(4)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

/// Entradas agrupadas por día dentro de un rango de fechas.
pub fn entradas_por_dia_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<EntradasPorDia>, String> {
    validar_rango_fechas(fecha_inicio, fecha_fin)?;

    let mut stmt = conn
        .prepare(
            r#"SELECT
                DATE(fecha)                        AS fecha,
                COUNT(id_movimiento)               AS numero_movimientos,
                COALESCE(SUM(cantidad), 0)         AS cantidad_total_ingresada
            FROM movimientos_entrada
            WHERE DATE(fecha) BETWEEN DATE(?1) AND DATE(?2)
            GROUP BY DATE(fecha)
            ORDER BY fecha DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(rusqlite::params![fecha_inicio, fecha_fin], |row| {
            Ok(EntradasPorDia {
                fecha: row.get(0)?,
                numero_movimientos: row.get(1)?,
                cantidad_total_ingresada: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Entradas agrupadas por usuario en un rango de fechas.
pub fn entradas_por_usuario_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<EntradasPorUsuario>, String> {
    validar_rango_fechas(fecha_inicio, fecha_fin)?;

    let mut stmt = conn
        .prepare(
            r#"SELECT
                me.id_usuario,
                u.nombre_completo,
                COUNT(me.id_movimiento)        AS numero_movimientos,
                COALESCE(SUM(me.cantidad), 0)  AS cantidad_total_ingresada
            FROM movimientos_entrada me
            INNER JOIN usuarios u ON me.id_usuario = u.id_usuario
            WHERE DATE(me.fecha) BETWEEN DATE(?1) AND DATE(?2)
            GROUP BY me.id_usuario
            ORDER BY cantidad_total_ingresada DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(rusqlite::params![fecha_inicio, fecha_fin], |row| {
            Ok(EntradasPorUsuario {
                id_usuario: row.get(0)?,
                nombre_usuario: row.get(1)?,
                numero_movimientos: row.get(2)?,
                cantidad_total_ingresada: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Entradas agrupadas por tipo de producto en un rango de fechas.
pub fn entradas_por_tipo_producto_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<EntradasPorTipoProducto>, String> {
    validar_rango_fechas(fecha_inicio, fecha_fin)?;

    let mut stmt = conn
        .prepare(
            r#"SELECT
                p.tipo_producto,
                COUNT(me.id_movimiento)        AS numero_movimientos,
                COALESCE(SUM(me.cantidad), 0)  AS cantidad_total_ingresada
            FROM movimientos_entrada me
            INNER JOIN productos p ON me.id_producto = p.id_producto
            WHERE DATE(me.fecha) BETWEEN DATE(?1) AND DATE(?2)
            GROUP BY p.tipo_producto
            ORDER BY cantidad_total_ingresada DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(rusqlite::params![fecha_inicio, fecha_fin], |row| {
            Ok(EntradasPorTipoProducto {
                tipo_producto: row.get(0)?,
                numero_movimientos: row.get(1)?,
                cantidad_total_ingresada: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// KPIs del módulo de entradas: movimientos de hoy y de la semana en curso.
pub fn dashboard_entradas_logic(conn: &Connection) -> Result<DashboardEntradas, String> {
    let fecha: String = conn
        .query_row("SELECT DATE('now', 'localtime')", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    let entradas_hoy: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM movimientos_entrada WHERE DATE(fecha) = DATE('now', 'localtime')",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let cantidad_ingresada_hoy: i32 = conn
        .query_row(
            r#"SELECT COALESCE(SUM(cantidad), 0)
               FROM movimientos_entrada
               WHERE DATE(fecha) = DATE('now', 'localtime')"#,
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let entradas_semana: i32 = conn
        .query_row(
            r#"SELECT COUNT(*)
               FROM movimientos_entrada
               WHERE DATE(fecha) >= DATE('now', 'localtime', '-6 days')"#,
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let cantidad_ingresada_semana: i32 = conn
        .query_row(
            r#"SELECT COALESCE(SUM(cantidad), 0)
               FROM movimientos_entrada
               WHERE DATE(fecha) >= DATE('now', 'localtime', '-6 days')"#,
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let productos_con_entradas_hoy: i32 = conn
        .query_row(
            r#"SELECT COUNT(DISTINCT id_producto)
               FROM movimientos_entrada
               WHERE DATE(fecha) = DATE('now', 'localtime')"#,
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    Ok(DashboardEntradas {
        fecha,
        entradas_hoy,
        cantidad_ingresada_hoy,
        entradas_semana,
        cantidad_ingresada_semana,
        productos_con_entradas_hoy,
    })
}