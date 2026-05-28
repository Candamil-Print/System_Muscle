<script>
  import Search from 'lucide-svelte/icons/search';
</script>

<div class="bg-white border border-slate-200 rounded-2xl p-4">

  <div class="relative">
    <Search
      class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400"
    />

    <input
      type="text"
      placeholder="Buscar por producto..."
      class="w-full h-11 pl-10 pr-4 rounded-xl border border-slate-200 focus:outline-none focus:ring-2 focus:ring-cyan-700/20"
    />
  </div>

</div>