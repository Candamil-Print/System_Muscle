// src/lib/services/api/history/history.api.ts
import { invoke } from "@tauri-apps/api/core";
import type {
	HistorialDetalle,
	FiltroHistorial,
    HistorialAccionBasico,
    Usuario,
    Turno,
    TipoTurno
} from "./history.types";



export async function listarHistorialConDetalle(
	filtro: FiltroHistorial = {}
): Promise<HistorialDetalle[]> {
	try {
		// 1. Obtener el historial básico
		const historialBasico = await invoke<HistorialAccionBasico[]>("listar_historial", { filtro });
		
		if (historialBasico.length === 0) return [];
		
		// 2. Obtener todos los usuarios únicos
		const idsUsuarios = [...new Set(historialBasico.map(h => h.id_usuario))];
		const usuariosPromises = idsUsuarios.map(id => 
			invoke<Usuario>("obtener_usuario", { id }).catch(() => null)
		);
		const usuariosResult = await Promise.all(usuariosPromises);
		const usuariosMap = new Map<number, string>();
		usuariosResult.forEach(usuario => {
			if (usuario) usuariosMap.set(usuario.id_usuario, usuario.nombre_completo);
		});
		
		// 3. Obtener todos los turnos únicos
		const idsTurnos = [...new Set(historialBasico.map(h => h.id_turno).filter(id => id !== 0))];
		const turnosPromises = idsTurnos.map(id => 
			invoke<Turno>("obtener_turno", { id }).catch(() => null)
		);
		const turnosResult = await Promise.all(turnosPromises);
		
		// 4. Obtener tipos de turno
		const tiposTurnoMap = new Map<number, string>();
		for (const turno of turnosResult) {
			if (turno) {
				const tipoTurno = await invoke<TipoTurno>("obtener_tipo_turno", { id: turno.id_tipo_turno })
					.catch(() => null);
				if (tipoTurno) {
					tiposTurnoMap.set(turno.id_turno, tipoTurno.nombre);
				}
			}
		}
		
		// 5. Combinar los datos
		const resultado: HistorialDetalle[] = historialBasico.map(h => ({
			id_historial: h.id_historial,
			usuario: usuariosMap.get(h.id_usuario) || `Usuario ${h.id_usuario}`,
			accion: h.accion,
			tabla_afectada: h.tabla_afectada,
			id_registro_afectado: h.id_registro_afectado,
			descripcion: h.descripcion,
			fecha: h.fecha,
			hora: h.hora,
			turno: tiposTurnoMap.get(h.id_turno) || (h.id_turno === 0 ? "Sin turno" : `Turno ${h.id_turno}`)
		}));
		
		return resultado;
	} catch (error) {
		console.error("Error al obtener historial con detalle:", error);
		return [];
	}
}

// Función para últimos registros con detalle
export async function obtenerUltimosHistorialConDetalle(
	limite: number = 50
): Promise<HistorialDetalle[]> {
	return await listarHistorialConDetalle({});
}

export async function obtenerStatsHistorial(
	fechaInicio?: string,
	fechaFin?: string
) {
	const historial = await listarHistorialConDetalle({
		fecha_inicio: fechaInicio,
		fecha_fin: fechaFin
	});

	const hoy = new Date().toISOString().split('T')[0];

	return {
		total: historial.length,

		hoy: historial.filter(
			(item) => item.fecha === hoy
		).length,

		manana: historial.filter(
			(item) =>
				item.turno?.toLowerCase().includes('mañana')
		).length,

		tarde: historial.filter(
			(item) =>
				item.turno?.toLowerCase().includes('tarde')
		).length
	};
}