<script>
  import Plus from 'lucide-svelte/icons/plus';

  export let movement;
</script>

<tr class="border-t border-slate-100 hover:bg-slate-50 transition">

  <td class=" py-4">
    <span class="px-6 py-4 text-sm text-slate-600">
      {movement.product}
    </span>
  </td>

  <td class="px-6 py-4">
    <span class="flex items-center gap-1 px-3 py-1 rounded-full bg-[#1c5476]/10 text-[#1c5476] text-xs font-medium w-fit">
      <Plus class="w-3 h-3" />
      {movement.quantity}
    </span>
  </td>

  <td class="px-6 py-4 text-sm  text-slate-600">
    {movement.previousStock}
  </td>

  <td class="px-6 py-4 text-sm text-slate-600">
    {movement.newStock}
  </td>

  <td class="px-6 py-4 text-sm text-slate-600">
    {movement.receivedBy}
  </td>

  <td class="px-6 py-4 text-sm text-slate-600">
    {movement.date}
  </td>

</tr>