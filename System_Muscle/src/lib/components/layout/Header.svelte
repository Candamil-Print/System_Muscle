<script lang="ts">
	import {
		Bell,
		Clock,
		Moon,
		ChevronDown,
		TriangleAlert,
		CheckCheck,
		Trash2
	} from 'lucide-svelte';
	import { turnoStore } from '$lib/stores/shifts/turnoStore';
	import { onMount } from 'svelte';

	import type { Notificacion } from '$lib/services/api/notifications';

	import {
		listarNotificaciones,
		contarNotificacionesNoLeidas,
		marcarTodasLeidas,
		limpiarNotificaciones
	} from '$lib/services/api/notifications';
	
	let openTurnos = false;
	let turnoActual: any = null;

	let openNotificaciones = false;



	let notificaciones: Notificacion[] = [];
	let totalNotificaciones = 0;


	
	
	// Tipos de turno disponibles (hardcodeados por ahora)
	const tiposTurno = [
		{ id_tipo_turno: 1, nombre: 'MAÑANA', horario: '05:00 - 13:00' },
		{ id_tipo_turno: 2, nombre: 'TARDE_LJ', horario: '13:00 - 22:00' },
		{ id_tipo_turno: 3, nombre: 'TARDE_V', horario: '13:00 - 21:00' },
		{ id_tipo_turno: 4, nombre: 'UNICO_SF', horario: '08:00 - 15:00' }
	];
	
	// Suscribirse al store
	turnoStore.subscribe(turno => {
		turnoActual = turno;
		console.log('Turno actual en store:', turnoActual);
	});
	
	onMount(async () => {
		await turnoStore.inicializar();

		try {
			notificaciones = await listarNotificaciones(true);
			totalNotificaciones = await contarNotificacionesNoLeidas();
		} catch (error) {
			console.error('Error cargando notificaciones:', error);
		}
	});
	
	async function seleccionarTurno(idTipoTurno: number, nombre: string, horario: string) {
		try {
			await turnoStore.seleccionarTurno(idTipoTurno, nombre, horario);
			openTurnos = false;
		} catch (error) {
			console.error('Error al abrir turno:', error);
			alert('No se pudo abrir el turno. Verifica que no haya otro turno activo.');
		}
	}

	async function marcarComoLeidas() {
	try {

		await marcarTodasLeidas();

		notificaciones = await listarNotificaciones(false);

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

			notificaciones = [];

			totalNotificaciones = 0;

		} catch (error) {

			console.error(
				'Error limpiando notificaciones:',
				error
			);

		}
	}
</script>

<header
	class="sticky top-0 z-40 flex h-16 items-center justify-between border-b border-zinc-200 bg-white px-6 dark:border-zinc-800 dark:bg-[#0f172a]"
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
				<span>{turnoActual?.nombre || 'Seleccionar turno'}</span>
				<ChevronDown size={16} />
			</button>

			{#if openTurnos}
				<div
					class="absolute right-0 mt-2 w-80 overflow-hidden rounded-xl border border-zinc-200 bg-white shadow-lg dark:border-zinc-700 dark:bg-slate-900"
				>
					<div class="border-b border-zinc-200 px-4 py-3 dark:border-zinc-700">
						<p class="font-medium dark:text-white">
							Seleccionar turno
						</p>
					</div>

					{#each tiposTurno as turno}
						<button
							on:click={() => seleccionarTurno(turno.id_tipo_turno, turno.nombre, turno.horario)}
							class="flex w-full flex-col items-start px-4 py-3 text-left transition hover:bg-zinc-50 dark:hover:bg-slate-800"
						>
							<span class="font-medium dark:text-white">
								{turno.nombre}
							</span>
							<span class="text-xs text-zinc-500">
								{turno.horario}
							</span>
						</button>
					{/each}
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
								class="flex items-center gap-1 rounded-lg border border-zinc-200 px-2 py-1 text-xs transition hover:bg-zinc-100 dark:border-zinc-700 dark:hover:bg-slate-800"
							>
								<CheckCheck size={14} />
								<span>Leídas</span>
							</button>

							<button
								on:click={limpiarTodas}
								class="flex items-center gap-1 rounded-lg border border--[#26557C] px-2 py-1 text-xs  text-[#26557C] transition hover:bg--[#26557C]/10 "
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
													class="text-bg-[#26557C]"
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
			class="flex h-10 w-10 items-center justify-center rounded-full border border-zinc-200 dark:border-zinc-700 dark:text-zinc-200"
		>
			<Moon size={18} />
		</button>
	</div>
</header>