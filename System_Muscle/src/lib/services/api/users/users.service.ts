import { invoke } from '@tauri-apps/api/core';
import type { NuevoUsuario, Usuario } from './users.types';

export async function crearUsuario(data: NuevoUsuario) {
	return await invoke<number>('crear_usuario', {
		nuevo: data
	});
}

export async function listarUsuarios() {
	return await invoke<Usuario[]>('listar_usuarios', {
		soloActivos: false
	});
}

export async function modificarUsuario(
	id: number,
	data: {
		direccion?: string;
		telefono?: string;
		correo?: string;
	}
) {
	return await invoke('modificar_usuario', {
		id,
		modificacion: data
	});
}

export async function habilitarUsuario(id: number) {
	return invoke('habilitar_usuario', { id });
}

export async function deshabilitarUsuario(id: number) {
	return invoke('deshabilitar_usuario', { id });
}