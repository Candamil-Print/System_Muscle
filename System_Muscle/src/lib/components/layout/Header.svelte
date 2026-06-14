<script lang="ts">
	import {
		Bell,
		Clock,
		Moon,
		Sun,
		ChevronDown,
		TriangleAlert,
		CheckCheck,
		Trash2
	} from 'lucide-svelte';
	import { turnoStore } from '$lib/stores/shifts/turnoStore';
	import { theme } from '$lib/stores/theme.store';
	import { onMount } from 'svelte';

	import type { Notificacion } from '$lib/services/api/notifications';
	import type { TipoTurno } from '$lib/services/api/shifts/shifts.types';
	import type { Turno } from '$lib/stores/shifts/turnoStore';

	import {
		listarNotificaciones,
		contarNotificacionesNoLeidas,
		marcarTodasLeidas,
		limpiarNotificaciones
	} from '$lib/services/api/notifications';
	import { obtenerTiposTurno } from '$lib/services/api/shifts/shifts.service';
	
	let openTurnos = false;
	let turnoActual: Turno | null = null;
	let openNotificaciones = false;
	let currentTheme: 'light' | 'dark' = 'light';

	let notificaciones: Notificacion[] = [];
	let totalNotificaciones = 0;

	let tiposTurno: TipoTurno[] = [];


	// Suscribirse al store del tema
	theme.subscribe(value => {
		currentTheme = value;
	});

	// Suscribirse al store
	turnoStore.subscribe((turno: Turno | null) => {
		turnoActual = turno;
		console.log('Turno actual en store:', turnoActual);
	});
	
	onMount(async () => {
		await turnoStore.inicializar();

		try {
			tiposTurno = await obtenerTiposTurno();
			notificaciones = await listarNotificaciones(true);
			totalNotificaciones = await contarNotificacionesNoLeidas();
		} catch (error) {
			console.error('Error cargando datos:', error);
		}
	});
	
	async function seleccionarTurno(
	idTipoTurno: number,
	nombre: string,
	horario: string
): Promise<void> {
	try {
		await turnoStore.seleccionarTurno(
			idTipoTurno,
			nombre,
			horario
		);

		openTurnos = false;
	} catch (error) {
		console.error(
			'Error al abrir turno:',
			error
		);

		alert(
			'No se pudo abrir el turno. Verifica que no haya otro turno activo.'
		);
	}
}

	async function marcarComoLeidas() {
		try {

			await marcarTodasLeidas();

			notificaciones = await listarNotificaciones(true);

			totalNotificaciones = 0;

		} catch (error) {

			console.error(
				'Error marcando notificaciones:',
				error
			);

		}
	}
	

	async function limpiarTodas() {
		try {

			await limpiarNotificaciones();

			notificaciones = await listarNotificaciones(true);

			totalNotificaciones =
				await contarNotificacionesNoLeidas();

		} catch (error) {

			console.error(
				'Error limpiando notificaciones:',
				error
			);

		}
	}

	function nombreTurnoLegible(nombre: string | undefined): string {
		switch (nombre?.toUpperCase()) {
			case 'TARDE_LJ':
				return 'Turno: Tarde Lunes - Jueves';

			case 'MAÑANA':
				return 'Turno: Mañana';

			case 'TARDE_V':
				return 'Turno: Tarde Viernes';

			case 'UNICO_SF':
				return 'Turno: Único Sábado - Domingo - Festivo';

			default:
				return 'Sin turno';
		}
	}

	function formatearHora12h(hora: string): string {
		const [h, m] = hora.split(':');

		const fecha = new Date();
		fecha.setHours(Number(h));
		fecha.setMinutes(Number(m));

		return fecha.toLocaleTimeString('es-CO', {
			hour: 'numeric',
			minute: '2-digit',
			hour12: true
		});
	}

	function formatearHorario(horario: string | undefined): string {
		if (!horario || horario.trim() === '') {
			return 'No hay un turno configurado para esta franja horaria';
		}

		const partes = horario.split(' - ');

		if (partes.length !== 2) {
			return 'No hay un turno configurado para esta franja horaria';
		}

		const [inicio, fin] = partes;

		return `${formatearHora12h(inicio)} - ${formatearHora12h(fin)}`;
	}

	const toggleTheme = () => {
		const isDark = document.documentElement.classList.contains('dark');

		if (isDark) {
			document.documentElement.classList.remove('dark');
			localStorage.setItem('theme', 'light');
			theme.set('light');
		} else {
			document.documentElement.classList.add('dark');
			localStorage.setItem('theme', 'dark');
			theme.set('dark');
		}
	};
</script>

<header
	class="sticky top-0 z-40 flex h-16 items-center justify-between border-b border-zinc-200 bg-white px-6 dark:border-[#334156] dark:bg-[#1E293B]"
>
	<h1 class="text-lg font-semibold dark:text-white">
		Steel Body Gym
	</h1>

	<div class="flex items-center gap-3">
		<div class="relative">
			<button
				on:click={() => (openTurnos = !openTurnos)}
				class="flex items-center gap-2 rounded-full border border-zinc-200 px-4 py-2 text-sm dark:border-zinc-700 dark:text-zinc-200"
			>
				<Clock size={18} />
				<span>{nombreTurnoLegible(turnoActual?.nombre)}</span>
				<ChevronDown size={16} />
			</button>

{#if openTurnos}
	<div
		class="absolute right-0 mt-2 w-80 overflow-hidden rounded-xl border border-zinc-200 bg-white shadow-lg dark:border-zinc-700 dark:bg-slate-900"
	>
		<div class="border-b border-zinc-200 px-4 py-3 dark:border-zinc-700">
			<p class="font-medium dark:text-white">
				Información del turno
			</p>
		</div>

		<div class="space-y-3 p-4">

			<div>
				<p class="text-xs text-zinc-500">
					Turno actual
				</p>

				<p class="font-semibold dark:text-white">
					{nombreTurnoLegible(turnoActual?.nombre)}
				</p>
			</div>

			<div>
				<p class="text-xs text-zinc-500">
					Horario
				</p>

				<p class="dark:text-zinc-200">
					{formatearHorario(turnoActual?.horario)}
				</p>
			</div>

		</div>
	</div>
{/if}
		</div>

		<div class="relative">

			<button
				on:click={() => (openNotificaciones = !openNotificaciones)}
				class="relative flex h-10 w-10 items-center justify-center rounded-full border border-zinc-200 transition hover:bg-zinc-100 dark:border-zinc-700 dark:text-zinc-200 dark:hover:bg-slate-800"
			>
				<Bell
					size={18}
					class={totalNotificaciones > 0 ? 'animate-pulse text-[#26557C]' : ''}
				/>

				{#if totalNotificaciones > 0}
					<span
						class="absolute -right-1 -top-1 flex h-5 min-w-[20px] items-center justify-center rounded-full border-2 border-white bg-gradient-to-r from-[#0C4A6E] to-[#0284C7] px-1 text-[10px] font-bold text-white shadow-lg"
					>
						{totalNotificaciones}
					</span>
				{/if}
			</button>

			{#if openNotificaciones}
				<div
					class="absolute right-0 mt-2 w-96 overflow-hidden rounded-2xl border border-white/20 bg-white/95 shadow-2xl backdrop-blur-xl dark:border-slate-700 dark:bg-slate-900/95"
				>

					<!-- Header -->
					<div
						class="flex items-center justify-between border-b border-zinc-200 px-4 py-3 dark:border-zinc-700"
					>
						<div>
							<p class="font-semibold dark:text-white">
								Notificaciones
							</p>

							<p class="text-xs text-zinc-500">
								{totalNotificaciones} pendientes
							</p>
						</div>

						<div class="flex gap-2">

							<button
								on:click={marcarComoLeidas}
								class="flex items-center gap-1 rounded-lg border border-zinc-200 px-2 py-1 text-xs transition hover:bg-zinc-100 dark:border-zinc-700 dark:hover:bg-slate-800 dark:text-white"
							>
								<CheckCheck size={14} />
								<span>Leídas</span>
							</button>

							<button
								on:click={limpiarTodas}
								class="flex items-center gap-1 rounded-lg border border-[#26557C] px-2 py-1 text-xs text-[#26557C] transition hover:bg-[#26557C]/10 "
							>
								<Trash2 size={14} />
								<span>Limpiar</span>
							</button>

						</div>
					</div>

					{#if notificaciones.length === 0}

						<div class="p-8 text-center">

							<Bell
								size={40}
								class="mx-auto mb-3 text-zinc-300"
							/>

							<p class="text-sm text-zinc-500">
								No hay notificaciones
							</p>

						</div>

					{:else}

						<div
							class={`${notificaciones.length > 2 ? 'max-h-[260px] overflow-y-auto' : ''}`}
						>

							{#each notificaciones as notificacion}

								<div
									class="group relative border-b border-zinc-100 p-4 transition-all duration-300 hover:bg-[#26557C]/10 dark:border-zinc-800 dark:hover:bg-red-950/20"
								>

									<div
										class="absolute left-0 top-0 h-full w-1 rounded-r-full bg-[#26557C]"
									></div>

									<div class="pl-3">

										<div class="flex items-start gap-3">

											<div
												class="flex h-10 w-10 items-center justify-center rounded-xl bg-[#26557C]/10"
											>
												<TriangleAlert
													size={18}
													class="text-[#26557C]"
												/>
											</div>

											<div class="flex-1">

												<p
													class="font-semibold text-slate-800 dark:text-white"
												>
													{notificacion.nombre_producto}
												</p>

												<p
													class="text-xs uppercase tracking-wider text-[#26557C]"
												>
													Stock Bajo
												</p>

											</div>

										</div>

										<p
											class="mt-3 text-sm leading-relaxed text-zinc-600 dark:text-zinc-300"
										>
											{notificacion.mensaje}
										</p>

										<div class="mt-4 flex gap-2">

											<div
												class="rounded-lg px-3 py-1.5 text-xs font-medium bg-[#26557C]/10 text-[#26557C]"
											>
												Actual: {notificacion.stock_actual}
											</div>

											<div
												class="rounded-lg  px-3 py-1.5 text-xs font-medium bg-[#26557C]/10 text-[#26557C]"
											>
												Mínimo: {notificacion.stock_minimo}
											</div>

										</div>

										<div
											class="mt-3 flex items-center justify-between"
										>

											<p class="text-xs text-zinc-400">
												{notificacion.fecha}
											</p>

											<span
												class="rounded-full  px-2 py-1 text-[10px] font-semibold uppercase tracking-wider bg-[#26557C]/10 text-[#26557C]"
											>
												Pendiente
											</span>

										</div>

									</div>

								</div>

							{/each}

						</div>

					{/if}

				</div>
			{/if}

		</div>

		<button
			on:click={toggleTheme}
			class="flex h-10 w-10 items-center justify-center rounded-full border border-zinc-200 transition hover:bg-zinc-100 dark:border-zinc-700 dark:text-zinc-200 dark:hover:bg-slate-800"
		>
			{#if currentTheme === 'dark'}
				<Sun size={18} class="dark:text-[#39BDF8]" />
			{:else}
				<Moon size={18}  />
			{/if}
		</button>
	</div>
</header>