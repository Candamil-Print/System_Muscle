<script lang="ts">

  import { Search } from 'lucide-svelte';

  export let search = '';

</script>

<div
  class="rounded-2xl border border-slate-200 bg-white dark:bg-[#1E293B] dark:border-[#334156] p-4 shadow-sm"
>

  <div class="relative">


<Search
  size={18}
  class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-slate-400"
/>

<input
  bind:value={search}
  type="text"
  placeholder="Buscar por nombre, documento o email..."
  class="w-full rounded-xl border border-slate-200 py-3 pl-10 pr-4 outline-none transition focus:border-sky-500 dark:border-[#334156] dark:text-white"
/>


  </div>

</div>
