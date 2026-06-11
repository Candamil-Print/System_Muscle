<script lang="ts">

  import { Toaster } from 'svelte-sonner';
  import Header from '$lib/components/layout/Header.svelte';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';

  import PageTittle from '$lib/components/inventario/InventoryTittle.svelte';
  import InventoryFilters from '$lib/components/inventario/InventoryFilters.svelte';
  import InventoryTable from '$lib/components/inventario/InventoryTable.svelte';

  import CreateProductModal from '$lib/components/inventario/CreateProductModal.svelte';

  import { onMount } from 'svelte';
  import { listarProductos } from '$lib/services/api/inventory';

  import InventoryGrid from '$lib/components/inventario/InventoryGrid.svelte';

  let view: 'table' | 'grid' = 'table';
  let showCreateModal = false;

  // SEARCH + FILTERS
  let search = '';
  let category = 'Todos';

  interface Product {
    id: number;
    name: string;
    type: string;
    cost: number;
    sale: number;
    stock: number;
    stockMax: number;
    status: string;
    image: string;
  }

  let products: Product[] = [];

  // FILTERED PRODUCTS
  $: filteredProducts = products.filter((product) => {

    const matchesSearch =
      product.name
        .toLowerCase()
        .includes(search.toLowerCase());

    const matchesCategory =
      category === 'Todos'
        ? true
        : product.type === category;

    return matchesSearch && matchesCategory;
  });

  async function loadProducts() {

    try {

      const response = await listarProductos();

      console.log('RESPONSE BACKEND =>', response);

      const mappedProducts = response.map((p) => ({
        id: p.id_producto,
        name: p.nombre,
        type: p.tipo_producto,
        cost: p.precio_costo,
        sale: p.precio_sugerido,

        stock: p.stock_actual ?? 0,
        stockMax: p.stock_maximo ?? 0,

        status: p.activo ? 'Activo' : 'Inactivo',
        image: p.imagen_url ?? ''
      }));

      console.log('MAPPED =>', mappedProducts);

      products = [...mappedProducts];

    } catch (error) {

      console.error(error);
    }
  }

  onMount(() => {
    loadProducts();
  });

</script>

<Toaster position="top-center" />

<div class="flex min-h-screen bg-slate-50 dark:bg-[#111827]">

  <Sidebar />

  <div class="ml-[280px] flex flex-1 flex-col">

    <!-- HEADER GLOBAL -->
    <Header />

    <!-- CONTENIDO -->
    <main class="space-y-6 p-6">

      <!-- TITULO -->
      <PageTittle
        on:create={() => showCreateModal = true}
      />

      <!-- FILTROS -->
      <InventoryFilters
        bind:view
        bind:search
        bind:category
      />

      <!-- TABLA -->
      <div>

        {#if view === 'table'}

          <InventoryTable
            products={filteredProducts}
            onUpdated={loadProducts}
          />

        {:else}

          <InventoryGrid
            products={filteredProducts}
            onUpdated={loadProducts}
          />

        {/if}

      </div>

    </main>

  </div>

</div>

<CreateProductModal
  open={showCreateModal}
  onClose={() => showCreateModal = false}
  onCreated={loadProducts}
/>