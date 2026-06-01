export interface Notificacion {
	id_notificacion: number;
	id_producto: number;
	nombre_producto: string;
	mensaje: string;
	stock_actual: number;
	stock_minimo: number;
	fecha: string;
	estado: number;
}