<script lang="ts">
  import Search from 'lucide-svelte/icons/search';
  import LayoutList from 'lucide-svelte/icons/layout-list';
  import LayoutGrid from 'lucide-svelte/icons/layout-grid';

  export let view: 'table' | 'grid' = 'table';
</script>

<div class="bg-white border border-slate-200 rounded-2xl p-4">
  <div class="flex flex-col xl:flex-row gap-4 xl:items-center">

    <div class="relative flex-1">
      <Search
        class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400"
      />

      <input
        type="text"
        placeholder="Buscar producto..."
        class="w-full h-11 pl-10 pr-4 rounded-xl border border-slate-200 focus:outline-none focus:ring-2 focus:ring-cyan-700/20"
      />
    </div>

    <select
      class="h-11 px-4 pr-10 rounded-xl border border-slate-200 bg-white text-sm focus:outline-none focus:ring-2 focus:ring-cyan-700/20"
    >
      <option>Todos</option>
      <option>Snacks</option>
      <option>Suplementos</option>
      <option>Bebidas</option>
    </select>

    <div class="flex overflow-hidden border border-slate-200 rounded-xl">

      <button
        on:click={() => (view = 'table')}
        class={`w-11 h-11 flex items-center justify-center transition ${
          view === 'table'
            ? 'bg-slate-100 text-slate-900'
            : 'bg-white text-slate-500'
        }`}
      >
        <LayoutList class="w-4 h-4" />
      </button>

      <button
        on:click={() => (view = 'grid')}
        class={`w-11 h-11 flex items-center justify-center transition ${
          view === 'grid'
            ? 'bg-slate-100 text-slate-900'
            : 'bg-white text-slate-500'
        }`}
      >
        <LayoutGrid class="w-4 h-4" />
      </button>

    </div>

  </div>
</div>