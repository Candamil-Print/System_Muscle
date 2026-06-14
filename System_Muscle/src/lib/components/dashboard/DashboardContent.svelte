<script lang="ts">
	import DashboardStats from './DashboardStats.svelte';
	import DashboardUsers from './DashboardUsers.svelte';
	import DashboardChart from './DashboardChart.svelte';
	import DashboardAlerts from './DashboardAlerts.svelte';

	import { onMount } from 'svelte';
	import { turnoStore } from '$lib/stores/shifts/turnoStore';
	import type { Turno } from '$lib/stores/shifts/turnoStore';

	let nombreUsuario = 'Usuario';
	let turnoActual: Turno | null = null;

	turnoStore.subscribe((turno) => {
		turnoActual = turno;
	});

	function nombreTurnoLegible(nombre: string | undefined): string {
		switch (nombre) {
			case 'TARDE_LJ':
				return 'Turno Tarde Lunes - Jueves';

			case 'TARDE_V':
				return 'Turno Tarde Viernes';

			case 'MAÑANA':
				return 'Turno Mañana';

			case 'UNICO_SF':
				return 'Turno Único Sábado - Domingo - Festivo';

			default:
				return nombre ?? 'Sin turno';
		}
	}

	

	onMount(() => {
		const sesion = localStorage.getItem('sesion');

		if (sesion) {
			const usuario = JSON.parse(sesion);

			nombreUsuario =
				usuario.nombre_completo ??
				usuario.nombre ??
				'Usuario';
		}
	});
</script>

<div class="space-y-6">

	<!-- PAGE HEADER -->
	<div
		class="flex flex-col gap-4 lg:flex-row lg:items-center lg:justify-between"
	>
		<div>
			<h1
				class="text-3xl font-bold text-zinc-900 dark:text-white"
			>
				Dashboard
			</h1>

			<p class="text-sm text-zinc-500 dark:text-zinc-400">
				Bienvenido, {nombreUsuario}
			</p>
		</div>

		<div
			class="rounded-full  bg-[#0C4A6E]/10 px-5 py-2.5 text-sm font-medium text-zinc-700 dark:border-zinc-800  dark:text-[#39BDF8] dark:bg-[#0C4A6E]/20 "
		>
			<p class="text-[#0C4A6E]">
				{nombreTurnoLegible(turnoActual?.nombre)}
				({turnoActual?.horario})
			</p>
		</div>
	</div>

	<DashboardStats />

	<DashboardUsers />

	<div class="grid grid-cols-1 gap-4 xl:grid-cols-2">
		<DashboardChart />
		<DashboardAlerts />
	</div>

</div>