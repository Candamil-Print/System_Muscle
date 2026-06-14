//! Tests de ventas por turno
//! Ejecutar con: cargo test --test ventas_por_turno_test -- --nocapture

use system_muscle_lib::services::db::connection::get_db_connection;
use system_muscle_lib::commands::reportes::logic::*;
use system_muscle_lib::commands::turnos::logic::*;
use system_muscle_lib::commands::caja::logic::*;
use system_muscle_lib::commands::ventas::logic::*;
use system_muscle_lib::models::turnos::turno::NuevoTurno;
use system_muscle_lib::models::caja::caja::NuevaCaja;
use system_muscle_lib::models::ventas::venta::{NuevaVenta, LineaVenta};

/// Limpiar datos de prueba
fn limpiar_datos_prueba(conn: &rusqlite::Connection) {
    let _ = conn.execute("DELETE FROM ventas WHERE id_venta > 1", []);
    let _ = conn.execute("DELETE FROM detalle_venta WHERE id_detalle > 1", []);
    let _ = conn.execute("DELETE FROM turnos WHERE id_turno > 2", []);
    let _ = conn.execute("DELETE FROM caja WHERE id_caja > 1", []);
    let _ = conn.execute("UPDATE turnos SET estado = 'CERRADO' WHERE estado = 'ABIERTO'", []);
}

/// Obtener un producto válido (que exista en la BD)
fn obtener_producto_valido(conn: &rusqlite::Connection) -> i32 {
    conn.query_row("SELECT id_producto FROM productos WHERE activo = 1 LIMIT 1", [], |row| row.get(0))
        .unwrap_or(1)
}

/// Crear un turno de prueba
fn crear_turno_prueba(conn: &rusqlite::Connection, id_usuario: i32, id_tipo: i32) -> i32 {
    let nuevo = NuevoTurno {
        id_usuario,
        id_tipo_turno: id_tipo,
    };
    abrir_turno_logic(conn, &nuevo).unwrap()
}

/// Crear una caja de prueba
fn crear_caja_prueba(conn: &rusqlite::Connection, id_usuario: i32, id_turno: i32) -> i32 {
    let nueva = NuevaCaja {
        monto_apertura: 500000.0,
        id_usuario_apertura: id_usuario,
        id_turno,
    };
    abrir_caja_logic(conn, &nueva).unwrap()
}

/// Crear una venta de prueba
fn crear_venta_prueba(conn: &rusqlite::Connection, id_usuario: i32, id_caja: i32, id_turno: i32, lineas: Vec<LineaVenta>) -> i32 {
    let nueva_venta = NuevaVenta {
        id_usuario,
        id_caja,
        id_turno: Some(id_turno),
        lineas,
    };
    registrar_venta_logic(conn, &nueva_venta).unwrap()
}

// ==============================================
// TESTS DE VENTAS POR TURNO
// ==============================================

#[test]
fn test_ventas_por_turno() {
    println!("\n📊 TEST: Resumen de ventas por turno");
    let conn = get_db_connection().unwrap();
    
    limpiar_datos_prueba(&conn);
    let id_producto = obtener_producto_valido(&conn);
    println!("   📌 Producto de prueba ID: {}", id_producto);
    
    // 1. Crear un turno
    let id_turno = crear_turno_prueba(&conn, 1, 1);
    println!("   📌 Turno creado con ID: {}", id_turno);
    
    // 2. Crear una caja asociada al turno
    let id_caja = crear_caja_prueba(&conn, 1, id_turno);
    println!("   📌 Caja creada con ID: {}", id_caja);
    
    // 3. Crear una venta asociada al turno
    let lineas = vec![
        LineaVenta {
            id_producto,
            cantidad: 2,
            precio_unitario: 50000.0,
            metodo_pago: 1, // EFECTIVO
        },
        LineaVenta {
            id_producto,
            cantidad: 1,
            precio_unitario: 30000.0,
            metodo_pago: 2, // TRANSFERENCIA
        },
    ];
    
    let id_venta = crear_venta_prueba(&conn, 1, id_caja, id_turno, lineas);
    println!("   📌 Venta creada con ID: {}", id_venta);
    
    // 4. Cerrar el turno
    cerrar_turno_logic(&conn, id_turno).unwrap();
    println!("   📌 Turno cerrado");
    
    // 5. Obtener ventas por turno
    let resultado = ventas_por_turno_logic(&conn, false);
    
    match resultado {
        Ok(ventas) => {
            println!("\n   ✅ Ventas por turno encontradas: {}", ventas.len());
            for v in &ventas {
                println!("\n   📋 Turno ID: {}", v.id_turno);
                println!("      Tipo de turno: {}", v.tipo_turno);
                println!("      Usuario: {}", v.usuario);
                println!("      Fecha inicio: {}", v.fecha_inicio);
                println!("      Fecha fin: {:?}", v.fecha_fin);
                println!("      Total ventas: ${:.2}", v.total_ventas);
                println!("      Total efectivo: ${:.2}", v.total_efectivo);
                println!("      Total transferencia: ${:.2}", v.total_transferencia);
                println!("      Número de ventas: {}", v.numero_ventas);
                println!("      Productos vendidos: {}", v.numero_productos_vendidos);
            }
            assert!(!ventas.is_empty());
        }
        Err(e) => panic!("❌ Error: {}", e),
    }
}

#[test]
fn test_ventas_por_turno_detalle() {
    println!("\n🔍 TEST: Detalle de ventas por turno");
    let conn = get_db_connection().unwrap();
    
    limpiar_datos_prueba(&conn);
    let id_producto = obtener_producto_valido(&conn);
    
    // 1. Crear un turno
    let id_turno = crear_turno_prueba(&conn, 1, 1);
    println!("   📌 Turno creado con ID: {}", id_turno);
    
    // 2. Crear una caja
    let id_caja = crear_caja_prueba(&conn, 1, id_turno);
    
    // 3. Crear ventas
    let lineas1 = vec![
        LineaVenta {
            id_producto,
            cantidad: 2,
            precio_unitario: 50000.0,
            metodo_pago: 1,
        },
    ];
    let id_venta1 = crear_venta_prueba(&conn, 1, id_caja, id_turno, lineas1);
    println!("   📌 Venta 1 creada con ID: {}", id_venta1);
    
    let lineas2 = vec![
        LineaVenta {
            id_producto,
            cantidad: 3,
            precio_unitario: 20000.0,
            metodo_pago: 2,
        },
    ];
    let id_venta2 = crear_venta_prueba(&conn, 1, id_caja, id_turno, lineas2);
    println!("   📌 Venta 2 creada con ID: {}", id_venta2);
    
    // 4. Obtener detalle de ventas del turno
    let resultado = ventas_por_turno_detalle_logic(&conn, id_turno);
    
    match resultado {
        Ok(detalles) => {
            println!("\n   ✅ Detalle encontrado: {} ventas", detalles.len());
            for d in &detalles {
                println!("\n   🧾 Venta ID: {}", d.id_venta);
                println!("      Fecha: {}", d.fecha);
                println!("      Vendedor: {}", d.vendedor);
                println!("      Producto: {}", d.producto);
                println!("      Cantidad: {}", d.cantidad);
                println!("      Precio unitario: ${:.2}", d.precio_unitario);
                println!("      Subtotal: ${:.2}", d.subtotal);
                println!("      Método de pago: {}", d.metodo_pago);
                println!("      Caja ID: {}", d.id_caja);
                println!("      Caja inicial: ${:.2}", d.caja_inicial);
            }
            assert_eq!(detalles.len(), 2);
        }
        Err(e) => panic!("❌ Error: {}", e),
    }
}

#[test]
fn test_ventas_del_turno_actual() {
    println!("\n🟢 TEST: Ventas del turno actual");
    let conn = get_db_connection().unwrap();
    
    limpiar_datos_prueba(&conn);
    let id_producto = obtener_producto_valido(&conn);
    
    // 1. Cerrar cualquier turno abierto
    let _ = conn.execute("UPDATE turnos SET estado = 'CERRADO' WHERE estado = 'ABIERTO'", []);
    
    // 2. Crear un turno abierto (actual)
    let id_turno = crear_turno_prueba(&conn, 1, 1);
    println!("   📌 Turno actual abierto con ID: {}", id_turno);
    
    // 3. Crear una caja
    let id_caja = crear_caja_prueba(&conn, 1, id_turno);
    
    // 4. Crear una venta
    let lineas = vec![
        LineaVenta {
            id_producto,
            cantidad: 2,
            precio_unitario: 50000.0,
            metodo_pago: 1,
        },
    ];
    let id_venta = crear_venta_prueba(&conn, 1, id_caja, id_turno, lineas);
    println!("   📌 Venta creada con ID: {}", id_venta);
    
    // 5. Obtener ventas del turno actual
    let resultado = ventas_del_turno_actual_logic(&conn);
    
    match resultado {
        Ok(ventas) => {
            println!("\n   ✅ Ventas del turno actual: {}", ventas.len());
            for v in &ventas {
                println!("\n   📋 Turno ID: {}", v.id_turno);
                println!("      Tipo de turno: {}", v.tipo_turno);
                println!("      Usuario: {}", v.usuario);
                println!("      Total ventas: ${:.2}", v.total_ventas);
                println!("      Número de ventas: {}", v.numero_ventas);
            }
            assert!(!ventas.is_empty());
        }
        Err(e) => panic!("❌ Error: {}", e),
    }
}