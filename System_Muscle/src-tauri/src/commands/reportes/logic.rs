use rusqlite::Connection;
use crate::models::reportes::reporte::{
    DashboardResumen, DetalleMargenProducto, ProductoMasVendido, ReporteCaja, ReporteEntradasProducto,
    ReporteInventario, ReporteMargenGanancia, ReporteStockBajo, ResumenVentasDiario, ResumenVentasRango,
    VentasPorMetodoPago, VentasPorUsuario, VentasPorTurno, VentaDetallePorTurno, ReporteConsolidadoVentas,
    ReporteVentasDetallado, DashboardVentasGeneral, ResumenVentasProducto, VentasPorMetodoPagoTotal,
};



/// Dashboard general de ventas (sin rango de fechas)
pub fn dashboard_ventas_general_logic(conn: &Connection) -> Result<DashboardVentasGeneral, String> {
    let fecha: String = conn
        .query_row("SELECT DATE('now', 'localtime')", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    // Ventas de hoy
    let ventas_hoy: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM ventas WHERE DATE(fecha) = DATE('now', 'localtime')",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let total_vendido_hoy: f64 = conn
        .query_row(
            r#"SELECT COALESCE(SUM(dv.subtotal), 0)
               FROM detalle_venta dv
               INNER JOIN ventas v ON dv.id_venta = v.id_venta
               WHERE DATE(v.fecha) = DATE('now', 'localtime')"#,
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    // Ventas de la semana (últimos 7 días)
    let ventas_semana: i32 = conn
        .query_row(
            r#"SELECT COUNT(*)
               FROM ventas
               WHERE DATE(fecha) >= DATE('now', 'localtime', '-6 days')"#,
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let total_vendido_semana: f64 = conn
        .query_row(
            r#"SELECT COALESCE(SUM(dv.subtotal), 0)
               FROM detalle_venta dv
               INNER JOIN ventas v ON dv.id_venta = v.id_venta
               WHERE DATE(v.fecha) >= DATE('now', 'localtime', '-6 days')"#,
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    // Ventas del mes (últimos 30 días)
    let ventas_mes: i32 = conn
        .query_row(
            r#"SELECT COUNT(*)
               FROM ventas
               WHERE DATE(fecha) >= DATE('now', 'localtime', '-29 days')"#,
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let total_vendido_mes: f64 = conn
        .query_row(
            r#"SELECT COALESCE(SUM(dv.subtotal), 0)
               FROM detalle_venta dv
               INNER JOIN ventas v ON dv.id_venta = v.id_venta
               WHERE DATE(v.fecha) >= DATE('now', 'localtime', '-29 days')"#,
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    // Totales generales
    let total_ventas: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM ventas",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let total_general: f64 = conn
        .query_row(
            r#"SELECT COALESCE(SUM(dv.subtotal), 0)
               FROM detalle_venta dv"#,
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let total_efectivo: f64 = conn
        .query_row(
            r#"SELECT COALESCE(SUM(dv.subtotal), 0)
               FROM detalle_venta dv
               INNER JOIN metodos_pago mp ON dv.metodo_pago = mp.id_metodo
               WHERE mp.nombre = 'EFECTIVO'"#,
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let total_transferencia: f64 = conn
        .query_row(
            r#"SELECT COALESCE(SUM(dv.subtotal), 0)
               FROM detalle_venta dv
               INNER JOIN metodos_pago mp ON dv.metodo_pago = mp.id_metodo
               WHERE mp.nombre = 'TRANSFERENCIA'"#,
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    // Productos distintos vendidos
    let productos_distintos_vendidos: i32 = conn
        .query_row(
            "SELECT COUNT(DISTINCT id_producto) FROM detalle_venta",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    // Número de métodos de pago usados
    let numero_metodos_pago: i32 = conn
        .query_row(
            "SELECT COUNT(DISTINCT metodo_pago) FROM detalle_venta",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    // Número de vendedores
    let numero_vendedores: i32 = conn
        .query_row(
            "SELECT COUNT(DISTINCT id_usuario) FROM ventas",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    Ok(DashboardVentasGeneral {
        fecha,
        ventas_hoy,
        total_vendido_hoy,
        ventas_semana,
        total_vendido_semana,
        ventas_mes,
        total_vendido_mes,
        total_ventas,
        total_general,
        total_efectivo,
        total_transferencia,
        productos_distintos_vendidos,
        numero_metodos_pago,
        numero_vendedores,
    })
}


/// Resumen de ventas agrupado por producto (sin rango de fechas)
pub fn resumen_ventas_por_producto_logic(conn: &Connection) -> Result<Vec<ResumenVentasProducto>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT 
                dv.id_producto,
                p.nombre,
                p.tipo_producto,
                SUM(dv.cantidad) AS cantidad_vendida,
                SUM(dv.subtotal) AS total_ventas,
                COUNT(DISTINCT v.id_venta) AS numero_ventas,
                COALESCE(GROUP_CONCAT(DISTINCT mp.nombre), '') AS metodos_pago,
                COALESCE(GROUP_CONCAT(DISTINCT u.nombre_completo), '') AS vendedores
            FROM detalle_venta dv
            INNER JOIN ventas v ON dv.id_venta = v.id_venta
            INNER JOIN productos p ON dv.id_producto = p.id_producto
            INNER JOIN metodos_pago mp ON dv.metodo_pago = mp.id_metodo
            INNER JOIN usuarios u ON v.id_usuario = u.id_usuario
            GROUP BY dv.id_producto
            ORDER BY cantidad_vendida DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(ResumenVentasProducto {
                id_producto: row.get(0)?,
                nombre_producto: row.get(1)?,
                tipo_producto: row.get(2)?,
                cantidad_vendida: row.get(3)?,
                total_ventas: row.get(4)?,
                numero_ventas: row.get(5)?,
                metodos_pago: row.get(6)?,
                vendedores: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}


/// Ventas agrupadas por método de pago (total general, sin rango de fechas)
pub fn ventas_por_metodo_pago_total_logic(conn: &Connection) -> Result<Vec<VentasPorMetodoPagoTotal>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT 
                mp.id_metodo,
                mp.nombre,
                COUNT(dv.id_detalle) AS cantidad_lineas,
                COALESCE(SUM(dv.subtotal), 0) AS total
            FROM detalle_venta dv
            INNER JOIN metodos_pago mp ON dv.metodo_pago = mp.id_metodo
            GROUP BY mp.id_metodo
            ORDER BY total DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(VentasPorMetodoPagoTotal {
                id_metodo: row.get(0)?,
                nombre_metodo: row.get(1)?,
                cantidad_lineas: row.get(2)?,
                total: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Resumen diario de ventas desde la vista del sistema.
pub fn resumen_ventas_diario_logic(conn: &Connection) -> Result<Vec<ResumenVentasDiario>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT fecha, numero_ventas, total_efectivo, total_transferencia, total_general
               FROM vista_resumen_ventas_diario
               ORDER BY fecha DESC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(ResumenVentasDiario {
                fecha: row.get(0)?,
                numero_ventas: row.get(1)?,
                total_efectivo: row.get(2)?,
                total_transferencia: row.get(3)?,
                total_general: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Resumen de ventas diario filtrado por rango de fechas (YYYY-MM-DD). Si no hay fechas, retorna todos.
pub fn resumen_ventas_diario_rango_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<ResumenVentasDiario>, String> {
    let (sql, params_vec): (String, Vec<String>) = if fecha_inicio.is_empty() || fecha_fin.is_empty() {
        (
            r#"SELECT fecha, numero_ventas, total_efectivo, total_transferencia, total_general
               FROM vista_resumen_ventas_diario
               ORDER BY fecha DESC"#.to_string(),
            vec![]
        )
    } else {
        (
            r#"SELECT fecha, numero_ventas, total_efectivo, total_transferencia, total_general
               FROM vista_resumen_ventas_diario
               WHERE DATE(fecha) BETWEEN DATE(?1) AND DATE(?2)
               ORDER BY fecha DESC"#.to_string(),
            vec![fecha_inicio.to_string(), fecha_fin.to_string()]
        )
    };

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let rows = stmt.query_map(rusqlite::params_from_iter(params_vec.iter()), |row| {
        Ok(ResumenVentasDiario {
            fecha: row.get(0)?,
            numero_ventas: row.get(1)?,
            total_efectivo: row.get(2)?,
            total_transferencia: row.get(3)?,
            total_general: row.get(4)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Totales consolidados de ventas en un rango de fechas. Si no hay fechas, retorna todos.
pub fn resumen_ventas_rango_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<ResumenVentasRango, String> {
    let (sql, params_vec): (String, Vec<String>) = if fecha_inicio.is_empty() || fecha_fin.is_empty() {
        (
            r#"SELECT COUNT(DISTINCT v.id_venta),
                  COALESCE(SUM(CASE WHEN mp.nombre = 'EFECTIVO' THEN dv.subtotal ELSE 0 END), 0),
                  COALESCE(SUM(CASE WHEN mp.nombre = 'TRANSFERENCIA' THEN dv.subtotal ELSE 0 END), 0),
                  COALESCE(SUM(dv.subtotal), 0)
           FROM ventas v
           INNER JOIN detalle_venta dv ON v.id_venta = dv.id_venta
           INNER JOIN metodos_pago mp ON dv.metodo_pago = mp.id_metodo"#.to_string(),
            vec![]
        )
    } else {
        (
            r#"SELECT COUNT(DISTINCT v.id_venta),
                  COALESCE(SUM(CASE WHEN mp.nombre = 'EFECTIVO' THEN dv.subtotal ELSE 0 END), 0),
                  COALESCE(SUM(CASE WHEN mp.nombre = 'TRANSFERENCIA' THEN dv.subtotal ELSE 0 END), 0),
                  COALESCE(SUM(dv.subtotal), 0)
           FROM ventas v
           INNER JOIN detalle_venta dv ON v.id_venta = dv.id_venta
           INNER JOIN metodos_pago mp ON dv.metodo_pago = mp.id_metodo
           WHERE DATE(v.fecha) BETWEEN DATE(?1) AND DATE(?2)"#.to_string(),
            vec![fecha_inicio.to_string(), fecha_fin.to_string()]
        )
    };

    conn.query_row(
        &sql,
        rusqlite::params_from_iter(params_vec.iter()),
        |row| {
            Ok(ResumenVentasRango {
                fecha_inicio: fecha_inicio.to_string(),
                fecha_fin: fecha_fin.to_string(),
                numero_ventas: row.get(0)?,
                total_efectivo: row.get(1)?,
                total_transferencia: row.get(2)?,
                total_general: row.get(3)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

/// Top de productos más vendidos en un rango (por cantidad). Si no hay fechas, retorna todos.
pub fn productos_mas_vendidos_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
    limite: i32,
) -> Result<Vec<ProductoMasVendido>, String> {
    let limite = limite.max(1).min(100);

    let (sql, params_vec): (String, Vec<String>) = if fecha_inicio.is_empty() || fecha_fin.is_empty() {
        (
            r#"SELECT dv.id_producto, p.nombre, p.tipo_producto,
                      SUM(dv.cantidad) AS cantidad_vendida,
                      SUM(dv.subtotal) AS total_ventas,
                      COALESCE(GROUP_CONCAT(DISTINCT mp.nombre), '') AS metodo_pago,
                      COALESCE(GROUP_CONCAT(DISTINCT u.nombre_completo), '') AS vendedor
               FROM detalle_venta dv
               INNER JOIN ventas v ON dv.id_venta = v.id_venta
               INNER JOIN productos p ON dv.id_producto = p.id_producto
               INNER JOIN metodos_pago mp ON dv.metodo_pago = mp.id_metodo
               INNER JOIN usuarios u ON v.id_usuario = u.id_usuario
               GROUP BY dv.id_producto
               ORDER BY cantidad_vendida DESC
               LIMIT ?1"#.to_string(),
            vec![limite.to_string()]
        )
    } else {
        (
            r#"SELECT dv.id_producto, p.nombre, p.tipo_producto,
                      SUM(dv.cantidad) AS cantidad_vendida,
                      SUM(dv.subtotal) AS total_ventas,
                      COALESCE(GROUP_CONCAT(DISTINCT mp.nombre), '') AS metodo_pago,
                      COALESCE(GROUP_CONCAT(DISTINCT u.nombre_completo), '') AS vendedor
               FROM detalle_venta dv
               INNER JOIN ventas v ON dv.id_venta = v.id_venta
               INNER JOIN productos p ON dv.id_producto = p.id_producto
               INNER JOIN metodos_pago mp ON dv.metodo_pago = mp.id_metodo
               INNER JOIN usuarios u ON v.id_usuario = u.id_usuario
               WHERE DATE(v.fecha) BETWEEN DATE(?1) AND DATE(?2)
               GROUP BY dv.id_producto
               ORDER BY cantidad_vendida DESC
               LIMIT ?3"#.to_string(),
            vec![fecha_inicio.to_string(), fecha_fin.to_string(), limite.to_string()]
        )
    };

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let rows = stmt.query_map(rusqlite::params_from_iter(params_vec.iter()), |row| {
        Ok(ProductoMasVendido {
            id_producto: row.get(0)?,
            nombre_producto: row.get(1)?,
            tipo_producto: row.get(2)?,
            cantidad_vendida: row.get(3)?,
            total_ventas: row.get(4)?,
            metodo_pago: row.get(5)?,
            vendedor: row.get(6)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Productos con stock bajo (vista `vista_productos_stock_bajo`).
pub fn reporte_stock_bajo_logic(conn: &Connection) -> Result<Vec<ReporteStockBajo>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT id_producto, nombre, tipo_producto,
                      stock_actual, stock_maximo, stock_minimo, porcentaje_stock
               FROM vista_productos_stock_bajo
               ORDER BY porcentaje_stock ASC"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(ReporteStockBajo {
                id_producto: row.get(0)?,
                nombre: row.get(1)?,
                tipo_producto: row.get(2)?,
                stock_actual: row.get(3)?,
                stock_maximo: row.get(4)?,
                stock_minimo: row.get(5)?,
                porcentaje_stock: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Inventario actual de productos activos con stock.
pub fn reporte_inventario_logic(conn: &Connection) -> Result<Vec<ReporteInventario>, String> {
    let mut stmt = conn
        .prepare(
            r#"SELECT p.id_producto, p.nombre, p.tipo_producto,
                      s.stock_actual, s.stock_maximo, s.stock_minimo,
                      p.precio_costo, p.precio_sugerido
               FROM productos p
               INNER JOIN stock s ON p.id_producto = s.id_producto
               WHERE p.activo = 1
               ORDER BY p.nombre"#,
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(ReporteInventario {
                id_producto: row.get(0)?,
                nombre: row.get(1)?,
                tipo_producto: row.get(2)?,
                stock_actual: row.get(3)?,
                stock_maximo: row.get(4)?,
                stock_minimo: row.get(5)?,
                precio_costo: row.get(6)?,
                precio_sugerido: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Entradas de inventario agrupadas por producto en un rango de fechas. Si no hay fechas, retorna todos.
pub fn reporte_entradas_rango_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<ReporteEntradasProducto>, String> {
    let (sql, params_vec): (String, Vec<String>) = if fecha_inicio.is_empty() || fecha_fin.is_empty() {
        (
            r#"SELECT me.id_producto, p.nombre,
                      SUM(me.cantidad) AS cantidad_ingresada,
                      COUNT(me.id_movimiento) AS numero_movimientos
               FROM movimientos_entrada me
               INNER JOIN productos p ON me.id_producto = p.id_producto
               GROUP BY me.id_producto
               ORDER BY cantidad_ingresada DESC"#.to_string(),
            vec![]
        )
    } else {
        (
            r#"SELECT me.id_producto, p.nombre,
                      SUM(me.cantidad) AS cantidad_ingresada,
                      COUNT(me.id_movimiento) AS numero_movimientos
               FROM movimientos_entrada me
               INNER JOIN productos p ON me.id_producto = p.id_producto
               WHERE DATE(me.fecha) BETWEEN DATE(?1) AND DATE(?2)
               GROUP BY me.id_producto
               ORDER BY cantidad_ingresada DESC"#.to_string(),
            vec![fecha_inicio.to_string(), fecha_fin.to_string()]
        )
    };

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let rows = stmt.query_map(rusqlite::params_from_iter(params_vec.iter()), |row| {
        Ok(ReporteEntradasProducto {
            id_producto: row.get(0)?,
            nombre_producto: row.get(1)?,
            cantidad_ingresada: row.get(2)?,
            numero_movimientos: row.get(3)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Ventas totales por usuario en un rango de fechas. Si no hay fechas, retorna todos.
pub fn ventas_por_usuario_reporte_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<VentasPorUsuario>, String> {
    let (sql, params_vec): (String, Vec<String>) = if fecha_inicio.is_empty() || fecha_fin.is_empty() {
        (
            r#"SELECT v.id_usuario, u.nombre_completo,
                      COUNT(DISTINCT v.id_venta) AS numero_ventas,
                      COALESCE(SUM(dv.subtotal), 0) AS total_vendido
               FROM ventas v
               INNER JOIN usuarios u ON v.id_usuario = u.id_usuario
               INNER JOIN detalle_venta dv ON v.id_venta = dv.id_venta
               GROUP BY v.id_usuario
               ORDER BY total_vendido DESC"#.to_string(),
            vec![]
        )
    } else {
        (
            r#"SELECT v.id_usuario, u.nombre_completo,
                      COUNT(DISTINCT v.id_venta) AS numero_ventas,
                      COALESCE(SUM(dv.subtotal), 0) AS total_vendido
               FROM ventas v
               INNER JOIN usuarios u ON v.id_usuario = u.id_usuario
               INNER JOIN detalle_venta dv ON v.id_venta = dv.id_venta
               WHERE DATE(v.fecha) BETWEEN DATE(?1) AND DATE(?2)
               GROUP BY v.id_usuario
               ORDER BY total_vendido DESC"#.to_string(),
            vec![fecha_inicio.to_string(), fecha_fin.to_string()]
        )
    };

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let rows = stmt.query_map(rusqlite::params_from_iter(params_vec.iter()), |row| {
        Ok(VentasPorUsuario {
            id_usuario: row.get(0)?,
            nombre_usuario: row.get(1)?,
            numero_ventas: row.get(2)?,
            total_vendido: row.get(3)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Ventas agrupadas por método de pago en un rango de fechas. Si no hay fechas, retorna todos.
pub fn ventas_por_metodo_pago_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<VentasPorMetodoPago>, String> {
    let (sql, params_vec): (String, Vec<String>) = if fecha_inicio.is_empty() || fecha_fin.is_empty() {
        (
            r#"SELECT mp.id_metodo, mp.nombre,
                      COUNT(dv.id_detalle) AS cantidad_lineas,
                      COALESCE(SUM(dv.subtotal), 0) AS total
               FROM detalle_venta dv
               INNER JOIN ventas v ON dv.id_venta = v.id_venta
               INNER JOIN metodos_pago mp ON dv.metodo_pago = mp.id_metodo
               GROUP BY mp.id_metodo
               ORDER BY total DESC"#.to_string(),
            vec![]
        )
    } else {
        (
            r#"SELECT mp.id_metodo, mp.nombre,
                      COUNT(dv.id_detalle) AS cantidad_lineas,
                      COALESCE(SUM(dv.subtotal), 0) AS total
               FROM detalle_venta dv
               INNER JOIN ventas v ON dv.id_venta = v.id_venta
               INNER JOIN metodos_pago mp ON dv.metodo_pago = mp.id_metodo
               WHERE DATE(v.fecha) BETWEEN DATE(?1) AND DATE(?2)
               GROUP BY mp.id_metodo
               ORDER BY total DESC"#.to_string(),
            vec![fecha_inicio.to_string(), fecha_fin.to_string()]
        )
    };

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let rows = stmt.query_map(rusqlite::params_from_iter(params_vec.iter()), |row| {
        Ok(VentasPorMetodoPago {
            id_metodo: row.get(0)?,
            nombre_metodo: row.get(1)?,
            cantidad_lineas: row.get(2)?,
            total: row.get(3)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// Cajas registradas en un rango de fechas de apertura. Si no hay fechas, retorna todas.
pub fn reporte_cajas_rango_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<ReporteCaja>, String> {
    let (sql, params_vec): (String, Vec<String>) = if fecha_inicio.is_empty() || fecha_fin.is_empty() {
        (
            r#"SELECT c.id_caja, c.fecha_apertura, c.fecha_cierre, c.estado,
                      c.monto_apertura, c.monto_cierre, c.total_efectivo,
                      c.total_transferencia, u.nombre_completo
               FROM caja c
               INNER JOIN usuarios u ON c.id_usuario_apertura = u.id_usuario
               ORDER BY c.fecha_apertura DESC"#.to_string(),
            vec![]
        )
    } else {
        (
            r#"SELECT c.id_caja, c.fecha_apertura, c.fecha_cierre, c.estado,
                      c.monto_apertura, c.monto_cierre, c.total_efectivo,
                      c.total_transferencia, u.nombre_completo
               FROM caja c
               INNER JOIN usuarios u ON c.id_usuario_apertura = u.id_usuario
               WHERE DATE(c.fecha_apertura) BETWEEN DATE(?1) AND DATE(?2)
               ORDER BY c.fecha_apertura DESC"#.to_string(),
            vec![fecha_inicio.to_string(), fecha_fin.to_string()]
        )
    };

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let rows = stmt.query_map(rusqlite::params_from_iter(params_vec.iter()), |row| {
        Ok(ReporteCaja {
            id_caja: row.get(0)?,
            fecha_apertura: row.get(1)?,
            fecha_cierre: row.get(2)?,
            estado: row.get(3)?,
            monto_apertura: row.get(4)?,
            monto_cierre: row.get(5)?,
            total_efectivo: row.get(6)?,
            total_transferencia: row.get(7)?,
            nombre_usuario_apertura: row.get(8)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut lista = Vec::new();
    for item in rows {
        lista.push(item.map_err(|e| e.to_string())?);
    }
    Ok(lista)
}

/// KPIs del día actual para el panel principal.
pub fn dashboard_resumen_logic(conn: &Connection) -> Result<DashboardResumen, String> {
    let ventas_hoy: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM ventas WHERE DATE(fecha) = DATE('now', 'localtime')",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let total_vendido_hoy: f64 = conn
        .query_row(
            r#"SELECT COALESCE(SUM(dv.subtotal), 0)
               FROM detalle_venta dv
               INNER JOIN ventas v ON dv.id_venta = v.id_venta
               WHERE DATE(v.fecha) = DATE('now', 'localtime')"#,
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let productos_stock_bajo: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM vista_productos_stock_bajo",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let notificaciones_pendientes: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM notificaciones WHERE estado = 0",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let entradas_hoy: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM movimientos_entrada WHERE DATE(fecha) = DATE('now', 'localtime')",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let fecha: String = conn
        .query_row(
            "SELECT DATE('now', 'localtime')",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    Ok(DashboardResumen {
        fecha,
        ventas_hoy,
        total_vendido_hoy,
        productos_stock_bajo,
        notificaciones_pendientes,
        entradas_hoy,
    })
}

/// Reporte del margen de ganancia en un rango de fechas. Si no hay fechas, retorna todos.
pub fn reporte_margen_ganancia_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<ReporteMargenGanancia, String> {
    // 1. Obtener totales consolidados
    let (sql_totales, params_vec): (String, Vec<String>) = if fecha_inicio.is_empty() || fecha_fin.is_empty() {
        (
            r#"SELECT 
                  COALESCE(SUM(dv.subtotal), 0.0) AS total_ventas,
                  COALESCE(SUM(dv.cantidad * p.precio_costo), 0.0) AS total_costo
               FROM detalle_venta dv
               INNER JOIN ventas v ON dv.id_venta = v.id_venta
               INNER JOIN productos p ON dv.id_producto = p.id_producto"#.to_string(),
            vec![]
        )
    } else {
        (
            r#"SELECT 
                  COALESCE(SUM(dv.subtotal), 0.0) AS total_ventas,
                  COALESCE(SUM(dv.cantidad * p.precio_costo), 0.0) AS total_costo
               FROM detalle_venta dv
               INNER JOIN ventas v ON dv.id_venta = v.id_venta
               INNER JOIN productos p ON dv.id_producto = p.id_producto
               WHERE DATE(v.fecha) BETWEEN DATE(?1) AND DATE(?2)"#.to_string(),
            vec![fecha_inicio.to_string(), fecha_fin.to_string()]
        )
    };

    let (total_ventas, total_costo): (f64, f64) = conn.query_row(
        &sql_totales,
        rusqlite::params_from_iter(params_vec.iter()),
        |row| {
            Ok((row.get(0)?, row.get(1)?))
        },
    ).map_err(|e| e.to_string())?;

    let ganancia_neta = total_ventas - total_costo;
    let margen_porcentaje = if total_ventas > 0.0 {
        (ganancia_neta / total_ventas) * 100.0
    } else {
        0.0
    };

    // 2. Obtener desglose por producto
    let sql_productos = if fecha_inicio.is_empty() || fecha_fin.is_empty() {
        r#"SELECT 
              p.id_producto,
              p.nombre,
              SUM(dv.cantidad) AS cantidad_vendida,
              SUM(dv.subtotal) AS total_ventas_prod,
              SUM(dv.cantidad * p.precio_costo) AS total_costo_prod
           FROM detalle_venta dv
           INNER JOIN ventas v ON dv.id_venta = v.id_venta
           INNER JOIN productos p ON dv.id_producto = p.id_producto
           GROUP BY p.id_producto, p.nombre
           ORDER BY total_ventas_prod DESC"#.to_string()
    } else {
        r#"SELECT 
              p.id_producto,
              p.nombre,
              SUM(dv.cantidad) AS cantidad_vendida,
              SUM(dv.subtotal) AS total_ventas_prod,
              SUM(dv.cantidad * p.precio_costo) AS total_costo_prod
           FROM detalle_venta dv
           INNER JOIN ventas v ON dv.id_venta = v.id_venta
           INNER JOIN productos p ON dv.id_producto = p.id_producto
           WHERE DATE(v.fecha) BETWEEN DATE(?1) AND DATE(?2)
           GROUP BY p.id_producto, p.nombre
           ORDER BY total_ventas_prod DESC"#.to_string()
    };

    let mut stmt = conn.prepare(&sql_productos).map_err(|e| e.to_string())?;

    let rows = stmt.query_map(rusqlite::params_from_iter(params_vec.iter()), |row| {
        let id_producto: i32 = row.get(0)?;
        let nombre_producto: String = row.get(1)?;
        let cantidad_vendida: i32 = row.get(2)?;
        let total_ventas_prod: f64 = row.get(3)?;
        let total_costo_prod: f64 = row.get(4)?;
        let ganancia_neta_prod = total_ventas_prod - total_costo_prod;
        let margen_porcentaje_prod = if total_ventas_prod > 0.0 {
            (ganancia_neta_prod / total_ventas_prod) * 100.0
        } else {
            0.0
        };

        Ok(DetalleMargenProducto {
            id_producto,
            nombre_producto,
            cantidad_vendida,
            total_ventas: total_ventas_prod,
            total_costo: total_costo_prod,
            ganancia_neta: ganancia_neta_prod,
            margen_porcentaje: margen_porcentaje_prod,
        })
    }).map_err(|e| e.to_string())?;

    let mut productos = Vec::new();
    for item in rows {
        productos.push(item.map_err(|e| e.to_string())?);
    }

    Ok(ReporteMargenGanancia {
        fecha_inicio: fecha_inicio.to_string(),
        fecha_fin: fecha_fin.to_string(),
        total_ventas,
        total_costo,
        ganancia_neta,
        margen_porcentaje,
        productos,
    })
}

// ==============================================
// VENTAS POR TURNO
// ==============================================

/// Resumen de ventas agrupado por turno
pub fn ventas_por_turno_logic(conn: &Connection, solo_abiertos: bool) -> Result<Vec<VentasPorTurno>, String> {
    let estado_filtro = if solo_abiertos {
        "AND t.estado = 'ABIERTO'"
    } else {
        ""
    };
    
    let query = format!(
        r#"
        SELECT 
            t.id_turno,
            tt.nombre as tipo_turno,
            u.nombre_completo as usuario,
            t.fecha_inicio,
            t.fecha_fin,
            COALESCE(SUM(dv.subtotal), 0) as total_ventas,
            COALESCE(SUM(CASE WHEN mp.nombre = 'EFECTIVO' THEN dv.subtotal ELSE 0 END), 0) as total_efectivo,
            COALESCE(SUM(CASE WHEN mp.nombre = 'TRANSFERENCIA' THEN dv.subtotal ELSE 0 END), 0) as total_transferencia,
            COUNT(DISTINCT v.id_venta) as numero_ventas,
            COUNT(DISTINCT dv.id_detalle) as numero_productos_vendidos
        FROM turnos t
        LEFT JOIN tipos_turno tt ON t.id_tipo_turno = tt.id_tipo_turno
        LEFT JOIN usuarios u ON t.id_usuario = u.id_usuario
        LEFT JOIN ventas v ON v.id_turno = t.id_turno
        LEFT JOIN detalle_venta dv ON dv.id_venta = v.id_venta
        LEFT JOIN metodos_pago mp ON dv.metodo_pago = mp.id_metodo
        WHERE 1=1 {}
        GROUP BY t.id_turno
        ORDER BY t.fecha_inicio DESC
        "#,
        estado_filtro
    );
    
    let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| {
        Ok(VentasPorTurno {
            id_turno: row.get(0)?,
            tipo_turno: row.get(1)?,
            usuario: row.get(2)?,
            fecha_inicio: row.get(3)?,
            fecha_fin: row.get(4)?,
            total_ventas: row.get(5)?,
            total_efectivo: row.get(6)?,
            total_transferencia: row.get(7)?,
            numero_ventas: row.get(8)?,
            numero_productos_vendidos: row.get(9)?,
        })
    }).map_err(|e| e.to_string())?;
    
    let mut resultados = Vec::new();
    for row in rows {
        resultados.push(row.map_err(|e| e.to_string())?);
    }
    
    Ok(resultados)
}

/// Detalle de ventas de un turno específico
pub fn ventas_por_turno_detalle_logic(conn: &Connection, id_turno: i32) -> Result<Vec<VentaDetallePorTurno>, String> {
    let mut stmt = conn.prepare(
        r#"
        SELECT 
            v.id_venta,
            v.fecha,
            u.nombre_completo as vendedor,
            p.nombre as producto,
            dv.cantidad,
            dv.precio_unitario,
            dv.subtotal,
            mp.nombre as metodo_pago,
            c.id_caja,
            c.monto_apertura as caja_inicial,
            c.fecha_apertura as caja_inicial_hora,
            c.monto_cierre as caja_final,
            c.fecha_cierre as caja_final_hora,
            (c.monto_cierre + c.monto_apertura) as caja_total,
            c.total_efectivo,
            c.total_transferencia,
            (c.total_efectivo + c.total_transferencia) as total_final
        FROM ventas v
        INNER JOIN turnos t ON v.id_turno = t.id_turno
        INNER JOIN usuarios u ON v.id_usuario = u.id_usuario
        INNER JOIN detalle_venta dv ON v.id_venta = dv.id_venta
        INNER JOIN productos p ON dv.id_producto = p.id_producto
        INNER JOIN metodos_pago mp ON dv.metodo_pago = mp.id_metodo
        INNER JOIN caja c ON v.id_caja = c.id_caja
        WHERE v.id_turno = ?1
        ORDER BY v.fecha DESC
        "#
    ).map_err(|e| e.to_string())?;
    
    let rows = stmt.query_map([id_turno], |row| {
        Ok(VentaDetallePorTurno {
            id_venta: row.get(0)?,
            fecha: row.get(1)?,
            vendedor: row.get(2)?,
            producto: row.get(3)?,
            cantidad: row.get(4)?,
            precio_unitario: row.get(5)?,
            subtotal: row.get(6)?,
            metodo_pago: row.get(7)?,
            id_caja: row.get(8)?,
            caja_inicial: row.get(9)?,
            caja_inicial_hora: row.get(10)?,
            caja_final: row.get(11)?,
            caja_final_hora: row.get(12)?,
            caja_total: row.get(13)?,
            total_efectivo: row.get(14)?,
            total_transferencia: row.get(15)?,
            total_final: row.get(16)?,
        })
    }).map_err(|e| e.to_string())?;
    
    let mut resultados = Vec::new();
    for row in rows {
        resultados.push(row.map_err(|e| e.to_string())?);
    }
    
    Ok(resultados)
}

/// Obtener las ventas del turno actual (el que está abierto)
pub fn ventas_del_turno_actual_logic(conn: &Connection) -> Result<Vec<VentasPorTurno>, String> {
    // Primero obtener el turno activo
    let turno_activo = crate::commands::turnos::logic::obtener_turno_activo_general_logic(conn)?;
    
    if let Some(turno) = turno_activo {
        // Si hay turno activo, obtener sus ventas
        ventas_por_turno_logic(conn, false)
            .map(|ventas| ventas.into_iter().filter(|v| v.id_turno == turno.id_turno).collect())
    } else {
        Ok(Vec::new())
    }
}

/// Obtener reporte consolidado de ventas: productos más vendidos, métodos de pago y vendedor
pub fn reporte_consolidado_ventas_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
    limite_productos: i32,
) -> Result<ReporteConsolidadoVentas, String> {
    let productos_mas_vendidos = productos_mas_vendidos_logic(conn, fecha_inicio, fecha_fin, limite_productos)?;
    let metodos_pago = ventas_por_metodo_pago_logic(conn, fecha_inicio, fecha_fin)?;
    let ventas_por_vendedor = ventas_por_usuario_reporte_logic(conn, fecha_inicio, fecha_fin)?;

    Ok(ReporteConsolidadoVentas {
        fecha_inicio: fecha_inicio.to_string(),
        fecha_fin: fecha_fin.to_string(),
        productos_mas_vendidos,
        metodos_pago,
        ventas_por_vendedor,
    })
}

/// Reporte detallado de ventas por rango de fechas. Si no hay fechas, retorna todos.
pub fn reporte_ventas_detallado_logic(
    conn: &Connection,
    fecha_inicio: &str,
    fecha_fin: &str,
) -> Result<Vec<ReporteVentasDetallado>, String> {
    let (sql, params_vec): (String, Vec<String>) = if fecha_inicio.is_empty() || fecha_fin.is_empty() {
        (
            r#"SELECT 
                v.id_venta,
                v.fecha,
                u.nombre_completo as vendedor,
                p.nombre as producto,
                dv.cantidad,
                dv.precio_unitario,
                dv.subtotal,
                mp.nombre as metodo_pago,
                c.id_caja,
                c.monto_apertura as caja_inicial_valor,
                c.fecha_apertura as caja_inicial_hora,
                c.monto_cierre as caja_final_valor,
                c.fecha_cierre as caja_final_hora,
                c.total_efectivo,
                c.total_transferencia,
                (c.total_efectivo + c.total_transferencia) as total_final,
                (c.monto_cierre + c.monto_apertura) as caja_total
            FROM ventas v
            INNER JOIN usuarios u ON v.id_usuario = u.id_usuario
            INNER JOIN detalle_venta dv ON v.id_venta = dv.id_venta
            INNER JOIN productos p ON dv.id_producto = p.id_producto
            INNER JOIN metodos_pago mp ON dv.metodo_pago = mp.id_metodo
            INNER JOIN caja c ON v.id_caja = c.id_caja
            ORDER BY v.fecha DESC"#.to_string(),
            vec![]
        )
    } else {
        (
            r#"SELECT 
                v.id_venta,
                v.fecha,
                u.nombre_completo as vendedor,
                p.nombre as producto,
                dv.cantidad,
                dv.precio_unitario,
                dv.subtotal,
                mp.nombre as metodo_pago,
                c.id_caja,
                c.monto_apertura as caja_inicial_valor,
                c.fecha_apertura as caja_inicial_hora,
                c.monto_cierre as caja_final_valor,
                c.fecha_cierre as caja_final_hora,
                c.total_efectivo,
                c.total_transferencia,
                (c.total_efectivo + c.total_transferencia) as total_final,
                (c.monto_cierre + c.monto_apertura) as caja_total
            FROM ventas v
            INNER JOIN usuarios u ON v.id_usuario = u.id_usuario
            INNER JOIN detalle_venta dv ON v.id_venta = dv.id_venta
            INNER JOIN productos p ON dv.id_producto = p.id_producto
            INNER JOIN metodos_pago mp ON dv.metodo_pago = mp.id_metodo
            INNER JOIN caja c ON v.id_caja = c.id_caja
            WHERE DATE(v.fecha) BETWEEN DATE(?1) AND DATE(?2)
            ORDER BY v.fecha DESC"#.to_string(),
            vec![fecha_inicio.to_string(), fecha_fin.to_string()]
        )
    };

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;

    let rows = stmt.query_map(rusqlite::params_from_iter(params_vec.iter()), |row| {
        Ok(ReporteVentasDetallado {
            id_venta: row.get(0)?,
            fecha: row.get(1)?,
            vendedor: row.get(2)?,
            producto: row.get(3)?,
            cantidad: row.get(4)?,
            precio_unitario: row.get(5)?,
            subtotal: row.get(6)?,
            metodo_pago: row.get(7)?,
            id_caja: row.get(8)?,
            caja_inicial_valor: row.get(9)?,
            caja_inicial_hora: row.get(10)?,
            caja_final_valor: row.get(11)?,
            caja_final_hora: row.get(12)?,
            total_efectivo: row.get(13)?,
            total_transferencia: row.get(14)?,
            total_final: row.get(15)?,
            caja_total: row.get(16)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut resultados = Vec::new();
    for row in rows {
        resultados.push(row.map_err(|e| e.to_string())?);
    }
    Ok(resultados)
}