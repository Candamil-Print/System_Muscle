<script lang="ts">
  import { onMount } from 'svelte';
  import { toast } from 'svelte-sonner';
  import ReportsChartCard from './ChartCard.svelte';
  import type { ChartData } from 'chart.js';

  import {
    obtenerResumenVentasDiarioRango,
    obtenerProductosMasVendidos,
    obtenerVentasPorUsuario,
    obtenerMargenGanancia
  } from '$lib/services/api/reports/reports.service';

  export let filtros = {
    fechaInicio: '',
    fechaFin: '',
    metodoPago: 'todos',
    vendedor: 'todos'
  };

  // ==========================================
  // Datos originales de la API
  // ==========================================

  let ventasDiariasData: any[] = [];

  // TOP 5 para el gráfico
  let productosMasVendidosData: any[] = [];

  // TODOS para el PDF
  let todosProductosVendidosData: any[] = [];

  let ventasPorUsuarioData: any[] = [];
  let margenGananciaData: any = null;

  // ==========================================
  // Datos para Chart.js
  // ==========================================

  let salesByDay: ChartData<'line'> = {
    labels: [],
    datasets: []
  };

  let profitMargin: ChartData<'bar'> = {
    labels: [],
    datasets: []
  };

  let topProducts: ChartData<'bar'> = {
    labels: [],
    datasets: []
  };

  let salesBySeller: ChartData<'bar'> = {
    labels: [],
    datasets: []
  };

  let margenData: any[] = [];

  let loading = true;
  let error = '';

  // ==========================================
  // Construir gráficos
  // ==========================================

  function construirGraficos() {
    const dark =
      document.documentElement.classList.contains('dark');

    // -------------------------
    // Ventas por día
    // -------------------------

    salesByDay = {
      labels: ventasDiariasData.map((v) =>
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

          data: ventasDiariasData.map(
            (v) => v.total_general
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

    // -------------------------
    // Margen
    // -------------------------

    const topMargen =
      [...(margenGananciaData?.productos ?? [])]
        .sort(
          (a, b) =>
            b.margen_porcentaje -
            a.margen_porcentaje
        )
        .slice(0, 5);

    profitMargin = {
      labels: topMargen.map((p) =>
        p.nombre_producto.length > 18
          ? p.nombre_producto.slice(0, 18) + '...'
          : p.nombre_producto
      ),

      datasets: [
        {
          label: 'Margen de Ganancias (%)',

          data: topMargen.map((p) =>
            Number(
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

    // -------------------------
    // Top 5 productos
    // -------------------------

    topProducts = {
      labels: productosMasVendidosData.map(
        (p) => p.nombre_producto
      ),

      datasets: [
        {
          label: 'Unidades Vendidas',

          data:
            productosMasVendidosData.map(
              (p) => p.cantidad_vendida
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

    // -------------------------
    // Vendedores
    // -------------------------

    const topVendedores =
      [...ventasPorUsuarioData]
        .sort(
          (a, b) =>
            b.total_vendido -
            a.total_vendido
        )
        .slice(0, 5);

    salesBySeller = {
      labels: topVendedores.map(
        (u) => u.nombre_usuario
      ),

      datasets: [
        {
          label: 'Total Vendido',

          data: topVendedores.map(
            (u) => u.total_vendido
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
  }

  // ==========================================
  // Cargar API
  // ==========================================

  async function cargarReportes(
    fechaInicio: string,
    fechaFin: string,
    mostrarToast = false
  ) {
    try {
      loading = true;
      error = '';

      const [
        ventasDiarias,
        productosMasVendidos,
        todosProductos,
        ventasPorUsuario,
        margenGanancia
      ] = await Promise.all([
        obtenerResumenVentasDiarioRango(
          fechaInicio,
          fechaFin
        ),

        // TOP 5
        obtenerProductosMasVendidos(
          fechaInicio,
          fechaFin,
          5
        ),

        // TODOS
        obtenerProductosMasVendidos(
          fechaInicio,
          fechaFin
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

      ventasDiariasData = ventasDiarias;

      // TOP 5
      productosMasVendidosData =
        productosMasVendidos;

      // TODOS
      todosProductosVendidosData =
        todosProductos;

      ventasPorUsuarioData =
        ventasPorUsuario;

      margenGananciaData =
        margenGanancia;

      margenData =
        margenGanancia.productos;

      console.log(
        'TOP 5:',
        productosMasVendidosData.length
      );

      console.log(
        'TODOS:',
        todosProductosVendidosData.length
      );

      if (
        mostrarToast &&
        ventasDiarias.length === 0 &&
        productosMasVendidos.length === 0 &&
        ventasPorUsuario.length === 0
      ) {
        toast.warning(
          'No se encontraron ventas para los filtros seleccionados',
          {
            id: 'sin-resultados-reportes',
            description:
              'Prueba con otro rango de fechas o cambia los filtros.'
          }
        );
      }

      construirGraficos();
    } catch (err) {
      console.error(err);

      toast.error(
        'Error al cargar los reportes'
      );

      error =
        'Error al cargar los datos de reportes';
    } finally {
      loading = false;
    }
  }

  // ==========================================
  // Primera carga
  // ==========================================

  onMount(() => {
    const observer =
      new MutationObserver(() => {
        construirGraficos();
      });

    observer.observe(
      document.documentElement,
      {
        attributes: true,
        attributeFilter: ['class']
      }
    );

    const today = new Date();

    const sevenDaysAgo = new Date(
      today.getTime() -
        7 * 24 * 60 * 60 * 1000
    );

    cargarReportes(
      sevenDaysAgo
        .toISOString()
        .split('T')[0],

      today
        .toISOString()
        .split('T')[0]
    );

    return () => observer.disconnect();
  });

  // ==========================================
  // Cambio de filtros
  // ==========================================

  $: if (
    filtros.fechaInicio &&
    filtros.fechaFin
  ) {
    cargarReportes(
      filtros.fechaInicio,
      filtros.fechaFin,
      true
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
      reportData={ventasDiariasData}
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
      reportData={productosMasVendidosData}
      todosProductosData={todosProductosVendidosData}
      horizontal
    />

    <ReportsChartCard
      title="Ventas por Vendedor"
      type="bar"
      data={salesBySeller}
      reportData={ventasPorUsuarioData}
      tall
    />

  </div>
{/if}