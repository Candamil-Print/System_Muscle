export interface DashboardResumen {
  fecha: string;
  ventas_hoy: number;
  total_vendido_hoy: number;
  productos_stock_bajo: number;
  notificaciones_pendientes: number;
  entradas_hoy: number;
}