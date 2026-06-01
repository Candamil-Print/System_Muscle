import { invoke } from '@tauri-apps/api/core';

import type {
	CredencialesLogin,
	SesionUsuario
} from './login.types';

export async function login(
	credenciales: CredencialesLogin
): Promise<SesionUsuario | null> {
	return invoke<SesionUsuario | null>(
		'login',
		{
			credenciales
		}
	);
}

export async function verificarSesion(
	id_usuario: number
): Promise<SesionUsuario | null> {
	return invoke<SesionUsuario | null>(
		'verificar_sesion',
		{
			id_usuario
		}
	);
}