<script lang="ts">
	import { onMount } from 'svelte';
	import { obtenerStatsHistorial } from '$lib/services/api/history/history.service';
	import TrendingUp from 'lucide-svelte/icons/trending-up';
	import Calendar from 'lucide-svelte/icons/calendar';
	import Clock from 'lucide-svelte/icons/clock';

	export let filtros = {
		fechaInicio: '',
		fechaFin: ''
	};

	let stats = {
		total: 0,
		hoy: 0,
		manana: 0,
		tarde: 0
	};

	async function cargarStats() {
		stats = await obtenerStatsHistorial(
			filtros.fechaInicio,
			filtros.fechaFin
		);
	}

	onMount(cargarStats);

	$: if (
		filtros?.fechaInicio &&
		filtros?.fechaFin
	) {
		cargarStats();
	}
</script>

<div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-4">
	<div class="rounded-xl border border-slate-200 bg-white p-5">
		<div class="flex items-center justify-between">
			<p class="text-sm font-medium text-slate-500">
				Total Acciones
			</p>
			<TrendingUp class="w-5 h-5 text-slate-400" />
		</div>

		<p class="mt-2 text-3xl font-bold text-slate-900">
			{stats.total}
		</p>

		<p class="mt-1 text-xs text-slate-500">
			acciones registradas
		</p>
	</div>

	<div class="rounded-xl border border-slate-200 bg-white p-5">
		<div class="flex items-center justify-between">
			<p class="text-sm font-medium text-slate-500">
				Acciones Hoy
			</p>
			<Calendar class="w-5 h-5 text-slate-400" />
		</div>

		<p class="mt-2 text-3xl font-bold text-slate-900">
			{stats.hoy}
		</p>

		<p class="mt-1 text-xs text-slate-500">
			acciones del día
		</p>
	</div>

	<div class="rounded-xl border border-slate-200 bg-white p-5">
		<div class="flex items-center justify-between">
			<p class="text-sm font-medium text-slate-500">
				Turno Mañana
			</p>
			<Clock class="w-5 h-5 text-slate-400" />
		</div>

		<p class="mt-2 text-3xl font-bold text-[#0C4A6E]">
			{stats.manana}
		</p>

		<p class="mt-1 text-xs text-slate-500">
			5am - 1pm
		</p>
	</div>

	<div class="rounded-xl border border-slate-200 bg-white p-5">
		<div class="flex items-center justify-between">
			<p class="text-sm font-medium text-slate-500">
				Turno Tarde
			</p>
			<Clock class="w-5 h-5 text-slate-400" />
		</div>

		<p class="mt-2 text-3xl font-bold text-[#0C4A6E]">
			{stats.tarde}
		</p>

		<p class="mt-1 text-xs text-slate-500">
			1pm - 10pm
		</p>
	</div>
</div>