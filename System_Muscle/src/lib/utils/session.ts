import type { SesionUsuario } from '$lib/services/api/login';

export function getSesion(): SesionUsuario | null {
    const data = localStorage.getItem('sesion');

    if (!data) return null;

    try {
        return JSON.parse(data) as SesionUsuario;
    } catch {
        return null;
    }
}

export function esAdministrador(): boolean {
    return getSesion()?.id_rol === 1;
}

export function esRecepcionista(): boolean {
    return getSesion()?.id_rol === 2;
}