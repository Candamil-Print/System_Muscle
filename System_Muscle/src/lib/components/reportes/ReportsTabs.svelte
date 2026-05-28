<script>
  import DollarSign from 'lucide-svelte/icons/dollar-sign';
  import ArrowDownCircle from 'lucide-svelte/icons/arrow-down-circle';
</script>

<div class="flex gap-2 border-b border-slate-200">
  
  <button
    class="flex items-center gap-2 px-5 py-3 border-b-2 border-[#0C4A6E] text-[#0C4A6E] font-semibold"
  >
    <DollarSign class="w-4 h-4" />

    Reporte Ventas
  </button>

  <button
    class="flex items-center gap-2 px-5 py-3 text-slate-500"
  >
    <ArrowDownCircle class="w-4 h-4" />

    Reporte Entradas
  </button>

</div>