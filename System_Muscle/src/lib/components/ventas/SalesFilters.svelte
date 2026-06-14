<script lang="ts">
  import { ChevronDown, BrushCleaning } from 'lucide-svelte';
  import { createEventDispatcher } from 'svelte';

  export let categorias: string[] = [];

  const dispatch = createEventDispatcher();

  let busqueda = '';
  let categoria = 'Todos';

  function actualizarFiltros() {
    dispatch('change', {
      busqueda,
      categoria
    });
  }

  function limpiarFiltros() {
    busqueda = '';
    categoria = 'Todos';

    actualizarFiltros();
  }

</script>

<div
  class="rounded-2xl border border-slate-200 bg-white dark:bg-[#1E293B] dark:border-[#334156] p-4 shadow-sm"
>
  <div class="flex flex-col gap-3 md:flex-row md:items-center">

    <input
      bind:value={busqueda}
      on:input={actualizarFiltros}
      type="text"
      placeholder="Buscar producto..."
      class="h-11 flex-1 rounded-xl border border-slate-200 px-4 text-sm outline-none transition focus:border-cyan-600 dark:border-[#334156] dark:text-white dark:focus:border-[#39BDF8] dark:focus:ring-4 dark:focus:ring-[#39BDF8]/20"
    />

    <div class="relative w-full md:w-48">

      <select
        bind:value={categoria}
        on:change={actualizarFiltros}
        class="h-11 w-full rounded-xl border border-slate-200 px-4 pr-10 text-sm outline-none focus:border-cyan-600 appearance-none bg-transparent dark:border-[#334156] dark:text-white dark:focus:border-[#39BDF8] dark:focus:ring-4 dark:focus:ring-[#39BDF8]/20"
      >
        {#each categorias as item}
          <option value={item}>
            {item.charAt(0).toUpperCase() + item.slice(1).toLowerCase()}
          </option>
        {/each}
      </select>

      <ChevronDown
        size={18}
        class="absolute right-3 top-1/2 -translate-y-1/2 pointer-events-none text-slate-400 dark:text-white"
      />



    </div>

    <button
        on:click={limpiarFiltros}
        class="flex h-11 items-center justify-center gap-2 rounded-xl border border-slate-200 px-4 transition hover:bg-slate-100 dark:border-[#334156] dark:text-white dark:hover:bg-[#0F172A]"
        title="Limpiar filtros"
      >
        <BrushCleaning size={18} />
    </button>

  </div>
</div>