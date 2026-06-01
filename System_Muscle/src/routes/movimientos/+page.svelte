<script lang="ts">
  import { Toaster } from 'svelte-sonner';
  
  import Header from '$lib/components/layout/Header.svelte';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';

  import MovementsTitle from '$lib/components/movimientos/MovementsTitle.svelte';
  import MovementsStats from '$lib/components/movimientos/MovementsStats.svelte';
  import MovementsFilters from '$lib/components/movimientos/MovementsFilters.svelte';
  import MovementsTable from '$lib/components/movimientos/MovementsTable.svelte';

  import { onMount } from 'svelte';

  import {
    listarMovements
  } from '$lib/services/api/movements';

  import type {
    MovementDetail
  } from '$lib/services/api/movements/movements.types';

  let movements: MovementDetail[] = [];

  let search = '';

  $: filteredMovements = movements.filter((movement) => {

    return movement.nombre_producto
      .toLowerCase()
      .includes(search.toLowerCase());

  });

  async function loadMovements() {

    try {

      const response = await listarMovements();

      console.log('MOVEMENTS =>', response);

      movements = response;

    } catch (error) {

      console.error(error);

    }

  }

  onMount(() => {

    loadMovements();

  });

</script>

<Toaster position="top-center" />

<div class="flex min-h-screen bg-slate-50">

  <Sidebar />

  <div class="ml-70 flex flex-1 flex-col">

    <Header />

    <main class="space-y-6 p-6">

      <MovementsTitle />

      <MovementsStats
        {movements}
      />

      <MovementsFilters
        bind:search
      />

      <MovementsTable
        movements={filteredMovements}
      />

    </main>

  </div>

</div>