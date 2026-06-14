//! Tests del módulo de reportes
//! Ejecutar con: cargo test --test reportes_test -- --nocapture

use system_muscle_lib::commands::*;
use system_muscle_lib::models::movimientos_entrada::movimiento_entrada::NuevoMovimientoEntrada;
use system_muscle_lib::models::productos::producto::NuevoProducto;
use system_muscle_lib::models::ventas::venta::{LineaVenta, NuevaVenta};
use system_muscle_lib::services::db::connection::get_db_connection;
use std::time::{SystemTime, UNIX_EPOCH};

fn nombre_unico(prefijo: &str) -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("{} {}", prefijo, ts)
}

fn crear_caja_abierta(conn: &rusqlite::Connection) -> i32 {
    conn.execute(
        r#"INSERT INTO caja (fecha_apertura, monto_apertura, estado, id_usuario_apertura)
           VALUES (datetime('now'), 0, 'ABIERTA', 1)"#,
        [],
    )
    .unwrap();
    conn.last_insert_rowid() as i32
}

fn limpiar_caja(conn: &rusqlite::Connection, id_caja: i32) {
    let _ = conn.execute("DELETE FROM movimientos_caja WHERE id_caja = ?1", [id_caja]);
    let _ = conn.execute("DELETE FROM ventas WHERE id_caja = ?1", [id_caja]);
    let _ = conn.execute("DELETE FROM caja WHERE id_caja = ?1", [id_caja]);
}

fn crear_producto_con_stock(conn: &rusqlite::Connection, nombre: &str, stock_inicial: i32) -> i32 {
    let id = crear_producto_logic(
        conn,
        &NuevoProducto {
            nombre: nombre.to_string(),
            tipo_producto: "SNACKS".to_string(),
            precio_costo: 5000.0,
            precio_sugerido: 8000.0,
            imagen_url: None,
            stock_maximo: 100,
        },
    )
    .unwrap();

    if stock_inicial > 0 {
        registrar_entrada_logic(
            conn,
            &NuevoMovimientoEntrada {
                id_producto: id,
                cantidad: stock_inicial,
                id_usuario: 1,
            },
        )
        .unwrap();
    }
    id
}

fn limpiar_producto(conn: &rusqlite::Connection, id_producto: i32) {
    let _ = conn.execute("DELETE FROM detalle_venta WHERE id_producto = ?1", [id_producto]);
    let _ = conn.execute("DELETE FROM movimientos_entrada WHERE id_producto = ?1", [id_producto]);
    let _ = conn.execute("DELETE FROM stock WHERE id_producto = ?1", [id_producto]);
    let _ = conn.execute("DELETE FROM productos WHERE id_producto = ?1", [id_producto]);
}

#[test]
fn test_dashboard_resumen() {
    println!("\n📊 TEST: Dashboard resumen");
    let conn = get_db_connection().unwrap();
    let dashboard = dashboard_resumen_logic(&conn).unwrap();
    assert!(!dashboard.fecha.is_empty());
    println!("   ✅ Dashboard fecha: {}", dashboard.fecha);
}

#[test]
fn test_resumen_ventas_rango() {
    println!("\n📊 TEST: Resumen ventas por rango");
    let conn = get_db_connection().unwrap();
    let id_caja = crear_caja_abierta(&conn);
    let nombre = nombre_unico("ReporteTest Venta");
    let id_producto = crear_producto_con_stock(&conn, &nombre, 50);

    registrar_venta_logic(
        &conn,
        &NuevaVenta {
            id_usuario: 1,
            id_caja,
            id_turno: None,
            lineas: vec![LineaVenta {
                id_producto,
                cantidad: 5,
                precio_unitario: 10000.0,
                metodo_pago: 1,
            }],
        },
    )
    .unwrap();

    let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
    let resumen = resumen_ventas_rango_logic(&conn, &hoy, &hoy).unwrap();
    assert!(resumen.numero_ventas >= 1);
    assert!(resumen.total_general >= 50000.0);
    println!("   ✅ Ventas en rango: {} por ${}", resumen.numero_ventas, resumen.total_general);

    limpiar_producto(&conn, id_producto);
    limpiar_caja(&conn, id_caja);
}

#[test]
fn test_productos_mas_vendidos() {
    println!("\n📊 TEST: Productos más vendidos");
    let conn = get_db_connection().unwrap();
    let id_caja = crear_caja_abierta(&conn);
    let nombre = nombre_unico("ReporteTest Top");
    let id_producto = crear_producto_con_stock(&conn, &nombre, 30);

    registrar_venta_logic(
        &conn,
        &NuevaVenta {
            id_usuario: 1,
            id_caja,
            id_turno: None,
            lineas: vec![LineaVenta {
                id_producto,
                cantidad: 8,
                precio_unitario: 5000.0,
                metodo_pago: 2,
            }],
        },
    )
    .unwrap();

    let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
    let top = productos_mas_vendidos_logic(&conn, &hoy, &hoy, 10).unwrap();
    assert!(top.iter().any(|p| p.id_producto == id_producto && p.cantidad_vendida == 8));

    limpiar_producto(&conn, id_producto);
    limpiar_caja(&conn, id_caja);
}

#[test]
fn test_reporte_inventario() {
    println!("\n📊 TEST: Reporte inventario");
    let conn = get_db_connection().unwrap();
    let nombre = nombre_unico("ReporteTest Inv");
    let id = crear_producto_con_stock(&conn, &nombre, 15);

    let inventario = reporte_inventario_logic(&conn).unwrap();
    assert!(inventario.iter().any(|p| p.id_producto == id && p.stock_actual == 15));

    limpiar_producto(&conn, id);
}

#[test]
fn test_reporte_entradas_rango() {
    println!("\n📊 TEST: Reporte entradas por rango");
    let conn = get_db_connection().unwrap();
    let nombre = nombre_unico("ReporteTest Entrada");
    let id = crear_producto_con_stock(&conn, &nombre, 25);

    let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
    let entradas = reporte_entradas_rango_logic(&conn, &hoy, &hoy).unwrap();
    let item = entradas.iter().find(|e| e.id_producto == id).unwrap();
    assert_eq!(item.cantidad_ingresada, 25);

    limpiar_producto(&conn, id);
}

#[test]
fn test_ventas_por_metodo_pago_reporte() {
    println!("\n📊 TEST: Ventas por método de pago");
    let conn = get_db_connection().unwrap();
    let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
    let metodos = ventas_por_metodo_pago_logic(&conn, &hoy, &hoy).unwrap();
    assert!(metodos.iter().all(|m| m.id_metodo == 1 || m.id_metodo == 2));
}

#[test]
fn test_resumen_ventas_diario() {
    println!("\n📊 TEST: Resumen ventas diario (vista)");
    let conn = get_db_connection().unwrap();
    let resumen = resumen_ventas_diario_logic(&conn).unwrap();
    for r in &resumen {
        assert!(r.total_general >= 0.0);
    }
    println!("   ✅ Días con datos: {}", resumen.len());
}

#[test]
fn test_rango_fechas_invalido() {
    println!("\n📊 TEST: Rango de fechas inválido");
    let conn = get_db_connection().unwrap();
    let resultado = resumen_ventas_rango_logic(&conn, "", "2026-01-01");
    assert!(resultado.is_err());
}

#[test]
fn test_reporte_margen_ganancia() {
    println!("\n📊 TEST: Reporte Margen Ganancia");
    let conn = get_db_connection().unwrap();
    let id_caja = crear_caja_abierta(&conn);
    let nombre_prod_1 = nombre_unico("ReporteTest Margen1");
    let nombre_prod_2 = nombre_unico("ReporteTest Margen2");
    
    // El helper crea productos con precio_costo: 5000.0 y precio_sugerido: 8000.0
    let id_prod_1 = crear_producto_con_stock(&conn, &nombre_prod_1, 20);
    let id_prod_2 = crear_producto_con_stock(&conn, &nombre_prod_2, 20);

    // Venta de producto 1: 5 unidades a 10000.0 (Venta = 50000.0, Costo = 25000.0, Ganancia = 25000.0)
    registrar_venta_logic(
        &conn,
        &NuevaVenta {
            id_usuario: 1,
            id_caja,
            id_turno: None,
            lineas: vec![LineaVenta {
                id_producto: id_prod_1,
                cantidad: 5,
                precio_unitario: 10000.0,
                metodo_pago: 1,
            }],
        },
    )
    .unwrap();

    // Venta de producto 2: 2 unidades a 6000.0 (Venta = 12000.0, Costo = 10000.0, Ganancia = 2000.0)
    registrar_venta_logic(
        &conn,
        &NuevaVenta {
            id_usuario: 1,
            id_caja,
            id_turno: None,
            lineas: vec![LineaVenta {
                id_producto: id_prod_2,
                cantidad: 2,
                precio_unitario: 6000.0,
                metodo_pago: 2,
            }],
        },
    )
    .unwrap();

    let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
    let reporte = reporte_margen_ganancia_logic(&conn, &hoy, &hoy).unwrap();

    // Verificaciones consolidadas
    // total_ventas = 50000.0 + 12000.0 = 62000.0
    // total_costo = 25000.0 + 10000.0 = 35000.0
    // ganancia_neta = 27000.0
    // margen_porcentaje = (27000.0 / 62000.0) * 100.0 = 43.548...%
    assert!(reporte.total_ventas >= 62000.0);
    assert!(reporte.total_costo >= 35000.0);
    assert!(reporte.ganancia_neta >= 27000.0);
    assert!(reporte.margen_porcentaje > 43.0 && reporte.margen_porcentaje < 44.0);

    // Verificaciones por producto
    let p1_res = reporte.productos.iter().find(|p| p.id_producto == id_prod_1).unwrap();
    assert_eq!(p1_res.cantidad_vendida, 5);
    assert_eq!(p1_res.total_ventas, 50000.0);
    assert_eq!(p1_res.total_costo, 25000.0);
    assert_eq!(p1_res.ganancia_neta, 25000.0);
    assert_eq!(p1_res.margen_porcentaje, 50.0);

    let p2_res = reporte.productos.iter().find(|p| p.id_producto == id_prod_2).unwrap();
    assert_eq!(p2_res.cantidad_vendida, 2);
    assert_eq!(p2_res.total_ventas, 12000.0);
    assert_eq!(p2_res.total_costo, 10000.0);
    assert_eq!(p2_res.ganancia_neta, 2000.0);
    assert!((p2_res.margen_porcentaje - 16.666).abs() < 0.1);

    // Verificar reporte vacío (fecha lejana)
    let reporte_vacio = reporte_margen_ganancia_logic(&conn, "1990-01-01", "1990-01-01").unwrap();
    assert_eq!(reporte_vacio.total_ventas, 0.0);
    assert_eq!(reporte_vacio.total_costo, 0.0);
    assert_eq!(reporte_vacio.ganancia_neta, 0.0);
    assert_eq!(reporte_vacio.margen_porcentaje, 0.0);
    assert!(reporte_vacio.productos.is_empty());

    println!("   ✅ Reporte de margen de ganancia calculado correctamente");

    // Limpieza
    limpiar_producto(&conn, id_prod_1);
    limpiar_producto(&conn, id_prod_2);
    limpiar_caja(&conn, id_caja);
}

#[test]
fn test_reporte_consolidado_ventas() {
    println!("\n📊 TEST: Reporte Consolidado de Ventas");
    let conn = get_db_connection().unwrap();
    let id_caja = crear_caja_abierta(&conn);
    let nombre_prod = nombre_unico("ReporteTest Consolidado");
    let id_producto = crear_producto_con_stock(&conn, &nombre_prod, 10);

    // Registrar una venta
    registrar_venta_logic(
        &conn,
        &NuevaVenta {
            id_usuario: 1,
            id_caja,
            id_turno: None,
            lineas: vec![LineaVenta {
                id_producto,
                cantidad: 3,
                precio_unitario: 12000.0,
                metodo_pago: 1, // EFECTIVO
            }],
        },
    )
    .unwrap();

    let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
    let reporte = reporte_consolidado_ventas_logic(&conn, &hoy, &hoy, 5).unwrap();

    // Verificaciones
    assert_eq!(reporte.fecha_inicio, hoy);
    assert_eq!(reporte.fecha_fin, hoy);

    // Debe contener el producto más vendido
    let top_prod = reporte.productos_mas_vendidos.iter().find(|p| p.id_producto == id_producto);
    assert!(top_prod.is_some());
    let tp = top_prod.unwrap();
    assert_eq!(tp.cantidad_vendida, 3);
    assert_eq!(tp.total_ventas, 36000.0);

    // Debe contener las ventas por método de pago (EFECTIVO es ID 1)
    let mp = reporte.metodos_pago.iter().find(|m| m.id_metodo == 1);
    assert!(mp.is_some());
    assert!(mp.unwrap().total >= 36000.0);

    // Debe contener las ventas por vendedor (usuario ID 1)
    let vendedor = reporte.ventas_por_vendedor.iter().find(|u| u.id_usuario == 1);
    assert!(vendedor.is_some());
    assert!(vendedor.unwrap().total_vendido >= 36000.0);

    println!("   ✅ Reporte consolidado verificado con éxito");

    // Limpieza
    limpiar_producto(&conn, id_producto);
    limpiar_caja(&conn, id_caja);
}


