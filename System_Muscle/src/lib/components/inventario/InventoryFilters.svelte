<script lang="ts">
  import Search from 'lucide-svelte/icons/search';
  import LayoutList from 'lucide-svelte/icons/layout-list';
  import LayoutGrid from 'lucide-svelte/icons/layout-grid';
  import ChevronDown from 'lucide-svelte/icons/chevron-down';

  export let view: 'table' | 'grid' = 'table';

  export let search = '';
  export let category = 'Todos';
</script>

<div class="rounded-2xl border border-slate-200 bg-white p-4">

  <div class="flex flex-col gap-4 xl:flex-row xl:items-center">

    <!-- SEARCH -->
    <div class="relative flex-1">

      <Search
        class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-slate-400"
      />

      <input
        type="text"
        bind:value={search}
        placeholder="Buscar producto..."
        class="h-11 w-full rounded-xl border border-slate-200 pl-10 pr-4 focus:outline-none focus:ring-2 focus:ring-cyan-700/20"
      />

    </div>

    <!-- CATEGORY -->
    <div class="relative">
      <select
        bind:value={category}
        class="h-11 w-full appearance-none rounded-xl border border-slate-200 bg-white px-4 py-3 pr-11 text-sm text-slate-700 outline-none transition duration-200 focus:border-[#0C4A6E] focus:ring-4 focus:ring-sky-100 focus:scale-[1.01]"
      >
        <option value="Todos">Todos</option>
        <option value="SNACKS">Snacks</option>
        <option value="SUPLEMENTOS">Suplementos</option>
        <option value="BEBIDAS">Bebidas</option>
      </select>

      <ChevronDown
        size={18}
        class="pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 text-slate-400"
      />
    </div>

    <!-- VIEW -->
    <div class="flex overflow-hidden rounded-xl border border-slate-200">

      <button
        onclick={() => (view = 'table')}
        class={`flex h-11 w-11 items-center justify-center transition ${
          view === 'table'
            ? 'bg-slate-100 text-slate-900'
            : 'bg-white text-slate-500'
        }`}
      >
        <LayoutList class="h-4 w-4" />
      </button>

      <button
        onclick={() => (view = 'grid')}
        class={`flex h-11 w-11 items-center justify-center transition ${
          view === 'grid'
            ? 'bg-slate-100 text-slate-900'
            : 'bg-white text-slate-500'
        }`}
      >
        <LayoutGrid class="h-4 w-4" />
      </button>

    </div>

  </div>

</div>