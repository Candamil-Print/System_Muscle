//! Tests del módulo de reportes de entrada
//! Ejecutar con: cargo test --test reportes_entrada_test -- --nocapture

use system_muscle_lib::commands::*;
use system_muscle_lib::models::movimientos_entrada::movimiento_entrada::NuevoMovimientoEntrada;
use system_muscle_lib::models::productos::producto::NuevoProducto;
use system_muscle_lib::services::db::connection::get_db_connection;
use std::time::{SystemTime, UNIX_EPOCH};

fn nombre_unico(prefijo: &str) -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("{} {}", prefijo, ts)
}

fn crear_producto_con_stock(conn: &rusqlite::Connection, nombre: &str, cantidad: i32) -> i32 {
    let id = crear_producto_logic(
        conn,
        &NuevoProducto {
            nombre: nombre.to_string(),
            tipo_producto: "SNACKS".to_string(),
            precio_costo: 5000.0,
            precio_sugerido: 8000.0,
            imagen_url: None,
            stock_maximo: 200,
        },
    )
    .unwrap();

    if cantidad > 0 {
        registrar_entrada_logic(
            conn,
            &NuevoMovimientoEntrada {
                id_producto: id,
                cantidad,
                id_usuario: 1,
            },
        )
        .unwrap();
    }
    id
}

fn limpiar_producto(conn: &rusqlite::Connection, id_producto: i32) {
    let _ = conn.execute("DELETE FROM movimientos_entrada WHERE id_producto = ?1", [id_producto]);
    let _ = conn.execute("DELETE FROM stock WHERE id_producto = ?1", [id_producto]);
    let _ = conn.execute("DELETE FROM productos WHERE id_producto = ?1", [id_producto]);
}

// ─── Dashboard ────────────────────────────────────────────────────────────────

#[test]
fn test_dashboard_entradas() {
    println!("\n📊 TEST: Dashboard de entradas");
    let conn = get_db_connection().unwrap();

    let dashboard = dashboard_entradas_logic(&conn).unwrap();

    assert!(!dashboard.fecha.is_empty());
    assert!(dashboard.entradas_hoy >= 0);
    assert!(dashboard.cantidad_ingresada_hoy >= 0);
    assert!(dashboard.entradas_semana >= dashboard.entradas_hoy);
    assert!(dashboard.cantidad_ingresada_semana >= dashboard.cantidad_ingresada_hoy);

    println!("   📅 Fecha: {}", dashboard.fecha);
    println!("   📦 Entradas hoy: {} movimientos / {} unidades", dashboard.entradas_hoy, dashboard.cantidad_ingresada_hoy);
    println!("   📆 Entradas semana: {} movimientos / {} unidades", dashboard.entradas_semana, dashboard.cantidad_ingresada_semana);
    println!("   🏷️  Productos distintos hoy: {}", dashboard.productos_con_entradas_hoy);
    println!("   ✅ Dashboard de entradas OK");
}

// ─── Resumen por producto ─────────────────────────────────────────────────────

#[test]
fn test_resumen_entradas_por_producto() {
    println!("\n📊 TEST: Resumen entradas por producto");
    let conn = get_db_connection().unwrap();

    let nombre = nombre_unico("RptEntrada Prod");
    let id = crear_producto_con_stock(&conn, &nombre, 30);

    let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
    let lista = resumen_entradas_por_producto_logic(&conn, &hoy, &hoy).unwrap();

    let item = lista.iter().find(|e| e.id_producto == id).unwrap();
    assert_eq!(item.cantidad_total_ingresada, 30);
    assert_eq!(item.numero_movimientos, 1);
    assert!(!item.primera_entrada.is_empty());
    assert!(!item.ultima_entrada.is_empty());

    println!("   ✅ Producto '{}': {} unidades en {} movimiento(s)", item.nombre_producto, item.cantidad_total_ingresada, item.numero_movimientos);

    limpiar_producto(&conn, id);
}

#[test]
fn test_resumen_entradas_multiples_movimientos() {
    println!("\n📊 TEST: Resumen con múltiples movimientos al mismo producto");
    let conn = get_db_connection().unwrap();

    let nombre = nombre_unico("RptEntrada Multi");
    let id = crear_producto_logic(
        &conn,
        &NuevoProducto {
            nombre: nombre.clone(),
            tipo_producto: "SUPLEMENTOS".to_string(),
            precio_costo: 10000.0,
            precio_sugerido: 15000.0,
            imagen_url: None,
            stock_maximo: 500,
        },
    )
    .unwrap();

    for cantidad in [10, 20, 15] {
        registrar_entrada_logic(
            &conn,
            &NuevoMovimientoEntrada { id_producto: id, cantidad, id_usuario: 1 },
        )
        .unwrap();
    }

    let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
    let lista = resumen_entradas_por_producto_logic(&conn, &hoy, &hoy).unwrap();
    let item = lista.iter().find(|e| e.id_producto == id).unwrap();

    assert_eq!(item.numero_movimientos, 3);
    assert_eq!(item.cantidad_total_ingresada, 45);

    println!("   ✅ 3 movimientos totalizan 45 unidades");

    limpiar_producto(&conn, id);
}

// ─── Totales globales ─────────────────────────────────────────────────────────

#[test]
fn test_totales_entradas_rango() {
    println!("\n📊 TEST: Totales globales de entradas en rango");
    let conn = get_db_connection().unwrap();

    let nombre = nombre_unico("RptEntrada Total");
    let id = crear_producto_con_stock(&conn, &nombre, 50);

    let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
    let totales = totales_entradas_rango_logic(&conn, &hoy, &hoy).unwrap();

    assert!(totales.numero_movimientos >= 1);
    assert!(totales.cantidad_total_ingresada >= 50);
    assert!(totales.productos_distintos >= 1);
    assert_eq!(totales.fecha_inicio, hoy);
    assert_eq!(totales.fecha_fin, hoy);

    println!("   ✅ Movimientos: {}, Unidades: {}, Productos: {}", totales.numero_movimientos, totales.cantidad_total_ingresada, totales.productos_distintos);

    limpiar_producto(&conn, id);
}

#[test]
fn test_totales_entradas_rango_invalido() {
    println!("\n📊 TEST: Totales con rango inválido");
    let conn = get_db_connection().unwrap();

    let resultado = totales_entradas_rango_logic(&conn, "", "2026-01-01");
    assert!(resultado.is_err());
    println!("   ✅ Error esperado: {}", resultado.unwrap_err());
}

// ─── Entradas por día ─────────────────────────────────────────────────────────

#[test]
fn test_entradas_por_dia() {
    println!("\n📊 TEST: Entradas agrupadas por día");
    let conn = get_db_connection().unwrap();

    let nombre = nombre_unico("RptEntrada Dia");
    let id = crear_producto_con_stock(&conn, &nombre, 20);

    let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
    let por_dia = entradas_por_dia_logic(&conn, &hoy, &hoy).unwrap();

    assert!(!por_dia.is_empty());
    let dia_hoy = por_dia.iter().find(|d| d.fecha == hoy).unwrap();
    assert!(dia_hoy.numero_movimientos >= 1);
    assert!(dia_hoy.cantidad_total_ingresada >= 20);

    println!("   ✅ Hoy ({}): {} movimientos / {} unidades", dia_hoy.fecha, dia_hoy.numero_movimientos, dia_hoy.cantidad_total_ingresada);

    limpiar_producto(&conn, id);
}

// ─── Entradas por usuario ─────────────────────────────────────────────────────

#[test]
fn test_entradas_por_usuario() {
    println!("\n📊 TEST: Entradas agrupadas por usuario");
    let conn = get_db_connection().unwrap();

    let nombre = nombre_unico("RptEntrada Usuario");
    let id = crear_producto_con_stock(&conn, &nombre, 40);

    let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
    let por_usuario = entradas_por_usuario_logic(&conn, &hoy, &hoy).unwrap();

    assert!(!por_usuario.is_empty());
    let usuario = por_usuario.iter().find(|u| u.id_usuario == 1).unwrap();
    assert!(usuario.numero_movimientos >= 1);
    assert!(usuario.cantidad_total_ingresada >= 40);
    assert!(!usuario.nombre_usuario.is_empty());

    println!("   ✅ Usuario '{}': {} movimientos / {} unidades", usuario.nombre_usuario, usuario.numero_movimientos, usuario.cantidad_total_ingresada);

    limpiar_producto(&conn, id);
}

// ─── Entradas por tipo de producto ───────────────────────────────────────────

#[test]
fn test_entradas_por_tipo_producto() {
    println!("\n📊 TEST: Entradas agrupadas por tipo de producto");
    let conn = get_db_connection().unwrap();

    let nombre = nombre_unico("RptEntrada Tipo");
    let id = crear_producto_logic(
        &conn,
        &NuevoProducto {
            nombre: nombre.clone(),
            tipo_producto: "BEBIDAS".to_string(),
            precio_costo: 3000.0,
            precio_sugerido: 5000.0,
            imagen_url: None,
            stock_maximo: 100,
        },
    )
    .unwrap();
    registrar_entrada_logic(
        &conn,
        &NuevoMovimientoEntrada { id_producto: id, cantidad: 25, id_usuario: 1 },
    )
    .unwrap();

    let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();
    let por_tipo = entradas_por_tipo_producto_logic(&conn, &hoy, &hoy).unwrap();

    let bebidas = por_tipo.iter().find(|t| t.tipo_producto == "BEBIDAS").unwrap();
    assert!(bebidas.cantidad_total_ingresada >= 25);

    println!("   ✅ BEBIDAS: {} movimientos / {} unidades", bebidas.numero_movimientos, bebidas.cantidad_total_ingresada);

    limpiar_producto(&conn, id);
}

// ─── Flujo completo ───────────────────────────────────────────────────────────

#[test]
fn test_flujo_completo_reportes_entrada() {
    println!("\n🔄 TEST: Flujo completo del módulo de reportes de entrada");
    let conn = get_db_connection().unwrap();

    // Crear dos productos con múltiples entradas
    let nombre_a = nombre_unico("RptFlujo A");
    let nombre_b = nombre_unico("RptFlujo B");

    let id_a = crear_producto_logic(
        &conn,
        &NuevoProducto {
            nombre: nombre_a.clone(),
            tipo_producto: "SUPLEMENTOS".to_string(),
            precio_costo: 20000.0,
            precio_sugerido: 35000.0,
            imagen_url: None,
            stock_maximo: 300,
        },
    )
    .unwrap();

    let id_b = crear_producto_logic(
        &conn,
        &NuevoProducto {
            nombre: nombre_b.clone(),
            tipo_producto: "BEBIDAS".to_string(),
            precio_costo: 4000.0,
            precio_sugerido: 6000.0,
            imagen_url: None,
            stock_maximo: 200,
        },
    )
    .unwrap();

    println!("   1️⃣ Registrando entradas...");
    for cantidad in [10, 20] {
        registrar_entrada_logic(&conn, &NuevoMovimientoEntrada { id_producto: id_a, cantidad, id_usuario: 1 }).unwrap();
    }
    registrar_entrada_logic(&conn, &NuevoMovimientoEntrada { id_producto: id_b, cantidad: 50, id_usuario: 1 }).unwrap();
    println!("      ✅ Entradas registradas");

    let hoy = chrono::Local::now().format("%Y-%m-%d").to_string();

    println!("   2️⃣ Totales globales...");
    let totales = totales_entradas_rango_logic(&conn, &hoy, &hoy).unwrap();
    assert!(totales.productos_distintos >= 2);
    assert!(totales.cantidad_total_ingresada >= 80);
    println!("      ✅ {} productos / {} unidades / {} movimientos", totales.productos_distintos, totales.cantidad_total_ingresada, totales.numero_movimientos);

    println!("   3️⃣ Resumen por producto...");
    let por_producto = resumen_entradas_por_producto_logic(&conn, &hoy, &hoy).unwrap();
    let item_a = por_producto.iter().find(|e| e.id_producto == id_a).unwrap();
    assert_eq!(item_a.cantidad_total_ingresada, 30);
    assert_eq!(item_a.numero_movimientos, 2);
    println!("      ✅ Producto A: {} uds en {} movimientos", item_a.cantidad_total_ingresada, item_a.numero_movimientos);

    println!("   4️⃣ Entradas por día...");
    let por_dia = entradas_por_dia_logic(&conn, &hoy, &hoy).unwrap();
    assert!(!por_dia.is_empty());
    println!("      ✅ {} días con datos", por_dia.len());

    println!("   5️⃣ Entradas por usuario...");
    let por_usuario = entradas_por_usuario_logic(&conn, &hoy, &hoy).unwrap();
    assert!(!por_usuario.is_empty());
    println!("      ✅ {} usuario(s) con entradas", por_usuario.len());

    println!("   6️⃣ Entradas por tipo de producto...");
    let por_tipo = entradas_por_tipo_producto_logic(&conn, &hoy, &hoy).unwrap();
    assert!(!por_tipo.is_empty());
    println!("      ✅ {} tipo(s) de producto con entradas", por_tipo.len());

    println!("   7️⃣ Dashboard...");
    let dash = dashboard_entradas_logic(&conn).unwrap();
    assert!(dash.entradas_hoy >= 3);
    println!("      ✅ Dashboard OK: {} entradas hoy", dash.entradas_hoy);

    limpiar_producto(&conn, id_a);
    limpiar_producto(&conn, id_b);

    println!("\n   ✅ FLUJO COMPLETO EXITOSO");
}

