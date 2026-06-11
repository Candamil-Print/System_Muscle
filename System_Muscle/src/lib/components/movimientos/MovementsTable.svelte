<script lang="ts">
  import MovementRow from './MovementRow.svelte';
  import ClipboardClock from 'lucide-svelte/icons/clipboard-clock';
  import Pagination from './Pagination.svelte';
  import Search from 'lucide-svelte/icons/search';

  import type {
    MovementDetail
  } from '$lib/services/api/movements/movements.types';

  export let movements: MovementDetail[] = [];

  // PAGINACIÓN
  let currentPage = 1;

  const itemsPerPage = 8;

  // TOTAL PÁGINAS
  $: totalPages = Math.ceil(
    movements.length / itemsPerPage
  );

  // ELEMENTOS DE LA PÁGINA ACTUAL
  $: paginatedMovements = movements.slice(
    (currentPage - 1) * itemsPerPage,
    currentPage * itemsPerPage
  );
</script>

<div class="bg-white border border-slate-200 rounded-2xl overflow-hidden dark:bg-[#1E293B] dark:border-[#334156]">

  <div class="flex items-start justify-between px-6 py-4 border-b border-slate-200 dark:border-[#334156]">

    <div class="flex flex-col items-start gap-0">

      <div class="flex items-center gap-2">

        <ClipboardClock class="w-5 h-5 text-slate-700 dark:text-[#39BDF8]" />
        
        <h3 class="text-lg font-semibold text-slate-800 dark:text-white">
          Historial de Entradas
        </h3>

      </div>

      <p class="text-sm text-slate-500 dark:text-slate-400">
        {movements.length} movimientos encontrados
      </p>

    </div>

  </div>

  <div class="overflow-x-auto px-6 py-6 dark:bg-[#1E293B]">

    <div class="border border-slate-200 rounded-xl overflow-hidden dark:border-[#475569]">

      <table class="w-full">

        <thead class="bg-[#26557c] dark:bg-[#334156] border-b-2 border-slate-300 dark:border-b-[#475569]">

          <tr class="text-left">

            <th class="px-6 py-4 text-sm font-medium text-[#dee6eb]">
              Producto
            </th>

            <th class="px-6 py-4 text-sm font-medium text-[#dee6eb]">
              Cantidad
            </th>

            <th class="px-6 py-4 text-sm font-medium text-[#dee6eb]">
              Stock Anterior
            </th>

            <th class="px-6 py-4 text-sm font-medium text-[#dee6eb]">
              Stock Nuevo
            </th>

            <th class="px-6 py-4 text-sm font-medium text-[#dee6eb]">
              Recibido Por
            </th>

            <th class="px-6 py-4 text-sm font-medium text-[#dee6eb]">
              Fecha y Hora
            </th>

          </tr>

        </thead>

        <tbody class="divide-y divide-slate-200 dark:divide-[#334156]">

          {#if paginatedMovements.length > 0}

            {#each paginatedMovements as movement}

              <MovementRow {movement} />

            {/each}

          {:else}

            <tr>

              <td
                colspan="6"
                class="px-6 py-16 text-center"
              >

                <div class="flex flex-col items-center justify-center">

                  <Search class="mb-3 h-10 w-10 text-slate-300 dark:text-slate-600" />

                  <h3 class="text-base font-semibold text-slate-700 dark:text-slate-300">
                    No se encontraron resultados
                  </h3>

                  <p class="mt-1 text-sm text-slate-500 dark:text-slate-400">
                    Intenta buscar otro producto
                  </p>

                </div>

              </td>

            </tr>

          {/if}

        </tbody>

      </table>

    </div>

  </div>

  <!-- PAGINACIÓN -->
  <div class="px-6 pb-6 dark:bg-[#1E293B]">

    <Pagination
      {currentPage}
      {totalPages}
      onPageChange={(page) => {

        currentPage = page;

      }}
    />

  </div>

</div>

