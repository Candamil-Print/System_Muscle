import { writable } from 'svelte/store';
import { abrirTurno, obtenerTurnoActivo, obtenerTiposTurno } from '$lib/services/api/shifts/shifts.service';

export interface Turno {
    id_turno: number;
    id_tipo_turno: number;
    nombre: string;
    horario: string;
}

async function cargarTurnoUsuario(): Promise<Turno | null> {
    try {
        const tiposTurno = await obtenerTiposTurno();
        const turnoActivo = await obtenerTurnoActivo();

        console.log('Tipos de turno disponibles:', tiposTurno);
        console.log('Turno activo del usuario:', turnoActivo);

        if (!turnoActivo) {
            console.log('No hay turno activo para este usuario');
            return null;
        }

        const tipo = tiposTurno.find(
            (tt) => tt.id_tipo_turno === turnoActivo.id_tipo_turno
        );

        return {
            id_turno: turnoActivo.id_turno,
            id_tipo_turno: turnoActivo.id_tipo_turno,
            nombre: tipo?.nombre ?? 'Desconocido',
            horario: tipo
                ? `${tipo.hora_inicio} - ${tipo.hora_fin}`
                : '00:00 - 00:00'
        };
    } catch (error) {
        console.error(
            'Error cargando turno activo:',
            error
        );

        return null;
    }
}

function createTurnoStore() {
    // Cargar del localStorage al inicializar el store
    let inicial: Turno | null = null;
    if (typeof localStorage !== 'undefined') {
        const guardado = localStorage.getItem('turnoSeleccionado');
        inicial = guardado ? JSON.parse(guardado) : null;
    }
    
    const { subscribe, set } = writable<Turno | null>(inicial);

    let turnoActual: Turno | null = inicial;

    return {
        subscribe,

        async inicializar(): Promise<void> {
            const turno = await cargarTurnoUsuario();

            if (turno) {
                turnoActual = turno;
                set(turno);
                if (typeof localStorage !== 'undefined') {
                    localStorage.setItem(
                        'turnoSeleccionado',
                        JSON.stringify(turno)
                    );
                }
            } else {
                console.log('Sin turno activo, esperando selección');
                set(null);
            }
        },

        async seleccionarTurno(
            idTipoTurno: number,
            nombre: string,
            horario: string
        ): Promise<void> {
            try {
                console.log('Seleccionando turno:', { idTipoTurno, nombre, horario });
                
                // Crear turno en BD
                const idTurno = await abrirTurno(
                    idTipoTurno
                );

                console.log('Turno creado con ID:', idTurno);

                const nuevoTurno: Turno = {
                    id_turno: idTurno,
                    id_tipo_turno: idTipoTurno,
                    nombre,
                    horario
                };

                turnoActual = nuevoTurno;

                set(nuevoTurno);

                if (typeof localStorage !== 'undefined') {
                    localStorage.setItem(
                        'turnoSeleccionado',
                        JSON.stringify(nuevoTurno)
                    );
                }
            } catch (error) {
                console.error(
                    'Error al abrir turno:',
                    error
                );

                throw error;
            }
        },

        limpiar(): void {
            turnoActual = null;

            set(null);

            if (typeof localStorage !== 'undefined') {
                localStorage.removeItem(
                    'turnoSeleccionado'
                );
            }
        },

        getTurnosDisponibles() {
            return obtenerTiposTurno();
        }
    };
}

export const turnoStore = createTurnoStore();