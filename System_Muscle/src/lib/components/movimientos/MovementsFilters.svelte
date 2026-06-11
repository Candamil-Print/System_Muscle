<script lang="ts">

  import Search from 'lucide-svelte/icons/search';

  export let search = '';

</script>

<div class="rounded-2xl border border-slate-200 bg-white p-4 dark:bg-[#1E293B] dark:border-[#334156]">

  <div class="relative">

    <Search
      class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-slate-400"
    />

    <input
      bind:value={search}
      type="text"
      placeholder="Buscar por producto..."
      class="h-11 w-full rounded-xl border border-slate-200 pl-10 pr-4 focus:outline-none focus:ring-2 focus:ring-cyan-700/20 dark:border-[#334156] dark:text-white"
    />

  </div>

</div>