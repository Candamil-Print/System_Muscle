<script lang="ts">

  import InventoryRow from './InventoryRow.svelte';
  import Pagination from './Pagination.svelte';
  import Package from 'lucide-svelte/icons/package';

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

  let currentPage = 1;
  const itemsPerPage = 8;

  $: totalPages = Math.ceil(products.length / itemsPerPage);
  $: paginatedProducts = products.slice(
    (currentPage - 1) * itemsPerPage,
    currentPage * itemsPerPage
  );

  function handlePageChange(page: number) {
    currentPage = page;
  }
</script>

<div class="bg-white border border-slate-200 rounded-2xl overflow-hidden">

  <div class="flex items-start justify-between px-6 py-4 border-b border-slate-200">
    <div class="flex flex-col items-start gap-0">
      <div class="flex items-center gap-2">
        <Package class="w-5 h-5 text-slate-700" />
        
        <h3 class="text-lg font-semibold text-slate-800">
          Lista de Productos
        </h3>
      </div>

      <p class="text-sm text-slate-500">
        {products.length} productos encontrados
      </p>
    </div>
  </div>

  <div class="overflow-x-auto px-6 py-6">
    <div class="border border-slate-200 rounded-xl overflow-hidden">
      <table class="w-full">

        <thead class="bg-[#26557c]">
          <tr class="text-left">
            <th class="px-6 py-4 text-sm font-medium text-[#dee6eb]">
              Producto
            </th>

            <th class="px-6 py-4 text-sm font-medium text-[#dee6eb]">
              Tipo
            </th>

            <th class="px-6 py-4 text-sm font-medium text-[#dee6eb]">
              Precio Costo
            </th>

            <th class="px-6 py-4 text-sm font-medium text-[#dee6eb]">
              Precio Venta
            </th>

            <th class="px-6 py-4 text-sm font-medium text-[#dee6eb]">
              Stock
            </th>

            <th class="px-6 py-4 text-right text-sm font-medium text-[#dee6eb]">
              Acciones
            </th>
          </tr>
        </thead>

        <tbody class="divide-y divide-slate-200">

          {#if products.length === 0}

            <tr>

              <td colspan="6" class="px-6 py-16 text-center">

                <div class="flex flex-col items-center justify-center">

                  <Package class="mb-3 h-10 w-10 text-slate-300" />

                  <h3 class="text-base font-semibold text-slate-700">
                    No hay productos registrados
                  </h3>

                  <p class="mt-1 text-sm text-slate-500">
                    Agrega tu primer producto para comenzar
                  </p>

                </div>

              </td>

            </tr>

          {:else}

            {#each paginatedProducts as product (product.id)}
              <InventoryRow
                {product}
                {onUpdated}
              />
            {/each}

          {/if}

        </tbody>

      </table>
    </div>
  </div>

  <!-- PAGINATION -->
  <div class="px-6 py-6">
    <Pagination
      {currentPage}
      {totalPages}
      onPageChange={handlePageChange}
    />
  </div>

</div>