import { invoke } from '@tauri-apps/api/core';

import type {
  ProductoVenta,
  NuevaVenta,
  VentaResumen,
  VentaDetallePorTurno,
  DetalleVentaDetalle
} from './sale.types';


// OBTENER PRODUCTOS PARA VENTA
export async function obtenerProductosVenta():

Promise<ProductoVenta[]> {

  return await invoke(
    'obtener_productos_venta'
  );

}


// REGISTRAR VENTA
export async function registrarVenta(
  venta: NuevaVenta
): Promise<number> {

  return await invoke(
    'registrar_venta',
    { venta }
  );

}

// LISTAR VENTAS
export async function listarVentas():
Promise<VentaResumen[]> {

  return await invoke(
    'listar_ventas'
  );
}


// VENTAS POR USUARIO
export async function ventasPorUsuario(
  idUsuario: number
): Promise<VentaResumen[]> {

  return await invoke(
    'ventas_por_usuario',
    { idUsuario }
  );

}

// VENTAS POR TURNO
export async function ventasPorTurno(
  idTurno: number
): Promise<VentaDetallePorTurno[]> {

  return await invoke(
    'ventas_por_turno_detalle',
    {
      idTurno
    }
  );

}

export async function listarDetalleVenta(
  idVenta: number
): Promise<DetalleVentaDetalle[]> {
  return await invoke(
    'listar_detalle_venta',
    {
      idVenta
    }
  );
}



