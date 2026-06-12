<script lang="ts">
  import ProductCard from './ProductCard.svelte';

  import type { ProductoVenta } from '$lib/services/api/sale/sale.types';

  export let productos: ProductoVenta[] = [];

  let loading = false;

</script>

<!-- LOADING -->
{#if loading}

  <div
    class="flex items-center justify-center py-20 text-slate-500"
  >
    Cargando productos...
  </div>

{:else}

  <!-- SIN RESULTADOS -->
  {#if productos.length === 0}

    <div
      class="flex items-center justify-center rounded-2xl border border-dashed border-slate-300 bg-white py-20 text-slate-500 dark:bg-[#1E293B] dark:border-[#334156] dark:text-slate-400"
    >
      No hay productos disponibles
    </div>

  {:else}

    <!-- GRID -->
    <div class="grid gap-5 sm:grid-cols-2 lg:grid-cols-3">

      {#each productos as producto (producto.id_producto)}
        <ProductCard {producto} />
      {/each}

    </div>

  {/if}

{/if}

