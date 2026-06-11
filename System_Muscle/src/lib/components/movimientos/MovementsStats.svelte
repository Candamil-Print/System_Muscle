<script lang="ts">

  import Package from 'lucide-svelte/icons/package';
  import TrendingUp from 'lucide-svelte/icons/trending-up';
  import ArrowDownWideNarrow from 'lucide-svelte/icons/arrow-down-wide-narrow';

  import type {
    MovementDetail
  } from '$lib/services/api/movements/movements.types';

  export let movements: MovementDetail[] = [];


  // TOTAL MOVIMIENTOS
  $: totalMovements = movements.length;

  // FECHA HOY
  const today = new Date().toISOString().split('T')[0];

  // ENTRADAS HOY
  $: todayEntries = movements.filter((movement) => {

    return movement.fecha?.startsWith(today);

  }).length;

  // TOTAL UNIDADES
  $: totalUnits = movements.reduce((acc, movement) => {

    return acc + movement.cantidad;

  }, 0);

</script>

<div class="grid grid-cols-1 gap-5 md:grid-cols-3">

  <!-- TOTAL MOVIMIENTOS -->
  <div class="rounded-2xl border border-slate-200 bg-white p-5 dark:border-[#334156] dark:bg-[#1E293B] ">

    <div class="flex items-center justify-between">

      <span class="text-sm text-slate-500">
        Total Movimientos
      </span>

      <TrendingUp class="h-5 w-5 text-[#0C4A6E] dark:text-[#39BDF8]" />

    </div>

    <h2 class="mt-4 text-4xl font-bold text-slate-800 dark:text-white">
      {totalMovements}
    </h2>

    <p class="mt-1 text-sm text-slate-500">
      entradas registradas
    </p>

  </div>

  <!-- ENTRADAS HOY -->
  <div class="rounded-2xl border border-slate-200 bg-white p-5 dark:border-[#334156] dark:bg-[#1E293B]">

    <div class="flex items-center justify-between">

      <span class="text-sm text-slate-500">
        Entradas Hoy
      </span>

      <Package class="h-5 w-5 text-[#0C4A6E] dark:text-[#39BDF8]" />

    </div>

    <h2 class="mt-4 text-4xl font-bold text-slate-800 dark:text-white">
      {todayEntries}
    </h2>

    <p class="mt-1 text-sm text-slate-500">
      movimientos del día
    </p>

  </div>

  <!-- UNIDADES -->
  <div class="rounded-2xl border border-slate-200 bg-white p-5 dark:border-[#334156] dark:bg-[#1E293B]">

    <div class="flex items-center justify-between">

      <span class="text-sm text-slate-500">
        Unidades Ingresadas
      </span>

      <ArrowDownWideNarrow class="h-5 w-5 text-[#0C4A6E] dark:text-[#39BDF8]" />

    </div>

    <h2 class="mt-4 text-4xl font-bold text-[#0C4A6E] dark:text-[#39BDF8]">
      {totalUnits.toLocaleString('es-CO')}
    </h2>

    <p class="mt-1 text-sm text-slate-500">
      unidades totales
    </p>

  </div>

</div>