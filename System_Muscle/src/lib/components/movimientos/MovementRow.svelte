<script lang="ts">
  import Plus from 'lucide-svelte/icons/plus';

  import type {
    MovementDetail
  } from '$lib/services/api/movements/movements.types';

  export let movement: MovementDetail;

  function formatDate(fecha: string) {
    const date = new Date(fecha);

    return {
      fecha: date.toLocaleDateString('es-CO', {
        day: 'numeric',
        month: 'long',
        year: 'numeric'
      }),
      hora: date.toLocaleTimeString('es-CO', {
        hour: 'numeric',
        minute: '2-digit',
        hour12: true
      })
    };
  }

  $: formattedDate = formatDate(movement.fecha);
</script>

<tr class="border-t border-slate-100 transition hover:bg-slate-50 dark:border-[#334156] dark:hover:bg-[#0F172A]">

  <!-- PRODUCTO -->
  <td class="px-6 py-4">
    <span class="text-sm text-slate-600 dark:text-[#E2E8F0]">
      {movement.nombre_producto}
    </span>
  </td>

  <!-- CANTIDAD -->
  <td class="px-6 py-4">
    <span class="flex w-fit items-center gap-1 rounded-full bg-[#1c5476]/10 px-3 py-1 text-xs font-medium text-[#1c5476] dark:bg-[#0C4A6E]/20 dark:text-[#39BDF8]">
      <Plus class="h-3 w-3" />
      {movement.cantidad}
    </span>
  </td>

  <!-- STOCK ANTERIOR -->
  <td class="px-6 py-4 text-sm text-slate-600 dark:text-[#E2E8F0]">
    {movement.stock_anterior}
  </td>

  <!-- STOCK NUEVO -->
  <td class="px-6 py-4 text-sm text-slate-600 dark:text-[#E2E8F0]">
    {movement.stock_nuevo}
  </td>

  <!-- USUARIO -->
  <td class="px-6 py-4 text-sm text-slate-600 dark:text-[#E2E8F0]">
    {movement.nombre_usuario}
  </td>

  <!-- FECHA -->
  <td class="px-6 py-4">
    <div class="flex flex-col">
      <span class="text-sm font-semibold text-slate-800 dark:text-white">
        {formattedDate.fecha}
      </span>

      <span class="text-xs text-slate-500 dark:text-slate-400">
        {formattedDate.hora}
      </span>
    </div>
  </td>

</tr>