<script lang="ts">
  import { onMount } from 'svelte';
  import { Toaster } from 'svelte-sonner';

  import Header from '$lib/components/layout/Header.svelte';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';

  import ReportsTabs from '$lib/components/reportes/ReportsTabs.svelte';

  // Ventas
  import ReportsFilters from '$lib/components/reportes/ReportsFilters.svelte';
  import ReportsStats from '$lib/components/reportes/ReportsStats.svelte';
  import ReportsCharts from '$lib/components/reportes/ReportsCharts.svelte';
  import ReportsTable from '$lib/components/reportes/ReportsTable.svelte';

  // Entradas
  import EntriesFilters from '$lib/components/reportes/EntriesFilters.svelte';
  import EntriesStats from '$lib/components/reportes/EntriesStats.svelte';
  import EntriesCharts from '$lib/components/reportes/EntriesCharts.svelte';
  import EntriesTable from '$lib/components/reportes/EntriesTable.svelte';

  import {
    salesHistory,
    salesStats
  } from '$lib/data/reportsData';

  import {
    listarMovimientosEntrada
  } from '$lib/services/api/reports/entries/entries.service';

  let activeTab = 'ventas';

  let filtros = {
    fechaInicio: '',
    fechaFin: '',
    metodoPago: 'todos',
    vendedor: 'todos'
  };

  let filtrosEntradas = {
    fechaInicio: '',
    fechaFin: '',
    tipoProducto: 'todos',
    vendedorEntrada: 'todos'
  };

  let totalIngresado = 0;
  let totalProductos = 0;
  let totalEntradas = 0;
  let stockBajo = 0;

  let entradas: any[] = [];

  async function cargarEstadisticasEntradas() {
    try {
      const movimientos =
        await listarMovimientosEntrada();

         console.log('TODOS LOS MOVIMIENTOS:');
    console.log(movimientos);

    console.log('PRIMER MOVIMIENTO:');
    console.log(movimientos[0]);

    movimientos.forEach((m) => {
    console.log({
        id: m.id_movimiento,
        fecha: m.fecha,
        id_usuario: m.id_usuario,
        nombre_usuario: m.nombre_usuario
    });
});

      // Estadísticas
      totalEntradas =
        movimientos.length;

      totalIngresado =
        movimientos.reduce(
          (sum, item) =>
            sum + Number(item.cantidad),
          0
        );

      totalProductos =
        new Set(
          movimientos.map(
            item => item.id_producto
          )
        ).size;

      // Por ahora fijo hasta crear el servicio
      stockBajo = 0;

      // Tabla de historial
entradas = movimientos.map(item => ({
  id: item.id_movimiento,

  producto: item.nombre_producto,

  cantidad: `${item.cantidad} Uni`,

  fecha: new Date(item.fecha).toLocaleString(
    'es-CO',
    {
      dateStyle: 'short',
      timeStyle: 'short'
    }
  ),

  recibe: item.nombre_usuario,

  tipo: item.tipo_producto
}));
    } catch (error) {
      console.error(
        'Error cargando estadísticas:',
        error
      );
    }
  }

  onMount(() => {
    cargarEstadisticasEntradas();
  });

  function aplicarFiltros(
    nuevosFiltros
  ) {
    filtros = nuevosFiltros;
  }

  function aplicarFiltrosEntradas(
    nuevosFiltros
  ) {
    console.log("Llegaron filtros");
    filtrosEntradas = nuevosFiltros;
  }
</script>

<Toaster
  position="top-center"
  theme="dark"
  toastOptions={{
    class: '!bg-[#1E293B] !border !border-[#334156] !text-white'
  }}
/>

<div class="flex min-h-screen bg-slate-50 dark:bg-[#111827]">

  <Sidebar />

  <div class="ml-[280px] flex flex-1 flex-col">

    <Header />

    <main class="space-y-6 p-6">

      <div>

        <h1 class="text-3xl font-bold text-slate-800 dark:text-white">
          Reportes
        </h1>

        <p class="mt-1 text-slate-500">

          {#if activeTab === 'ventas'}
            Visualización y análisis de ventas
          {:else}
            Visualización y análisis de entradas de inventario
          {/if}

        </p>

      </div>

      <ReportsTabs bind:activeTab />

      {#if activeTab === 'ventas'}

        <ReportsFilters
          onFilter={aplicarFiltros}
        />

        <ReportsStats
          stats={salesStats}
        />

        <ReportsCharts
          {filtros}
        />

        <ReportsTable
          sales={salesHistory}
        />

      {:else}

        <EntriesFilters 
          onFilter={aplicarFiltrosEntradas}
        />

        <EntriesStats
          {totalIngresado}
          {totalProductos}
          {stockBajo}
          {totalEntradas}
        />

        <EntriesCharts 
          {filtrosEntradas}
        />

        <EntriesTable
          entradas={entradas}
          {filtrosEntradas}
        />

      {/if}

    </main>

  </div>

</div>