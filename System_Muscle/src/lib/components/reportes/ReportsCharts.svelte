<script lang="ts">
  import { onMount } from 'svelte';
  import { toast } from 'svelte-sonner';
  import ReportsChartCard from './ChartCard.svelte';
  import type { ChartData } from 'chart.js';

  import {
    obtenerResumenVentasDiarioRango,
    obtenerProductosMasVendidos,
    obtenerVentasPorUsuario,
    obtenerVentasPorMetodoPago,
    obtenerMargenGanancia
  } from '$lib/services/api/reports/reports.service';

  export let filtros = {
    fechaInicio: '',
    fechaFin: '',
    metodoPago: 'todos',
    vendedor: 'todos'
  };

  let salesByDay: ChartData<'line'> = { labels: [], datasets: [] };
  let profitMargin: ChartData<'bar'> = { labels: [], datasets: [] };
  let topProducts: ChartData<'bar'> = { labels: [], datasets: [] };
  let salesBySeller: ChartData<'pie'> = { labels: [], datasets: [] };

  let margenData: any[] = [];

  let loading = true;
  let error = '';

  async function cargarReportes(
    fechaInicio: string,
    fechaFin: string
  ) {
    try {
      loading = true;
      error = '';

      const [
        ventasDiarias,
        productosMasVendidos,
        ventasPorUsuario,
        margenGanancia
        
      ] = await Promise.all([
        obtenerResumenVentasDiarioRango(
          fechaInicio,
          fechaFin
        ),
        

        obtenerProductosMasVendidos(
          fechaInicio,
          fechaFin,
          5
        ),

        obtenerVentasPorUsuario(
          fechaInicio,
          fechaFin
        ),

        obtenerMargenGanancia(
          fechaInicio,
          fechaFin
        )
      ]);

      margenData = margenGanancia.productos;

      // SIN RESULTADOS
      if (
        ventasDiarias.length === 0 &&
        productosMasVendidos.length === 0 &&
        ventasPorUsuario.length === 0
      ) {
        toast.warning(
          'No se encontraron ventas para los filtros seleccionados',
          {
            description:
              'Prueba con otro rango de fechas o cambia los filtros.'
          }
        );
      }

      // Gráfico 1 - Ventas por Día (Line)
      salesByDay = {
        labels: ventasDiarias.map(v =>
          new Date(v.fecha).toLocaleDateString(
            'es-ES',
            {
              day: '2-digit',
              month: 'short'
            }
          )
        ),

        datasets: [
          {
            label: 'Total Ventas',
            data: ventasDiarias.map(
              v => v.total_general
            ),
            borderColor: '#0c4a6e',
            backgroundColor:
              'rgba(12,74,110,.1)',
            borderWidth: 2,
            fill: true,
            tension: 0.4,
            pointBackgroundColor: '#0c4a6e',
            pointBorderColor: '#ffffff',
            pointBorderWidth: 2,
            pointRadius: 5,
            pointHoverRadius: 7
          }
        ]
      };

      // Gráfico 2 - Margen de Ganancias (Bar vertical)
      profitMargin = {
        labels: margenGanancia.productos.map(
          p => p.nombre_producto
        ),

        datasets: [
          {
            label: 'Margen de Ganancias (%)',

            data: margenGanancia.productos.map(
              p => Number(
                p.margen_porcentaje.toFixed(2)
              )
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

      // Gráfico 3 - Top 5 Productos (Bar horizontal)
      topProducts = {
        labels: productosMasVendidos.map(
          p => p.nombre_producto
        ),

        datasets: [
          {
            label: 'Unidades Vendidas',
            data: productosMasVendidos.map(
              p => p.cantidad_vendida
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

      // Gráfico 4 - Ventas por Vendedor (Bar)
      salesBySeller = {
        labels: ventasPorUsuario.map(
          u => u.nombre_usuario
        ),

        datasets: [
          {
            label: 'Total Vendido',
            data: ventasPorUsuario.map(
              u => u.total_vendido
            ),
            backgroundColor: [
              '#0c4a6e',
              '#1565a0',
              '#1e7ab8'
            ],
            borderRadius: 8,
            borderSkipped: false
          }
        ]
      };
    }
  catch (err) {
    console.error(
      'Error cargando reportes:',
      err
    );

    toast.error(
      'Error al cargar los reportes'
    );

    error =
      'Error al cargar los datos de reportes';
  }
    finally {
      loading = false;
    }
  }

  onMount(async () => {
    const today = new Date();

    const sevenDaysAgo = new Date(
      today.getTime() -
        7 * 24 * 60 * 60 * 1000
    );

    const fechaInicio =
      sevenDaysAgo
        .toISOString()
        .split('T')[0];

    const fechaFin =
      today.toISOString().split('T')[0];

    await cargarReportes(
      fechaInicio,
      fechaFin
    );
  });

  $: if (
    filtros?.fechaInicio &&
    filtros?.fechaFin
  ) {
    cargarReportes(
      filtros.fechaInicio,
      filtros.fechaFin
    );
  }

  toast.warning(
  'No se encontraron ventas para los filtros seleccionados',
  {
    id: 'sin-resultados-reportes'
  }
);
</script>

{#if loading}
  <div class="flex items-center justify-center py-12">
    <p class="text-slate-500">
      Cargando gráficos...
    </p>
  </div>

{:else if error}
  <div class="rounded-2xl border border-red-200 bg-red-50 p-5">
    <p class="text-red-600">
      {error}
    </p>
  </div>

{:else}
  <div class="grid grid-cols-1 gap-5 xl:grid-cols-2">

    <ReportsChartCard
      title="Ventas por Día"
      type="line"
      data={salesByDay}
    />

    <ReportsChartCard
      title="Reporte de Margen de Ganancias"
      type="bar"
      data={profitMargin}
      reportData={margenData}
    />

    <ReportsChartCard
      title="Top 5 Productos Vendidos"
      type="bar"
      data={topProducts}
      horizontal={true}
    />

    <ReportsChartCard
      title="Ventas por Vendedor"
      type="bar"
      data={salesBySeller}
    />

  </div>
{/if}