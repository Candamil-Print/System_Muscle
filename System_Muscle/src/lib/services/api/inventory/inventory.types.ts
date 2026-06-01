export interface ProductoConStock {
  id_producto: number;
  nombre: string;
  tipo_producto: string;
  precio_costo: number;
  precio_sugerido: number;
  imagen_url?: string | null;
  fecha_creacion: string;
  activo: number;

  stock_actual?: number | null;
  stock_maximo?: number | null;
  stock_minimo?: number | null;
  fecha_actualizacion_stock?: string | null;
}

export interface NuevoProducto {
  nombre: string;
  tipo_producto: string;
  precio_costo: number;
  precio_sugerido: number;
  imagen_url?: string | null;
  stock_maximo: number;
}