<script>
  import Header from '$lib/components/layout/Header.svelte';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';

  import MovementsTitle from '$lib/components/movimientos/MovementsTitle.svelte';
  import MovementsStats from '$lib/components/movimientos/MovementsStats.svelte';
  import MovementsFilters from '$lib/components/movimientos/MovementsFilters.svelte';
  import MovementsTable from '$lib/components/movimientos/MovementsTable.svelte';

  import { movementsData } from '$lib/data/movementsData';
</script>

<div class="flex min-h-screen bg-slate-50">

  <Sidebar />

  <div class="flex flex-1 flex-col ml-70">

    <Header />

    <main class="p-6 space-y-6">

      <MovementsTitle />

      <MovementsStats />

      <MovementsFilters />

      <MovementsTable
        movements={movementsData}
      />

    </main>

  </div>

</div>