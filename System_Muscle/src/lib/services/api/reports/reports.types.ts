export interface DashboardResumen {
	fecha: string;
	ventas_hoy: number;
	total_vendido_hoy: number;
	productos_stock_bajo: number;
	notificaciones_pendientes: number;
	entradas_hoy: number;
}

export interface ResumenVentasDiario {
	fecha: string;
	numero_ventas: number;
	total_efectivo: number;
	total_transferencia: number;
	total_general: number;
}

export interface ProductoMasVendido {
	id_producto: number;
	nombre_producto: string;
	tipo_producto: string;
	cantidad_vendida: number;
	total_ventas: number;
}

export interface ReporteStockBajo {
	id_producto: number;
	nombre: string;
	tipo_producto: string;
	stock_actual: number;
	stock_maximo: number;
	stock_minimo: number;
	porcentaje_stock: number;
}

export interface ReporteInventario {
	id_producto: number;
	nombre: string;
	tipo_producto: string;
	stock_actual: number;
	stock_maximo: number;
	stock_minimo: number;
	precio_costo: number;
	precio_sugerido: number;
}

export interface ReporteEntradasProducto {
	id_producto: number;
	nombre_producto: string;
	cantidad_ingresada: number;
	numero_movimientos: number;
}

export interface VentasPorUsuario {
	id_usuario: number;
	nombre_usuario: string;
	numero_ventas: number;
	total_vendido: number;
}

export interface VentasPorMetodoPago {
	id_metodo: number;
	nombre_metodo: string;
	cantidad_lineas: number;
	total: number;
}

export interface ReporteCaja {
	id_caja: number;
	fecha_apertura: string;
	fecha_cierre: string | null;
	estado: number;
	monto_apertura: number;
	monto_cierre: number;
	total_efectivo: number;
	total_transferencia: number;
	nombre_usuario_apertura: string;
}

export interface ResumenVentasRango {
	fecha_inicio: string;
	fecha_fin: string;
	numero_ventas: number;
	total_efectivo: number;
	total_transferencia: number;
	total_general: number;
}

export interface DetalleMargenProducto {
	id_producto: number;
	nombre_producto: string;
	cantidad_vendida: number;
	total_ventas: number;
	total_costo: number;
	ganancia_neta: number;
	margen_porcentaje: number;
}

export interface ReporteMargenGanancia {
	fecha_inicio: string;
	fecha_fin: string;
	total_ventas: number;
	total_costo: number;
	ganancia_neta: number;
	margen_porcentaje: number;
	productos: DetalleMargenProducto[];
}