<script lang="ts">
	import { onMount } from 'svelte';
	import Download from 'lucide-svelte/icons/download';
	import Chart from 'chart.js/auto';
	import type { ChartData, ChartType } from 'chart.js';

	export let title = '';
	export let type: ChartType = 'bar';
	export let data: ChartData;

	// NUEVA PROP PARA GRÁFICOS HORIZONTALES
	export let horizontal = false;

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

				// Permite barras horizontales
				indexAxis: horizontal ? 'y' : 'x',

				plugins: {
					legend: {
						labels: {
							color: '#64748b',
							font: { size: 12, weight: '500' },
							padding: 16,
							usePointStyle: true
						}
					}
				},

				scales:
					type !== 'pie'
						? horizontal
							? {
									// CONFIGURACIÓN PARA BARRAS HORIZONTALES
									x: {
										beginAtZero: true,
										ticks: {
											color: '#64748b',
											font: {
												size: 11
											}
										},
										grid: {
											color: '#e2e8f0'
										}
									},

									y: {
										ticks: {
											color: '#64748b',
											font: {
												size: 11
											},
											callback: function (value: any) {
												const label = this.getLabelForValue(value);

												return label.length > 10
													? label.substring(0, 10) + '...'
													: label;
											}
										},
										grid: {
											display: false
										}
									}
								}
							: {
									// CONFIGURACIÓN ORIGINAL PARA BARRAS VERTICALES
									x: {
										ticks: {
											color: '#64748b',
											font: {
												size: 10
											},
											maxRotation: 0,
											minRotation: 0
										},
										grid: {
											display: false
										}
									},

									y: {
										beginAtZero: true,
										ticks: {
											color: '#64748b',
											font: {
												size: 11
											},
											callback: (value: any) => {
												if (type === 'line') {
													return '$' + (value / 1000).toFixed(0) + 'K';
												}

												return (
													value +
													(type === 'bar' &&
													data.datasets[0].label?.includes('%')
														? '%'
														: '')
												);
											}
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

<div class="rounded-2xl border border-slate-200 bg-white p-5">
	<div class="mb-5 flex items-center justify-between">
		<h3 class="text-lg font-semibold text-slate-800">
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

	<div class={horizontal ? 'h-[380px]' : 'h-[300px]'}>
		<canvas bind:this={canvas}></canvas>
	</div>
</div>