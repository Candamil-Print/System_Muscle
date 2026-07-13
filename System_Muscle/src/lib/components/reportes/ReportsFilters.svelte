<script lang="ts">
  import { onMount } from 'svelte';
  import Search from 'lucide-svelte/icons/search';
  import Download from 'lucide-svelte/icons/download';
  import { ChevronDown } from 'lucide-svelte';
  import CalendarDays from 'lucide-svelte/icons/calendar-days';
  import { toast } from 'svelte-sonner';
  import jsPDF from 'jspdf';
  import autoTable from 'jspdf-autotable';

  import { obtenerVentasPorUsuario, obtenerProductosMasVendidos, obtenerVentasPorMetodoPago } from '$lib/services/api/reports/reports.service';
  import type { VentasPorUsuario } from '$lib/services/api/reports/reports.types';

  import flatpickr from 'flatpickr';
  import { Spanish } from 'flatpickr/dist/l10n/es.js';

  let inicioInput: HTMLInputElement;
  let finInput: HTMLInputElement;

  let finPicker: flatpickr.Instance;

  export let onFilter: (filters: { fechaInicio: string; fechaFin: string; metodoPago: string; vendedor: string }) => void = () => {};
  export let loading = false;

  let fechaInicio = '';
  let fechaFin = '';
  let metodoPago = 'todos';
  let vendedor = 'todos';
  let vendedores: VentasPorUsuario[] = [];

  async function cargarVendedores() {
    try {
      const today = new Date();
      const treintaDiasAgo = new Date(today.getTime() - 30 * 24 * 60 * 60 * 1000);
      
      const inicio = treintaDiasAgo.toISOString().split('T')[0];
      const fin = today.toISOString().split('T')[0];

      vendedores = await obtenerVentasPorUsuario(inicio, fin);
    } catch (err) {
      console.error('Error cargando vendedores:', err);
    }
  }

 

  function handleFilter() {
    if (!fechaInicio || !fechaFin) {
      toast.warning('Debes seleccionar ambas fechas');
      return;
    }

    onFilter({
      fechaInicio,
      fechaFin,
      metodoPago,
      vendedor
    });
  }

async function handleDownload() {
  try {
    toast.info('Generando reporte...');

    const [
      productos,
      ventasPorUsuario,
      ventasPorMetodo
    ] = await Promise.all([
      obtenerProductosMasVendidos(
        fechaInicio,
        fechaFin,
        100
      ),
      obtenerVentasPorUsuario(
        fechaInicio,
        fechaFin
      ),
      obtenerVentasPorMetodoPago(
        fechaInicio,
        fechaFin
      )
    ]);

    if (productos.length === 0) {
      toast.warning(
        'No hay datos para generar el reporte'
      );
      return;
    }

    const doc = new jsPDF();

    doc.setFontSize(18);
    doc.text(
      'Reporte de Ventas',
      14,
      20
    );

    doc.setFontSize(10);

    doc.text(
      `Fecha inicio: ${fechaInicio}`,
      14,
      30
    );

    doc.text(
      `Fecha fin: ${fechaFin}`,
      14,
      36
    );

    doc.text(
      `Método de pago: ${
        metodoPago === 'todos'
          ? 'Todos'
          : metodoPago
      }`,
      14,
      42
    );

    doc.text(
      `Vendedor: ${
        vendedor === 'todos'
          ? 'Todos'
          : vendedores.find(
              v =>
                String(v.id_usuario) ===
                String(vendedor)
            )?.nombre_usuario ||
            vendedor
      }`,
      14,
      48
    );

    autoTable(doc, {
      startY: 60,

      head: [[
        'Producto',
        'Cantidad Vendida',
        'Total Ventas'
      ]],

      body: productos.map(
        producto => [
          producto.nombre_producto,
          producto.cantidad_vendida,
          `$${producto.total_ventas.toLocaleString(
            'es-CO'
          )}`
        ]
      )
    });

    doc.save(
      `reporte-ventas-${fechaInicio}-${fechaFin}.pdf`
    );

    toast.success(
      'Reporte descargado correctamente'
    );

  } catch (error) {
    console.error(error);

    toast.error(
      'Error al generar el reporte'
    );
  }
}

onMount(() => {
  cargarVendedores();

  flatpickr(inicioInput, {
    locale: Spanish,
    dateFormat: 'Y-m-d',
    allowInput: true,

    onChange: (selectedDates) => {
      if (selectedDates[0]) {
        fechaInicio = selectedDates[0]
          .toISOString()
          .split('T')[0];

        // Actualiza la fecha mínima permitida
        finPicker?.set('minDate', fechaInicio);

        // Si la fecha fin quedó menor, la corregimos
        if (
          fechaFin &&
          new Date(fechaFin) < new Date(fechaInicio)
        ) {
          fechaFin = fechaInicio;
          finPicker?.setDate(fechaInicio);
        }
      }
    }
  });

  finPicker = flatpickr(finInput, {
    locale: Spanish,
    dateFormat: 'Y-m-d',
    allowInput: true,
    minDate: fechaInicio,

    onChange: (selectedDates) => {
      if (selectedDates[0]) {
        fechaFin = selectedDates[0]
          .toISOString()
          .split('T')[0];
      }
    }
  });
});
</script>

<div class="bg-white border border-slate-200 rounded-2xl p-5 dark:bg-[#1E293B] dark:border-[#334156]">

  <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-5 gap-4 ">

  <div>
    <label class="text-xs font-semibold text-slate-500 dark:text-white uppercase">
      Fecha Inicio
    </label>

    <div class="relative mt-2">

      <CalendarDays
        class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400 dark:text-slate-500"
      />

      <input
        bind:this={inicioInput}
        placeholder="Seleccione fecha"
        class="w-full h-11 pl-10 pr-4 rounded-xl border border-slate-200 bg-white text-slate-700 dark:bg-[#1E293B] dark:border-[#334156] dark:text-[#E2E8F0]"
      />

    </div>
  </div>

  <div>
    <label class="text-xs font-semibold text-slate-500 uppercase dark:text-white">
      Fecha Fin
    </label>

    <div class="relative mt-2">

      <CalendarDays
        class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400 dark:text-slate-500"
      />

      <input
        bind:this={finInput}
        placeholder="Seleccione fecha"
        class="w-full h-11 pl-10 pr-4 rounded-xl border border-slate-200 bg-white text-slate-700 dark:bg-[#1E293B] dark:border-[#334156] dark:text-[#E2E8F0]"
      />

    </div>
  </div>

    <div class="relative">
      <label class="text-xs font-semibold text-slate-500 dark:text-white uppercase">
        Método Pago
      </label>

      <select
        bind:value={metodoPago}
        class="h-11 mt-2 w-full rounded-xl border border-slate-200 px-4 pr-10 text-sm outline-none focus:border-cyan-600 appearance-none bg-transparent dark:border-[#334156] dark:text-white"
      >
        <option value="todos">Todos</option>
        <option value="efectivo">Efectivo</option>
        <option value="transferencia">Transferencia</option>
      </select>

       <ChevronDown
        size={18}
        class="absolute right-3 top-11 pointer-events-none text-slate-400 dark:text-white"
      />
    </div>

    <div class="relative">
      <label class="text-xs font-semibold text-slate-500 dark:text-white uppercase">
        Vendedor
      </label>

      <select
        bind:value={vendedor}
        class="h-11 mt-2 w-full rounded-xl border border-slate-200 px-4 pr-10 text-sm outline-none focus:border-cyan-600 appearance-none bg-transparent dark:border-[#334156] dark:text-white"
      >
        <option value="todos">Todos</option>
        {#each vendedores as v (v.id_usuario)}
          <option value={v.id_usuario}>
            {v.nombre_usuario}
          </option>
        {/each}
      </select>

       <ChevronDown
        size={18}
        class="absolute  right-3 top-11  pointer-events-none text-slate-400 dark:text-white"
      />
    </div>

    <div class="flex items-end gap-3">

      <button
        on:click={handleFilter}
        disabled={loading}
        class="h-11 px-5 rounded-xl bg-[#0C4A6E] dark:text-[#39BDF8]   text-white flex items-center gap-2 disabled:opacity-50 cursor-pointer"
      >
        <Search class="w-4 h-4" />
        {loading ? 'Cargando...' : 'Filtrar'}
      </button>

      <button
        on:click={handleDownload}
        class="h-11 w-11 rounded-xl border border-slate-200 dark:border-[#334156] dark:text-[#39BDF8] flex items-center justify-center hover:bg-slate-50 cursor-pointer "
      >
        <Download class="w-4 h-4" />
      </button>

    </div>

  </div>

</div>