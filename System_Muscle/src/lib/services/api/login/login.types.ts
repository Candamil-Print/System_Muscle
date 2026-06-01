export interface CredencialesLogin {
	documento: string;
	password: string;
}

export interface SesionUsuario {
	id_usuario: number;
	nombre_completo: string;
	tipo_documento: string;
	numero_documento: string;
	correo?: string;
	telefono?: string;
	id_rol: number;
	nombre_rol: string;
	estado: number;
}