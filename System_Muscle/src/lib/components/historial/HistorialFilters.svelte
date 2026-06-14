<script lang="ts">
	import { createEventDispatcher, onMount } from "svelte";
	import { BrushCleaning, CalendarDays } from 'lucide-svelte';

	import flatpickr from 'flatpickr';
  	import { Spanish } from 'flatpickr/dist/l10n/es.js';
	

	const dispatch = createEventDispatcher();

	let search = "";
	let turno = "";
	let fecha = "";

	let fechaInput: HTMLInputElement;

	function update() {
		dispatch("filter", {
			search,
			turno,
			fecha
		});
	}

	function limpiarFiltros() {
		search = "";
		turno = "";
		fecha = "";

		fechaInput.value = "";

		update();
	}

	onMount(() => {
		flatpickr(fechaInput, {
			locale: Spanish,
			dateFormat: "Y-m-d",
			allowInput: false,
			onChange: (selectedDates, dateStr) => {
				fecha = dateStr;
				update();
			}
		});
	});
</script>

<div class="rounded-xl border border-slate-200 bg-white dark:border-[#334156] dark:bg-[#1E293B]">
	<div class="flex flex-wrap items-center gap-3 p-4">
		<div class="flex-1 min-w-[220px]">
			<input
				type="text"
				placeholder="Buscar por acción, detalle o usuario..."
				bind:value={search}
				on:input={update}
				class="w-full h-11 rounded-xl border border-slate-200 px-4 text-sm outline-none transition focus:border-cyan-600 dark:border-[#334156] dark:bg-[#1E293B] dark:text-white"
			/>
		</div>

		<div class="relative w-40 ">
			<select
				bind:value={turno}
				on:change={update}
				class="w-full appearance-none rounded-xl border border-slate-200 bg-white px-4 py-3 pr-11 text-sm text-slate-700 outline-none transition duration-200 focus:border-[#0C4A6E] focus:ring-4 focus:ring-sky-100 focus:scale-[1.01] dark:border-[#334156] dark:bg-[#1E293B] dark:text-white"
			>
				<option value="">Turnos</option>
				<option value="MAÑANA">Mañana</option>
				<option value="TARDE_LJ">Tarde Lunes - Jueves</option>
				<option value="TARDE_V">Tarde Viernes</option>
				<option value="UNICO_SF">Único Sabados - Domingo - Festivos</option>
			</select>

			<!-- FLECHA -->
			<div
				class="pointer-events-none absolute inset-y-0 right-4 flex items-center"
			>
				<svg
					class="h-4 w-4 text-slate-500"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M19 9l-7 7-7-7"
					/>
				</svg>
			</div>
		</div>

		<div class="relative">
			<CalendarDays
				class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400 dark:text-slate-500"
			/>

			<input
				bind:this={fechaInput}
				placeholder="Seleccione fecha"
				class="h-11 w-48 pl-10 pr-4 rounded-xl border border-slate-200 bg-white text-sm text-slate-700 outline-none transition duration-200 focus:border-[#0C4A6E] focus:ring-4 focus:ring-sky-100 dark:border-[#334156] dark:bg-[#1E293B] dark:text-white"
			/>
		</div>

		<button
			type="button"
			on:click={limpiarFiltros}
			class="flex items-center justify-center rounded-xl border border-slate-200 bg-white px-4 py-3 text-slate-600 transition duration-200 hover:bg-slate-50 hover:text-[#0C4A6E] focus:ring-4 focus:ring-sky-100 dark:border-[#334156] dark:bg-[#1E293B] dark:text-white dark:hover:bg-[#0F172A]"
			title="Limpiar filtros"
		>
			<BrushCleaning class="w-5 h-5" />
		</button>
	</div>
</div>