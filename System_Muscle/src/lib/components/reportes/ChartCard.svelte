<script lang="ts">
	import { onMount, onDestroy, afterUpdate } from 'svelte';
	import Download from 'lucide-svelte/icons/download';
	import Chart from 'chart.js/auto';
	import type { ChartData, ChartType } from 'chart.js';
	

	import jsPDF from 'jspdf';
	import autoTable from 'jspdf-autotable';

	export let title = '';
	export let type: ChartType = 'bar';
	export let data: ChartData;

	export let reportData: any[] = []; 
	export let todosProductosData: any[] = []; 

	// NUEVA PROP PARA GRÁFICOS HORIZONTALES
	export let horizontal = false;

	let canvas: HTMLCanvasElement;
	let chart: Chart | null = null;

	let lastTheme = '';

	export let tall = false;

	function formatearFechaLarga(fecha: string) {
		const [anio, mes, dia] = fecha.split('-');

		const meses = [
			'enero',
			'febrero',
			'marzo',
			'abril',
			'mayo',
			'junio',
			'julio',
			'agosto',
			'septiembre',
			'octubre',
			'noviembre',
			'diciembre'
		];

		return `${Number(dia)} de ${meses[Number(mes) - 1]} de ${anio}`;
	}

	function formatearFechaReporte(fecha = new Date()) {
		const texto = fecha.toLocaleDateString(
			'es-CO',
			{
				day: 'numeric',
				month: 'long',
				year: 'numeric'
			}
		);

		return texto.charAt(0).toUpperCase() + texto.slice(1);
	}

	function crearGrafico() {
		if (!canvas) return;

		chart?.destroy();

		const dark = document.documentElement.classList.contains('dark');

		lastTheme = dark ? 'dark' : 'light';

		const esDona = type === 'doughnut' || type === 'pie';

		chart = new Chart(canvas, {
			type,

			data: {
				...data,

				datasets: data.datasets.map((dataset) => ({
					...dataset,

					...(esDona && {
						// elimina los bordes blancos
						borderWidth: 0,
						borderColor: 'transparent',

						// separa ligeramente al pasar el mouse
						hoverOffset: 12,

						// redondea las puntas
						borderRadius: 8
					})
				}))
			},

			options: {
				responsive: true,
				maintainAspectRatio: false,

				indexAxis: horizontal ? 'y' : 'x',

				// Configuración exclusiva para dona/pie
				...(esDona && {
					cutout: '68%'
				}),

				plugins: {
					legend: {
						position: 'top',

						labels: {
							color: dark ? '#CBD5E1' : '#64748b',

							font: {
								size: 12,
								weight: '500'
							},

							usePointStyle: true,
							pointStyle: 'circle',

							generateLabels(chart) {
								return Chart.defaults.plugins.legend.labels.generateLabels(
									chart
								).map((label) => ({
									...label,

									// mantener el color real de cada segmento
									fillStyle: label.fillStyle,
									strokeStyle: label.strokeStyle,

									lineWidth: 0
								}));
							},

							padding: 22
						}
					}
				},

				scales:
					!esDona
						? horizontal
							? {
									x: {
										beginAtZero: true,

										ticks: {
											color: dark ? '#CBD5E1' : '#64748b',

											font: {
												size: 11
											}
										},

										grid: {
											color: dark
												? '#334155'
												: '#e2e8f0'
										}
									},

									y: {
										ticks: {
											color: dark ? '#CBD5E1' : '#64748b',

											font: {
												size: 11
											},

											callback(value: any) {
												const label =
													this.getLabelForValue(value);

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
									x: {
										ticks: {
											color: dark ? '#CBD5E1' : '#64748b',

											font: {
												size: 10
											},

											autoSkip: false,
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
											color: dark ? '#CBD5E1' : '#64748b',

											font: {
												size: 11
											},

											callback: (value: any) => {
												if (type === 'line') {
													return (
														'$' +
														(value / 1000).toFixed(0) +
														'K'
													);
												}

												const label =
													data?.datasets?.[0]?.label ?? '';

												return (
													value +
													(type === 'bar' &&
													label.includes('%')
														? '%'
														: '')
												);
											}
										},

										grid: {
											color: dark
												? '#334155'
												: '#e2e8f0'
										}
									}
							}
						: {}
			}
		});
	}

	function descargarReporteVentas() {

		console.log('REPORT DATA');
		console.log(reportData);

		console.log('PRIMER REGISTRO');
		console.log(reportData[0]);

		const doc = new jsPDF('p', 'mm', 'a4');

		// ==========================
		// CALCULOS
		// ==========================

		const totalVendido = reportData.reduce(
			(sum, dia) => sum + Number(dia.total_general || 0),
			0
		);

		const numeroVentas = reportData.reduce(
			(sum, dia) => sum + Number(dia.numero_ventas || 0),
			0
		);

		const promedioDiario =
			reportData.length > 0
				? totalVendido / reportData.length
				: 0;

		const diaMasFuerte =
			reportData.length > 0
				? [...reportData].sort(
						(a, b) =>
							b.total_general - a.total_general
				)[0]
				: null;

		const diaMasDebil =
			reportData.length > 0
				? [...reportData].sort(
						(a, b) =>
							a.total_general - b.total_general
				)[0]
				: null;

		// ==========================
		// HEADER
		// ==========================

		doc.setFillColor(12, 74, 110);

		doc.rect(
			0,
			0,
			210,
			35,
			'F'
		);

		doc.setTextColor(255, 255, 255);

		doc.setFontSize(22);
		doc.setFont('helvetica', 'bold');

		doc.text(
			'REPORTE DE VENTAS',
			105,
			15,
			{
				align: 'center'
			}
		);

		doc.setFontSize(11);

		doc.text(
			`Generado: ${formatearFechaReporte()}`,
			105,
			25,
			{
				align: 'center'
			}
		);

		// ==========================
		// RESUMEN EJECUTIVO
		// ==========================

		doc.setTextColor(0, 0, 0);

		doc.setFontSize(16);
		doc.setFont('helvetica', 'bold');

		doc.text(
			'Resumen Ejecutivo',
			14,
			50
		);

		// Tarjetas KPI

		const cards = [
			[
				'Total Vendido',
				`$${totalVendido.toLocaleString(
					'es-CO'
				)}`
			],

			[
				'Número Ventas',
				numeroVentas.toString()
			],

			[
				'Promedio Diario',
				`$${Math.round(
					promedioDiario
				).toLocaleString('es-CO')}`
			]
		];

		let x = 14;

		cards.forEach(([titulo, valor]) => {
			doc.setFillColor(
				245,
				247,
				250
			);

			doc.roundedRect(
				x,
				58,
				56,
				24,
				2,
				2,
				'F'
			);

			doc.setFontSize(9);
			doc.setFont(
				'helvetica',
				'normal'
			);

			doc.text(
				titulo,
				x + 4,
				67
			);

			doc.setFontSize(12);
			doc.setFont(
				'helvetica',
				'bold'
			);

			doc.text(
				valor,
				x + 4,
				76
			);

			x += 62;
		});

	// ==========================
	// INDICADORES
	// ==========================

	doc.setFontSize(14);
	doc.setFont('helvetica', 'bold');

	doc.text(
		'Indicadores Clave',
		14,
		98
	);

	// ==========================
	// DÍA MÁS FUERTE
	// ==========================

	doc.setFillColor(245, 247, 250);

	doc.roundedRect(
		14,
		104,
		88,
		18,
		2,
		2,
		'F'
	);

	doc.setFontSize(8);
	doc.setFont('helvetica', 'normal');

	doc.text(
		'Día más fuerte',
		18,
		111
	);

	if (diaMasFuerte) {
		doc.setFontSize(9);
		doc.setFont('helvetica', 'bold');

		doc.text(
			formatearFechaLarga(
				diaMasFuerte.fecha
			),
			18,
			117
		);

		doc.text(
			`$${Number(
				diaMasFuerte.total_general
			).toLocaleString('es-CO')}`,
			65,
			117,
			{
				align: 'right'
			}
		);
	}

	// ==========================
	// DÍA MÁS DÉBIL
	// ==========================

	doc.setFillColor(245, 247, 250);

	doc.roundedRect(
		108,
		104,
		88,
		18,
		2,
		2,
		'F'
	);

	doc.setFontSize(8);
	doc.setFont('helvetica', 'normal');

	doc.text(
		'Día más débil',
		112,
		111
	);

	if (diaMasDebil) {
		doc.setFontSize(9);
		doc.setFont('helvetica', 'bold');

		doc.text(
			formatearFechaLarga(
				diaMasDebil.fecha
			),
			112,
			117
		);

		doc.text(
			`$${Number(
				diaMasDebil.total_general
			).toLocaleString('es-CO')}`,
			190,
			117,
			{
				align: 'right'
			}
		);
	}
		// ==========================
		// TABLA
		// ==========================

		autoTable(doc, {
			startY: 130,

			head: [[
				'Fecha',
				'Número de Ventas',
				'Total Vendido'
			]],

			body: reportData.map((v) => [
				formatearFechaLarga(v.fecha),

				v.numero_ventas ?? 0,

				`$${Number(
					v.total_general
				).toLocaleString('es-CO')}`
			]),
			headStyles: {
				fillColor: [12, 74, 110],
				fontStyle: 'bold'
			},

			alternateRowStyles: {
				fillColor: [245, 247, 250]
			},

			styles: {
				fontSize: 10,       // antes 9
			cellPadding: 4,     // antes 2 o 3
			valign: 'middle'
			}
		});

		// ==========================
		// FOOTER
		// ==========================

		const pageCount =
			doc.getNumberOfPages();

		for (
			let i = 1;
			i <= pageCount;
			i++
		) {
			doc.setPage(i);

			
			doc.setTextColor(
				120,
				120,
				120
			);


			doc.setFontSize(8);
			doc.setFont(
				'helvetica',
				'normal'
			);

			doc.text(
				`Generado: ${formatearFechaReporte()}`,
				105,
				25,
				{
					align: 'center'
				}
			);

			doc.text(
				`Página ${i} de ${pageCount}`,
				170,
				290
			);

			// Restaurar negro para evitar afectar otras páginas
			doc.setTextColor(
				0,
				0,
				0
			);
		}

		doc.save(
			`reporte-ventas-${new Date()
				.toISOString()
				.split('T')[0]}.pdf`
		);
	}

	onMount(() => {
		crearGrafico();
	});

	afterUpdate(() => {
		if (!chart) return;

		const dark = document.documentElement.classList.contains('dark');

		if (lastTheme !== (dark ? 'dark' : 'light')) {
			crearGrafico();
		}
	});

	$: if (chart && data) {
	chart.data = data;
	chart.update();
	}

	function descargarReporteMargen() {
		if (!canvas) return;

		const doc = new jsPDF('p', 'mm', 'a4');

		// ==========================
		// CALCULOS
		// ==========================

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

		const productoMasRentable =
			reportData.length > 0
				? [...reportData].sort(
						(a, b) =>
							b.ganancia_neta - a.ganancia_neta
				)[0]
				: null;

		const productoMayorMargen =
			reportData.length > 0
				? [...reportData].sort(
						(a, b) =>
							b.margen_porcentaje -
							a.margen_porcentaje
				)[0]
				: null;

		// ==========================
		// HEADER
		// ==========================

		doc.setFillColor(12, 74, 110);

		doc.rect(0, 0, 210, 35, 'F');

		doc.setTextColor(255, 255, 255);

		doc.setFontSize(22);
		doc.setFont('helvetica', 'bold');

		doc.text(
			'REPORTE DE MÁRGENES',
			105,
			15,
			{ align: 'center' }
		);

		doc.setFontSize(11);

		doc.text(
			`Generado: ${formatearFechaReporte()}`,
			105,
			25,
			{
				align: 'center'
			}
		);

		// ==========================
		// RESUMEN EJECUTIVO
		// ==========================

		doc.setTextColor(0, 0, 0);

		doc.setFontSize(16);
		doc.setFont('helvetica', 'bold');

		doc.text(
			'Resumen Ejecutivo',
			14,
			50
		);

		const cards = [
			[
				'Ventas Totales',
				`$${totalVentas.toLocaleString('es-CO')}`
			],
			[
				'Ganancia Neta',
				`$${totalGanancia.toLocaleString('es-CO')}`
			],
			[
				'Margen General',
				`${margenGeneral.toFixed(2)}%`
			]
		];

		let x = 14;

		cards.forEach(([titulo, valor]) => {
			doc.setFillColor(
				245,
				247,
				250
			);

			doc.roundedRect(
				x,
				58,
				56,
				24,
				2,
				2,
				'F'
			);

			doc.setFontSize(9);
			doc.setFont(
				'helvetica',
				'normal'
			);

			doc.text(
				titulo,
				x + 4,
				67
			);

			doc.setFontSize(12);
			doc.setFont(
				'helvetica',
				'bold'
			);

			doc.text(
				valor,
				x + 4,
				76
			);

			x += 62;
		});

		// ==========================
		// INDICADORES
		// ==========================

		doc.setFontSize(14);
		doc.setFont(
			'helvetica',
			'bold'
		);

		doc.text(
			'Indicadores Clave',
			14,
			98
		);

		// Producto más rentable

		doc.setFillColor(
			245,
			247,
			250
		);

		doc.roundedRect(
			14,
			104,
			88,
			18,
			2,
			2,
			'F'
		);

		doc.setFontSize(8);
		doc.setFont(
			'helvetica',
			'normal'
		);

		doc.text(
			'Más rentable',
			18,
			111
		);

		if (productoMasRentable) {
			doc.setFontSize(9);
			doc.setFont(
				'helvetica',
				'bold'
			);

			doc.text(
				productoMasRentable.nombre_producto.slice(
					0,
					20
				),
				18,
				117
			);

			doc.text(
				`$${Number(
					productoMasRentable.ganancia_neta
				).toLocaleString('es-CO')}`,
				95,
				117,
				{ align: 'right' }
			);
		}

		// Mayor margen %

		doc.setFillColor(
			245,
			247,
			250
		);

		doc.roundedRect(
			108,
			104,
			88,
			18,
			2,
			2,
			'F'
		);

		doc.setFontSize(8);
		doc.setFont(
			'helvetica',
			'normal'
		);

		doc.text(
			'Mayor margen',
			112,
			111
		);

		if (productoMayorMargen) {
			doc.setFontSize(9);
			doc.setFont(
				'helvetica',
				'bold'
			);

			doc.text(
				productoMayorMargen.nombre_producto.slice(
					0,
					20
				),
				112,
				117
			);

			doc.text(
				`${Number(
					productoMayorMargen.margen_porcentaje
				).toFixed(2)}%`,
				190,
				117,
				{ align: 'right' }
			);
		}

		// ==========================
		// TABLA
		// ==========================

	autoTable(doc, {
	startY: 130,

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
		Number(p.cantidad_vendida || 0),

		`$${Number(
		p.total_ventas || 0
		).toLocaleString('es-CO')}`,

		`$${Number(
		p.total_costo || 0
		).toLocaleString('es-CO')}`,

		`$${Number(
		p.ganancia_neta || 0
		).toLocaleString('es-CO')}`,

		`${Number(
		p.margen_porcentaje || 0
		).toFixed(2)}%`
	]),

	headStyles: {
		fillColor: [12, 74, 110],
		fontStyle: 'bold'
	},

	alternateRowStyles: {
		fillColor: [245, 247, 250]
	},

	styles: {
		fontSize: 10,
		cellPadding: 4,
		valign: 'middle'
	}
	});

		// ==========================
		// FOOTER
		// ==========================

		const pageCount =
			doc.getNumberOfPages();

		for (
			let i = 1;
			i <= pageCount;
			i++
		) {
			doc.setPage(i);

			doc.setTextColor(
				120,
				120,
				120
			);

			doc.setFontSize(8);

			doc.text(
				`Generado el ${new Date().toLocaleString('es-CO')}`,
				14,
				290
			);

			doc.text(
				`Página ${i} de ${pageCount}`,
				170,
				290
			);

			doc.setTextColor(
				0,
				0,
				0
			);
		}

		doc.save(
			`reporte-margen-${new Date()
				.toISOString()
				.split('T')[0]}.pdf`
		);
	}

	function descargarReporteTopProductos() {

		console.log('TOP 5 GRAFICO');
		console.log(reportData);

		console.log('TODOS LOS PRODUCTOS');
		console.log(todosProductosData);

		// Todos los productos para la tabla completa
		const productos = [
			...(todosProductosData.length > 0
				? todosProductosData
				: reportData)
		].sort(
			(a, b) =>
				Number(b.cantidad_vendida || 0) -
				Number(a.cantidad_vendida || 0)
		);

		// Solo Top 5 para la mini tabla
		const top5 = productos.slice(0, 5);

		const doc = new jsPDF('p', 'mm', 'a4');

		// ==========================
		// CALCULOS
		// ==========================

		const totalUnidades = productos.reduce(
			(sum, p) => sum + Number(p.cantidad_vendida || 0),
			0
		);

		const totalVentas = productos.reduce(
			(sum, p) => sum + Number(p.total_ventas || 0),
			0
		);

		const productoTop =
			productos.length > 0
				? productos[0]
				: null;

		// ==========================
		// HEADER
		// ==========================

		doc.setFillColor(12, 74, 110);
		doc.rect(0, 0, 210, 35, 'F');

		doc.setTextColor(255, 255, 255);

		doc.setFontSize(22);
		doc.setFont('helvetica', 'bold');

		doc.text(
			'REPORTE DE PRODUCTOS VENDIDOS',
			105,
			15,
			{ align: 'center' }
		);

		doc.setFontSize(11);

		doc.text(
			`Generado: ${formatearFechaReporte()}`,
			105,
			25,
			{ align: 'center' }
		);

		// ==========================
		// RESUMEN
		// ==========================

		doc.setTextColor(0, 0, 0);

		doc.setFontSize(16);
		doc.setFont('helvetica', 'bold');

		doc.text(
			'Resumen Ejecutivo',
			14,
			50
		);

		const cards = [
			[
				'Productos',
				productos.length.toString()
			],
			[
				'Unidades Vendidas',
				totalUnidades.toLocaleString('es-CO')
			],
			[
				'Ventas Totales',
				`$${totalVentas.toLocaleString('es-CO')}`
			]
		];

		let x = 14;

		cards.forEach(([titulo, valor]) => {
			doc.setFillColor(245, 247, 250);

			doc.roundedRect(
				x,
				58,
				56,
				24,
				2,
				2,
				'F'
			);

			doc.setFontSize(9);
			doc.setFont('helvetica', 'normal');

			doc.text(
				titulo,
				x + 4,
				67
			);

			doc.setFontSize(12);
			doc.setFont('helvetica', 'bold');

			doc.text(
				valor,
				x + 4,
				76
			);

			x += 62;
		});

		// ==========================
		// PRODUCTO TOP
		// ==========================

		doc.setFontSize(14);
		doc.setFont('helvetica', 'bold');

		doc.text(
			'Producto Más Vendido',
			14,
			98
		);

		doc.setFillColor(245, 247, 250);

		doc.roundedRect(
			14,
			104,
			182,
			18,
			2,
			2,
			'F'
		);

		if (productoTop) {
			doc.setFontSize(9);
			doc.setFont('helvetica', 'bold');

			doc.text(
				productoTop.nombre_producto.slice(0, 40),
				18,
				116
			);

			doc.text(
				`${productoTop.cantidad_vendida} unidades`,
				190,
				116,
				{ align: 'right' }
			);
		}

		// ==========================
		// MINI TABLA TOP 5
		// ==========================

		doc.setFontSize(14);
		doc.setFont('helvetica', 'bold');

		doc.text(
			'Ranking Top 5 Productos',
			14,
			132
		);

		autoTable(doc, {
			startY: 138,

			head: [[
				'#',
				'Producto',
				'Cantidad'
			]],

			body: top5.map((p, index) => [
				index + 1,
				p.nombre_producto,
				Number(p.cantidad_vendida || 0)
			]),

			headStyles: {
				fillColor: [12, 74, 110],
				fontStyle: 'bold'
			},

			alternateRowStyles: {
				fillColor: [245, 247, 250]
			},

			styles: {
				fontSize: 9,
				cellPadding: 3,
				valign: 'middle'
			}
		});

		// ==========================
		// TABLA COMPLETA
		// ==========================

		const inicioTabla =
			(doc as any).lastAutoTable.finalY + 12;

		doc.setFontSize(14);
		doc.setFont('helvetica', 'bold');

		doc.text(
			'Listado Completo de Productos',
			14,
			inicioTabla
		);

		autoTable(doc, {
			startY: inicioTabla + 6,

			head: [[
				'#',
				'Producto',
				'Cantidad',
				'Ventas'
			]],

			body: productos.map((p, index) => [
				index + 1,
				p.nombre_producto,
				Number(p.cantidad_vendida || 0),
				`$${Number(
					p.total_ventas || 0
				).toLocaleString('es-CO')}`
			]),

			headStyles: {
				fillColor: [12, 74, 110],
				fontStyle: 'bold'
			},

			alternateRowStyles: {
				fillColor: [245, 247, 250]
			},

			styles: {
				fontSize: 9,
				cellPadding: 3,
				valign: 'middle'
			}
		});

		// ==========================
		// FOOTER
		// ==========================

		const pageCount = doc.getNumberOfPages();

		for (let i = 1; i <= pageCount; i++) {

			doc.setPage(i);

			doc.setTextColor(120, 120, 120);

			doc.setFontSize(8);

			doc.text(
				`Generado: ${formatearFechaReporte()}`,
				14,
				290
			);

			doc.text(
				`Página ${i} de ${pageCount}`,
				170,
				290
			);

			doc.setTextColor(0, 0, 0);
		}

		doc.save(
			`reporte-productos-${new Date()
				.toISOString()
				.split('T')[0]}.pdf`
		);
	}

	function descargarReporteVentasPorVendedor() {

		const vendedores = reportData ?? [];

		const doc = new jsPDF('p', 'mm', 'a4');

		// ==========================
		// CALCULOS
		// ==========================

		const totalVendido = vendedores.reduce(
			(sum, v) => sum + Number(v.total_vendido || 0),
			0
		);

		const totalVentas = vendedores.reduce(
			(sum, v) => sum + Number(v.numero_ventas || 0),
			0
		);

		const vendedorTop =
			vendedores.length > 0
				? [...vendedores].sort(
						(a, b) =>
							Number(b.total_vendido || 0) -
							Number(a.total_vendido || 0)
				)[0]
				: null;

		const promedioVentas =
			vendedores.length > 0
				? totalVendido / vendedores.length
				: 0;

		// ==========================
		// HEADER
		// ==========================

		doc.setFillColor(12, 74, 110);

		doc.rect(0, 0, 210, 35, 'F');

		doc.setTextColor(255, 255, 255);

		doc.setFontSize(22);
		doc.setFont('helvetica', 'bold');

		doc.text(
			'REPORTE DE VENTAS POR VENDEDOR',
			105,
			15,
			{ align: 'center' }
		);

		doc.setFontSize(11);

		doc.text(
			`Generado: ${formatearFechaReporte()}`,
			105,
			25,
			{ align: 'center' }
		);

		// ==========================
		// RESUMEN
		// ==========================

		doc.setTextColor(0, 0, 0);

		doc.setFontSize(16);
		doc.setFont('helvetica', 'bold');

		doc.text(
			'Resumen Ejecutivo',
			14,
			50
		);

		const cards = [
			[
				'Vendedores',
				vendedores.length.toString()
			],
			[
				'Ventas Registradas',
				totalVentas.toLocaleString('es-CO')
			],
			[
				'Total Vendido',
				`$${totalVendido.toLocaleString('es-CO')}`
			]
		];

		let x = 14;

		cards.forEach(([titulo, valor]) => {
			doc.setFillColor(245, 247, 250);

			doc.roundedRect(
				x,
				58,
				56,
				24,
				2,
				2,
				'F'
			);

			doc.setFontSize(9);
			doc.setFont('helvetica', 'normal');

			doc.text(
				titulo,
				x + 4,
				67
			);

			doc.setFontSize(12);
			doc.setFont('helvetica', 'bold');

			doc.text(
				valor,
				x + 4,
				76
			);

			x += 62;
		});

		// ==========================
		// VENDEDOR TOP
		// ==========================

		doc.setFontSize(14);
		doc.setFont('helvetica', 'bold');

		doc.text(
			'Mejor Vendedor',
			14,
			98
		);

		doc.setFillColor(
			245,
			247,
			250
		);

		doc.roundedRect(
			14,
			104,
			182,
			18,
			2,
			2,
			'F'
		);

		if (vendedorTop) {
			doc.setFontSize(9);
			doc.setFont('helvetica', 'bold');

			doc.text(
				vendedorTop.nombre_usuario,
				18,
				116
			);

			doc.text(
				`$${Number(
					vendedorTop.total_vendido
				).toLocaleString('es-CO')}`,
				190,
				116,
				{
					align: 'right'
				}
			);
		}

		// ==========================
		// TABLA
		// ==========================

		autoTable(doc, {
			startY: 130,

			head: [[
				'#',
				'Vendedor',
				'Número Ventas',
				'Total Vendido'
			]],

			body: vendedores.map((v, index) => [
				index + 1,
				v.nombre_usuario,
				Number(v.numero_ventas || 0),
				`$${Number(
					v.total_vendido || 0
				).toLocaleString('es-CO')}`
			]),

			headStyles: {
				fillColor: [12, 74, 110],
				fontStyle: 'bold'
			},

			alternateRowStyles: {
				fillColor: [245, 247, 250]
			},

			styles: {
				fontSize: 9,
				cellPadding: 3,
				valign: 'middle'
			}
		});

		// ==========================
		// FOOTER
		// ==========================

		const pageCount =
			doc.getNumberOfPages();

		for (
			let i = 1;
			i <= pageCount;
			i++
		) {
			doc.setPage(i);

			doc.setTextColor(
				120,
				120,
				120
			);

			doc.setFontSize(8);

			doc.text(
				`Generado: ${formatearFechaReporte()}`,
				14,
				290
			);

			doc.text(
				`Página ${i} de ${pageCount}`,
				170,
				290
			);

			doc.setTextColor(
				0,
				0,
				0
			);
		}

		doc.save(
			`reporte-vendedores-${new Date()
				.toISOString()
				.split('T')[0]}.pdf`
		);
	}

function descargarReporteEntradas() {
	console.log('ENTRADAS DATA');
	console.log(reportData);

	if (
		!reportData ||
		!reportData.labels ||
		!reportData.datasets?.length
	) {
		alert('No hay datos para generar el reporte.');
		return;
	}

	// ==========================
	// CONVERTIR DATOS DEL GRÁFICO
	// ==========================

	const entradas = reportData.labels.map(
		(fecha: string, index: number) => ({
			fecha,
			total: Number(
				reportData.datasets[0].data[index] || 0
			)
		})
	);

	console.log('ENTRADAS FORMATEADAS');
	console.log(entradas);

	const doc = new jsPDF('p', 'mm', 'a4');

	// ==========================
	// CÁLCULOS
	// ==========================

	const totalEntradas = entradas.reduce(
		(sum, item) => sum + item.total,
		0
	);

	const promedioDiario =
		entradas.length > 0
			? totalEntradas / entradas.length
			: 0;

	const diaMasAlto =
		entradas.length > 0
			? [...entradas].sort(
					(a, b) => b.total - a.total
			  )[0]
			: null;

	const diaMasBajo =
		entradas.length > 0
			? [...entradas].sort(
					(a, b) => a.total - b.total
			  )[0]
			: null;

	// ==========================
	// HEADER
	// ==========================

	doc.setFillColor(12, 74, 110);

	doc.rect(
		0,
		0,
		210,
		35,
		'F'
	);

	doc.setTextColor(255, 255, 255);

	doc.setFontSize(22);
	doc.setFont('helvetica', 'bold');

	doc.text(
		'REPORTE DE ENTRADAS',
		105,
		15,
		{ align: 'center' }
	);

	doc.setFontSize(11);

	doc.text(
		`Generado: ${formatearFechaReporte()}`,
		105,
		25,
		{ align: 'center' }
	);

	// ==========================
	// RESUMEN EJECUTIVO
	// ==========================

	doc.setTextColor(0, 0, 0);

	doc.setFontSize(16);
	doc.setFont('helvetica', 'bold');

	doc.text(
		'Resumen Ejecutivo',
		14,
		50
	);

	const cards = [
		[
			'Total Entradas',
			totalEntradas.toLocaleString('es-CO')
		],
		[
			'Días Analizados',
			entradas.length.toString()
		],
		[
			'Promedio Diario',
			Math.round(promedioDiario).toLocaleString('es-CO')
		]
	];

	let x = 14;

	cards.forEach(([titulo, valor]) => {
		doc.setFillColor(
			245,
			247,
			250
		);

		doc.roundedRect(
			x,
			58,
			56,
			24,
			2,
			2,
			'F'
		);

		doc.setFontSize(9);
		doc.setFont(
			'helvetica',
			'normal'
		);

		doc.text(
			titulo,
			x + 4,
			67
		);

		doc.setFontSize(12);
		doc.setFont(
			'helvetica',
			'bold'
		);

		doc.text(
			String(valor),
			x + 4,
			76
		);

		x += 62;
	});

	// ==========================
	// INDICADORES
	// ==========================

	doc.setFontSize(14);
	doc.setFont('helvetica', 'bold');

	doc.text(
		'Indicadores Clave',
		14,
		98
	);

	// Día con más entradas

	doc.setFillColor(
		245,
		247,
		250
	);

	doc.roundedRect(
		14,
		104,
		88,
		18,
		2,
		2,
		'F'
	);

	doc.setFontSize(8);
	doc.setFont('helvetica', 'normal');

	doc.text(
		'Día con más entradas',
		18,
		111
	);

	if (diaMasAlto) {
		doc.setFontSize(9);
		doc.setFont(
			'helvetica',
			'bold'
		);

		doc.text(
			diaMasAlto.fecha,
			18,
			117
		);

		doc.text(
			diaMasAlto.total.toLocaleString('es-CO'),
			90,
			117,
			{
				align: 'right'
			}
		);
	}

	// Día con menos entradas

	doc.setFillColor(
		245,
		247,
		250
	);

	doc.roundedRect(
		108,
		104,
		88,
		18,
		2,
		2,
		'F'
	);

	doc.setFontSize(8);
	doc.setFont('helvetica', 'normal');

	doc.text(
		'Día con menos entradas',
		112,
		111
	);

	if (diaMasBajo) {
		doc.setFontSize(9);
		doc.setFont(
			'helvetica',
			'bold'
		);

		doc.text(
			diaMasBajo.fecha,
			112,
			117
		);

		doc.text(
			diaMasBajo.total.toLocaleString('es-CO'),
			190,
			117,
			{
				align: 'right'
			}
		);
	}

	// ==========================
	// TABLA
	// ==========================

	autoTable(doc, {
		startY: 130,

		head: [[
			'Fecha',
			'Total Entradas'
		]],

		body: entradas.map((item) => [
			item.fecha,
			item.total.toLocaleString('es-CO')
		]),

		headStyles: {
			fillColor: [12, 74, 110],
			fontStyle: 'bold'
		},

		alternateRowStyles: {
			fillColor: [245, 247, 250]
		},

		styles: {
			fontSize: 10,
			cellPadding: 4,
			valign: 'middle'
		}
	});

	// ==========================
	// FOOTER
	// ==========================

	const pageCount =
		doc.getNumberOfPages();

	for (
		let i = 1;
		i <= pageCount;
		i++
	) {
		doc.setPage(i);

		doc.setTextColor(
			120,
			120,
			120
		);

		doc.setFontSize(8);
		doc.setFont(
			'helvetica',
			'normal'
		);

		doc.text(
			`Generado: ${formatearFechaReporte()}`,
			105,
			285,
			{
				align: 'center'
			}
		);

		doc.text(
			`Página ${i} de ${pageCount}`,
			170,
			290
		);

		doc.setTextColor(
			0,
			0,
			0
		);
	}

	doc.save(
		`reporte-entradas-${new Date()
			.toISOString()
			.split('T')[0]}.pdf`
	);
}
</script>

<div class="rounded-2xl border border-slate-200 bg-white p-5 dark:bg-[#1E293B] dark:border-[#334156]">
	<div class="mb-2 flex items-center justify-between">
		<h3 class="text-lg font-semibold text-slate-800 dark:text-white">
			{title}
		</h3>

	<button
		on:click={() => {
				if (title === 'Top 5 Productos Vendidos') {
			descargarReporteTopProductos();
			}
			else if (title === 'Reporte de Margen de Ganancias') {
				descargarReporteMargen();
			}
			else if (title === 'Ventas por Día') {
				descargarReporteVentas();
			}
			else if (title === 'Ventas por Vendedor') {
				descargarReporteVentasPorVendedor();
			}
			else if (title === 'Entradas por Día')
				descargarReporteEntradas()
			else {
				// descargarGrafico();
			}
	}}
	class="h-10 w-10 rounded-lg border border-slate-200 flex items-center justify-center hover:bg-slate-50 transition-colors dark:border-[#334156]"
	title="Descargar gráfico"
	>
	<Download class="w-5 h-5 text-slate-700 dark:text-[#39BDF8]" />
	</button>
	</div>

	<div
		class={
			horizontal
			? 'h-[380px]'
			: tall
				? 'h-[320px]'
				: 'h-[300px]'
		}
		>
		<canvas bind:this={canvas}></canvas>
	</div>
</div>