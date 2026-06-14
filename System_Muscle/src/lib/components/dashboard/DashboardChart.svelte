<script lang="ts">
	import { BarChart3 } from 'lucide-svelte';
	import { onMount } from 'svelte';
	import Chart from 'chart.js/auto';
	import type { ChartOptions } from 'chart.js';

	import { obtenerResumenVentasDiario } from '$lib/services/api/reports';

	let chartContainer: HTMLCanvasElement;
	let chart: Chart | null = null;

	const options: ChartOptions<'line'> = {
		responsive: true,
		maintainAspectRatio: false,
		plugins: {
			legend: {
				labels: {
					usePointStyle: true,
					pointStyle: 'circle'
				}
			}
		}
	};

	onMount(() => {
		let observer: MutationObserver;

		async function cargarGrafico() {
			try {
				const isDark =
					document.documentElement.classList.contains('dark');

				const ventas = await obtenerResumenVentasDiario();

				const labels = ventas
					.slice(0, 7)
					.reverse()
					.map((item) => {
						const fecha = new Date(item.fecha);

						return fecha.toLocaleDateString('es-CO', {
							weekday: 'short'
						});
					});

				const valores = ventas
					.slice(0, 7)
					.reverse()
					.map((item) => item.total_general);

				if (chartContainer) {
					const ctx = chartContainer.getContext('2d');

					if (ctx) {
						// Destruir gráfica anterior
						if (chart) {
							chart.destroy();
						}

						chart = new Chart(ctx, {
							type: 'line',
							data: {
								labels,
								datasets: [
									{
										label: 'Ventas ($)',
										data: valores,
										borderColor: isDark
											? '#39BDF8'
											: '#0C4A6E',
										backgroundColor: isDark
											? 'rgba(57, 189, 248, 0.2)'
											: 'rgba(12, 74, 110, 0.1)',
										fill: true,
										tension: 0.4,
										pointRadius: 5,
										pointBackgroundColor: isDark
											? '#39BDF8'
											: '#0C4A6E',
										pointBorderColor: isDark
											? '#39BDF8'
											: '#0C4A6E'
									}
								]
							},
							options
						});
					}
				}
			} catch (error) {
				console.error(
					'Error cargando gráfico:',
					error
				);
			}
		}

		cargarGrafico();

		// Detectar cambios de tema
		observer = new MutationObserver(() => {
			cargarGrafico();
		});

		observer.observe(document.documentElement, {
			attributes: true,
			attributeFilter: ['class']
		});

		return () => {
			observer.disconnect();
			chart?.destroy();
		};
	});
</script>

<div
	class="rounded-lg border border-zinc-200 bg-white p-6 dark:border-[#334156] dark:bg-[#1E293B]"
>
	<div class="mb-2 flex items-center gap-2">
		<BarChart3
			size={20}
			class="text-zinc-700 dark:text-[#39BDF8]"
		/>

		<h3
			class="text-lg font-semibold text-zinc-900 dark:text-white"
		>
			Ventas Recientes
		</h3>
	</div>

	<p
		class="mb-6 text-sm text-zinc-500 dark:text-zinc-400"
	>
		Últimos 7 días de ventas
	</p>

	<div class="h-75">
		<canvas bind:this={chartContainer}></canvas>
	</div>
</div>