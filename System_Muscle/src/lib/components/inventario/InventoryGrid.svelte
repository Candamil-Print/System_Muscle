<script lang="ts">
  import EllipsisVertical from 'lucide-svelte/icons/ellipsis-vertical';
  import Package from 'lucide-svelte/icons/package';

  import InventoryActions from './InventoryActions.svelte';
  import Pagination from './Pagination.svelte';

  interface Product {
    id: number;
    name: string;
    type: string;
    cost: number;
    sale: number;
    stock: number;
    status: string;
    image: string;
  }

  export let products: Product[] = [];
  export let onUpdated: () => void;

  let openMenu: number | null = null;
  let currentPage = 1;
  const itemsPerPage = 8;

  const getStockPercentage = (stock: number) => {
    const current = stock;
    const total = 100;

    return (current / total) * 100;
  };

  $: totalPages = Math.ceil(products.length / itemsPerPage);
  $: paginatedProducts = products.slice(
    (currentPage - 1) * itemsPerPage,
    currentPage * itemsPerPage
  );

  function handlePageChange(page: number) {
    currentPage = page;
  }
</script>

<div class="rounded-2xl border border-slate-200 dark:border-[#334156] bg-white dark:bg-[#1E293B] overflow-visible">

  <!-- HEADER -->
  <div class="flex items-start justify-between px-6 py-4 border-b border-slate-200 dark:border-[#334156]">
    <div class="flex flex-col items-start gap-0">
      <div class="flex items-center gap-2">
        <Package class="w-5 h-5 text-slate-700 dark:text-[#94A3B8]" />
        
        <h3 class="text-lg font-semibold text-slate-800 dark:text-white">
          Lista de Productos
        </h3>
      </div>

      <p class="text-sm text-slate-500 dark:text-[#64748B]">
        {products.length} productos encontrados
      </p>
    </div>
  </div>

  <!-- GRID -->
  <div class="grid grid-cols-1 gap-6 p-6 sm:grid-cols-2 xl:grid-cols-4 overflow-visible">
    {#if products.length === 0}

      <div class="col-span-full">

        <div class="flex flex-col items-center justify-center rounded-2xl border border-slate-300 dark:border-[#334156] py-20">

          <Package class="mb-3 h-10 w-10 text-slate-300 dark:text-[#475569]" />

          <h3 class="text-lg font-semibold text-slate-700 dark:text-white">
            No hay productos registrados
          </h3>

          <p class="mt-2 text-sm text-slate-500 dark:text-[#64748B]">
            Agrega tu primer producto para comenzar.
          </p>

        </div>

      </div>

    {:else}

      {#each paginatedProducts as product (product.id)}

        <div class="relative rounded-2xl border border-slate-200 dark:border-[#334156] bg-white dark:bg-[#111827] overflow-visible">

          <!-- IMAGE -->
          <div class="h-52 overflow-hidden rounded-t-2xl bg-slate-100 dark:bg-[#334156]">

            {#if product.image && product.image.trim() !== ''}

              <img
                src={product.image}
                alt={product.name}
                class="h-full w-full object-cover rounded-t-2xl"
              />

            {:else}

              <div
                class="flex h-full w-full items-center justify-center bg-slate-100 dark:bg-[#334156]"
              >
                <div
                  class="flex h-40 w-40 items-center justify-center rounded-[2rem]"
                >
                  <Package class="h-16 w-16 text-slate-400 dark:text-[#64748B]" />
                </div>
              </div>

            {/if}

          </div>

          <!-- CONTENT -->
          <div class="p-4">

            <!-- TOP -->
            <div class="mb-3 flex items-start justify-between">

              <p class="text-xs uppercase tracking-wide text-slate-400 dark:text-[#64748B]">
                {product.type}
              </p>

              <div class="relative">

                <button
                  class="rounded-lg p-2 transition hover:bg-slate-100 dark:hover:bg-[#334156]"
                  onclick={() =>
                    openMenu =
                      openMenu === product.id
                        ? null
                        : product.id
                  }
                >
                  <EllipsisVertical class="h-4 w-4 text-slate-500 dark:text-[#94A3B8]" />
                </button>

                {#if openMenu === product.id}

                  <div
                    class="absolute right-0 top-11 z-9999 w-44 rounded-xl border border-slate-200 dark:border-[#334156] bg-white dark:bg-[#1E293B] p-2 shadow-2xl"
                  >

                    <InventoryActions
                      {product}
                      {onUpdated}
                    />

                  </div>

                {/if}

              </div>

            </div>

            <h4 class="text-xl font-semibold text-slate-800 dark:text-white">
              {product.name}
            </h4>

            <div class="mt-3 flex items-center gap-2">

              <span class="text-3xl font-bold text-[#0f4c81] dark:text-[#0EA5E9]">
                ${product.sale.toLocaleString('es-CO', { minimumFractionDigits: 0, maximumFractionDigits: 0 })}
              </span>

              <span class="text-sm text-slate-400 dark:text-[#64748B] line-through">
                ${product.cost.toLocaleString('es-CO', { minimumFractionDigits: 0, maximumFractionDigits: 0 })}
              </span>

            </div>

            <div class="mt-5">

              <div class="mb-2 flex items-center justify-between text-sm text-slate-500 dark:text-[#94A3B8]">
                <span>Stock</span>
                <span>{product.stock}</span>
              </div>

              <div class="h-2 w-full overflow-hidden rounded-full bg-slate-100 dark:bg-[#334156]">

                <div
                  class="h-full rounded-full bg-[#0f4c81] dark:bg-[#0EA5E9]"
                  style={`width:${getStockPercentage(product.stock)}%`}
                ></div>

              </div>

            </div>

          </div>

        </div>

      {/each}

    {/if}

  </div>

  <!-- PAGINATION -->
  <div class="px-6 pb-6">
    <Pagination
      {currentPage}
      {totalPages}
      onPageChange={handlePageChange}
    />
  </div>

</div>