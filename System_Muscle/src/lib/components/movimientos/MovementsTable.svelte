<script lang="ts">
  import MovementRow from './MovementRow.svelte';
  import ClipboardClock from 'lucide-svelte/icons/clipboard-clock';

  interface Movement {
    id: number;
    product: string;
    quantity: number;
    previousStock: number;
    newStock: number;
    receivedBy: string;
    date: string;
  }

  export let movements: Movement[] = [];
</script>

<div class="bg-white border border-slate-200 rounded-2xl overflow-hidden">

  <div class="flex items-start justify-between px-6 py-4 border-b border-slate-200">
    <div class="flex flex-col items-start gap-0">
      <div class="flex items-center gap-2">
        <ClipboardClock class="w-5 h-5 text-slate-700" />
        
        <h3 class="text-lg font-semibold text-slate-800">
          Historial de Entradas
        </h3>
      </div>

      <p class="text-sm text-slate-500">
        {movements.length} movimientos encontrados
      </p>
    </div>
  </div>

  <div class="overflow-x-auto px-6 py-6">
    <div class="border border-slate-200 rounded-xl overflow-hidden">
      <table class="w-full">

        <thead class="bg-[#26557c]">
          <tr class="text-left">
            <th class="px-6 py-4 text-sm font-medium text-[#ffffff]">
              Producto
            </th>

            <th class="px-6 py-4 text-sm font-medium text-[#ffffff]">
              Cantidad
            </th>

            <th class="px-6 py-4 text-sm font-medium text-[#ffffff]">
              Stock Anterior
            </th>

            <th class="px-6 py-4 text-sm font-medium text-[#ffffff]">
              Stock Nuevo
            </th>

            <th class="px-6 py-4 text-sm font-medium text-[#ffffff]">
              Recibido Por
            </th>

            <th class="px-6 py-4  text-sm font-medium text-[#ffffff]">
              Fecha y Hora
            </th>
          </tr>
        </thead>

        <tbody class="divide-y divide-slate-200">
          {#each movements as movement}
            <MovementRow {movement} />
          {/each}
        </tbody>

      </table>
    </div>
  </div>
</div>