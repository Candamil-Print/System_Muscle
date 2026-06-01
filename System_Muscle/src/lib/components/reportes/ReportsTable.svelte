<script lang="ts">
  import { onMount } from 'svelte';
  import CreditCard from 'lucide-svelte/icons/credit-card';
  import Banknote from 'lucide-svelte/icons/banknote';
  import ShoppingCart from 'lucide-svelte/icons/shopping-cart';
  import Search from 'lucide-svelte/icons/search';
  import Download from 'lucide-svelte/icons/download';
  import Pagination from '../movimientos/Pagination.svelte';

  import {
    obtenerProductosMasVendidos,
    obtenerVentasPorUsuario,
    obtenerVentasPorMetodoPago
  } from '$lib/services/api/reports/reports.service';

  export let filtros = {
    fechaInicio: '',
    fechaFin: '',
    metodoPago: 'todos',
    vendedor: 'todos'
  };

  let sales: any[] = [];
  let loading = true;
  let error = '';
  let currentPage = 1;
  const itemsPerPage = 8;

  $: totalPages = Math.ceil(sales.length / itemsPerPage);

  $: paginatedSales = sales.slice(
    (currentPage - 1) * itemsPerPage,
    currentPage * itemsPerPage
  );

  function getMetodoEstilo(metodo: string) {
    if (metodo === 'Efectivo') {
      return {
        bg: 'bg-green-50',
        text: 'text-green-700',
        border: 'border-green-200',
        icon: Banknote
      };
    }

    if (metodo === 'Transferencia') {
      return {
        bg: 'bg-blue-50',
        text: 'text-blue-700',
        border: 'border-blue-200',
        icon: CreditCard
      };
    }

    return {
      bg: 'bg-slate-50',
      text: 'text-slate-700',
      border: 'border-slate-200',
      icon: null
    };
  }

  function descargarCSV() {
    if (sales.length === 0) {
      alert('No hay datos para descargar');
      return;
    }

    // Crear headers del CSV
    const headers = ['Producto', 'Precio', 'Método', 'Vendedor', 'Fecha y Hora'];
    
    // Crear filas del CSV
    const rows = sales.map(sale => [
      sale.producto,
      `$${sale.precio.toLocaleString('es-ES')}`,
      sale.metodo,
      sale.vendedor,
      sale.fecha
    ]);

    // Combinar headers y rows
    const csvContent = [
      headers.join(','),
      ...rows.map(row => row.map(cell => `"${cell}"`).join(','))
    ].join('\n');

    // Crear blob y descargar
    const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });
    const link = document.createElement('a');
    const url = URL.createObjectURL(blob);
    
    link.setAttribute('href', url);
    link.setAttribute('download', `reporte-ventas-${new Date().toISOString().split('T')[0]}.csv`);
    link.style.visibility = 'hidden';
    
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  }

  export async function actualizarTabla(
    fechaInicio: string,
    fechaFin: string,
    metodoPago = 'todos',
    vendedor = 'todos'
  ) {
    loading = true;
    error = '';
    currentPage = 1;

    try {
      const [productos, ventasPorUsuario, ventasPorMetodo] = await Promise.all([
        obtenerProductosMasVendidos(fechaInicio, fechaFin, 100),
        obtenerVentasPorUsuario(fechaInicio, fechaFin),
        obtenerVentasPorMetodoPago(fechaInicio, fechaFin)
      ]);

      let usuariosFiltrados = ventasPorUsuario;

      if (vendedor !== 'todos') {
        usuariosFiltrados = ventasPorUsuario.filter(
          (u) => String(u.id_usuario) === String(vendedor)
        );
      }

      let metodoSeleccionado =
        ventasPorMetodo[0]?.nombre_metodo || 'N/A';

      if (metodoPago !== 'todos') {
        metodoSeleccionado =
          metodoPago === 'efectivo'
            ? 'Efectivo'
            : 'Transferencia';
      }

      sales = productos.map((producto, index) => ({
        producto: producto.nombre_producto,

        precio: producto.total_ventas,

        metodo: metodoSeleccionado,

        vendedor:
          usuariosFiltrados[index]?.nombre_usuario ||
          usuariosFiltrados[0]?.nombre_usuario ||
          'N/A',

        fecha: new Date().toLocaleDateString('es-ES', {
          year: 'numeric',
          month: '2-digit',
          day: '2-digit',
          hour: '2-digit',
          minute: '2-digit',
          second: '2-digit'
        })
      }));

      loading = false;
    } catch (err) {
      console.error('Error cargando ventas:', err);
      error = 'Error al cargar el historial de ventas';
      loading = false;
    }
  }

  onMount(() => {
    const today = new Date();

    const sevenDaysAgo = new Date(
      today.getTime() - 7 * 24 * 60 * 60 * 1000
    );

    actualizarTabla(
      sevenDaysAgo.toISOString().split('T')[0],
      today.toISOString().split('T')[0]
    );
  });

  $: if (
    filtros?.fechaInicio &&
    filtros?.fechaFin
  ) {
    actualizarTabla(
      filtros.fechaInicio,
      filtros.fechaFin,
      filtros.metodoPago,
      filtros.vendedor
    );
  }
</script>

<div class="bg-white border border-slate-200 rounded-2xl overflow-hidden">

  <div class="flex items-center justify-between px-6 py-4 border-b border-slate-200">

    <div class="flex flex-col items-start gap-0">

      <div class="flex items-center gap-2">

        <ShoppingCart class="w-5 h-5 text-slate-700" />
        
        <h3 class="text-lg font-semibold text-slate-800">
          Historial de Ventas
        </h3>

      </div>

      <p class="text-sm text-slate-500">
        {sales.length} ventas encontradas
      </p>

    </div>

    <button
      on:click={descargarCSV}
      disabled={loading || sales.length === 0}
      class="h-11 w-11 rounded-xl border border-slate-200 flex items-center justify-center hover:bg-slate-50 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
      title="Descargar reporte"
    >
      <Download class="w-5 h-5 text-slate-700" />
    </button>

  </div>

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <p class="text-slate-500">Cargando historial...</p>
    </div>
  {:else if error}
    <div class="px-6 py-6">
      <div class="rounded-2xl border border-red-200 bg-red-50 p-5">
        <p class="text-red-600">{error}</p>
      </div>
    </div>
  {:else}
    <div class="overflow-x-auto px-6 py-6">

      <div class="border border-slate-200 rounded-xl overflow-hidden">

        <table class="w-full">

          <thead class="bg-[#26557c]">

            <tr class="text-left">

              <th class="px-6 py-4 text-sm font-medium text-[#ffffff]">
                Producto
              </th>

              <th class="px-6 py-4 text-sm font-medium text-[#ffffff]">
                Precio
              </th>

              <th class="px-6 py-4 text-sm font-medium text-[#ffffff]">
                Método
              </th>

              <th class="px-6 py-4 text-sm font-medium text-[#ffffff]">
                Vendedor
              </th>

              <th class="px-6 py-4 text-sm font-medium text-[#ffffff]">
                Fecha y Hora
              </th>

            </tr>

          </thead>

          <tbody class="divide-y divide-slate-200">

            {#if paginatedSales.length > 0}

              {#each paginatedSales as sale (sale.producto + sale.vendedor)}
                {@const metodoEstilo = getMetodoEstilo(sale.metodo)}

                <tr class="bg-white hover:bg-slate-50 transition-colors">

                  <td class="px-6 py-5">
                    <span class="font-medium text-slate-700">
                      {sale.producto}
                    </span>
                  </td>

                  <td class="px-6 py-5">
                    <span class="text-slate-700">
                      ${sale.precio.toLocaleString('es-ES')}
                    </span>
                  </td>

                  <td class="px-6 py-5">
                    <div class="inline-flex items-center gap-2 px-3 py-2 rounded-lg border {metodoEstilo.bg} {metodoEstilo.border}">
                      {#if metodoEstilo.icon}
                        <svelte:component this={metodoEstilo.icon} class="w-4 h-4 {metodoEstilo.text}" />
                      {/if}
                      <span class="text-sm font-medium {metodoEstilo.text}">
                        {sale.metodo}
                      </span>
                    </div>
                  </td>

                  <td class="px-6 py-5">
                    <span class="text-slate-700">
                      {sale.vendedor}
                    </span>
                  </td>

                  <td class="px-6 py-5">
                    <span class="text-slate-500 text-sm">
                      {sale.fecha}
                    </span>
                  </td>

                </tr>

              {/each}

            {:else}

              <tr>

                <td colspan="5" class="px-6 py-16 text-center">

                  <div class="flex flex-col items-center justify-center">

                    <Search class="mb-3 h-10 w-10 text-slate-300" />

                    <h3 class="text-base font-semibold text-slate-700">
                      No se encontraron resultados
                    </h3>

                    <p class="mt-1 text-sm text-slate-500">
                      Intenta con otras fechas
                    </p>

                  </div>

                </td>

              </tr>

            {/if}

          </tbody>

        </table>

      </div>

    </div>

    <!-- PAGINACIÓN -->
    <div class="px-6 pb-6">

      <Pagination
        {currentPage}
        {totalPages}
        onPageChange={(page) => {
          currentPage = page;
        }}
      />

    </div>
  {/if}

</div>