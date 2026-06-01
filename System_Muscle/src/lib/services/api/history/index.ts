// src/lib/services/api/history/index.ts
export * from './history.types';
export * from './history.service'; // Si tienes este archivo
// O exporta directamente desde donde está la función
export { listarHistorialConDetalle, obtenerUltimosHistorialConDetalle } from './history.service';