<script lang="ts">
  import { PackageSearch } from 'lucide-svelte';

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
      class="flex flex-col items-center justify-center rounded-2xl border border-dashed border-slate-300 bg-white py-20 text-center dark:border-[#334156] dark:bg-[#1E293B]"
    >
      <div
        class="mb-4 flex h-16 w-16 items-center justify-center rounded-2xl bg-slate-100 dark:bg-[#0F172A]"
      >
        <PackageSearch
          size={32}
          class="text-slate-500 dark:text-[#39BDF8]"
        />
      </div>

      <h3
        class="text-lg font-semibold text-slate-700 dark:text-white"
      >
        No se encontraron productos
      </h3>

      <p
        class="mt-2 max-w-md text-sm text-slate-500 dark:text-slate-400"
      >
        No existen productos que coincidan con los criterios de búsqueda ingresados.
        Verifica los filtros aplicados o intenta con otros términos.
      </p>
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

