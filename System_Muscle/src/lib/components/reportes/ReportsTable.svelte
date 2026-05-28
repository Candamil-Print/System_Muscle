<script>
  export let sales = [];
</script>

<div class="bg-white border border-slate-200 rounded-2xl overflow-hidden">

  <div class="px-6 py-5 border-b border-slate-200">

    <h3 class="text-lg font-semibold text-slate-800">
      Historial de Ventas
    </h3>

  </div>

  <div class="overflow-x-auto p-6">

    <table class="w-full border border-slate-200 rounded-xl overflow-hidden">

      <thead class="bg-[#26557c]">

        <tr>

          <th class="px-6 py-4 text-left text-[#dee6eb]">
            Producto
          </th>

          <th class="px-6 py-4 text-left text-[#dee6eb]">
            Precio
          </th>

          <th class="px-6 py-4 text-left text-[#dee6eb]">
            Método
          </th>

          <th class="px-6 py-4 text-left text-[#dee6eb]">
            Vendedor
          </th>

          <th class="px-6 py-4 text-left text-[#dee6eb]">
            Fecha
          </th>

        </tr>

      </thead>

      <tbody class="divide-y divide-slate-200">

        {#each sales as sale}

          <tr class="hover:bg-slate-50">

            <td class="px-6 py-5 font-medium text-slate-700">
              {sale.product}
            </td>

            <td class="px-6 py-5">
              {sale.price}
            </td>

            <td class="px-6 py-5">
              {sale.payment}
            </td>

            <td class="px-6 py-5">
              {sale.seller}
            </td>

            <td class="px-6 py-5 text-slate-500">
              {sale.date}
            </td>

          </tr>

        {/each}

      </tbody>

    </table>

  </div>

</div>