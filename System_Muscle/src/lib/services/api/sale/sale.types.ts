export interface ProductoVenta {

  id_producto: number;

  nombre: string;

  categoria: string;

  precio_venta: number;

  stock_actual: number;

  imagen: string;

}

export interface LineaVenta {

  id_producto: number;

  cantidad: number;

  precio_unitario: number;

  metodo_pago: number;

}

export interface NuevaVenta {

  id_usuario: number;

  id_caja: number;

  id_turno?: number;

  lineas: LineaVenta[];

}

export interface VentaResumen {

  id_venta: number;

  fecha: string;

  id_usuario: number;

  nombre_usuario: string;

  id_caja: number;

  total: number;

}

export interface VentaResumen {
  id_venta: number;

  fecha: string;

  id_usuario: number;

  nombre_usuario: string;

  id_caja: number;

  total: number;
}

