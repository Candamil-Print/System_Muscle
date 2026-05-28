<script lang="ts">
  import Header from '$lib/components/layout/Header.svelte';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';


  import PageTittle from '$lib/components/inventario/InventoryTittle.svelte'
  import InventoryFilters from '$lib/components/inventario/InventoryFilters.svelte';
  import InventoryTable from '$lib/components/inventario/InventoryTable.svelte';

  import { inventoryProducts } from '$lib/data/inventoryData';

  import InventoryGrid from '$lib/components/inventario/InventoryGrid.svelte';

  let view: 'table' | 'grid' = 'table';
</script>

<div class="flex min-h-screen bg-slate-50">
  <Sidebar />

  <div class="flex flex-1 flex-col ml-[280px]">
    <!-- HEADER GLOBAL -->
    <Header />

    <!-- CONTENIDO -->
    <main class="p-6 space-y-6">

      <!-- TITULO -->
      <PageTittle />

      <!-- FILTROS -->
      <InventoryFilters bind:view />

      <!-- TABLA -->
      <div>

        <!-- TABLA -->
       {#if view === 'table'}
        <InventoryTable products={inventoryProducts} />
      {:else}
        <InventoryGrid products={inventoryProducts} />
      {/if}

      </div>

    </main>
  </div>
</div>