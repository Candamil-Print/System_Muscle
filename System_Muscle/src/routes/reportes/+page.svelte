<script lang="ts">
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

  let activeTab = 'ventas';

  let filtros = {
    fechaInicio: '',
    fechaFin: '',
    metodoPago: 'todos',
    vendedor: 'todos'
  };

  function aplicarFiltros(nuevosFiltros) {
    filtros = nuevosFiltros;
  }
</script>

<div class="flex min-h-screen bg-slate-50">
  <Sidebar />

  <div class="ml-[280px] flex flex-1 flex-col">
    <Header />

    <main class="space-y-6 p-6">
      <div>
        <h1 class="text-3xl font-bold text-slate-800">
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

        <EntriesFilters />

        <EntriesStats
          totalIngresado={2450}
          totalProductos={12}
          stockBajo={3}
        />

        <EntriesCharts />

        <EntriesTable />

      {/if}

    </main>
  </div>
</div>