export interface MovementDetail {
id_movimiento: number;
id_producto: number;
nombre_producto: string;
tipo_producto: string;
cantidad: number;
fecha: string;
id_usuario: number;
nombre_usuario: string;
stock_anterior: number;
stock_nuevo: number;
}

export interface NewMovement {
id_producto: number;
cantidad: number;
id_usuario: number;
}
