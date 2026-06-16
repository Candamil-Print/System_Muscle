<script lang="ts">
	import { onMount } from 'svelte';

	import ChartCard from '$lib/components/reportes/ChartCard.svelte';

	import {
		obtenerReporteInventario
	} from '$lib/services/api/reports/reports.service';

	import {
		listarMovimientosEntrada
	} from '$lib/services/api/reports/entries/entries.service';


	type ChartData = {
		labels: string[];
		datasets: any[];
	};

	let entradasPorDia: ChartData = {
		labels: [],
		datasets: []
	};

	$: console.log('ENTRADAS POR DIA:', entradasPorDia);

	let entradasPorTipo: ChartData = {
		labels: [],
		datasets: []
	};

	let productosIngresados: ChartData = {
		labels: [],
		datasets: []
	};

	let stockComparacion: ChartData = {
		labels: [],
		datasets: []
	};

	let historialEntradas: any[] = [];

	$: reporteEntradas = entradasPorDia.labels.map((fecha, index) => ({
        fecha,
        total_entradas: entradasPorDia.datasets[0].data[index]
    }));
	

async function cargarGraficas() {
    try {
        const [movimientos, inventario] =
            await Promise.all([
                listarMovimientosEntrada(),
                obtenerReporteInventario()
            ]);

        const dark =
            document.documentElement.classList.contains('dark');

        // =====================
        // HISTORIAL DE ENTRADAS
        // =====================

        historialEntradas = [...movimientos]
            .sort(
                (a, b) =>
                    new Date(b.fecha).getTime() -
                    new Date(a.fecha).getTime()
            )
            .map((m) => ({
                producto: m.nombre_producto,

                cantidad: `${m.cantidad} Uni`,

                fecha: new Date(m.fecha).toLocaleString('es-CO', {
                    day: '2-digit',
                    month: '2-digit',
                    year: 'numeric',
                    hour: '2-digit',
                    minute: '2-digit'
                }),

                recibe:
                    m.nombre_usuario ||
                    m.recibe ||
                    'Sin registrar',

                tipo: m.tipo_producto
            }));

        // =====================
        // ENTRADAS POR DÍA
        // =====================

        const porDia: Record<string, number> = {};

        movimientos.forEach((m) => {
            const fecha = new Date(m.fecha).toLocaleDateString('es-CO');

            porDia[fecha] =
                (porDia[fecha] || 0) + m.cantidad;
        });

        entradasPorDia = {
            labels: Object.keys(porDia),

            datasets: [
                {
                    label: 'Entradas',

                    data: Object.values(porDia),

                    borderColor: dark
                        ? '#39BDF8'
                        : '#0C4A6E',

                    backgroundColor: dark
                        ? 'rgba(57,189,248,.20)'
                        : 'rgba(12,74,110,.12)',

                    borderWidth: 2,

                    fill: true,

                    tension: 0.4,

                    pointBackgroundColor: dark
                        ? '#39BDF8'
                        : '#0C4A6E',

                    pointBorderColor: dark
                        ? '#39BDF8'
                        : '#0C4A6E',

                    pointRadius: 5,

                    pointHoverRadius: 7
                }
            ]
        };

        // =====================
        // ENTRADAS POR TIPO
        // =====================

        const porTipo: Record<string, number> = {};

        movimientos.forEach((m) => {
            porTipo[m.tipo_producto] =
                (porTipo[m.tipo_producto] || 0) +
                m.cantidad;
        });

        entradasPorTipo = {
            labels: Object.keys(porTipo),

            datasets: [
                {
                    data: Object.values(porTipo),

                    backgroundColor: [
                        '#0c4a6e',
                        '#1565a0',
                        '#1e7ab8',
                        '#2d8ad0',
                        '#3d9ae8'
                    ]
                }
            ]
        };

        // =====================
        // PRODUCTOS INGRESADOS
        // ÚLTIMOS 5 MOVIMIENTOS
        // =====================

        const ultimosProductos = [...movimientos]
            .sort(
                (a, b) =>
                    new Date(b.fecha).getTime() -
                    new Date(a.fecha).getTime()
            )
            .slice(0, 5);

        productosIngresados = {
            labels: ultimosProductos.map(
                (m) => m.nombre_producto
            ),

            datasets: [
                {
                    label: 'Unidades Ingresadas',

                    data: ultimosProductos.map(
                        (m) => m.cantidad
                    ),

                    backgroundColor: [
                        '#0c4a6e',
                        '#1565a0',
                        '#1e7ab8',
                        '#2d8ad0',
                        '#3d9ae8'
                    ],

                    borderRadius: 8,

                    borderSkipped: false
                }
            ]
        };

        // =====================
        // STOCK ACTUAL VS MÍNIMO
        // TOP 5 MENOR STOCK
        // =====================

        const top5Stock = [...inventario]
            .sort(
                (a, b) =>
                    Number(a.stock_actual) -
                    Number(b.stock_actual)
            )
            .slice(0, 5);

        stockComparacion = {
            labels: top5Stock.map(
                (item) => item.nombre
            ),

            datasets: [
                {
                    label: 'Stock Actual',

                    data: top5Stock.map(
                        (item) => item.stock_actual
                    ),

                    backgroundColor: '#0c4a6e',

                    borderRadius: 8,

                    borderSkipped: false
                },

                {
                    label: 'Stock Mínimo',

                    data: top5Stock.map(
                        (item) => item.stock_minimo
                    ),

                    backgroundColor: '#94A3B8',

                    borderRadius: 8,

                    borderSkipped: false
                }
            ]
        };

        console.log('Movimientos:', movimientos);
        console.log('Inventario:', inventario);
    } catch (error) {
        console.error(
            'Error cargando gráficas:',
            error
        );
    }
}

	onMount(() => {
		cargarGraficas();

		const observer =
			new MutationObserver(() => {
				cargarGraficas();
			});

		observer.observe(
			document.documentElement,
			{
				attributes: true,
				attributeFilter: ['class']
			}
		);

		return () => observer.disconnect();
	});
</script>

<div class="grid grid-cols-1 gap-5 xl:grid-cols-2">

<ChartCard
    title="Entradas por Día"
    type="line"
    data={entradasPorDia}
    reportData={reporteEntradas}
/>

<ChartCard
    title="Entradas por Tipo"
    type="pie"
    data={entradasPorTipo}
/>

<ChartCard
    title="Productos Ingresados"
    type="bar"
    data={productosIngresados}
/>

<ChartCard
    title="Stock Actual vs Stock Mínimo"
    type="bar"
    data={stockComparacion}
/>



</div>