// Resumen de entradas agrupado por producto
export interface ResumenEntradasProducto {
  id_producto: number;
  nombre_producto: string;
  tipo_producto: string;
  numero_movimientos: number;
  cantidad_total_ingresada: number;
  primera_entrada: string;
  ultima_entrada: string;
}

// Totales globales de entradas
export interface TotalesEntradas {
  fecha_inicio: string;
  fecha_fin: string;
  numero_movimientos: number;
  cantidad_total_ingresada: number;
  productos_distintos: number;
}

// Entradas agrupadas por día
export interface EntradasPorDia {
  fecha: string;
  numero_movimientos: number;
  cantidad_total_ingresada: number;
}

// Entradas agrupadas por usuario
export interface EntradasPorUsuario {
  id_usuario: number;
  nombre_usuario: string;
  numero_movimientos: number;
  cantidad_total_ingresada: number;
}

// Entradas agrupadas por tipo de producto
export interface EntradasPorTipoProducto {
  tipo_producto: string;
  numero_movimientos: number;
  cantidad_total_ingresada: number;
}

// Dashboard de entradas
export interface DashboardEntradas {
  fecha: string;
  entradas_hoy: number;
  cantidad_ingresada_hoy: number;
  entradas_semana: number;
  cantidad_ingresada_semana: number;
  productos_con_entradas_hoy: number;
}

// Reporte detallado de entradas
export interface ReporteEntradaDetallado {
  id_movimiento: number;
  fecha: string;
  usuario: string;
  producto: string;
  tipo_producto: string;
  cantidad: number;
}

// Stock actual y mínimo
export interface StockActualMinimo {
  id_producto: number;
  nombre_producto: string;
  tipo_producto: string;
  stock_actual: number;
  stock_minimo: number;
}