<script lang="ts">

  export let ventasHoy: number = 0;

  export let totalVentas: number = 0;

  const formatear = (valor: number): string =>
    new Intl.NumberFormat('es-CO').format(valor);

</script>

<div
  class="flex items-center gap-6 rounded-2xl border border-slate-200 bg-white px-6 py-4 shadow-sm dark:bg-[#1E293B] dark:border-[#334156]"
>

  <div class="text-center">

    <p class="text-xs text-slate-500">
      Ventas Hoy
    </p>

    <h3 class="text-lg font-bold text-slate-800 dark:text-white">
      {ventasHoy}
    </h3>

  </div>

  <div class="h-10 w-px bg-slate-200 dark:bg-[#334156]"></div>

  <div class="text-center">

    <p class="text-xs text-slate-500">
      Total
    </p>

    <h3 class="text-lg font-bold text-[#0C4A6E] dark:text-[#39BDF8]">

      $
      {formatear(totalVentas)}

    </h3>

  </div>

</div>