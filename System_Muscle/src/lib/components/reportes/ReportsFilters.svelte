<script>
  import Search from 'lucide-svelte/icons/search';
  import Download from 'lucide-svelte/icons/download';
</script>

<div class="bg-white border border-slate-200 rounded-2xl p-5">

  <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-5 gap-4">

    <div>
      <label class="text-xs font-semibold text-slate-500 uppercase">
        Fecha Inicio
      </label>

      <input
        type="date"
        class="w-full mt-2 h-11 px-4 rounded-xl border border-slate-200"
      />
    </div>

    <div>
      <label class="text-xs font-semibold text-slate-500 uppercase">
        Fecha Fin
      </label>

      <input
        type="date"
        class="w-full mt-2 h-11 px-4 rounded-xl border border-slate-200"
      />
    </div>

    <div>
      <label class="text-xs font-semibold text-slate-500 uppercase">
        Método Pago
      </label>

      <select
        class="w-full mt-2 h-11 px-4 rounded-xl border border-slate-200"
      >
        <option>Todos</option>
        <option>Efectivo</option>
        <option>Transferencia</option>
      </select>
    </div>

    <div>
      <label class="text-xs font-semibold text-slate-500 uppercase">
        Vendedor
      </label>

      <select
        class="w-full mt-2 h-11 px-4 rounded-xl border border-slate-200"
      >
        <option>Todos</option>
      </select>
    </div>

    <div class="flex items-end gap-3">

      <button
        class="h-11 px-5 rounded-xl bg-[#0C4A6E] text-white flex items-center gap-2"
      >
        <Search class="w-4 h-4" />

        Filtrar
      </button>

      <button
        class="h-11 w-11 rounded-xl border border-slate-200 flex items-center justify-center"
      >
        <Download class="w-4 h-4" />
      </button>

    </div>

  </div>

</div>