<script lang="ts">
  import X from 'lucide-svelte/icons/x';
  import Package from 'lucide-svelte/icons/package';
  import { toast } from 'svelte-sonner';

  import {
    crearMovement
  } from '$lib/services/api/movements';

  import { createEventDispatcher } from 'svelte';

  import SuccessModal from './SuccessModal.svelte';

  let successOpen = false;

  interface Product {
    id: number;
    name: string;
    type: string;
    stock: number;
    stockMax: number;
    image: string;
  }

  export let open = false;

  export let products: Product[] = [];

  export let onClose: () => void;
  export let onCreated: () => void;

  const dispatch = createEventDispatcher();

  let selectedProductId = '';

  // STRING PARA PODER FORMATEAR
  let quantity = '';

  let loading = false;

  // FORMATEAR NUMERO CON PUNTOS
  function formatNumber(value: string) {

    const numbers = value.replace(/\D/g, '');

    return numbers.replace(
      /\B(?=(\d{3})+(?!\d))/g,
      '.'
    );

  }

  // CONVERTIR A NUMERO REAL
  function parseNumber(value: string) {

    return Number(
      value.replace(/\./g, '')
    );

  }

  $: selectedProduct =
    products.find(
      (p) => p.id === Number(selectedProductId)
    );

  $: newStock =
    selectedProduct
      ? selectedProduct.stock + parseNumber(quantity || '0')
      : 0;

  async function handleSubmit() {

    // VALIDAR PRODUCTO
    if (!selectedProductId) {

      toast.error('Debes seleccionar un producto');

      return;
    }

    // VALIDAR VACIO
    if (!quantity || quantity.trim() === '') {

      toast.error('La cantidad es obligatoria');

      return;
    }

    const parsedQuantity = parseNumber(quantity);

    // VALIDAR MAYOR A 0
    if (parsedQuantity <= 0) {

      toast.error('La cantidad debe ser mayor a 0');

      return;
    }

    try {

      loading = true;

      await crearMovement({

        id_producto: Number(selectedProductId),

        cantidad: parsedQuantity,

        id_usuario: 1

      });

      await onCreated();

      // CERRAR MODAL PRINCIPAL
      handleClose();

      // ABRIR MODAL DE EXITO
      setTimeout(() => {

        successOpen = true;

      }, 100);

    } catch (error) {

      console.error(error);

      toast.error('Error registrando entrada');

    } finally {

      loading = false;

    }

  }

  function handleClose() {

    selectedProductId = '';

    quantity = '';

    onClose();

  }
</script>

{#if open}

  <!-- OVERLAY -->
  <div
    class="fixed left-0 top-0 z-[9998] h-dvh w-dvw bg-black/50 backdrop-blur-sm"
  ></div>

  <!-- MODAL CONTAINER -->
  <div
    class="fixed left-0 top-0 z-[9999] flex h-dvh w-dvw items-center justify-center p-4"
  >

    <!-- MODAL -->
    <div
      class="animate-in fade-in zoom-in duration-200 relative w-full max-w-md rounded-3xl border border-slate-200 bg-white p-8 shadow-2xl"
    >

      <!-- CLOSE -->
      <button
        on:click={handleClose}
        class="absolute right-6 top-6 rounded-xl p-2 text-slate-400 transition hover:bg-slate-100 hover:text-slate-700"
      >
        <X class="h-5 w-5" />
      </button>

      <!-- HEADER -->
      <div class="mb-8">

        <h2 class="text-2xl font-bold tracking-tight text-slate-900">
          Registrar Entrada
        </h2>

        <p class="mt-2 text-sm text-slate-500">
          Ingresa los datos del movimiento de entrada
        </p>

      </div>

      <!-- FORM -->
      <div class="space-y-6">

        <!-- PRODUCT -->
        <div>

          <label class="mb-1 block text-sm font-semibold text-slate-700">
            Producto
          </label>

          <div class="relative">

            <select
              bind:value={selectedProductId}
              class="w-full appearance-none rounded-xl border border-slate-200 bg-white px-4 py-3 pr-11 text-sm text-slate-700 outline-none transition duration-200 focus:border-[#0C4A6E] focus:ring-4 focus:ring-sky-100 focus:scale-[1.01]"
            >

              <option value="">
                Selecciona un producto
              </option>

              {#each products as product}

                <option value={product.id}>
                  {product.name}
                </option>

              {/each}

            </select>

            <!-- FLECHA -->
            <div
              class="pointer-events-none absolute inset-y-0 right-4 flex items-center"
            >

              <svg
                class="h-4 w-4 text-slate-500"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >

                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M19 9l-7 7-7-7"
                />

              </svg>

            </div>

          </div>

        </div>

        <!-- PRODUCT INFO -->
        {#if selectedProduct}

          <div
            class="flex items-center gap-4 rounded-xl border border-slate-200 bg-slate-50 p-4"
          >

            <!-- IMAGE -->
            <div
              class="flex h-16 w-16 flex-shrink-0 items-center justify-center overflow-hidden rounded-lg bg-slate-200"
            >

              {#if selectedProduct.image}

                <img
                  src={selectedProduct.image}
                  alt={selectedProduct.name}
                  class="h-full w-full object-cover"
                />

              {:else}

                <Package class="h-6 w-6 text-slate-500" />

              {/if}

            </div>

            <!-- INFO -->
            <div>

              <h3 class="text-base font-bold text-slate-900">
                {selectedProduct.name}
              </h3>

              <p class="mt-0.5 text-sm text-slate-500">
                Stock actual:
                {selectedProduct.stock}
                /
                {selectedProduct.stockMax}
              </p>

            </div>

          </div>

        {/if}

        <!-- QUANTITY -->
        <div>

          <label class="mb-1 block text-sm font-semibold text-slate-700">
            Cantidad a Ingresar
          </label>

          <div
            class="group flex overflow-hidden rounded-xl border border-slate-200 bg-white transition duration-200 focus-within:border-[#0C4A6E] focus-within:ring-4 focus-within:ring-sky-100 focus-within:scale-[1.01]"
          >

            <input
              value={quantity}
              on:input={(e) => {

                quantity = formatNumber(
                  e.currentTarget.value
                );

              }}
              type="text"
              inputmode="numeric"
              placeholder="0"
              class="w-full bg-transparent px-4 py-3 text-sm text-slate-700 outline-none"
            />

          </div>

          {#if selectedProduct}

            <p class="mt-2 text-sm text-slate-500">

              Nuevo stock:

              <span class="font-bold text-slate-900">
                {newStock.toLocaleString('es-CO')}
              </span>

            </p>

          {/if}

        </div>

      </div>

      <!-- FOOTER -->
      <div class="mt-8 flex justify-end gap-3">

        <!-- CANCEL -->
        <button
          on:click={handleClose}
          class="rounded-xl border border-slate-200 px-5 py-2.5 text-sm font-medium text-slate-700 transition hover:bg-slate-100"
        >
          Cancelar
        </button>

        <!-- CREATE -->
        <button
          on:click={handleSubmit}
          disabled={loading}
          class="rounded-xl bg-[#0C4A6E] px-5 py-2.5 text-sm font-medium text-white transition hover:bg-[#0a3a52] disabled:opacity-50"
        >

          {#if loading}
            Registrando...
          {:else}
            Registrar Entrada
          {/if}

        </button>

      </div>

    </div>

  </div>

{/if}

<!-- MODAL DE EXITO -->
<SuccessModal
  bind:open={successOpen}
  title="¡Entrada Exitosa!"
  description="La entrada fue registrada correctamente"
/>