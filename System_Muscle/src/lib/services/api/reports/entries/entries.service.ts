import { invoke } from '@tauri-apps/api/core';

import type {
  MovimientoEntrada,
  MovimientoEntradaDetalle
} from './entries.types';

export async function obtenerMovimiento(
  id: number
): Promise<MovimientoEntrada | null> {
  try {
    return await invoke<MovimientoEntrada>(
      'obtener_movimiento',
      { id }
    );
  } catch (error) {
    console.error(
      'Error obteniendo movimiento:',
      error
    );

    return null;
  }
}

export async function listarMovimientosEntrada(): Promise<
  MovimientoEntradaDetalle[]
> {
  try {
    return await invoke<
      MovimientoEntradaDetalle[]
    >(
      'listar_movimientos_entrada'
    );
  } catch (error) {
    console.error(
      'Error listando movimientos:',
      error
    );

    return [];
  }
}

export async function movimientosPorProducto(
  idProducto: number
): Promise<
  MovimientoEntradaDetalle[]
> {
  try {
    return await invoke<
      MovimientoEntradaDetalle[]
    >(
      'movimientos_por_producto',
      {
        idProducto
      }
    );
  } catch (error) {
    console.error(
      'Error obteniendo movimientos por producto:',
      error
    );

    return [];
  }
}

export async function movimientosPorUsuario(
  idUsuario: number
): Promise<
  MovimientoEntradaDetalle[]
> {
  try {
    return await invoke<
      MovimientoEntradaDetalle[]
    >(
      'movimientos_por_usuario',
      {
        idUsuario
      }
    );
  } catch (error) {
    console.error(
      'Error obteniendo movimientos por usuario:',
      error
    );

    return [];
  }
}

export async function movimientosPorRangoFechas(
  fechaInicio: string,
  fechaFin: string
): Promise<
  MovimientoEntradaDetalle[]
> {
  try {
    return await invoke<
      MovimientoEntradaDetalle[]
    >(
      'movimientos_por_rango_fechas',
      {
        fechaInicio,
        fechaFin
      }
    );
  } catch (error) {
    console.error(
      'Error obteniendo movimientos por rango:',
      error
    );

    return [];
  }
}

export async function totalEntradasPorProducto(
  idProducto: number
): Promise<number> {
  try {
    return await invoke<number>(
      'total_entradas_por_producto',
      {
        idProducto
      }
    );
  } catch (error) {
    console.error(
      'Error obteniendo total de entradas:',
      error
    );

    return 0;
  }
}