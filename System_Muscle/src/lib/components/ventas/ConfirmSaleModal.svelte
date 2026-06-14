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
  class="fixed top-0 left-0 w-screen h-screen z-9998 bg-black/50 backdrop-blur-md"
></div>

  <!-- MODAL -->
  <div
    class="fixed inset-0 z-9999 flex items-center justify-center p-4"
  >

  <div
    class="animate-in fade-in zoom-in duration-200 w-full max-w-md rounded-3xl border border-slate-200 bg-white shadow-2xl dark:bg-[#1E293B] dark:border-[#334156]"
  >

      <!-- HEADER -->
      <div class="sticky top-0 rounded-t-3xl border-b border-slate-100 bg-white dark:bg-[#1E293B] dark:border-[#334156] p-6">

        <h2
          class="text-2xl font-bold text-slate-800 dark:text-white"
        >
          Confirmar Venta
        </h2>

        <p
          class="mt-1 text-sm text-slate-500 dark:text-slate-400"
        >
          ¿Estas seguro de procesar esta venta?
        </p>

      </div>

      <!-- BODY -->
      <div class="space-y-5 p-5">

        <div class="space-y-3">

          {#each carrito as item}

            <div
              class="flex items-center justify-between rounded-xl   px-3 py-2 "
            >

             <span
                class="font-medium text-slate-900 dark:text-white"
              >
                {item.cantidad}x {item.nombre}
              </span>

              <span
                class="font-semibold text-[#0C4A6E] dark:text-[#39BDF8]"
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
        class="h-px bg-slate-200 dark:bg-[#334156]"
      ></div>

        <!-- TOTAL -->
        <div
          class="rounded-2xl bg-slate-50 p-4 dark:bg-[#111827]"
        >

          <span
            class="font-semibold text-slate-800 dark:text-white"
          >
            Total
          </span>

          <span
            class="text-3xl font-bold text-[#0C4A6E] dark:text-[#39BDF8]"
          >
            $
            {formatear(total)}
          </span>

        </div>

        <!-- METODO -->
        <p
          class="mt-3 text-sm text-slate-500 dark:text-slate-400"
        >
          Método de pago:
          {metodoPago}
        </p>

      </div>

      <!-- FOOTER -->
     <div
      class="sticky bottom-0 rounded-b-3xl border-t border-slate-100 bg-white dark:bg-[#1E293B] dark:border-[#334156] p-5"
    >
      <div class="flex justify-end gap-3">

        <button
          on:click={onClose}
          class="rounded-xl border border-slate-200 px-5 py-2.5 text-sm font-medium text-slate-700 dark:border-[#334156] dark:text-white transition hover:bg-slate-100 dark:hover:bg-[#162033]"
        >
          Cancelar
        </button>

        <button
          on:click={onConfirm}
          class="rounded-xl bg-[#0C4A6E] px-5 py-2.5 text-sm font-medium text-white dark:text-[#39BDF8] transition hover:bg-[#0a3a52]"
        >
          Confirmar Venta
        </button>

      </div>
      </div>  

    </div>

  </div>

{/if}