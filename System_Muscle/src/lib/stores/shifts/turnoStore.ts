import { writable } from 'svelte/store';
import { abrirTurno, obtenerTurnosActivos, obtenerTiposTurno } from '$lib/services/api/shifts';

export interface Turno {
	id_turno: number;
	id_tipo_turno: number;
	nombre: string;
	horario: string;
}

// Función para cargar turnos existentes del usuario
async function cargarTurnosUsuario(): Promise<Turno[]> {
  try {
    const tiposTurno = await obtenerTiposTurno();
    const turnosActivos = await obtenerTurnosActivos();
    
    return turnosActivos.map(turno => {
      const tipo = tiposTurno.find(tt => tt.id_tipo_turno === turno.id_tipo_turno);
      return {
        id_turno: turno.id_turno,
        id_tipo_turno: turno.id_tipo_turno,
        nombre: tipo?.nombre || 'Desconocido',
        horario: tipo ? `${tipo.hora_inicio} - ${tipo.hora_fin}` : '00:00 - 00:00'
      };
    });
  } catch (error) {
    console.error('Error cargando turnos del usuario:', error);
    return [];
  }
}

function createTurnoStore() {
  const { subscribe, set } = writable<Turno | null>(null);
  let turnosDelUsuario: Turno[] = [];

  return {
    subscribe,
    
    // Inicializar: cargar turnos existentes del usuario
    async inicializar(): Promise<void> {
      turnosDelUsuario = await cargarTurnosUsuario();
      
      if (turnosDelUsuario.length > 0) {
        // Si tiene turnos activos, usar el primero
        set(turnosDelUsuario[0]);
        if (typeof localStorage !== 'undefined') {
          localStorage.setItem('turnoSeleccionado', JSON.stringify(turnosDelUsuario[0]));
        }
      }
    },
    
    // Seleccionar/crear un turno
    async seleccionarTurno(idTipoTurno: number, nombre: string, horario: string): Promise<void> {
      try {
        // Verificar si ya tiene un turno activo de este tipo
        const turnoExistente = turnosDelUsuario.find(t => t.id_tipo_turno === idTipoTurno);
        
        if (turnoExistente) {
          // Usar turno existente
          set(turnoExistente);
          if (typeof localStorage !== 'undefined') {
            localStorage.setItem('turnoSeleccionado', JSON.stringify(turnoExistente));
          }
        } else {
          // Crear nuevo turno en la base de datos
          const nuevoTurno = await abrirTurno(idTipoTurno);
          const nuevoTurnoCompleto: Turno = {
            id_turno: nuevoTurno.id_turno,
            id_tipo_turno: idTipoTurno,
            nombre: nombre,
            horario: horario
          };
          
          // Agregar a la lista local
          turnosDelUsuario.push(nuevoTurnoCompleto);
          
          // Seleccionar el nuevo turno
          set(nuevoTurnoCompleto);
          if (typeof localStorage !== 'undefined') {
            localStorage.setItem('turnoSeleccionado', JSON.stringify(nuevoTurnoCompleto));
          }
        }
      } catch (error) {
        console.error('Error al seleccionar/crear turno:', error);
        throw error;
      }
    },
    
    getTurnosDisponibles(): { id_tipo_turno: number; nombre: string; horario: string }[] {
      // Devolver los tipos de turno disponibles (hardcodeados o desde backend)
      return [
        { id_tipo_turno: 1, nombre: 'MAÑANA', horario: '05:00 - 13:00' },
        { id_tipo_turno: 2, nombre: 'TARDE_LJ', horario: '13:00 - 22:00' },
        { id_tipo_turno: 3, nombre: 'TARDE_V', horario: '13:00 - 21:00' },
        { id_tipo_turno: 4, nombre: 'UNICO_SF', horario: '08:00 - 15:00' }
      ];
    }
  };
}

export const turnoStore = createTurnoStore();