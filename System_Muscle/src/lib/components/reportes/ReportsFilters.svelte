<script lang="ts">
  import { onMount } from 'svelte';
  import Search from 'lucide-svelte/icons/search';
  import Download from 'lucide-svelte/icons/download';
  import { ChevronDown } from 'lucide-svelte';
  import CalendarDays from 'lucide-svelte/icons/calendar-days';
  import { toast } from 'svelte-sonner';
  import jsPDF from 'jspdf';
  import autoTable from 'jspdf-autotable';

  import { obtenerVentasPorUsuario, obtenerReporteVentasDetallado } from '$lib/services/api/reports/reports.service';
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

      console.log("Vendedores:", vendedores);
      console.table(vendedores);
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
    if (!fechaInicio || !fechaFin) {
      toast.warning('Debes seleccionar ambas fechas');
      return;
    }

    const reporte = await obtenerReporteVentasDetallado(
      fechaInicio,
      fechaFin
    );

    // Aplicar filtros seleccionados
    let datos = [...reporte];

    if (metodoPago !== 'todos') {
      datos = datos.filter(
        item =>
          item.metodo_pago.toLowerCase() ===
          metodoPago.toLowerCase()
      );
    }

    if (vendedor !== 'todos') {
      const nombreVendedor = vendedores.find(
        v => String(v.id_usuario) === String(vendedor)
      )?.nombre_usuario;

      if (nombreVendedor) {
        datos = datos.filter(
          item => item.vendedor === nombreVendedor
        );
      }
    }

    if (datos.length === 0) {
      toast.warning(
        'No hay datos para generar el reporte.'
      );
      return;
    }

    const doc = new jsPDF({
      orientation: 'landscape'
    });

    const ahora = new Date();

    const fechaGeneracion = {
      fecha: ahora.toLocaleDateString("es-CO"),
      hora: ahora.toLocaleTimeString("es-CO", {
        hour: "2-digit",
        minute: "2-digit"
      })
    };

    // ==========================
    // HEADER
    // ==========================

    // Fondo azul
    doc.setFillColor(12, 74, 110);
    doc.rect(0, 0, 297, 42, "F");

    // Línea inferior
    doc.setDrawColor(200, 200, 200);
    doc.setLineWidth(0.6);
    doc.line(0, 42, 297, 42);

    // Título
    doc.setTextColor(255, 255, 255);
    doc.setFont("helvetica", "bold");
    doc.setFontSize(24);

    doc.text(
      "REPORTE DETALLADO DE VENTAS",
      148.5,
      16,
      { align: "center" }
    );

    // Subtítulo
    doc.setFont("helvetica", "normal");
    doc.setFontSize(10);

    doc.text(
      "Sistema de Gestión de Inventario",
      148.5,
      24,
      { align: "center" }
    );

    // Fecha generación
    doc.text(
      `${fechaGeneracion.fecha} • ${fechaGeneracion.hora}`,
      148.5,
      31,
      { align: "center" }
    );

    // ==========================
    // FILTROS APLICADOS
    // ==========================

    // Título
    doc.setTextColor(12,74,110);
    doc.setFont("helvetica","bold");
    doc.setFontSize(15);

    doc.text(
      "Filtros aplicados",
      14,
      54
    );

    // Línea decorativa
    doc.setDrawColor(12,74,110);
    doc.setLineWidth(0.4);
    doc.line(14,56,283,56);

    // Sombra
    doc.setFillColor(228,232,236);
    doc.roundedRect(
      15,
      61,
      268,
      24,
      3,
      3,
      "F"
    );

    // Tarjeta
    doc.setFillColor(250,250,250);
    doc.roundedRect(
      14,
      60,
      268,
      24,
      3,
      3,
      "F"
    );

    // Barra azul superior
    doc.setFillColor(12,74,110);
    doc.roundedRect(
      14,
      60,
      268,
      3,
      3,
      3,
      "F"
    );

    // Texto
    doc.setTextColor(70,70,70);
    doc.setFont("helvetica","normal");
    doc.setFontSize(10);

    doc.text(`Fecha inicio: ${fechaInicio}`,20,71);

    doc.text(`Fecha fin: ${fechaFin}`,20,78);

    doc.text(
      `Método de pago: ${
        metodoPago === "todos"
          ? "Todos"
          : metodoPago
      }`,
      120,
      71
    );

    doc.text(
      `Vendedor: ${
        vendedor === "todos"
          ? "Todos"
          : vendedores.find(
              v => String(v.id_usuario) === String(vendedor)
            )?.nombre_usuario ?? vendedor
      }`,
      120,
      78
    );

    autoTable(doc, {
      startY: 92,

      styles: {
        fontSize: 7,
        cellPadding: 2
      },

      headStyles: {
        fillColor: [12, 74, 110]
      },

      head: [[
        'Venta',
        'Fecha',
        'Vendedor',
        'Producto',
        'Cant.',
        'P. Unit.',
        'Subtotal',
        'Pago',
        'Caja',
        'Apertura',
        'Hora Apertura',
        'Cierre',
        'Hora Cierre',
        'Efectivo',
        'Transferencia',
        'Total',
        'Caja Total'
      ]],

      body: datos.map(item => [
        item.id_venta,
        item.fecha,
        item.vendedor,
        item.producto,
        item.cantidad,
        `$${item.precio_unitario.toLocaleString('es-CO')}`,
        `$${item.subtotal.toLocaleString('es-CO')}`,
        item.metodo_pago,
        item.id_caja,
        `$${item.caja_inicial_valor.toLocaleString('es-CO')}`,
        item.caja_inicial_hora,
        item.caja_final_valor == null
          ? '-'
          : `$${item.caja_final_valor.toLocaleString('es-CO')}`,
        item.caja_final_hora ?? '-',
        `$${item.total_efectivo.toLocaleString('es-CO')}`,
        `$${item.total_transferencia.toLocaleString('es-CO')}`,
        `$${item.total_final.toLocaleString('es-CO')}`,
        item.caja_total == null
          ? '-'
          : `$${item.caja_total.toLocaleString('es-CO')}`
      ]),

      headStyles:{
        fillColor:[12,74,110],
        textColor:[255,255,255],
        fontStyle:"bold",
        fontSize:9,
        halign:"center"
      },

      alternateRowStyles:{
        fillColor:[248,249,250]
      },

      styles:{
        font:"helvetica",
        fontSize:7,
        cellPadding:2.5,
        lineColor:[225,230,235],
        lineWidth:.2,
        valign:"middle"
      }
    });

    // ==========================
    // FOOTER
    // ==========================

    const pages = doc.getNumberOfPages();

    for (let i = 1; i <= pages; i++) {

      doc.setPage(i);

      // Línea superior
      doc.setDrawColor(220, 220, 220);
      doc.setLineWidth(0.3);
      doc.line(14, 198, 283, 198);

      doc.setFont("helvetica", "normal");
      doc.setFontSize(8);
      doc.setTextColor(120, 120, 120);

      // Izquierda
      doc.text(
        "Sistema de Gestión de Inventario",
        14,
        204
      );

      // Centro
      doc.text(
        `${fechaGeneracion.fecha} • ${fechaGeneracion.hora}`,
        148.5,
        204,
        {
          align: "center"
        }
      );

      // Derecha
      doc.text(
        `Página ${i} de ${pages}`,
        283,
        204,
        {
          align: "right"
        }
      );
    }

    doc.save(
      `reporte-detallado-${fechaInicio}-${fechaFin}.pdf`
    );

    toast.success('Reporte descargado correctamente.');

  } catch (err) {
    console.error(err);
    toast.error('Error al generar el reporte.');
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