import { invoke } from '@tauri-apps/api/core';

import type {
	DashboardResumen,
	ResumenVentasDiario,
	ProductoMasVendido,
	ReporteStockBajo,
	ReporteInventario,
	ReporteEntradasProducto,
	VentasPorUsuario,
	VentasPorMetodoPago,
    ResumenVentasRango,
	ReporteCaja,
	ReporteMargenGanancia
} from './reports.types';

// ==========================
// DASHBOARD
// ==========================

export async function obtenerDashboardResumen(): Promise<DashboardResumen> {
	return await invoke('dashboard_resumen');
}

// ==========================
// RESUMEN VENTAS
// ==========================

export async function obtenerResumenVentasDiario(): Promise<ResumenVentasDiario[]> {
	return await invoke('resumen_ventas_diario');
}

export async function obtenerResumenVentasDiarioRango(
	fechaInicio: string,
	fechaFin: string
): Promise<ResumenVentasDiario[]> {
	return await invoke('resumen_ventas_diario_rango', {
		fechaInicio,
		fechaFin
	});
}

// ==========================
// PRODUCTOS MÁS VENDIDOS
// ==========================

export async function obtenerProductosMasVendidos(
	fechaInicio: string,
	fechaFin: string,
	limite = 10
): Promise<ProductoMasVendido[]> {
	return await invoke('productos_mas_vendidos', {
		fechaInicio,
		fechaFin,
		limite
	});
}

// ==========================
// STOCK BAJO
// ==========================

export async function obtenerReporteStockBajo(): Promise<ReporteStockBajo[]> {
	return await invoke('reporte_stock_bajo');
}

// ==========================
// INVENTARIO
// ==========================

export async function obtenerReporteInventario(): Promise<ReporteInventario[]> {
	return await invoke('reporte_inventario');
}

// ==========================
// ENTRADAS
// ==========================

export async function obtenerReporteEntradas(
	fechaInicio: string,
	fechaFin: string
): Promise<ReporteEntradasProducto[]> {
	return await invoke('reporte_entradas_rango', {
		fechaInicio,
		fechaFin
	});
}

// ==========================
// VENTAS POR USUARIO
// ==========================

export async function obtenerVentasPorUsuario(
	fechaInicio: string,
	fechaFin: string
): Promise<VentasPorUsuario[]> {
	return await invoke('ventas_por_usuario_reporte', {
		fechaInicio,
		fechaFin
	});
}

// ==========================
// VENTAS POR MÉTODO DE PAGO
// ==========================

export async function obtenerVentasPorMetodoPago(
	fechaInicio: string,
	fechaFin: string
): Promise<VentasPorMetodoPago[]> {
	return await invoke('ventas_por_metodo_pago_reporte', {
		fechaInicio,
		fechaFin
	});
}

// ==========================
// CAJAS
// ==========================

export async function obtenerReporteCajas(
	fechaInicio: string,
	fechaFin: string
): Promise<ReporteCaja[]> {
	return await invoke('reporte_cajas_rango', {
		fechaInicio,
		fechaFin
	});
}

// ==========================
// RESUMEN VENTAS POR RANGO
// ==========================
export async function obtenerResumenVentasRango(
	fechaInicio: string,
	fechaFin: string
): Promise<ResumenVentasRango> {
	return await invoke('resumen_ventas_rango', {
		fechaInicio,
		fechaFin
	});
}

export async function obtenerMargenGanancia(
	fechaInicio: string,
	fechaFin: string
): Promise<ReporteMargenGanancia> {
	return await invoke('reporte_margen_ganancia', {
		fechaInicio,
		fechaFin
	});
}