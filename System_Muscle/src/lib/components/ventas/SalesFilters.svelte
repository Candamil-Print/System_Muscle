<script lang="ts">
  import { ChevronDown } from 'lucide-svelte';
</script>

<div class="rounded-2xl border border-slate-200 bg-white p-4 shadow-sm">
  <div class="flex flex-col gap-3 md:flex-row">
    <input
      type="text"
      placeholder="Buscar producto..."
      class="h-11 flex-1 rounded-xl border border-slate-200 px-4 text-sm outline-none transition focus:border-cyan-600"
    />

    <div class="relative w-full md:w-48">
      <select
        class="h-11 w-full rounded-xl border border-slate-200 px-4 pr-10 text-sm outline-none focus:border-cyan-600 appearance-none bg-transparent"
      >
        <option>Todos</option>
        <option>Snacks</option>
        <option>Suplementos</option>
        <option>Bebidas</option>
      </select>

      <ChevronDown
        size={18}
        class="absolute right-3 top-1/2 -translate-y-1/2 pointer-events-none text-slate-400"
      />
    </div>
  </div>
</div>