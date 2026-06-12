<script lang="ts">

  import Plus from 'lucide-svelte/icons/plus';

  import type {
    MovementDetail
  } from '$lib/services/api/movements/movements.types';

  export let movement: MovementDetail;

</script>

<tr class="border-t border-slate-100 dark:border-[#334156] transition hover:bg-slate-50 dark:hover:bg-[#0F172A]">

  <!-- PRODUCTO -->
  <td class="px-6 py-4">

    <span class="text-sm text-slate-600 dark:text-[#E2E8F0]">
      {movement.nombre_producto}
    </span>

  </td>

  <!-- CANTIDAD -->
  <td class="px-6 py-4">

    <span class="flex w-fit items-center gap-1 rounded-full bg-[#1c5476]/10 dark:bg-[#0C4A6E]/20 px-3 py-1 text-xs font-medium text-[#1c5476] dark:text-[#39BDF8]">

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
  <td class="px-6 py-4 text-sm text-slate-600 dark:text-[#E2E8F0]">
    {movement.fecha}
  </td>

</tr>