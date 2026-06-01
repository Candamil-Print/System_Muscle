<script lang="ts">
  import { onMount } from 'svelte';
  import { Package, X } from 'lucide-svelte';

  export let open = false;

  export let product:
    | {
        id: number;
        name: string;
        type: string;
        cost: number;
        sale: number;
        stock: number;
        stockMax?: number;
        image: string;
      }
    | undefined = undefined;

  export let onClose: () => void = () => {};

  let dialogElement: HTMLDivElement;

  const formatCurrency = (value: number) => {
    return new Intl.NumberFormat('es-CO', {
      minimumFractionDigits: 0,
      maximumFractionDigits: 0
    }).format(value);
  };

  // margen
  $: ganancia =
    product
      ? product.sale - product.cost
      : 0;

  $: porcentaje =
    product && product.cost > 0
      ? ((ganancia / product.cost) * 100).toFixed(1)
      : '0';

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      onClose();
    }
  }

  onMount(() => {
    window.addEventListener('keydown', handleKeydown);

    if (open) {
      document.body.style.overflow = 'hidden';

      setTimeout(() => {
        dialogElement?.focus();
      }, 50);
    }

    return () => {
      window.removeEventListener('keydown', handleKeydown);
      document.body.style.overflow = '';
    };
  });

  $: {
    if (open) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = '';
    }
  }
</script>

{#if open}

  <!-- Overlay -->
  <button
    type="button"
    aria-label="Cerrar modal"
    class="fixed inset-0 z-999 bg-black/50 backdrop-blur-sm"
    style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; width: 100vw; height: 100vh;"
    onclick={onClose}
  ></button>

  <!-- Container -->
  <div
    class="fixed inset-0 z-1000 flex items-center justify-center p-4"
    style="position: fixed; top: 0; left: 0; right: 0; bottom: 0;"
    role="button"
    tabindex="0"
    aria-label="Cerrar modal"
    onclick={onClose}
    onkeydown={(e) => {
      if (
        e.key === 'Escape' ||
        e.key === 'Enter' ||
        e.key === ' '
      ) {
        onClose();
      }
    }}
  >

    <!-- Modal -->
    <div
      bind:this={dialogElement}
      class="w-full max-w-md rounded-2xl bg-white shadow-2xl animate-in fade-in zoom-in duration-200"
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >

      <!-- HEADER -->
      <div class="border-b border-slate-100 px-5 py-4">

        <div class="flex items-start justify-between">

          <h2 class="text-lg font-bold text-slate-800">
            Detalle del Producto
          </h2>

          <button
            class="rounded-lg p-2 text-slate-400 transition hover:bg-slate-100"
            onclick={onClose}
          >
            <X size={18} />
          </button>

        </div>
      </div>

      <!-- BODY -->
      <div class="space-y-4 p-5">

        <!-- IMAGE -->
        <div class="flex justify-center">

          <div class="flex h-24 w-24 items-center justify-center overflow-hidden rounded-2xl bg-slate-100">

            {#if product?.image}
              <img
                src={product.image}
                alt={product.name}
                class="h-full w-full object-cover"
              />
            {:else}
              <Package size={36} class="text-slate-400" />
            {/if}

          </div>

        </div>

        <!-- INFO -->
        <div class="text-center">

          <h3 class="text-xl font-bold text-slate-800">
            {product?.name}
          </h3>

          <span class="mt-2 inline-block rounded-full bg-[#1c5476]/10 text-[#1c5476] px-3 py-1 text-xs font-medium ">
            {product?.type}
          </span>

        </div>

        <!-- STATS -->
        <div class="grid grid-cols-2 gap-3 rounded-2xl border border-slate-200 p-4">

          <!-- costo -->
          <div>
            <p class="text-xs text-slate-500">
              Precio Costo
            </p>

            <p class="mt-1 text-sm font-semibold text-slate-800">
              ${formatCurrency(product?.cost ?? 0)}
            </p>
          </div>

          <!-- venta -->
          <div>
            <p class="text-xs text-slate-500">
              Precio Venta
            </p>

            <p class="mt-1 text-sm font-bold text-[#1c5476]">
              ${formatCurrency(product?.sale ?? 0)}
            </p>
          </div>

          <!-- stock -->
          <div>
            <p class="text-xs text-slate-500">
              Stock Actual
            </p>

            <p class="mt-1 text-sm font-semibold text-slate-800">
              {product?.stock ?? 0}
            </p>
          </div>

          <!-- stock max -->
          <div>
            <p class="text-xs text-slate-500">
              Stock Máximo
            </p>

            <p class="mt-1 text-sm font-semibold text-slate-800">
              {product?.stockMax ?? 0}
            </p>
          </div>

        </div>

        <!-- MARGEN -->
        <div class="rounded-2xl border border-slate-200 p-4">

          <p class="text-xs text-slate-500">
            Margen de Ganancia
          </p>

          <p class="mt-2 text-2xl font-bold text-[#1c5476]">
            ${formatCurrency(ganancia)}
          </p>

          <p class="mt-1 text-xs text-slate-500">
            {porcentaje}% sobre el costo
          </p>

        </div>

      </div>

      <!-- FOOTER -->
      <div class="flex justify-end border-t border-slate-100 px-5 py-4">

        <button
          class="rounded-xl border border-slate-200 px-5 py-2.5 text-sm font-medium text-slate-700 transition hover:bg-slate-100"
          onclick={onClose}
        >
          Cerrar
        </button>

      </div>

    </div>
  </div>

{/if}