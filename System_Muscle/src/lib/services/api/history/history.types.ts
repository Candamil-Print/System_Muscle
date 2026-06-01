export interface HistorialDetalle {
	id_historial: number;
	usuario: string;
	accion: string;
	tabla_afectada: string;
	id_registro_afectado: number;
	descripcion: string;
	fecha: string;
	hora: string;
	turno: string;
}

export interface FiltroHistorial {
	id_usuario?: number;
	accion?: string;
	fecha_desde?: string;
	fecha_hasta?: string;
	id_turno?: number;

	fecha_inicio?: string;
	fecha_fin?: string;
}

// Interfaz para los datos básicos del backend
export interface HistorialAccionBasico {
	id_historial: number;
	id_usuario: number;
	accion: string;
	tabla_afectada: string;
	id_registro_afectado: number;
	descripcion: string;
	fecha: string;
	hora: string;
	id_turno: number;
}

// Interfaz para usuario
export interface Usuario {
	id_usuario: number;
	nombre_completo: string;
}

// Interfaz para turno
export interface Turno {
	id_turno: number;
	id_tipo_turno: number;
}

export interface TipoTurno {
	id_tipo_turno: number;
	nombre: string;
}
