<script lang="ts">

  import Package from 'lucide-svelte/icons/package';

  import {
    addToCart
  } from '$lib/stores/sales/cart';

interface Producto {
  id_producto: number;
  nombre: string;
  categoria: string;
  precio: number;
  stock: number;
  total: number;
  imagen: string;
}

  export let producto: Producto;

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
  class="cursor-pointer overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-sm transition hover:-translate-y-1 hover:shadow-lg"
>



  <!-- IMAGEN -->
  <div
    class="relative aspect-square overflow-hidden bg-slate-100"
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
        class="rounded-full bg-white/90 px-3 py-1 text-xs font-medium text-slate-700 backdrop-blur"
      >
        {producto.categoria}
      </span>

    </div>

  </div>

  <!-- CONTENIDO -->
  <div class="space-y-3 p-4">

    <div>

      <h3 class="font-semibold text-slate-800">
        {producto.nombre}
      </h3>

      <p
        class="mt-1 text-xl font-bold text-[#0C4A6E]"
      >
        $
        {formatear(producto.precio)}
      </p>

    </div>

    <div>

      <div
        class="mb-2 flex items-center justify-between text-xs text-slate-500"
      >

        <span>
          Stock
        </span>

        <span>
          {producto.stock} / {producto.total}
        </span>

      </div>

      <div
        class="h-2 overflow-hidden rounded-full bg-slate-200"
      >

        <div
          class="h-full rounded-full bg-[#0C4A6E]"
          style={`width: ${(producto.stock / producto.total) * 100}%`}
        ></div>

      </div>

    </div>

  </div>

</div>