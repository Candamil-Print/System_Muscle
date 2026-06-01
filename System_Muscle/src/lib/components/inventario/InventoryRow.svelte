<script lang="ts">
  import EditProductModal from './EditProductModal.svelte';
  import InventoryActions from './InventoryActions.svelte';
  import Package from 'lucide-svelte/icons/package';

  interface Product {
    id: number;
    name: string;
    type: string;
    cost: number;
    sale: number;
    stock: number;
    status: string;
    image: string;
  }

  let showEditModal = false;

  export let product: Product;
  export let onUpdated;

  const formatCurrency = (value: number) => {
    return new Intl.NumberFormat('es-CO', {
      minimumFractionDigits: 0,
      maximumFractionDigits: 0
    }).format(value);
  };
</script>

<tr class="border-t border-slate-100 hover:bg-slate-50 transition">
  <td class="px-6 py-4">

    <div class="flex items-center gap-3">

      {#if product.image && product.image.trim() !== ''}

        <img
          src={product.image}
          alt={product.name}
          class="w-11 h-11 rounded-xl object-cover"
        />

      {:else}

        <div
          class="flex h-11 w-11 items-center justify-center rounded-xl bg-slate-100"
        >
          <Package class="h-5 w-5 text-slate-400" />
        </div>

      {/if}

      <span class="font-medium text-slate-700">
        {product.name}
      </span>

    </div>

  </td>

  <td class="px-6 py-4">
    <span class="px-3 py-1 rounded-full bg-[#1c5476]/10 text-[#1c5476] text-xs font-medium">
      {product.type}
    </span>
  </td>

  <td class="px-6 py-4 text-sm text-slate-600">
    ${formatCurrency(product.cost)}
  </td>

  <td class="px-6 py-4 text-sm  text-slate-600">
    ${formatCurrency(product.sale)}
  </td>

  <td class="px-6 py-4 text-sm text-slate-600">
    {product.stock}
  </td>

  <td class="px-6 py-4">
   <InventoryActions
  {product}
  {onUpdated}
/>
  </td>
</tr>