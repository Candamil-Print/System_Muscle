<script>
  import Eye from 'lucide-svelte/icons/eye';
  import Pencil from 'lucide-svelte/icons/pencil';
  import Trash2 from 'lucide-svelte/icons/trash-2';
</script>

<div class="flex justify-end gap-2">

  <button
    class="w-9 h-9 rounded-xl flex items-center justify-center hover:bg-slate-100 transition"
  >
    <Eye class="w-4 h-4 text-slate-600" />
  </button>

  <button
    class="w-9 h-9 rounded-xl flex items-center justify-center hover:bg-slate-100 transition"
  >
    <Pencil class="w-4 h-4 text-slate-600" />
  </button>

  <button
    class="w-9 h-9 rounded-xl flex items-center justify-center hover:bg-[#1c5476]/10 transition"
  >
    <Trash2 class="w-4 h-4 text-[#1c5476]" />
  </button>

</div>