import { invoke } from '@tauri-apps/api/core';

import type {
  MovementDetail,
  NewMovement
} from './movements.types';


// LISTAR MOVIMIENTOS
export async function listarMovements(): Promise<MovementDetail[]> {

  return await invoke('listar_movimientos_entrada');

}

// REGISTRAR MOVIMIENTO
export async function crearMovement(
  movement: NewMovement
): Promise<number> {

  return await invoke('registrar_entrada', {
    entrada: movement
  });

}