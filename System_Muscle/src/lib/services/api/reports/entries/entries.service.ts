import { invoke } from '@tauri-apps/api/core';

import type {
  ResumenEntradasProducto,
  TotalesEntradas,
  EntradasPorDia,
  EntradasPorUsuario,
  EntradasPorTipoProducto,
  DashboardEntradas,
  ReporteEntradaDetallado,
  StockActualMinimo
} from './entries.types';

/**
 * Resumen de entradas por producto
 */
export async function resumenEntradasPorProducto(
  fechaInicio: string,
  fechaFin: string
): Promise<ResumenEntradasProducto[]> {
  try {
    return await invoke<ResumenEntradasProducto[]>(
      'resumen_entradas_por_producto',
      {
        fechaInicio,
        fechaFin
      }
    );
  } catch (error) {
    console.error(
      'Error obteniendo resumen de entradas por producto:',
      error
    );

    return [];
  }
}

/**
 * Totales de entradas
 */
export async function totalesEntradasRango(
  fechaInicio: string,
  fechaFin: string
): Promise<TotalesEntradas | null> {
  try {
    return await invoke<TotalesEntradas>(
      'totales_entradas_rango',
      {
        fechaInicio,
        fechaFin
      }
    );
  } catch (error) {
    console.error(
      'Error obteniendo totales de entradas:',
      error
    );

    return null;
  }
}

/**
 * Entradas por día
 */
export async function entradasPorDia(
  fechaInicio: string,
  fechaFin: string
): Promise<EntradasPorDia[]> {

  console.log("Antes del invoke");

  const data = await invoke<EntradasPorDia[]>(
    'entradas_por_dia',
    {
      fechaInicio,
      fechaFin
    }
  );

  console.log("Respuesta:", data);

  return data;
}

/**
 * Entradas por usuario
 */
export async function entradasPorUsuario(
  fechaInicio: string,
  fechaFin: string
): Promise<EntradasPorUsuario[]> {
  try {
    return await invoke<EntradasPorUsuario[]>(
      'entradas_por_usuario',
      {
        fechaInicio,
        fechaFin
      }
    );
  } catch (error) {
    console.error(
      'Error obteniendo entradas por usuario:',
      error
    );

    return [];
  }
}

/**
 * Entradas por tipo de producto
 */
export async function entradasPorTipoProducto(
  fechaInicio: string,
  fechaFin: string
): Promise<EntradasPorTipoProducto[]> {
  try {
    return await invoke<EntradasPorTipoProducto[]>(
      'entradas_por_tipo_producto',
      {
        fechaInicio,
        fechaFin
      }
    );
  } catch (error) {
    console.error(
      'Error obteniendo entradas por tipo de producto:',
      error
    );

    return [];
  }
}

/**
 * Dashboard de entradas
 */
export async function dashboardEntradas(): Promise<DashboardEntradas | null> {
  try {
    return await invoke<DashboardEntradas>(
      'dashboard_entradas'
    );
  } catch (error) {
    console.error(
      'Error obteniendo dashboard de entradas:',
      error
    );

    return null;
  }
}

/**
 * Reporte detallado de entradas
 */
export async function reporteEntradaDetallado(
  fechaInicio: string,
  fechaFin: string
): Promise<ReporteEntradaDetallado[]> {
  try {
    const data = await invoke<ReporteEntradaDetallado[]>(
      "reporte_entrada_detallado",
      {
        fechaInicio,
        fechaFin
      }
    );

    return data;
  } catch (error) {
    console.error("Error obteniendo reporte detallado:", error);
    return [];
  }
}

/**
 * Stock actual y mínimo
 */
export async function stockActualYMinimo(): Promise<StockActualMinimo[]> {
  try {
    return await invoke<StockActualMinimo[]>(
      'stock_actual_y_minimo'
    );
  } catch (error) {
    console.error(
      'Error obteniendo stock actual y mínimo:',
      error
    );

    return [];
  }
}