<script lang="ts">
  import { Trash2, X } from 'lucide-svelte';
  import { eliminarProducto } from '$lib/services/api/inventory';

  export let open = false;

  export let product:
    | {
        id: number;
        name: string;
      }
    | undefined = undefined;

  export let onClose: () => void = () => {};
  export let onDeleted: () => void = () => {};

  let loading = false;

  async function handleDelete() {
    if (!product) return;

    try {
      loading = true;

      await eliminarProducto(product.id);

      onDeleted();
      onClose();

    } catch (error) {
      console.error(error);
      alert('Error al eliminar producto');

    } finally {
      loading = false;
    }
  }
</script>

{#if open && product}

  <!-- Overlay -->
  <button
    type="button"
    aria-label="Cerrar modal"
    class="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm"
    onclick={onClose}
  >
  </button>

  <!-- Modal -->
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4">

    <div class="w-full max-w-md rounded-2xl bg-white dark:bg-[#1E293B] p-6 shadow-2xl">

      <!-- Header -->
      <div class="mb-4 flex items-start justify-between">

        <div class="flex items-center gap-3">

          <div class="flex h-12 w-12 items-center justify-center rounded-xl bg-red-100 dark:bg-[#39BDF8]/20">
            <Trash2 size={22} class="text-red-600 dark:text-[#39BDF8]" />
          </div>

          <div>
            <h2 class="text-lg font-bold text-slate-800 dark:text-white">
              ¿Eliminar producto?
            </h2>

            <p class="mt-1 text-sm text-slate-500 dark:text-[#94A3B8]">
              Esta acción no se puede deshacer
            </p>
          </div>

        </div>

        <button
          type="button"
          aria-label="Cerrar"
          class="rounded-lg p-2 text-slate-400 dark:text-[#94A3B8] transition hover:bg-slate-100 dark:hover:bg-[#334156]"
          onclick={onClose}
        >
          <X size={18} />
        </button>

      </div>

      <!-- Body -->
      <p class="text-sm leading-relaxed text-slate-600 dark:text-[#94A3B8]">
        Esta acción no se puede deshacer. Se eliminará permanentemente el producto
        <span class="font-semibold text-slate-800 dark:text-white">
          {product.name}
        </span>.
      </p>

      <!-- Footer -->
      <div class="mt-6 flex justify-end gap-3">

        <button
          type="button"
          class="rounded-xl border border-slate-200 dark:border-[#334156] px-4 py-2 text-sm font-medium text-slate-700 dark:text-white transition hover:bg-slate-100 dark:hover:bg-[#334156]"
          onclick={onClose}
        >
          Cancelar
        </button>

        <button
          type="button"
          class="rounded-xl  dark:bg-[#39BDF8] dark:hover:bg-[#1FA9E8] dark:text-white px-4 py-2 text-sm font-medium text-white transition disabled:opacity-50"
          onclick={handleDelete}
          disabled={loading}
        >
          {#if loading}
            Eliminando...
          {:else}
            Eliminar
          {/if}
        </button>

      </div>

    </div>

  </div>

{/if}
