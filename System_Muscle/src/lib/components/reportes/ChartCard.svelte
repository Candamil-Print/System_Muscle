<script lang="ts">
	import { onMount } from 'svelte';
	import Download from 'lucide-svelte/icons/download';
	import Chart from 'chart.js/auto';
	import type { ChartData, ChartType } from 'chart.js';

	import jsPDF from 'jspdf';
	import autoTable from 'jspdf-autotable';

	export let title = '';
	export let type: ChartType = 'bar';
	export let data: ChartData;

	export let reportData: any[] = [];

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


function descargarReporteMargen() {
	if (!canvas) return;

	const doc = new jsPDF();

	// =====================
	// TÍTULO
	// =====================

	doc.setFontSize(18);
	doc.setFont('helvetica', 'bold');

	doc.text(
		'Reporte de Margen de Ganancias',
		14,
		20
	);

	doc.setFontSize(10);
	doc.setFont('helvetica', 'normal');

	doc.text(
		`Generado: ${new Date().toLocaleString('es-ES')}`,
		14,
		28
	);

	// =====================
	// GRÁFICO
	// =====================

	const image = canvas.toDataURL('image/png');

	doc.addImage(
		image,
		'PNG',
		15,
		35,
		180,
		80
	);

	// =====================
	// TABLA
	// =====================

	autoTable(doc, {
		startY: 125,

		head: [[
			'Producto',
			'Cantidad',
			'Ventas',
			'Costo',
			'Ganancia',
			'Margen %'
		]],

		body: reportData.map((p) => [
			p.nombre_producto,
			p.cantidad_vendida,
			`$${Number(
				p.total_ventas
			).toLocaleString('es-CO')}`,
			`$${Number(
				p.total_costo
			).toLocaleString('es-CO')}`,
			`$${Number(
				p.ganancia_neta
			).toLocaleString('es-CO')}`,
			`${Number(
				p.margen_porcentaje
			).toFixed(2)}%`
		])
	});

	// =====================
	// RESUMEN
	// =====================

	const finalY =
		(doc as any).lastAutoTable?.finalY || 140;

	const totalVentas = reportData.reduce(
		(sum, p) => sum + Number(p.total_ventas || 0),
		0
	);

	const totalCosto = reportData.reduce(
		(sum, p) => sum + Number(p.total_costo || 0),
		0
	);

	const totalGanancia = reportData.reduce(
		(sum, p) => sum + Number(p.ganancia_neta || 0),
		0
	);

	const margenGeneral =
		totalVentas > 0
			? (totalGanancia / totalVentas) * 100
			: 0;

	doc.setFontSize(12);
	doc.setFont('helvetica', 'bold');

	doc.text(
		`Productos analizados: ${reportData.length}`,
		14,
		finalY + 15
	);

	doc.text(
		`Ventas Totales: $${totalVentas.toLocaleString('es-CO')}`,
		14,
		finalY + 25
	);

	doc.text(
		`Costo Total: $${totalCosto.toLocaleString('es-CO')}`,
		14,
		finalY + 35
	);

	doc.text(
		`Ganancia Neta: $${totalGanancia.toLocaleString('es-CO')}`,
		14,
		finalY + 45
	);

	doc.text(
		`Margen General: ${margenGeneral.toFixed(2)}%`,
		14,
		finalY + 55
	);

	doc.save(
		`reporte-margen-${new Date()
			.toISOString()
			.split('T')[0]}.pdf`
	);
}
</script>

<div class="rounded-2xl border border-slate-200 bg-white p-5 dark:bg-[#1E293B] dark:border-[#334156]">
	<div class="mb-5 flex items-center justify-between">
		<h3 class="text-lg font-semibold text-slate-800 dark:text-[#6C7280]">
			{title}
		</h3>

	<button
	on:click={() =>
		title === 'Reporte de Margen de Ganancias'
		? descargarReporteMargen()
		: descargarGrafico()
	}
	class="h-10 w-10 rounded-lg border border-slate-200 flex items-center justify-center hover:bg-slate-50 transition-colors dark:border-[#334156]"
	title="Descargar gráfico"
	>
	<Download class="w-5 h-5 text-slate-700 dark:text-[#39BDF8]" />
	</button>
	</div>

	<div class={horizontal ? 'h-[380px]' : 'h-[300px]'}>
		<canvas bind:this={canvas}></canvas>
	</div>
</div>