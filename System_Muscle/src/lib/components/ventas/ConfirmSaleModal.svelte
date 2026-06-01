<script lang="ts">

  interface CarritoItem {
    nombre: string;
    precio: number;
    cantidad: number;
  }

  export let open = false;

  export let carrito: CarritoItem[] = [];

  export let total = 0;

  export let metodoPago = 'efectivo';

  export let onClose: () => void = () => {};

  export let onConfirm: () => void = () => {};

  const formatear = (valor: number): string =>
    new Intl.NumberFormat('es-CO').format(valor);

</script>

{#if open}

  <!-- OVERLAY -->
  <div
    class="fixed inset-0 z-[9998] bg-black/50 backdrop-blur-sm"
  ></div>

  <!-- MODAL -->
  <div
    class="fixed inset-0 z-[9999] flex items-center justify-center p-4"
  >

    <div
      class="w-full max-w-md rounded-2xl bg-white shadow-2xl"
    >

      <!-- HEADER -->
      <div class="px-6 pb-0 pt-6">

        <h2
          class="text-lg font-semibold text-slate-900"
        >
          Confirmar Venta
        </h2>

        <p
          class="mt-1 text-sm text-slate-500"
        >
          ¿Estas seguro de procesar esta venta?
        </p>

      </div>

      <!-- BODY -->
      <div class="px-6 pb-6 pt-4">

        <div class="space-y-3">

          {#each carrito as item}

            <div
              class="flex justify-between text-sm"
            >

              <span class="text-slate-700">
                {item.cantidad}x {item.nombre}
              </span>

              <span
                class="font-medium text-slate-900"
              >
                $
                {formatear(
                  item.precio * item.cantidad
                )}
              </span>

            </div>

          {/each}

        </div>

        <!-- DIVIDER -->
        <div
          class="my-5 h-px bg-slate-200"
        ></div>

        <!-- TOTAL -->
        <div
          class="flex items-center justify-between"
        >

          <span
            class="font-medium text-slate-900"
          >
            Total
          </span>

          <span
            class="text-2xl font-bold text-[#0C4A6E]"
          >
            $
            {formatear(total)}
          </span>

        </div>

        <!-- METODO -->
        <p
          class="mt-2 text-sm text-slate-500"
        >
          Método de pago:
          {metodoPago}
        </p>

      </div>

      <!-- FOOTER -->
      <div
        class="flex justify-end gap-3 px-6 pb-6"
      >

        <button
          on:click={onClose}
          class="rounded-lg border border-slate-200 px-4 py-2 text-sm font-medium text-slate-700 transition hover:bg-slate-50"
        >
          Cancelar
        </button>

        <button
          on:click={onConfirm}
          class="rounded-lg bg-[#0C4A6E] px-4 py-2 text-sm font-medium text-white transition hover:bg-[#0a3d5c]"
        >
          Confirmar Venta
        </button>

      </div>

    </div>

  </div>

{/if}

