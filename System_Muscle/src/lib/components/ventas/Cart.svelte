<script lang="ts">
  import { ShoppingCart, Wallet, Landmark } from 'lucide-svelte';

  interface CarritoItem {
    nombre: string;
    precio: number;
    cantidad: number;
  }

  export let carrito: CarritoItem[] = [];

  let metodoPago: string = 'efectivo';

  $: total = carrito.reduce(
    (acc, item) => acc + item.precio * item.cantidad,
    0
  );

  const formatear = (valor: number): string =>
    new Intl.NumberFormat('es-CO').format(valor);
</script>

<aside class="space-y-6">
  <div
    class="sticky top-24 rounded-2xl border border-slate-200 bg-white shadow-sm"
  >
    <div class="border-b border-slate-200 p-5">
      <div class="flex items-center gap-2">
        <ShoppingCart size={20} class="text-[#0C4A6E]" />
        <h2 class="text-lg font-semibold text-slate-800">
          Carrito
        </h2>
      </div>

      <p class="mt-1 text-sm text-slate-500">
        {carrito.length} productos
      </p>
    </div>

    <div class="space-y-3 p-5">
      {#each carrito as item}
        <div class="rounded-xl bg-[#F3F4F6] p-4">
          <div class="flex items-center justify-between">
            <div>
              <h3 class="font-medium text-slate-800">
                {item.nombre}
              </h3>

              <p class="text-sm text-slate-500">
                $ {formatear(item.precio)}
              </p>
            </div>

            <div class="flex items-center gap-3">
              <button
                class="flex h-8 w-8 items-center justify-center rounded-lg border border-[#E5E7EB] bg-white hover:bg-slate-100"
              >
                −
              </button>

              <span class="w-6 text-center font-semibold">
                {item.cantidad}
              </span>

              <button
                class="flex h-8 w-8 items-center justify-center rounded-lg border border-[#E5E7EB] bg-white hover:bg-slate-100"
              >
                +
              </button>
            </div>
          </div>
        </div>
      {/each}
    </div>

    <div class="border-t border-slate-200 p-5">
      <div class="mb-5 flex items-center justify-between">
        <span class="text-slate-600">
          Total
        </span>

        <span class="text-3xl font-bold text-[#0C4A6E]">
          $ {formatear(total)}
        </span>
      </div>

      <div class="mb-5 space-y-3">
        <h3 class="text-sm font-semibold text-slate-800">
          Método de pago
        </h3>

        <div class="flex gap-3">
          <button
            on:click={() => (metodoPago = 'efectivo')}
            class={`flex-1 flex items-center justify-center gap-2 rounded-lg px-4 py-3 text-sm font-medium transition ${
              metodoPago === 'efectivo'
                ? 'bg-[#0C4A6E] text-white'
                : 'border border-slate-300 text-slate-700 hover:bg-slate-50'
            }`}
          >
            <Wallet size={18} />
            Efectivo
          </button>

          <button
            on:click={() => (metodoPago = 'transferencia')}
            class={`flex-1 flex items-center justify-center gap-2 rounded-lg px-4 py-3 text-sm font-medium transition ${
              metodoPago === 'transferencia'
                ? 'bg-[#0C4A6E] text-white'
                : 'border border-slate-300 text-slate-700 hover:bg-slate-50'
            }`}
          >
            <Landmark size={18} />
            Transferencia
          </button>
        </div>
      </div>

      <button
        class="w-full rounded-lg bg-[#0C4A6E] py-3.5 text-sm font-medium text-white transition hover:bg-[#0a3a52]"
      >
        Procesar Venta
      </button>
    </div>
  </div>
</aside>