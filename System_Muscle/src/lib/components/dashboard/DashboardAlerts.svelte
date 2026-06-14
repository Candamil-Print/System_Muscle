<script lang="ts">
	import { AlertTriangle } from 'lucide-svelte';
	import { onMount } from 'svelte';

	import { obtenerReporteStockBajo } from '$lib/services/api/reports';
	import type { ReporteStockBajo } from '$lib/services/api/reports';

	let alerts: ReporteStockBajo[] = [];

	onMount(async () => {
		try {
			alerts = await obtenerReporteStockBajo();
		} catch (error) {
			console.error(
				'Error cargando stock bajo:',
				error
			);
		}
	});
</script>

<div
	class="rounded-lg border border-zinc-200 bg-white p-6 dark:border-[#334156] dark:bg-[#1E293B]"
>
	<div class="mb-2 flex items-center gap-2">
		<AlertTriangle
			size={20}
			class="text-[#0C4A6E] dark:text-[#39BDF8]"
		/>

		<h3
			class="text-lg font-semibold text-[#0C4A6E] dark:text-white"
		>
			Alertas de Stock
		</h3>
	</div>

	<p
		class="mb-6 text-sm text-zinc-500 dark:text-zinc-400"
	>
		Productos que requieren reposición
	</p>

	{#if alerts.length === 0}

		<div
			class="relative overflow-hidden rounded-2xl border border-[#0C4A6E]/20  p-8 dark:bg-[#111827] dark:border-[#334156]"
		>

			<!-- Glow decorativo -->
			<div
				class="absolute -right-10 -top-10 h-32 w-32 rounded-full bg-[#0284C7]/10 blur-3xl"
			></div>

			<div
				class="absolute -bottom-10 -left-10 h-32 w-32 rounded-full bg-[#0C4A6E]/10 blur-3xl"
			></div>

			<div class="relative flex flex-col items-center text-center ">

				<div
					class="mb-4 flex h-16 w-16 items-center justify-center rounded-2xl bg-[#0C4A6E]/10"
				>
					<AlertTriangle
						size={30}
						class="text-[#0C4A6E]"
					/>
				</div>

				<h4
					class="mb-2 text-lg font-semibold text-[#0C4A6E] dark:text-white"
				>
					Todo bajo control
				</h4>

				<p
					class="max-w-xs text-sm leading-relaxed text-zinc-500 dark:text-zinc-400"
				>
					Actualmente no existen productos con stock bajo.
					Todos los productos se encuentran dentro de los niveles
					establecidos.
				</p>

				<div
					class="mt-4 rounded-full bg-[#0C4A6E]/10 px-4 py-1.5 text-xs font-semibold text-[#0C4A6E] "
				>
					✓ Buen Inventario
				</div>

			</div>

		</div>

	{:else}

		<div
			class={`space-y-3 ${
				alerts.length > 3
					? 'max-h-63.75 overflow-y-auto pr-2'
					: ''
			}`}
		>

			{#each alerts as alert}

				<div
					class="flex items-center justify-between rounded-lg bg-[#0C4A6E]/10 p-4 dark:bg-[#39BDF8]/20"
				>

					<div>

						<h4
							class="font-semibold text-[#0C4A6E] dark:text-[#39BDF8]"
						>
							{alert.nombre}
						</h4>

						<p
							class="text-sm text-zinc-500 dark:text-[#39BDF8]"
						>
							{alert.tipo_producto}
						</p>

					</div>

					<div class="text-right">

						<p
							class="font-semibold text-[#0C4A6E] dark:text-[#39BDF8]"
						>
							{alert.stock_actual} / {alert.stock_maximo}
						</p>

						<p
							class="text-xs text-zinc-500 dark:text-[#39BDF8]"
						>
							{alert.porcentaje_stock.toFixed(0)}%
							disponible
						</p>

					</div>

				</div>

			{/each}

		</div>

	{/if}
</div>