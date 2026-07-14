use serde::{Deserialize, Serialize};

/// Resumen de entradas agrupado por producto en un rango de fechas
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResumenEntradasProducto {
    pub id_producto: i32,
    pub nombre_producto: String,
    pub tipo_producto: String,
    pub numero_movimientos: i32,
    pub cantidad_total_ingresada: i32,
    pub primera_entrada: String,
    pub ultima_entrada: String,
}

/// Totales globales de entradas en un rango de fechas
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TotalesEntradas {
    pub fecha_inicio: String,
    pub fecha_fin: String,
    pub numero_movimientos: i32,
    pub cantidad_total_ingresada: i32,
    pub productos_distintos: i32,
}

/// Entradas agrupadas por día en un rango
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntradasPorDia {
    pub fecha: String,
    pub numero_movimientos: i32,
    pub cantidad_total_ingresada: i32,
}

/// Entradas agrupadas por usuario en un rango
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntradasPorUsuario {
    pub id_usuario: i32,
    pub nombre_usuario: String,
    pub numero_movimientos: i32,
    pub cantidad_total_ingresada: i32,
}

/// Entradas agrupadas por tipo de producto en un rango
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntradasPorTipoProducto {
    pub tipo_producto: String,
    pub numero_movimientos: i32,
    pub cantidad_total_ingresada: i32,
}

/// KPIs del módulo de entradas para el dashboard
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DashboardEntradas {
    pub fecha: String,
    pub entradas_hoy: i32,
    pub cantidad_ingresada_hoy: i32,
    pub entradas_semana: i32,
    pub cantidad_ingresada_semana: i32,
    pub productos_con_entradas_hoy: i32,
}

/// Reporte detallado de entradas para exportación / visualización
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReporteEntradaDetallado {
    pub id_movimiento: i32,
    pub fecha: String,
    pub usuario: String,
    pub producto: String,
    pub tipo_producto: String,
    pub cantidad: i32,
}

/// Stock actual y mínimo de un producto
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockActualMinimo {
    pub id_producto: i32,
    pub nombre_producto: String,
    pub tipo_producto: String,
    pub stock_actual: i32,
    pub stock_minimo: i32,
}