<script>
  import InventoryActions from './InventoryActions.svelte';

  export let product;
</script>

<tr class="border-t border-slate-100 hover:bg-slate-50 transition">
  <td class="px-6 py-4">

    <div class="flex items-center gap-3">

      <img
        src={product.image}
        alt={product.name}
        class="w-11 h-11 rounded-full object-cover"
      />

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
    {product.cost}
  </td>

  <td class="px-6 py-4 text-sm  text-slate-600">
    {product.sale}
  </td>

  <td class="px-6 py-4 text-sm text-slate-600">
    {product.stock}
  </td>

  <td class="px-6 py-4">
    <InventoryActions />
  </td>
</tr>