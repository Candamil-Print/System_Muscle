<script lang="ts">

  import Package from 'lucide-svelte/icons/package';

  import {
    addToCart
  } from '$lib/stores/sales/cart';

  import type { ProductoVenta } from '$lib/services/api/sale/sale.types';

  export let producto: ProductoVenta;

  const formatear = (
    valor: number
  ): string =>

    new Intl.NumberFormat(
      'es-CO'
    ).format(valor);

  // CLICK
  function handleAddToCart() {

    console.log('CLICK FUNCIONA');

addToCart({
  id_producto: producto.id_producto,
  nombre: producto.nombre,
  precio: producto.precio,
  imagen: producto.imagen,
  cantidad: 1
});

  }

</script>



<div
  role="button"
  tabindex="0"
  on:click={handleAddToCart}
  on:keydown={(e) => {

    if (
      e.key === 'Enter' ||
      e.key === ' '
    ) {

      handleAddToCart();

    }

  }}
  class="cursor-pointer overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-sm transition hover:-translate-y-1 hover:shadow-lg dark:border-[#334156] dark:bg-[#1E293B]"
>



  <!-- IMAGEN -->
  <div
    class="relative aspect-square overflow-hidden bg-slate-100 dark:bg-[#0F172A]"
  >

    {#if producto.imagen}

      <img
        src={producto.imagen}
        alt={producto.nombre}
        class="h-full w-full object-cover"
      />

    {:else}

      <div
        class="flex h-full w-full items-center justify-center"
      >

        <Package
          class="h-16 w-16 text-slate-400"
        />

      </div>

    {/if}

    <div class="absolute left-3 top-3">

      <span
        class="rounded-full bg-white/90 px-3 py-1 text-xs font-medium text-slate-700 backdrop-blur dark:bg-[#334156]/90 dark:text-slate-300"
      >
        {producto.categoria}
      </span>

    </div>

  </div>

  <!-- CONTENIDO -->
  <div class="space-y-3 p-4">

    <div>

      <h3 class="font-semibold text-slate-800 dark:text-white">
        {producto.nombre}
      </h3>

      <p
        class="mt-1 text-xl font-bold text-[#0C4A6E] dark:text-[#39BDF8]"
      >
        $
        {formatear(producto.precio)}
      </p>

    </div>

    <div>

      <div
        class="mb-2 flex items-center justify-between text-xs text-slate-500 dark:text-slate-400"
      >

        <span>
          Stock
        </span>

        <span>
          {producto.stock} / {producto.stock_maximo}
        </span>

      </div>

      <div
        class="h-2 overflow-hidden rounded-full bg-slate-200 dark:bg-[#334156]"
      >

        <div
          class="h-full rounded-full bg-[#0C4A6E] dark:bg-[#39BDF8]"
          style={`width: ${(producto.stock / producto.stock_maximo) * 100}%`}
        ></div>

      </div>

    </div>

  </div>

</div>