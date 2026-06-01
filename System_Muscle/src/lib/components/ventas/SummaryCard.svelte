<script lang="ts">

  export let ventasHoy: number = 0;

  export let totalVentas: number = 0;

  const formatear = (valor: number): string =>
    new Intl.NumberFormat('es-CO').format(valor);

</script>

<div
  class="flex items-center gap-6 rounded-2xl border border-slate-200 bg-white px-6 py-4 shadow-sm"
>

  <div class="text-center">

    <p class="text-xs text-slate-500">
      Ventas Hoy
    </p>

    <h3 class="text-lg font-bold text-slate-800">
      {ventasHoy}
    </h3>

  </div>

  <div class="h-10 w-px bg-slate-200"></div>

  <div class="text-center">

    <p class="text-xs text-slate-500">
      Total
    </p>

    <h3 class="text-lg font-bold text-[#0C4A6E]">

      $
      {formatear(totalVentas)}

    </h3>

  </div>

</div>