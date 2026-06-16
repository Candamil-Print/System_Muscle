<script lang="ts">

  import { onMount } from 'svelte';

  import RegisterEntryModal
  from '$lib/components/movimientos/RegisterEntryModal.svelte';

  import { listarProductos }
  from '$lib/services/api/inventory';

  export let onCreated: () => void = () => {};

  let showEntryModal = false;

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

  async function loadProducts() {

    try {

      const response = await listarProductos();

      products = response.map((p) => ({
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

    } catch (error) {

      console.error(error);

    }

  }

  onMount(() => {
    loadProducts();
  });

</script>

<div class="mb-6 flex items-center justify-between">

  <div>

    <h1 class="text-3xl font-bold text-slate-800 dark:text-white">
      Movimientos
    </h1>

    <p class="mt-1 text-sm text-slate-500">
      Gestiona los movimientos de productos
    </p>

  </div>

  <div class="mb-6 flex items-center justify-between">



    <button
      on:click={() => showEntryModal = true}
      class="rounded-lg bg-[#0C4A6E] dark:text-[#39BDF8] px-4 py-2.5 text-sm font-medium text-white hover:bg-[#0a3a52]"
    >
      + Nuevo Movimiento
    </button>

  </div>

</div>

<RegisterEntryModal
    open={showEntryModal}
    products={products}
    onClose={() => showEntryModal = false}
    onCreated={async () => {
        await loadProducts();   // actualizar inventario
        await onCreated();      // actualizar movimientos
    }}
/>