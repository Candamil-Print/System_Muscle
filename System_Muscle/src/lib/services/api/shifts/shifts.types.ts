export interface TipoTurno {
	id_tipo_turno: number;
	nombre: string;
	hora_inicio: string;
	hora_fin: string;
	dias_aplicacion: string;
}

export interface Caja {
  id_caja: number;
  fecha_apertura: string;
  fecha_cierre: string | null;
  monto_apertura: number;
  monto_cierre: number | null;
  total_efectivo: number;
  total_transferencia: number;
  estado: 'ABIERTA' | 'CERRADA';
  id_usuario_apertura: number;
  id_usuario_cierre: number | null;
  id_turno: number;
}