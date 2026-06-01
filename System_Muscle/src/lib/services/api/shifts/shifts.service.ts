import { invoke } from '@tauri-apps/api/core';
import type { TipoTurno, Caja } from './shifts.types';

export async function obtenerTiposTurno(): Promise<TipoTurno[]> {
	return await invoke('obtener_tipos_turno');
}

export async function obtenerCajaActiva(): Promise<Caja | null> {
  try {
    const caja = await invoke<Caja | null>("obtener_caja_activa");
    return caja;
  } catch (error) {
    console.error("Error al obtener caja activa:", error);
    return null;
  }
}

export async function listarCajas(soloAbiertas: boolean = false): Promise<Caja[]> {
  try {
    const cajas = await invoke<Caja[]>("listar_cajas", { soloAbiertas });
    return cajas;
  } catch (error) {
    console.error("Error al listar cajas:", error);
    return [];
  }
}

export async function abrirTurno(idTipoTurno: number): Promise<{ id_turno: number }> {
  const sesion = JSON.parse(localStorage.getItem('sesion') ?? '{}');
  
  const nuevoTurno = {
    id_usuario: sesion.id_usuario,
    id_tipo_turno: idTipoTurno
  };
  
  return await invoke("abrir_turno", { turno: nuevoTurno });
}


export async function obtenerTurnosActivos(): Promise<any[]> {
  const sesion = JSON.parse(localStorage.getItem('sesion') ?? '{}');
  return await invoke("obtener_turnos_activos", { idUsuario: sesion.id_usuario });
}
