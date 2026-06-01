import { invoke } from '@tauri-apps/api/core';
import type { DashboardResumen } from './dashboard.types';

export async function obtenerDashboardResumen(): Promise<DashboardResumen> {
  return await invoke('dashboard_resumen');
}