<script>
  import Package from 'lucide-svelte/icons/package';
  import TrendingUp from 'lucide-svelte/icons/trending-up';
  import ArrowDownWideNarrow from 'lucide-svelte/icons/arrow-down-wide-narrow';
</script>

<div class="grid grid-cols-1 md:grid-cols-3 gap-5">

  <div class="bg-white border border-slate-200 rounded-2xl p-5">
    <div class="flex items-center justify-between">
      <span class="text-sm text-slate-500">
        Total Movimientos
      </span>

      <TrendingUp class="w-5 h-5 text-[#0C4A6E]" />
    </div>

    <h2 class="text-4xl font-bold mt-4 text-slate-800">
      8
    </h2>

    <p class="text-sm text-slate-500 mt-1">
      entradas registradas
    </p>
  </div>

  <div class="bg-white border border-slate-200 rounded-2xl p-5">
    <div class="flex items-center justify-between">
      <span class="text-sm text-slate-500">
        Entradas Hoy
      </span>

      <Package class="w-5 h-5 text-[#0C4A6E]" />
    </div>

    <h2 class="text-4xl font-bold mt-4 text-slate-800">
      2
    </h2>

    <p class="text-sm text-slate-500 mt-1">
      movimientos del día
    </p>
  </div>

  <div class="bg-white border border-slate-200 rounded-2xl p-5">
    <div class="flex items-center justify-between">
      <span class="text-sm text-slate-500">
        Unidades Ingresadas
      </span>

      <ArrowDownWideNarrow class="w-5 h-5 text-[#0C4A6E]" />
    </div>

    <h2 class="text-4xl font-bold mt-4 text-[#0C4A6E]">
      150
    </h2>

    <p class="text-sm text-slate-500 mt-1">
      unidades totales
    </p>
  </div>

</div>