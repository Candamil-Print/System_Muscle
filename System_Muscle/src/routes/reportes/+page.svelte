<script>
  import Header from '$lib/components/layout/Header.svelte';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';

  import ReportsTabs from '$lib/components/reportes/ReportsTabs.svelte';
  import ReportsFilters from '$lib/components/reportes/ReportsFilters.svelte';
  import ReportsStats from '$lib/components/reportes/ReportsStats.svelte';
  import ReportsCharts from '$lib/components/reportes/ReportsCharts.svelte';
  import ReportsTable from '$lib/components/reportes/ReportsTable.svelte';

  import {
    salesHistory,
    salesStats
  } from '$lib/data/reportsData';

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

  <div class="flex flex-1 flex-col ml-[280px]">

    <Header />

    <main class="p-6 space-y-6">

      <div>

        <h1 class="text-3xl font-bold text-slate-800">
          Reportes
        </h1>

        <p class="text-slate-500 mt-1">
          Visualización y análisis de ventas
        </p>

      </div>

      <ReportsTabs />

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

    </main>

  </div>

</div>