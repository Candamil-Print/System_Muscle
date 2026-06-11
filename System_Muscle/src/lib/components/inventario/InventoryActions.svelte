<script lang="ts">
  import Eye from 'lucide-svelte/icons/eye';
  import Pencil from 'lucide-svelte/icons/pencil';
  import Trash2 from 'lucide-svelte/icons/trash-2';

  import EditProductModal from './EditProductModal.svelte';
  import DeleteProductModal from './DeleteProductModal.svelte';
  import ViewProductModal from './ViewProductModal.svelte';

  export let product;
  export let onUpdated;

  let showEditModal = false;
  let showDeleteModal = false;
  let showViewModal = false;
</script>

<div class="flex justify-end gap-2">

  <!-- VER -->
  <button
    class="flex h-9 w-9 items-center justify-center rounded-xl transition hover:bg-slate-100 dark:hover:bg-[#334156]"
    onclick={() => showViewModal = true}
  >
    <Eye class="h-4 w-4 text-slate-600 dark:text-slate-400" />
  </button>

  <!-- EDITAR -->
  <button
    class="flex h-9 w-9 items-center justify-center rounded-xl transition hover:bg-slate-100 dark:hover:bg-[#334156]"
    onclick={() => showEditModal = true}
  >
    <Pencil class="h-4 w-4 text-slate-600 dark:text-slate-400" />
  </button>

  <!-- ELIMINAR -->
  <button
    class="flex h-9 w-9 items-center justify-center rounded-xl transition hover:bg-[#1c5476]/10 dark:hover:bg-[#0C4A6E]/20"
    onclick={() => showDeleteModal = true}
  >
    <Trash2 class="h-4 w-4 text-[#1c5476] dark:text-[#39BDF8]" />
  </button>

</div>

<!-- MODAL -->
<ViewProductModal
  open={showViewModal}
  {product}
  onClose={() => showViewModal = false}
/>

<EditProductModal
  open={showEditModal}
  {product}
  onClose={() => showEditModal = false}
  onUpdated={onUpdated}
/>

<DeleteProductModal
  open={showDeleteModal}
  {product}
  onClose={() => showDeleteModal = false}
  onDeleted={onUpdated}
/>