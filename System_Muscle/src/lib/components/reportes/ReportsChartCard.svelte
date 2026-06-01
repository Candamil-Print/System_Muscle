<script lang="ts">
	import { onMount } from 'svelte';
	import Download from 'lucide-svelte/icons/download';
	import Chart from 'chart.js/auto';
	import type { ChartData, ChartType } from 'chart.js';

	export let title = '';
	export let type: ChartType = 'bar';
	export let data: ChartData;

	let canvas: HTMLCanvasElement;
	let chart: Chart | null = null;

	onMount(() => {
		if (!canvas) return;

		chart = new Chart(canvas, {
			type,
			data,

			options: {
				responsive: true,
				maintainAspectRatio: false,

				plugins: {
					legend: {
						labels: {
							color: '#64748b'
						}
					}
				},

				scales:
					type !== 'pie'
						? {
								x: {
									ticks: {
										color: '#64748b'
									},
									grid: {
										display: false
									}
								},

								y: {
									ticks: {
										color: '#64748b'
									},
									grid: {
										color: '#e2e8f0'
									}
								}
							}
						: {}
			}
		});

		return () => {
			chart?.destroy();
		};
	});

	$: if (chart && data) {
		chart.data = data;
		chart.update('none');
	}

	function descargarGrafico() {
		if (!canvas) return;
		
		const image = canvas.toDataURL('image/png');
		const link = document.createElement('a');
		link.href = image;
		link.download = `grafico-${title}-${new Date().toISOString().split('T')[0]}.png`;
		document.body.appendChild(link);
		link.click();
		document.body.removeChild(link);
	}
</script>

<div
	class="rounded-2xl border border-slate-200 bg-white p-5"
>
	<div class="mb-5 flex items-center justify-between">
		<h3 class="text-xl font-semibold text-slate-800">
			{title}
		</h3>

		<button
			on:click={descargarGrafico}
			class="h-10 w-10 rounded-lg border border-slate-200 flex items-center justify-center hover:bg-slate-50 transition-colors"
			title="Descargar gráfico"
		>
			<Download class="w-5 h-5 text-slate-700" />
		</button>
	</div>

	<div class="h-[300px]">
		<canvas bind:this={canvas}></canvas>
	</div>
</div>