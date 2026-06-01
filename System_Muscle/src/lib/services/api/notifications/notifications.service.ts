import { invoke } from '@tauri-apps/api/core';
import type { Notificacion } from './notifications.type';

export async function listarNotificaciones(
	soloNoLeidas = true
): Promise<Notificacion[]> {
	return await invoke('listar_notificaciones', {
		soloNoLeidas
	});
}

export async function contarNotificacionesNoLeidas(): Promise<number> {
	return await invoke(
		'contar_notificaciones_no_leidas'
	);
}

export async function marcarTodasLeidas() {
	return await invoke<number>(
		'marcar_todas_notificaciones_leidas'
	);
}

export async function limpiarNotificaciones() {
	return await invoke<number>(
		'eliminar_todas_notificaciones'
	);
}