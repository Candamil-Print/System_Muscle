<script lang="ts">
  interface Producto {
    nombre: string;
    categoria: string;
    precio: number;
    stock: number;
    total: number;
  }

  export let producto: Producto;

  const formatear = (valor: number): string =>
    new Intl.NumberFormat('es-CO').format(valor);
</script>

<div
  class="overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-sm transition hover:-translate-y-1 hover:shadow-lg"
>
  <div class="relative aspect-square overflow-hidden bg-slate-100">


    <div class="absolute left-3 top-3">
      <span
        class="rounded-full bg-white/90 px-3 py-1 text-xs font-medium text-slate-700 backdrop-blur"
      >
        {producto.categoria}
      </span>
    </div>
  </div>

  <div class="space-y-3 p-4">
    <div>
      <h3 class="font-semibold text-slate-800">
        {producto.nombre}
      </h3>

      <p class="mt-1 text-xl font-bold text-[#0C4A6E]">
        $ {formatear(producto.precio)}
      </p>
    </div>

    <div>
      <div class="mb-2 flex items-center justify-between text-xs text-slate-500">
        <span>Stock</span>

        <span>
          {producto.stock} / {producto.total}
        </span>
      </div>

      <div class="h-2 overflow-hidden rounded-full bg-slate-200">
        <div
          class="h-full rounded-full bg-[#0C4A6E]"
          style={`width: ${(producto.stock / producto.total) * 100}%`}
        ></div>
      </div>
    </div>

  </div>
</div>