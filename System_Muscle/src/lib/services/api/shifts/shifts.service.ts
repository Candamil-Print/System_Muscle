import { invoke } from '@tauri-apps/api/core';
import type { TipoTurno, Caja, TurnoBackend } from './shifts.types';

export async function obtenerTiposTurno(): Promise<TipoTurno[]> {
    try {
        console.log('Obteniendo tipos de turno...');
        
        // Usar listar_turnos_detalle pero filtrando por estado para obtener los tipos
        // Alternativa: crear datos de prueba o usar una query directa
        const tiposHardcoded: TipoTurno[] = [
            {
                id_tipo_turno: 1,
                nombre: 'MAÑANA',
                hora_inicio: '05:00:00',
                hora_fin: '13:00:00'
            },
            {
                id_tipo_turno: 2,
                nombre: 'TARDE_LJ',
                hora_inicio: '13:00:00',
                hora_fin: '22:00:00'
            },
            {
                id_tipo_turno: 3,
                nombre: 'TARDE_V',
                hora_inicio: '13:00:00',
                hora_fin: '21:00:00'
            },
            {
                id_tipo_turno: 4,
                nombre: 'UNICO_SF',
                hora_inicio: '08:00:00',
                hora_fin: '15:00:00'
            }
        ];

        console.log('Tipos de turno:', tiposHardcoded);
        return tiposHardcoded;
    } catch (error) {
        console.error('Error en obtenerTiposTurno:', error);
        return [];
    }
}

export async function obtenerCajaActiva(): Promise<Caja | null> {
	try {
		return await invoke<Caja | null>(
			'obtener_caja_activa'
		);
	} catch (error) {
		console.error(
			'Error obteniendo caja activa:',
			error
		);
		return null;
	}
}

export async function listarCajas(
	soloAbiertas = false
): Promise<Caja[]> {
	try {
		return await invoke<Caja[]>(
			'listar_cajas',
			{
				solo_abiertas: soloAbiertas
			}
		);
	} catch (error) {
		console.error(
			'Error listando cajas:',
			error
		);
		return [];
	}
}

export async function abrirTurno(
	idTipoTurno: number
): Promise<number> {
	const sesion = JSON.parse(
		localStorage.getItem('sesion') ?? '{}'
	);

	return await invoke<number>(
		'abrir_turno',
		{
			nuevo: {
				id_usuario: sesion.id_usuario,
				id_tipo_turno: idTipoTurno
			}
		}
	);
}

export async function cerrarTurno(
	idTurno: number
): Promise<void> {
	await invoke(
		'cerrar_turno',
		{
			id_turno: idTurno
		}
	);
}

export async function obtenerTurnoActivo(): Promise<TurnoBackend | null> {
    const sesion = JSON.parse(
        localStorage.getItem('sesion') ?? '{}'
    );

    try {
        return await invoke<TurnoBackend | null>(
            'obtener_turno_activo',
            {
                idUsuario: sesion.id_usuario
            }
        );
    } catch (error) {
        console.error(
            'Error obteniendo turno activo:',
            error
        );
        return null;
    }
}

export async function obtenerTurnoActivoGeneral() {
	try {
		return await invoke(
			'obtener_turno_activo_general'
		);
	} catch (error) {
		console.error(
			'Error obteniendo turno activo general:',
			error
		);
		return null;
	}
}

export async function obtenerTurno(
	id: number
) {
	try {
		return await invoke(
			'obtener_turno',
			{
				id
			}
		);
	} catch (error) {
		console.error(
			'Error obteniendo turno:',
			error
		);
		return null;
	}
}

export async function listarTurnos(
	filtro = {
		id_usuario: null,
		estado: null,
		fecha_desde: null,
		fecha_hasta: null
	}
) {
	try {
		return await invoke(
			'listar_turnos',
			{
				filtro
			}
		);
	} catch (error) {
		console.error(
			'Error listando turnos:',
			error
		);
		return [];
	}
}

export async function listarTurnosDetalle(
	filtro = {
		id_usuario: null,
		estado: null,
		fecha_desde: null,
		fecha_hasta: null
	}
) {
	try {
		return await invoke(
			'listar_turnos_detalle',
			{
				filtro
			}
		);
	} catch (error) {
		console.error(
			'Error listando detalle turnos:',
			error
		);
		return [];
	}
}

export async function abrirCaja(idTurno: number): Promise<number> {
    const sesion = JSON.parse(
        localStorage.getItem('sesion') ?? '{}'
    );

    return await invoke<number>(
        'abrir_caja',
        {
            nueva: {
                id_usuario: sesion.id_usuario,
                id_turno: idTurno,
                saldo_inicial: 0,
                monto_apertura: 0,
                id_usuario_apertura: sesion.id_usuario
            }
        }
    );
}

export async function cerrarCaja(
    idCaja: number
): Promise<void> {
    await invoke(
        'cerrar_caja',
        {
            id_caja: idCaja
        }
    );
}