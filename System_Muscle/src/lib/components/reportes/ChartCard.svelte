<script lang="ts">
	import { onMount, onDestroy, afterUpdate } from 'svelte';
	import Download from 'lucide-svelte/icons/download';
	import Chart from 'chart.js/auto';
	import { toast } from 'svelte-sonner';
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

	function formatearFechaHora(fecha = new Date()) {
		const fechaTexto = fecha.toLocaleDateString(
		'es-CO',
		{
			day: 'numeric',
			month: 'long',
			year: 'numeric'
		}
		);

		const horaTexto = fecha.toLocaleTimeString(
		'es-CO',
		{
			hour: 'numeric',
			minute: '2-digit',
			hour12: true
		}
		);

		return {
		fecha:
			fechaTexto.charAt(0).toUpperCase() +
			fechaTexto.slice(1),
		hora: horaTexto
		};
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
		try {
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

		const fechaGeneracion = formatearFechaHora(new Date());

		// ==========================
		// HEADER
		// ==========================

		doc.setFillColor(12, 74, 110);
		doc.rect(0, 0, 210, 42, "F");

		doc.setDrawColor(200, 200, 200);
		doc.setLineWidth(0.6);
		doc.line(0, 42, 210, 42);

		// Título

		doc.setTextColor(255,255,255);
		doc.setFont("helvetica","bold");
		doc.setFontSize(24);

		doc.text(
			"REPORTE DE VENTAS",
			105,
			16,
			{
				align:"center"
			}
		);

		doc.setFont("helvetica","normal");
		doc.setFontSize(10);

		doc.text(
			"Sistema de Gestión de Inventario",
			105,
			24,
			{
				align:"center"
			}
		);

		doc.text(
			`${fechaGeneracion.fecha} • ${fechaGeneracion.hora}`,
			105,
			31,
			{
				align:"center"
			}
		);

		// ==========================
		// RESUMEN EJECUTIVO
		// ==========================

		doc.setTextColor(12, 74, 110);
		doc.setFont("helvetica","bold");
		doc.setFontSize(16);

		doc.text(
			"Resumen Ejecutivo",
			14,
			50
		);

		doc.setDrawColor(12,74,110);
		doc.setLineWidth(.4);
		doc.line(
			14,
			52,
			196,
			52
		);

		const cards = [
			{
				titulo:"Total Vendido",
				valor:`$${totalVendido.toLocaleString("es-CO")}`
			},
			{
				titulo:"Número Ventas",
				valor:numeroVentas.toString()
			},
			{
				titulo:"Promedio Diario",
				valor:`$${Math.round(promedioDiario).toLocaleString("es-CO")}`
			}
		];

		let x = 14;

		cards.forEach(({titulo,valor})=>{

			// sombra
			doc.setFillColor(228,232,236);

			doc.roundedRect(
				x+1,
				59,
				56,
				26,
				3,
				3,
				"F"
			);

			// tarjeta

			doc.setFillColor(250,250,250);

			doc.roundedRect(
				x,
				58,
				56,
				26,
				3,
				3,
				"F"
			);

			// barra azul

			doc.setFillColor(12,74,110);

			doc.roundedRect(
				x,
				58,
				56,
				3,
				3,
				3,
				"F"
			);

			// titulo

			doc.setTextColor(130,130,130);
			doc.setFont("helvetica","normal");
			doc.setFontSize(9);

			doc.text(
				titulo,
				x+4,
				68
			);

			// valor

			doc.setTextColor(12,74,110);
			doc.setFont("helvetica","bold");
			doc.setFontSize(16);

			doc.text(
				valor,
				x+4,
				79
			);

			x+=62;

		});

	// ==========================
	// INDICADORES
	// ==========================

	doc.setTextColor(12,74,110);
	doc.setFont("helvetica","bold");
	doc.setFontSize(15);

	doc.text(
		"Indicador Principal",
		14,
		98
	);

	doc.setDrawColor(12,74,110);
	doc.setLineWidth(.4);
	doc.line(
		14,
		100,
		196,
		100
	);

	// sombra

	doc.setFillColor(228,232,236);

	doc.roundedRect(
		15,
		105,
		182,
		28,
		3,
		3,
		"F"
	);

	// tarjeta

	doc.setFillColor(250,250,250);

	doc.roundedRect(
		14,
		104,
		182,
		28,
		3,
		3,
		"F"
	);

	// barra azul

	doc.setFillColor(12,74,110);

	doc.roundedRect(
		14,
		104,
		182,
		2.5,
		3,
		3,
		"F"
	);

	// ==========================
	// DÍA MÁS FUERTE
	// ==========================

	doc.setFillColor(245, 247, 250);

	doc.roundedRect(
		18,
		108,
		78,
		18,
		2,
		2,
		"F"
	);

	doc.setFontSize(8);
	doc.setFont('helvetica', 'normal');

	doc.text(
		'Día más fuerte',
		22,
		114
	);

	if (diaMasFuerte) {
		doc.setFontSize(9);
		doc.setFont('helvetica', 'bold');

		doc.text(
			formatearFechaLarga(
				diaMasFuerte.fecha
			),
			22,
			121
		);

		doc.text(
			`$${Number(
				diaMasFuerte.total_general
			).toLocaleString('es-CO')}`,
			92,
			121,
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
			114,
			108,
			78,
			18,
			2,
			2,
			"F"
		);

		doc.setFontSize(8);
		doc.setFont('helvetica', 'normal');

		doc.text(
			'Día más débil',
			118,
			114
		);

		if (diaMasDebil) {
			doc.setFontSize(9);
			doc.setFont('helvetica', 'bold');

			doc.text(
				formatearFechaLarga(
					diaMasDebil.fecha
				),
				118,
				121
			);

			doc.text(
				`$${Number(
					diaMasDebil.total_general
				).toLocaleString('es-CO')}`,
				188,
				121,
				{
					align: 'right'
				}
			);
		}
			// ==========================
			// TABLA
			// ==========================

			autoTable(doc, {
				startY: 140,

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

				headStyles:{
					fillColor:[12,74,110],
					textColor:[255,255,255],
					fontStyle:"bold",
					fontSize:10,
					halign:"center"
				},

				alternateRowStyles:{
					fillColor:[248,249,250]
				},

				styles:{
					font:"helvetica",
					fontSize:9,
					cellPadding:4,
					lineColor:[225,230,235],
					lineWidth:.2,
					valign:"middle"
				}
			});

			// ==========================
			// FOOTER
			// ==========================

			const pages = doc.getNumberOfPages();

			for(let i=1;i<=pages;i++){

				doc.setPage(i);

				doc.setDrawColor(220,220,220);
				doc.setLineWidth(.3);

				doc.line(
					14,
					285,
					196,
					285
				);

				doc.setFont("helvetica","normal");
				doc.setFontSize(8);
				doc.setTextColor(120,120,120);

				doc.text(
					"Sistema de Gestión de Inventario",
					14,
					290
				);

				doc.text(
					`${fechaGeneracion.fecha} • ${fechaGeneracion.hora}`,
					105,
					290,
					{
						align:"center"
					}
				);

				doc.text(
					`Página ${i} de ${pages}`,
					196,
					290,
					{
						align:"right"
					}
				);
			}

			doc.save(
				`reporte-ventas-${new Date()
					.toISOString()
					.split('T')[0]}.pdf`
			);

			toast.success('Reporte descargado correctamente.');
		} catch (err) {
			console.error(err);
			toast.error('Error al generar el reporte.');
		}
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

	// ==========================
    // GENERADOR PDF MARGEN DE GANANCIA
    // ==========================
	function descargarReporteMargen() {
		try {
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

			const fechaGeneracion = formatearFechaHora(new Date());

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

			doc.setTextColor(12, 74, 110);
			doc.setFont("helvetica","bold");
			doc.setFontSize(16);

			doc.text(
				'Resumen Ejecutivo',
				14,
				50
			);

			doc.setDrawColor(12,74,110);
			doc.setLineWidth(.4);
			doc.line(
				14,
				52,
				196,
				52
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

				// sombra
				doc.setFillColor(228,232,236);

				doc.roundedRect(
					x+1,
					59,
					56,
					26,
					3,
					3,
					"F"
				);

				// tarjeta
				doc.setFillColor(250,250,250);

				doc.roundedRect(
					x,
					58,
					56,
					26,
					3,
					3,
					"F"
				);

				// barra azul
				doc.setFillColor(12,74,110);

				doc.roundedRect(
					x,
					58,
					56,
					3,
					3,
					3,
					"F"
				);

				// titulo
				doc.setTextColor(130,130,130);
				doc.setFont("helvetica","normal");
				doc.setFontSize(9);

				doc.text(
					titulo,
					x+4,
					68
				);

				// valor
				doc.setTextColor(12,74,110);
				doc.setFont("helvetica","bold");
				doc.setFontSize(16);

				doc.text(
					valor,
					x+4,
					79
				);

				x+=62;
			});

			// ==========================
			// INDICADORES
			// ==========================

			doc.setTextColor(12,74,110);
			doc.setFont("helvetica","bold");
			doc.setFontSize(15);

			doc.text(
				'Indicadores Clave',
				14,
				98
			);

			doc.setDrawColor(12,74,110);
			doc.setLineWidth(.4);
			doc.line(
				14,
				100,
				196,
				100
			);

			// sombra
			doc.setFillColor(228,232,236);

			doc.roundedRect(
				15,
				105,
				182,
				28,
				3,
				3,
				"F"
			);

			// tarjeta
			doc.setFillColor(250,250,250);

			doc.roundedRect(
				14,
				104,
				182,
				28,
				3,
				3,
				"F"
			);

			// barra azul
			doc.setFillColor(12,74,110);

			doc.roundedRect(
				14,
				104,
				182,
				2.5,
				3,
				3,
				"F"
			);

			// ==========================
			// MÁS RENTABLE
			// ==========================

			doc.setFillColor(245, 247, 250);

			doc.roundedRect(
				18,
				108,
				78,
				18,
				2,
				2,
				"F"
			);

			doc.setTextColor(90, 90, 90);
			doc.setFont("helvetica", "normal");
			doc.setFontSize(8);

			doc.text(
				"Más rentable",
				22,
				114
			);

			if (productoMasRentable) {

				doc.setTextColor(12,74,110);
				doc.setFont("helvetica","bold");
				doc.setFontSize(9);

				doc.text(
					productoMasRentable.nombre_producto.slice(0,20),
					22,
					121
				);

				doc.text(
					`$${Number(
						productoMasRentable.ganancia_neta
					).toLocaleString("es-CO")}`,
					92,
					121,
					{
						align:"right"
					}
				);
			}

			// ==========================
			// MAYOR MARGEN
			// ==========================

			doc.setFillColor(245,247,250);

			doc.roundedRect(
				114,
				108,
				78,
				18,
				2,
				2,
				"F"
			);

			doc.setTextColor(90,90,90);
			doc.setFont("helvetica","normal");
			doc.setFontSize(8);

			doc.text(
				"Mayor margen",
				118,
				114
			);

			if (productoMayorMargen) {

				doc.setTextColor(12,74,110);
				doc.setFont("helvetica","bold");
				doc.setFontSize(9);

				doc.text(
					productoMayorMargen.nombre_producto.slice(0,20),
					118,
					121
				);

				doc.text(
					`${Number(
						productoMayorMargen.margen_porcentaje
					).toFixed(2)}%`,
					188,
					121,
					{
						align:"right"
					}
				);
			}

			// ==========================
			// TABLA
			// ==========================

			autoTable(doc, {
				startY: 140,

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

				headStyles:{
					fillColor:[12,74,110],
					textColor:[255,255,255],
					fontStyle:"bold",
					fontSize:10,
					halign:"center"
				},

				alternateRowStyles:{
					fillColor:[248,249,250]
				},

				styles:{
					font:"helvetica",
					fontSize:9,
					cellPadding:4,
					lineColor:[225,230,235],
					lineWidth:.2,
					valign:"middle"
				}
			});

			// ==========================
			// FOOTER
			// ==========================

			const pages = doc.getNumberOfPages();

			for(let i=1;i<=pages;i++){

				doc.setPage(i);

				doc.setDrawColor(220,220,220);
				doc.setLineWidth(.3);

				doc.line(
					14,
					285,
					196,
					285
				);

				doc.setFont("helvetica","normal");
				doc.setFontSize(8);
				doc.setTextColor(120,120,120);

				doc.text(
					"Sistema de Gestión de Inventario",
					14,
					290
				);

				doc.text(
					`${fechaGeneracion.fecha} • ${fechaGeneracion.hora}`,
					105,
					290,
					{
						align:"center"
					}
				);

				doc.text(
					`Página ${i} de ${pages}`,
					196,
					290,
					{
						align:"right"
					}
				);
			}

			doc.save(
				`reporte-margen-${new Date()
					.toISOString()
					.split('T')[0]}.pdf`
			);

			toast.success('Reporte descargado correctamente.');

		} catch (err) {
			console.error(err);
			toast.error('Error al generar el reporte.');
		}
	}

	// ==========================
    // GENERADOR PDF TOP PRODUCTOS VENDIDOS
    // ==========================
	function descargarReporteTopProductos() {
		try {
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

			const fechaGeneracion = formatearFechaHora(new Date());

			// ==========================
			// HEADER
			// ==========================

			doc.setFillColor(12, 74, 110);
			doc.rect(0, 0, 210, 42, "F");

			doc.setDrawColor(200, 200, 200);
			doc.setLineWidth(0.6);
			doc.line(0, 42, 210, 42);

			// Título
			doc.setTextColor(255,255,255);
			doc.setFont("helvetica","bold");
			doc.setFontSize(24);

			doc.text(
				'REPORTE DE PRODUCTOS VENDIDOS',
				105,
				16,
				{ align: 'center' }
			);

			doc.setFont("helvetica","normal");
			doc.setFontSize(10);

			doc.text(
				"Sistema de Gestión de Inventario",
				105,
				24,
				{
					align:"center"
				}
			);

			doc.text(
				`${fechaGeneracion.fecha} • ${fechaGeneracion.hora}`,
				105,
				31,
				{
					align:"center"
				}
			);

			// ==========================
			// RESUMEN EJECUTIVO
			// ==========================

			doc.setTextColor(12, 74, 110);
			doc.setFont("helvetica","bold");
			doc.setFontSize(16);

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
				// sombra
				doc.setFillColor(228,232,236);

				doc.roundedRect(
					x+1,
					59,
					56,
					26,
					3,
					3,
					"F"
				);

				// tarjeta
				doc.setFillColor(250,250,250);

				doc.roundedRect(
					x,
					58,
					56,
					26,
					3,
					3,
					"F"
				);

				// barra azul

				doc.setFillColor(12,74,110);

				doc.roundedRect(
					x,
					58,
					56,
					3,
					3,
					3,
					"F"
				);

				// titulo

				doc.setTextColor(130,130,130);
				doc.setFont("helvetica","normal");
				doc.setFontSize(9);

				doc.text(
					titulo,
					x+4,
					68
				);

				// valor

				doc.setTextColor(12,74,110);
				doc.setFont("helvetica","bold");
				doc.setFontSize(16);

				doc.text(
					valor,
					x+4,
					79
				);

				x+=62;

			});

			// ==========================
			// PRODUCTO TOP
			// ==========================

			doc.setTextColor(12, 74, 110);
			doc.setFont("helvetica", "bold");
			doc.setFontSize(15);

			doc.text(
				"Producto Más Vendido",
				14,
				98
			);

			doc.setDrawColor(12, 74, 110);
			doc.setLineWidth(.4);

			doc.line(
				14,
				100,
				196,
				100
			);

			// ==========================
			// CONTENEDOR
			// ==========================

			// sombra
			doc.setFillColor(228, 232, 236);

			doc.roundedRect(
				15,
				105,
				182,
				28,
				3,
				3,
				"F"
			);

			// tarjeta
			doc.setFillColor(250, 250, 250);

			doc.roundedRect(
				14,
				104,
				182,
				28,
				3,
				3,
				"F"
			);

			// barra azul
			doc.setFillColor(12, 74, 110);

			doc.roundedRect(
				14,
				104,
				182,
				2.5,
				3,
				3,
				"F"
			);

			// ==========================
			// TARJETA INTERNA
			// ==========================

			doc.setFillColor(245, 247, 250);

			doc.roundedRect(
				18,
				108,
				174,
				18,
				2,
				2,
				"F"
			);

			// título
			doc.setTextColor(130, 130, 130);
			doc.setFont("helvetica", "normal");
			doc.setFontSize(8);

			doc.text(
				"Producto más vendido",
				22,
				114
			);

			if (productoTop) {

				// nombre
				doc.setTextColor(12, 74, 110);
				doc.setFont("helvetica", "bold");
				doc.setFontSize(9);

				doc.text(
					productoTop.nombre_producto.slice(0, 38),
					22,
					121
				);

				// cantidad
				doc.text(
					`${productoTop.cantidad_vendida} unidades`,
					188,
					121,
					{
						align: "right"
					}
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
				148
			);

			autoTable(doc, {
				startY: 153,

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

				headStyles:{
					fillColor:[12,74,110],
					textColor:[255,255,255],
					fontStyle:"bold",
					fontSize:10,
					halign:"center"
				},

				alternateRowStyles:{
					fillColor:[248,249,250]
				},

				styles:{
					font:"helvetica",
					fontSize:9,
					cellPadding:4,
					lineColor:[225,230,235],
					lineWidth:.2,
					valign:"middle"
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

				headStyles:{
					fillColor:[12,74,110],
					textColor:[255,255,255],
					fontStyle:"bold",
					fontSize:10,
					halign:"center"
				},

				alternateRowStyles:{
					fillColor:[248,249,250]
				},

				styles:{
					font:"helvetica",
					fontSize:9,
					cellPadding:4,
					lineColor:[225,230,235],
					lineWidth:.2,
					valign:"middle"
				}
			});

			// ==========================
			// FOOTER
			// ==========================

			const pages = doc.getNumberOfPages();

			for(let i=1;i<=pages;i++){

				doc.setPage(i);

				doc.setDrawColor(220,220,220);
				doc.setLineWidth(.3);

				doc.line(
					14,
					285,
					196,
					285
				);

				doc.setFont("helvetica","normal");
				doc.setFontSize(8);
				doc.setTextColor(120,120,120);

				doc.text(
					"Sistema de Gestión de Inventario",
					14,
					290
				);

				doc.text(
					`${fechaGeneracion.fecha} • ${fechaGeneracion.hora}`,
					105,
					290,
					{
						align:"center"
					}
				);

				doc.text(
					`Página ${i} de ${pages}`,
					196,
					290,
					{
						align:"right"
					}
				);
			}

			doc.save(
				`reporte-productos-${new Date()
					.toISOString()
					.split('T')[0]}.pdf`
			);

			toast.success('Reporte descargado correctamente.');

		} catch (err) {
			console.error(err);
			toast.error('Error al generar el reporte.');
		}
	}


	// ==========================
	// GENERADOR PDF VENTAS POR VENDEDOR
	// ==========================
	function descargarReporteVentasPorVendedor() {
		try {
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

			const fechaGeneracion = formatearFechaHora(new Date());

			// ==========================
			// HEADER
			// ==========================

			doc.setFillColor(12, 74, 110);
			doc.rect(0, 0, 210, 42, "F");

			doc.setDrawColor(200, 200, 200);
			doc.setLineWidth(0.6);
			doc.line(0, 42, 210, 42);

			// Título
			doc.setTextColor(255,255,255);
			doc.setFont("helvetica","bold");
			doc.setFontSize(24);

			doc.text(
				'REPORTE DE VENTAS POR VENDEDOR',
				105,
				15,
				{ align: 'center' }
			);

			doc.setFont("helvetica","normal");
			doc.setFontSize(10);

			doc.text(
				"Sistema de Gestión de Inventario",
				105,
				24,
				{
					align:"center"
				}
			);

			doc.text(
				`${fechaGeneracion.fecha} • ${fechaGeneracion.hora}`,
				105,
				31,
				{
					align:"center"
				}
			);

			// ==========================
			// RESUMEN
			// ==========================

			doc.setTextColor(12, 74, 110);
			doc.setFont("helvetica","bold");
			doc.setFontSize(16);

			doc.text(
				'Resumen Ejecutivo',
				14,
				50
			);

			doc.setDrawColor(12,74,110);
			doc.setLineWidth(.4);
			doc.line(
				14,
				52,
				196,
				52
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
				// sombra
				doc.setFillColor(228,232,236);

				doc.roundedRect(
					x+1,
					59,
					56,
					26,
					3,
					3,
					"F"
				);

				// tarjeta
				doc.setFillColor(250,250,250);

				doc.roundedRect(
					x,
					58,
					56,
					26,
					3,
					3,
					"F"
				);

				// barra azul
				doc.setFillColor(12,74,110);

				doc.roundedRect(
					x,
					58,
					56,
					3,
					3,
					3,
					"F"
				);

				// titulo
				doc.setTextColor(130,130,130);
				doc.setFont("helvetica","normal");
				doc.setFontSize(9);

				doc.text(
					titulo,
					x+4,
					68
				);

				// valor
				doc.setTextColor(12,74,110);
				doc.setFont("helvetica","bold");
				doc.setFontSize(16);

				doc.text(
					valor,
					x+4,
					79
				);

				x+=62;

			});

			// ==========================
			// VENDEDOR TOP
			// ==========================

			doc.setTextColor(12, 74, 110);
			doc.setFont("helvetica", "bold");
			doc.setFontSize(15);

			doc.text(
				"Mejor Vendedor",
				14,
				98
			);

			doc.setDrawColor(12, 74, 110);
			doc.setLineWidth(.4);

			doc.line(
				14,
				100,
				196,
				100
			);

			// ==========================
			// CONTENEDOR
			// ==========================

			// sombra
			doc.setFillColor(228, 232, 236);

			doc.roundedRect(
				15,
				105,
				182,
				28,
				3,
				3,
				"F"
			);

			// tarjeta
			doc.setFillColor(250, 250, 250);

			doc.roundedRect(
				14,
				104,
				182,
				28,
				3,
				3,
				"F"
			);

			// barra azul
			doc.setFillColor(12, 74, 110);

			doc.roundedRect(
				14,
				104,
				182,
				2.5,
				3,
				3,
				"F"
			);

			// ==========================
			// TARJETA INTERNA
			// ==========================

			doc.setFillColor(245, 247, 250);

			doc.roundedRect(
				18,
				108,
				174,
				18,
				2,
				2,
				"F"
			);

			// título
			doc.setTextColor(130, 130, 130);
			doc.setFont("helvetica", "normal");
			doc.setFontSize(8);

			doc.text(
				"Vendedor con mayores ventas",
				22,
				114
			);

			if (vendedorTop) {

				// nombre
				doc.setTextColor(12, 74, 110);
				doc.setFont("helvetica", "bold");
				doc.setFontSize(9);

				doc.text(
					vendedorTop.nombre_usuario,
					22,
					121
				);

				// total vendido
				doc.text(
					`$${Number(
						vendedorTop.total_vendido
					).toLocaleString("es-CO")}`,
					188,
					121,
					{
						align: "right"
					}
				);
			}
			// ==========================
			// TABLA
			// ==========================

			autoTable(doc, {
				startY: 140,

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

				headStyles:{
					fillColor:[12,74,110],
					textColor:[255,255,255],
					fontStyle:"bold",
					fontSize:10,
					halign:"center"
				},

				alternateRowStyles:{
					fillColor:[248,249,250]
				},

				styles:{
					font:"helvetica",
					fontSize:9,
					cellPadding:4,
					lineColor:[225,230,235],
					lineWidth:.2,
					valign:"middle"
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

 			toast.success('Reporte descargado correctamente.');

		} catch (err) {
			console.error(err);
			toast.error('Error al generar el reporte.');
		}
	}

	// ==========================
	// PDF DE REPORTE ENTRADA
	// ==========================

// ==========================
// GENERADOR PDF ENTRADAS POR DÍA
// ==========================
function descargarReporteEntradas() {
	try {
		if (!reportData || reportData.length === 0) {
			toast.error("No hay datos para generar el reporte.");
			return;
		}

		// ==========================
		// CONVERTIR DATOS DEL GRÁFICO
		// ==========================

		const entradas = reportData.map((item) => ({
			fecha: item.fecha,
			total: Number(item.total_entradas)
		}));

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

		const fechaGeneracion = formatearFechaHora(new Date());

		// ==========================
		// HEADER
		// ==========================

		doc.setFillColor(12, 74, 110);
		doc.rect(0, 0, 210, 35, 'F');

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
			{
				align: 'center'
			}
		);

		// ==========================
		// RESUMEN EJECUTIVO
		// ==========================

		doc.setTextColor(12, 74, 110);
		doc.setFont("helvetica","bold");
		doc.setFontSize(16);

		doc.text(
			'Resumen Ejecutivo',
			14,
			50
		);

		doc.setDrawColor(12,74,110);
		doc.setLineWidth(.4);
		doc.line(
			14,
			52,
			196,
			52
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
			// sombra
			doc.setFillColor(228,232,236);

			doc.roundedRect(
				x+1,
				59,
				56,
				26,
				3,
				3,
				"F"
			);

			// tarjeta
			doc.setFillColor(250,250,250);

			doc.roundedRect(
				x,
				58,
				56,
				26,
				3,
				3,
				"F"
			);

			// barra azul
			doc.setFillColor(12,74,110);

			doc.roundedRect(
				x,
				58,
				56,
				3,
				3,
				3,
				"F"
			);

			// titulo
			doc.setTextColor(130,130,130);
			doc.setFont("helvetica","normal");
			doc.setFontSize(9);

			doc.text(
				titulo,
				x+4,
				68
			);

			// valor
			doc.setTextColor(12,74,110);
			doc.setFont("helvetica","bold");
			doc.setFontSize(16);

			doc.text(
				valor,
				x+4,
				79
			);

			x+=62;

		});

		// ==========================
		// INDICADORES
		// ==========================

		doc.setTextColor(12,74,110);
		doc.setFont("helvetica","bold");
		doc.setFontSize(15);

		doc.text(
			"Indicadores Clave",
			14,
			98
		);

		doc.setDrawColor(12,74,110);
		doc.setLineWidth(.4);
		doc.line(
			14,
			100,
			196,
			100
		);

		// ==========================
		// CONTENEDOR
		// ==========================

		// sombra
		doc.setFillColor(228,232,236);

		doc.roundedRect(
			15,
			105,
			182,
			28,
			3,
			3,
			"F"
		);

		// tarjeta
		doc.setFillColor(250,250,250);

		doc.roundedRect(
			14,
			104,
			182,
			28,
			3,
			3,
			"F"
		);

		// barra azul
		doc.setFillColor(12,74,110);

		doc.roundedRect(
			14,
			104,
			182,
			2.5,
			3,
			3,
			"F"
		);

		// ==========================
		// DÍA CON MÁS ENTRADAS
		// ==========================

		doc.setFillColor(245,247,250);

		doc.roundedRect(
			18,
			108,
			78,
			18,
			2,
			2,
			"F"
		);

		doc.setTextColor(90,90,90);
		doc.setFont("helvetica","normal");
		doc.setFontSize(8);

		doc.text(
			"Día con más entradas",
			22,
			114
		);

		if (diaMasAlto) {

			doc.setTextColor(12,74,110);
			doc.setFont("helvetica","bold");
			doc.setFontSize(9);

			doc.text(
				diaMasAlto.fecha,
				22,
				121
			);

			doc.text(
				diaMasAlto.total.toLocaleString("es-CO"),
				92,
				121,
				{
					align:"right"
				}
			);
		}

		// ==========================
		// DÍA CON MENOS ENTRADAS
		// ==========================

		doc.setFillColor(245,247,250);

		doc.roundedRect(
			114,
			108,
			78,
			18,
			2,
			2,
			"F"
		);

		doc.setTextColor(90,90,90);
		doc.setFont("helvetica","normal");
		doc.setFontSize(8);

		doc.text(
			"Día con menos entradas",
			118,
			114
		);

		if (diaMasBajo) {

			doc.setTextColor(12,74,110);
			doc.setFont("helvetica","bold");
			doc.setFontSize(9);

			doc.text(
				diaMasBajo.fecha,
				118,
				121
			);

			doc.text(
				diaMasBajo.total.toLocaleString("es-CO"),
				188,
				121,
				{
					align:"right"
				}
			);
		}

		// ==========================
		// TABLA
		// ==========================

		autoTable(doc, {
			startY: 140,

			head: [[
				'Fecha',
				'Total Entradas'
			]],

			body: entradas.map((item) => [
				item.fecha,
				item.total.toLocaleString('es-CO')
			]),

			headStyles:{
				fillColor:[12,74,110],
				textColor:[255,255,255],
				fontStyle:"bold",
				fontSize:10,
				halign:"center"
			},

			alternateRowStyles:{
				fillColor:[248,249,250]
			},

			styles:{
				font:"helvetica",
				fontSize:9,
				cellPadding:4,
				lineColor:[225,230,235],
				lineWidth:.2,
				valign:"middle"
			}
		});

		// ==========================
		// FOOTER
		// ==========================

		const pages = doc.getNumberOfPages();

		for(let i=1;i<=pages;i++){

			doc.setPage(i);

			doc.setDrawColor(220,220,220);
			doc.setLineWidth(.3);

			doc.line(
				14,
				285,
				196,
				285
			);

			doc.setFont("helvetica","normal");
			doc.setFontSize(8);
			doc.setTextColor(120,120,120);

			doc.text(
				"Sistema de Gestión de Inventario",
				14,
				290
			);

			doc.text(
				`${fechaGeneracion.fecha} • ${fechaGeneracion.hora}`,
				105,
				290,
				{
					align:"center"
				}
			);

			doc.text(
				`Página ${i} de ${pages}`,
				196,
				290,
				{
					align:"right"
				}
			);
		}

		doc.save(
			`reporte-entradas-${new Date()
				.toISOString()
				.split('T')[0]}.pdf`
		);

		toast.success('Reporte descargado correctamente.');

	} catch (err) {
		console.error(err);
		toast.error('Error al generar el reporte.');
	}
}

// ==========================
// GENERADOR PDF ENTRADAS POR TIPO
// ==========================

function descargarReporteEntradasPorTipo() {
	try{
		if (!reportData || reportData.length === 0) {
			toast.error("No hay datos para generar el reporte.");
			return;
		}

		const tipos = reportData.map(item => ({
			tipo: item.tipo_producto,
			total: Number(item.cantidad_total_ingresada)
		}));

		const doc = new jsPDF("p","mm","a4");

		// ==========================
		// CÁLCULOS
		// ==========================

		const totalEntradas = tipos.reduce(
			(sum,item)=>sum+item.total,
			0
		);

		const totalTipos = tipos.length;

		const promedio =
			totalTipos > 0
				? totalEntradas / totalTipos
				: 0;

		const tipoMasIngresado =
			[...tipos].sort(
				(a,b)=>b.total-a.total
			)[0];

		const tipoMenosIngresado =
			[...tipos].sort(
				(a,b)=>a.total-b.total
			)[0];

		const fechaGeneracion = formatearFechaHora(new Date());

		// ==========================
		// HEADER
		// ==========================

		doc.setFillColor(12, 74, 110);
		doc.rect(0, 0, 210, 35, 'F');

		doc.setTextColor(255, 255, 255);
		doc.setFontSize(22);
		doc.setFont('helvetica', 'bold');

		doc.text(
			'REPORTE ENTRADAS POR TIPO',
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

		doc.setTextColor(12, 74, 110);
		doc.setFont("helvetica","bold");
		doc.setFontSize(16);

		doc.text(
			'Resumen Ejecutivo',
			14,
			50
		);

		doc.setDrawColor(12,74,110);
		doc.setLineWidth(.4);
		doc.line(
			14,
			52,
			196,
			52
		);

		const cards = [
			[
				"Total Entradas",
				totalEntradas.toLocaleString("es-CO")
			],

			[
				"Tipos de Producto",
				totalTipos.toString()
			],

			[
				"Promedio por Tipo",
				Math.round(promedio).toLocaleString("es-CO")
			]
		];

		let x = 14;

		cards.forEach(([titulo, valor]) => {

			// sombra
			doc.setFillColor(228,232,236);

			doc.roundedRect(
				x+1,
				59,
				56,
				26,
				3,
				3,
				"F"
			);

			// tarjeta
			doc.setFillColor(250,250,250);

			doc.roundedRect(
				x,
				58,
				56,
				26,
				3,
				3,
				"F"
			);

			// barra azul
			doc.setFillColor(12,74,110);

			doc.roundedRect(
				x,
				58,
				56,
				3,
				3,
				3,
				"F"
			);

			// titulo
			doc.setTextColor(130,130,130);
			doc.setFont("helvetica","normal");
			doc.setFontSize(9);

			doc.text(
				titulo,
				x+4,
				68
			);

			// valor
			doc.setTextColor(12,74,110);
			doc.setFont("helvetica","bold");
			doc.setFontSize(16);

			doc.text(
				valor,
				x+4,
				79
			);

			x+=62;
		});

		// ==========================
		// INDICADORES
		// ==========================

		doc.setTextColor(12,74,110);
		doc.setFont("helvetica","bold");
		doc.setFontSize(15);

		doc.text(
			"Indicadores Clave",
			14,
			98
		);

		doc.setDrawColor(12,74,110);
		doc.setLineWidth(.4);

		doc.line(
			14,
			100,
			196,
			100
		);

		// ==========================
		// CONTENEDOR
		// ==========================

		// sombra
		doc.setFillColor(228,232,236);

		doc.roundedRect(
			15,
			105,
			182,
			28,
			3,
			3,
			"F"
		);

		// tarjeta
		doc.setFillColor(250,250,250);

		doc.roundedRect(
			14,
			104,
			182,
			28,
			3,
			3,
			"F"
		);

		// barra azul
		doc.setFillColor(12,74,110);

		doc.roundedRect(
			14,
			104,
			182,
			2.5,
			3,
			3,
			"F"
		);

		// ==========================
		// TIPO MÁS INGRESADO
		// ==========================

		doc.setFillColor(245,247,250);

		doc.roundedRect(
			18,
			108,
			78,
			18,
			2,
			2,
			"F"
		);

		doc.setTextColor(90,90,90);
		doc.setFont("helvetica","normal");
		doc.setFontSize(8);

		doc.text(
			"Tipo más ingresado",
			22,
			114
		);

		if (tipoMasIngresado) {

			doc.setTextColor(12,74,110);
			doc.setFont("helvetica","bold");
			doc.setFontSize(9);

			doc.text(
				tipoMasIngresado.tipo,
				22,
				121
			);

			doc.text(
				tipoMasIngresado.total.toLocaleString("es-CO"),
				92,
				121,
				{
					align:"right"
				}
			);
		}

		// ==========================
		// TIPO MENOS INGRESADO
		// ==========================

		doc.setFillColor(245,247,250);

		doc.roundedRect(
			114,
			108,
			78,
			18,
			2,
			2,
			"F"
		);

		doc.setTextColor(90,90,90);
		doc.setFont("helvetica","normal");
		doc.setFontSize(8);

		doc.text(
			"Tipo menos ingresado",
			118,
			114
		);

		if (tipoMenosIngresado) {

			doc.setTextColor(12,74,110);
			doc.setFont("helvetica","bold");
			doc.setFontSize(9);

			doc.text(
				tipoMenosIngresado.tipo,
				118,
				121
			);

			doc.text(
				tipoMenosIngresado.total.toLocaleString("es-CO"),
				188,
				121,
				{
					align:"right"
				}
			);
		}

		// ==========================
		// TABLA
		// ==========================
		autoTable(doc,{

			startY:140,

			head:[[
				"Tipo de Producto",
				"Cantidad Ingresada"
			]],

			body:tipos.map(item=>[
				item.tipo,
				item.total.toLocaleString("es-CO")
			]),

			headStyles:{
				fillColor:[12,74,110],
				textColor:[255,255,255],
				fontStyle:"bold",
				fontSize:10,
				halign:"center"
			},

			alternateRowStyles:{
				fillColor:[248,249,250]
			},

			styles:{
				font:"helvetica",
				fontSize:9,
				cellPadding:4,
				lineColor:[225,230,235],
				lineWidth:.2,
				valign:"middle"
			}
		});

		// ==========================
		// FOOTER
		// ==========================

		const pages = doc.getNumberOfPages();

		for(let i=1;i<=pages;i++){

			doc.setPage(i);

			doc.setDrawColor(220,220,220);
			doc.setLineWidth(.3);

			doc.line(
				14,
				285,
				196,
				285
			);

			doc.setFont("helvetica","normal");
			doc.setFontSize(8);
			doc.setTextColor(120,120,120);

			doc.text(
				"Sistema de Gestión de Inventario",
				14,
				290
			);

			doc.text(
				`${fechaGeneracion.fecha} • ${fechaGeneracion.hora}`,
				105,
				290,
				{
					align:"center"
				}
			);

			doc.text(
				`Página ${i} de ${pages}`,
				196,
				290,
				{
					align:"right"
				}
			);
		}

		doc.save(
			`reporte-margen-${new Date()
				.toISOString()
				.split('T')[0]}.pdf`
		);

		toast.success('Reporte descargado correctamente.');

	} catch (err) {
		console.error(err);
		toast.error('Error al generar el reporte.');
	}
}

// ==========================
// GENERADOR PDF ENTRADAS PRODUCTOS INGRESADOS
// ==========================

function descargarReporteProductosIngresados() {
    try{

	} catch (err) {
		console.error(err);
		toast.error('Error al generar el reporte.');
	}
}

// ==========================
// GENERADOR PDF ENTRADAS STOCK ACTUAL VS MINIMO
// ==========================

function descargarReporteStockActual() {
    try{
		if (!reportData || reportData.length === 0) {
			toast.error("No hay datos para generar el reporte.");
			return;
		}

		const stock = reportData.map(item => ({
			producto: item.nombre_producto,
			actual: Number(item.stock_actual),
			minimo: Number(item.stock_minimo)
		}));

		const doc = new jsPDF("p","mm","a4");

		// ==========================
		// CÁLCULOS
		// ==========================

		const totalProductos = stock.length;

		const totalStockActual = stock.reduce(
			(sum,item)=>sum+item.actual,
			0
		);

		const totalStockMinimo = stock.reduce(
			(sum,item)=>sum+item.minimo,
			0
		);

		const productoMenorStock =
			[...stock].sort(
				(a,b)=>a.actual-b.actual
			)[0];

		const productoMayorStock =
			[...stock].sort(
				(a,b)=>b.actual-a.actual
			)[0];

		const productoMayorDeficit =
			[...stock].sort(
				(a, b) =>
					(b.minimo - b.actual) -
					(a.minimo - a.actual)
    		)[0];
		const fechaGeneracion =
			formatearFechaHora(new Date());
		
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

		doc.setTextColor(12, 74, 110);
		doc.setFont("helvetica","bold");
		doc.setFontSize(16);

		doc.text(
			'Resumen Ejecutivo',
			14,
			50
		);

		doc.setDrawColor(12,74,110);
		doc.setLineWidth(.4);
		doc.line(
			14,
			52,
			196,
			52
		);

		const cards = [

			[
				"Productos",
				totalProductos.toString()
			],

			[
				"Stock Actual",
				totalStockActual.toLocaleString("es-CO")
			],

			[
				"Stock Mínimo",
				totalStockMinimo.toLocaleString("es-CO")
			]
		];

		let x = 14;

		cards.forEach(([titulo, valor]) => {

			// sombra
			doc.setFillColor(228,232,236);

			doc.roundedRect(
				x+1,
				59,
				56,
				26,
				3,
				3,
				"F"
			);

			// tarjeta
			doc.setFillColor(250,250,250);

			doc.roundedRect(
				x,
				58,
				56,
				26,
				3,
				3,
				"F"
			);

			// barra azul
			doc.setFillColor(12,74,110);

			doc.roundedRect(
				x,
				58,
				56,
				3,
				3,
				3,
				"F"
			);

			// titulo
			doc.setTextColor(130,130,130);
			doc.setFont("helvetica","normal");
			doc.setFontSize(9);

			doc.text(
				titulo,
				x+4,
				68
			);

			// valor
			doc.setTextColor(12,74,110);
			doc.setFont("helvetica","bold");
			doc.setFontSize(16);

			doc.text(
				valor,
				x+4,
				79
			);

			x+=62;
		});

		// ==========================
		// INDICADORES
		// ==========================

		doc.setTextColor(12,74,110);
		doc.setFont("helvetica","bold");
		doc.setFontSize(15);

		doc.text(
			"Indicadores Clave",
			14,
			98
		);

		doc.setDrawColor(12,74,110);
		doc.setLineWidth(.4);

		doc.line(
			14,
			100,
			196,
			100
		);

		// ==========================
		// CONTENEDOR
		// ==========================

		// sombra
		doc.setFillColor(228,232,236);

		doc.roundedRect(
			15,
			105,
			182,
			28,
			3,
			3,
			"F"
		);

		// tarjeta
		doc.setFillColor(250,250,250);

		doc.roundedRect(
			14,
			104,
			182,
			28,
			3,
			3,
			"F"
		);

		// barra azul
		doc.setFillColor(12,74,110);

		doc.roundedRect(
			14,
			104,
			182,
			2.5,
			3,
			3,
			"F"
		);

		// ==========================
		// MENOR STOCK ACTUAL
		// ==========================

		doc.setFillColor(245,247,250);

		doc.roundedRect(
			18,
			108,
			78,
			18,
			2,
			2,
			"F"
		);

		doc.setTextColor(90,90,90);
		doc.setFont("helvetica","normal");
		doc.setFontSize(8);

		doc.text(
			"Menor stock",
			22,
			114
		);

		if (productoMenorStock) {

			doc.setTextColor(12,74,110);
			doc.setFont("helvetica","bold");
			doc.setFontSize(9);

			doc.text(
				productoMenorStock.producto.slice(0,20),
				22,
				121
			);

			doc.text(
				`${productoMenorStock.actual}`,
				92,
				121,
				{
					align:"right"
				}
			);
		}

		// ==========================
		// MAYOR DÉFICIT
		// ==========================

		doc.setFillColor(245,247,250);

		doc.roundedRect(
			114,
			108,
			78,
			18,
			2,
			2,
			"F"
		);

		doc.setTextColor(90,90,90);
		doc.setFont("helvetica","normal");
		doc.setFontSize(8);

		doc.text(
			"Mayor déficit",
			118,
			114
		);

		if (productoMayorDeficit) {

			const deficit =
				Math.max(
					0,
					Number(productoMayorDeficit.minimo) -
					Number(productoMayorDeficit.actual)
				);

			doc.setTextColor(12,74,110);
			doc.setFont("helvetica","bold");
			doc.setFontSize(9);

			doc.text(
				productoMayorDeficit.producto.slice(0,20),
				118,
				121
			);

			doc.text(
				`${deficit} Uni`,
				188,
				121,
				{
					align:"right"
				}
			);
		}

		// ==========================
		// TABLA
		// ==========================

		autoTable(doc,{

			startY:140,

			head:[[
				"Producto",
				"Stock Actual",
				"Stock Mínimo"
			]],

			body:stock.map(item=>[
				item.producto,
				item.actual.toLocaleString("es-CO"),
				item.minimo.toLocaleString("es-CO")
			]),

			headStyles:{
				fillColor:[12,74,110],
				textColor:[255,255,255],
				fontStyle:"bold",
				fontSize:10,
				halign:"center"
			},

			alternateRowStyles:{
				fillColor:[248,249,250]
			},

			styles:{
				font:"helvetica",
				fontSize:9,
				cellPadding:4,
				lineColor:[225,230,235],
				lineWidth:.2
			}
		});

		// ==========================
		// FOOTER
		// ==========================

		const pages = doc.getNumberOfPages();

		for(let i=1;i<=pages;i++){

			doc.setPage(i);

			doc.setDrawColor(220,220,220);
			doc.setLineWidth(.3);

			doc.line(
				14,
				285,
				196,
				285
			);

			doc.setFont("helvetica","normal");
			doc.setFontSize(8);
			doc.setTextColor(120,120,120);

			doc.text(
				"Sistema de Gestión de Inventario",
				14,
				290
			);

			doc.text(
				`${fechaGeneracion.fecha} • ${fechaGeneracion.hora}`,
				105,
				290,
				{
					align:"center"
				}
			);

			doc.text(
				`Página ${i} de ${pages}`,
				196,
				290,
				{
					align:"right"
				}
			);
		}

		doc.save(
			`reporte-stock-${new Date()
				.toISOString()
				.split("T")[0]}.pdf`
		);

		toast.success("Reporte descargado correctamente.");

	} catch (err) {
		console.error(err);
		toast.error('Error al generar el reporte.');
	}
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
			else if (title === 'Entradas por Día') {
				descargarReporteEntradas();
			}
			else if (title === 'Entradas por Tipo') {
				descargarReporteEntradasPorTipo();
			}
			else if (title === 'Stock Actual vs Stock Mínimo')
				descargarReporteStockActual();
			else {
				// descargarGrafico();
			}
	}}
	class="h-10 w-10 rounded-lg border border-slate-200 flex items-center justify-center hover:bg-slate-50 transition-colors dark:border-[#334156] cursor-pointer "
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