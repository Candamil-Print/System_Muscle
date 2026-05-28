<script lang="ts">
  interface Product {
    id: number;
    name: string;
    type: string;
    cost: string;
    sale: string;
    stock: string;
    status: string;
    image: string;
  }

  export let products: Product[] = [];

  const getStockPercentage = (stock: string) => {
    const [current, total] = stock.split('/').map(Number);
    return (current / total) * 100;
  };
</script>

<div class="bg-white border border-slate-200 rounded-2xl overflow-hidden">

  <div class="px-6 py-5 border-b border-slate-200">
    <h3 class="text-lg font-semibold text-slate-800">
      Lista de Productos
    </h3>

    <p class="text-sm text-slate-500 mt-1">
      {products.length} productos encontrados
    </p>
  </div>

  <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-4 gap-6 p-6">

    {#each products as product}
      <div class="border border-slate-200 rounded-2xl overflow-hidden bg-white">

        <!-- IMAGE -->
        <div class="h-52 bg-slate-100 flex items-center justify-center">
          <img
            src={product.image}
            alt={product.name}
            class="w-28 h-28 object-contain"
          />
        </div>

        <!-- CONTENT -->
        <div class="p-4">

          <p class="text-xs uppercase tracking-wide text-slate-400 mb-2">
            {product.type}
          </p>

          <h4 class="text-xl font-semibold text-slate-800">
            {product.name}
          </h4>

          <div class="flex items-center gap-2 mt-3">
            <span class="text-3xl font-bold text-[#0f4c81]">
              {product.sale}
            </span>

            <span class="text-sm line-through text-slate-400">
              {product.cost}
            </span>
          </div>

          <!-- STOCK -->
          <div class="mt-5">
            <div class="flex items-center justify-between text-sm text-slate-500 mb-2">
              <span>Stock</span>
              <span>{product.stock}</span>
            </div>

            <div class="w-full h-2 bg-slate-100 rounded-full overflow-hidden">
              <div
                class="h-full bg-[#0f4c81] rounded-full"
                style={`width:${getStockPercentage(product.stock)}%`}
              />
            </div>
          </div>

        </div>
      </div>
    {/each}

  </div>
</div>