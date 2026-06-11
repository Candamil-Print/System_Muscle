<script lang="ts">
	import { createEventDispatcher } from "svelte";

	const dispatch = createEventDispatcher();

	let search = "";
	let turno = "";
	let fecha = "";

	function update() {
		dispatch("filter", {
			search,
			turno,
			fecha
		});
	}
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
				<option value="Mañana">Mañana</option>
				<option value="Tarde">Tarde</option>
				<option value="Único">Único</option>
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

		<input
			type="date"
			bind:value={fecha}
			on:change={update}
			class=" appearance-none rounded-xl border border-slate-200 bg-white px-4 py-3 pr-11 text-sm text-slate-700 outline-none transition duration-200 focus:border-[#0C4A6E] focus:ring-4 focus:ring-sky-100 focus:scale-[1.01] dark:border-[#334156] dark:bg-[#1E293B] dark:text-white"
		/>
	</div>
</div>