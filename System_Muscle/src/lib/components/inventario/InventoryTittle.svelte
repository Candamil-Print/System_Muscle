<script>
  import { createEventDispatcher } from 'svelte';
  import { esAdministrador } from '$lib/utils/session';
  const dispatch = createEventDispatcher();
</script>

<div class="mb-6 flex items-center justify-between">
  <div>
    <h1 class="text-3xl font-bold text-slate-800 dark:text-white">
      Inventario
    </h1>

    <p class="mt-1 text-sm text-slate-500">
      Gestiona los productos del gimnasio
    </p>
  </div>

  {#if esAdministrador()}
    <button
      class="rounded-lg bg-[#0C4A6E] dark:text-[#39BDF8] px-4 py-2.5 text-sm font-medium text-white hover:bg-[#0a3a52]"
      onclick={() => dispatch('create')}
    >
      + Nuevo Producto
    </button>
  {/if}
</div>
