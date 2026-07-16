<script lang="ts">
	import { onMount } from 'svelte';

	import ChartCard from '$lib/components/reportes/ChartCard.svelte';

	import {
		entradasPorDia as obtenerEntradasPorDia,
        entradasPorTipoProducto,
        reporteEntradaDetallado,
        stockActualYMinimo,
        resumenEntradasPorProducto
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
            const hoy = new Date();

            const hace30Dias = new Date();
            hace30Dias.setDate(hoy.getDate() - 30);

            const fechaInicio = hace30Dias.toISOString().split('T')[0];
            const fechaFin = hoy.toISOString().split('T')[0];

            const [
                movimientos,
                graficaDias,
                graficaTipo,
                resumenProductos,
                stockProducto
            ] = await Promise.all([
                reporteEntradaDetallado(
                    fechaInicio,
                    fechaFin
                ),
                obtenerEntradasPorDia(
                    fechaInicio,
                    fechaFin
                ),
                entradasPorTipoProducto(
                    fechaInicio,
                    fechaFin
                ),
                resumenEntradasPorProducto(
                    fechaInicio,
                    fechaFin
                ),
                stockActualYMinimo()
            ]);

            console.log("graficaDias", graficaDias);
            console.log("graficaTipo:", graficaTipo);
            console.log("grafica Ingresos:", resumenProductos);
            console.log("Stock:", stockProducto);

            // =====================
            // APLICAR FILTROS A LOS MOVIMIENTOS ENTRADA
            // =====================
            
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

            entradasPorDia = {
                labels: graficaDias.map(item =>
                    new Date(item.fecha).toLocaleDateString('es-CO')
                ),

                datasets: [
                    {
                        label: 'Entradas',

                        data: graficaDias.map(
                            item => item.cantidad_total_ingresada
                        ),

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

            entradasPorTipo = {
                labels: graficaTipo.map(item => item.tipo_producto),
                datasets: [
                    {
                        label: 'Tipo de Producto',
                        data: graficaTipo.map(item => item.cantidad_total_ingresada),
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

           const topProductos = [...resumenProductos]
            .sort(
                (a, b) =>
                    b.cantidad_total_ingresada -
                    a.cantidad_total_ingresada
            )
            .slice(0, 5);

            productosIngresados = {
                labels: topProductos.map(
                    item => item.nombre_producto
                ),

                datasets: [
                    {
                        label: 'Cantidad Ingresada',

                        data: topProductos.map(
                            item => item.cantidad_total_ingresada
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

            const top5Stock = [...stockProducto]
                .sort(
                    (a, b) =>
                        Number(a.stock_actual) -
                        Number(b.stock_actual)
                )
                .slice(0, 5);

            stockComparacion = {
                labels: top5Stock.map(
                    (item) => item.nombre_producto
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
        } catch (error) {
            console.error(
                'Error cargando gráficas:',
                error
            );
        }
    }

    // ==========================
    // Carga el historial completo 
    // ==========================
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

    // =====================
    // Recarga las gráficas cuando cambian los filtros
    // =====================



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