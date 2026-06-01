export interface NuevoUsuario {
	nombre_completo: string;
	tipo_documento: string;
	numero_documento: string;
	direccion?: string;
	tipo_sangre?: string;
	eps?: string;
	genero?: string;
	correo?: string;
	telefono?: string;
	password: string;
}

export interface Usuario {
	id_usuario: number;
	nombre_completo: string;
	tipo_documento: string;
	numero_documento: string;
	direccion: string;
	tipo_sangre: string;
	eps: string;
	genero: string;
	correo: string;
	telefono: string;
	estado: number;
	id_rol: number;
	fecha_creacion: string;
}