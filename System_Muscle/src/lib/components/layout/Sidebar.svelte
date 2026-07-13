<script lang="ts">
    import { page } from '$app/stores';
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';
    import { invoke } from '@tauri-apps/api/core';
    import { toast } from 'svelte-sonner';
    import { esAdministrador } from '$lib/utils/session';

    let usuario = {
        nombre_completo: '',
        rol: ''
    };

onMount(() => {
	const sesion = localStorage.getItem('sesion');

	if (sesion) {
		const data = JSON.parse(sesion);

		console.log('SESION:', data);

		usuario = {
			nombre_completo: data.nombre_completo,
			rol: data.nombre_rol
		};
	}
});

function obtenerIniciales(nombre: string) {
	return nombre
		.split(' ')
		.map((p) => p[0])
		.slice(0, 2)
		.join('')
		.toUpperCase();
}
    
    import {
        LayoutGrid,
        ShoppingCart,
        Box,
        TrendingUp,
        FileText,
        Clock,
        Users,
        LogOut
    } from 'lucide-svelte';

    const menu = [
        {
            label: 'Dashboard',
            icon: LayoutGrid,
            href: '/dashboard'
        },
        {
            label: 'Ventas',
            icon: ShoppingCart,
            href: '/ventas'
        },
        {
            label: 'Inventario',
            icon: Box,
            href: '/inventario'
        },
        {
            label: 'Movimientos',
            icon: TrendingUp,
            href: '/movimientos',
            admin: true
        },
        {
            label: 'Reportes',
            icon: FileText,
            href: '/reportes',
            admin: true
        },
        {
            label: 'Historial',
            icon: Clock,
            href: '/historial',
            admin: true
        },
        {
            label: 'Usuarios',
            icon: Users,
            href: '/users',
            admin: true
        }
    ];

    async function cerrarSesion() {
	try {
		const sesion = localStorage.getItem('sesion');

		if (sesion) {
			const data = JSON.parse(sesion);

			// Buscar turno activo del usuario
			const turnoActivo = await invoke<any>(
				'obtener_turno_activo',
				{
					idUsuario: data.id_usuario
				}
			);

			// Si existe un turno abierto, cerrarlo
			if (turnoActivo) {
				await invoke(
					'cerrar_turno',
					{
						idTurno: turnoActivo.id_turno
					}
				);
			}
		}

		localStorage.removeItem('sesion');

		toast.success(
			'Sesión cerrada correctamente'
		);

		await goto('/');

	} catch (error) {
		console.error(
			'Error al cerrar sesión:',
			error
		);

		toast.error(
			'No fue posible cerrar la sesión'
		);
	}
}
</script>

<aside
    class="fixed left-0 top-0 flex h-screen w-[280px] flex-col border-r border-zinc-200 bg-white dark:border-[#334156] dark:bg-[#1E293B]"
>
    <div class="flex items-center gap-3 p-6 border-b border-zinc-200 dark:border-[#334156]">
        <div
            class="flex h-10 w-10 items-center justify-center rounded-lg bg-gradient-to-br from-[#0C4A6E] to-[#0284C7] font-bold text-white"
        >
            SM
        </div>

        <h1 class="text-lg font-semibold dark:text-white">
            System Muscle
        </h1>
    </div>

    <nav class="flex-1 px-4 py-4">
        <p
            class="mb-4 px-3 text-xs font-semibold uppercase tracking-wider text-zinc-500 dark:text-zinc-100"
        >
            Menu
        </p>

        <div class="space-y-2">
            {#each menu.filter(item => !item.admin || esAdministrador()) as item}
                <a
                    href={item.href}
                    class={`flex w-full items-center gap-3 rounded-lg px-4 py-3 text-sm font-medium transition-all ${
                        $page.url.pathname === item.href
                            ? 'bg-[#0C4A6E] dark:bg-[#0284C7] text-white'
                            : 'text-zinc-600 hover:bg-zinc-100 dark:text-zinc-300 dark:hover:bg-zinc-800'
                    }`}
                >
                    <item.icon size={20} />

                    {item.label}
                </a>
            {/each}
        </div>
    </nav>

    <div class="border-t border-zinc-200 p-4 dark:border-[#334156]">
        <div class="mb-4 flex items-center gap-3 rounded-lg bg-zinc-100 px-4 py-3 dark:bg-[#334156]">
            <div
                class="flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-full bg-gradient-to-br from-[#0C4A6E] to-[#0284C7] font-semibold text-white"
            >
                {obtenerIniciales(usuario.nombre_completo)}
            </div>

            <div>
                <p class="text-sm font-semibold text-zinc-900 dark:text-white">
                    {usuario.nombre_completo}
                </p>

                <p class="text-xs text-zinc-500 dark:text-zinc-400">
                    {usuario.rol}
                </p>
            </div>
        </div>

<button
	on:click={cerrarSesion}
	class="flex w-full items-center gap-3 rounded-lg px-4 py-3 text-sm text-zinc-600 transition-all hover:bg-zinc-100 dark:text-zinc-300 dark:hover:bg-zinc-800"
>
	<LogOut size={20} />

	Cerrar sesión
</button>
    </div>
</aside>