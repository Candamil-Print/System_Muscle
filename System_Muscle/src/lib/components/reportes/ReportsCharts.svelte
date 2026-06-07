<script lang="ts">
  import { onMount } from 'svelte';
  import ReportsChartCard from './ChartCard.svelte';
  import type { ChartData } from 'chart.js';

  import {
    obtenerResumenVentasDiarioRango,
    obtenerProductosMasVendidos,
    obtenerVentasPorUsuario,
    obtenerVentasPorMetodoPago
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
        ventasPorUsuario
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
        )
      ]);

      // Gráfico 1
      salesByDay = {
        labels: ventasDiarias.map(v =>
          new Date(v.fecha).toLocaleDateString(
            'es-ES',
            { weekday: 'short' }
          )
        ),

        datasets: [
          {
            label: 'Total Ventas',
            data: ventasDiarias.map(
              v => v.total_general
            ),
            borderColor: '#0C4A6E',
            backgroundColor:
              'rgba(12,74,110,.15)',
            fill: true,
            tension: 0.4
          }
        ]
      };

      // Gráfico 2
      profitMargin = {
        labels: productosMasVendidos.map(
          p => p.nombre_producto
        ),

        datasets: [
          {
            label: 'Ingresos por Producto',
            data: productosMasVendidos.map(
              p => p.total_ventas
            ),
            backgroundColor: '#0C4A6E'
          }
        ]
      };

      // Gráfico 3
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
            backgroundColor: '#0284C7'
          }
        ]
      };

      // Gráfico 4
      salesBySeller = {
        labels: ventasPorUsuario.map(
          u => u.nombre_usuario
        ),

        datasets: [
          {
            data: ventasPorUsuario.map(
              u => u.total_vendido
            ),

            backgroundColor: [
              '#0C4A6E',
              '#0284C7',
              '#38BDF8',
              '#7DD3FC',
              '#BAE6FD'
            ]
          }
        ]
      };
    }
    catch (err) {
      console.error(
        'Error cargando reportes:',
        err
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
      title="Ingresos por Producto"
      type="bar"
      data={profitMargin}
    />

    <ReportsChartCard
      title="Top Productos"
      type="bar"
      data={topProducts}
    />

    <ReportsChartCard
      title="Ventas por Vendedor"
      type="pie"
      data={salesBySeller}
    />

  </div>
{/if}