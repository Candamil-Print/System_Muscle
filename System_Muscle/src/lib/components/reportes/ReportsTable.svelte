<script lang="ts">
  import { onMount } from 'svelte';
  import { toast } from 'svelte-sonner';
  import CreditCard from 'lucide-svelte/icons/credit-card';
  import Banknote from 'lucide-svelte/icons/banknote';
  import ShoppingCart from 'lucide-svelte/icons/shopping-cart';
  import Search from 'lucide-svelte/icons/search';
  import Download from 'lucide-svelte/icons/download';
  import Pagination from '../movimientos/Pagination.svelte';

  import jsPDF from 'jspdf';
  import autoTable from 'jspdf-autotable';

  import { listarVentas, listarDetalleVenta } from '$lib/services/api/sale/sale.service';

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

  function formatCurrency(valor: number | string) {
  return Number(valor).toLocaleString('es-CO');
}

  function formatearFechaHora(fecha = new Date()) {
    const fechaTexto = fecha.toLocaleDateString(
      'es-CO',
      {
        day: 'numeric',
        month: 'long',
        year: 'numeric'
      }
    );

    const horaTexto = fecha.toLocaleTimeString(
      'es-CO',
      {
        hour: 'numeric',
        minute: '2-digit',
        hour12: true
      }
    );

    return {
      fecha:
        fechaTexto.charAt(0).toUpperCase() +
        fechaTexto.slice(1),
      hora: horaTexto
    };
  }
  

  function descargarPDF() {
    if (sales.length === 0) {
      alert('No hay datos para descargar');
      return;
    }

    const doc = new jsPDF('p', 'mm', 'a4');

    const fechaGeneracion = formatearFechaHora();

    // ==========================
    // CALCULOS
    // ==========================

    const totalVentas = sales.reduce(
      (acc, sale) =>
        acc + Number(sale.precio || 0),
      0
    );

    const promedioVenta =
      sales.length > 0
        ? totalVentas / sales.length
        : 0;

    const ventaMasAlta =
      sales.length > 0
        ? [...sales].sort(
            (a, b) =>
              Number(b.precio || 0) -
              Number(a.precio || 0)
          )[0]
        : null;

    // ==========================
    // HEADER
    // ==========================

    doc.setFillColor(12, 74, 110);
      doc.rect(0, 0, 210, 42, "F");

      doc.setDrawColor(200, 200, 200);  // Linea Gris clara
      doc.setLineWidth(0.6);
      doc.line(0, 42, 210, 42);

      // Titulo

      doc.setTextColor(255,255,255);
      doc.setFont("helvetica","bold");
      doc.setFontSize(24);

      doc.text(
        "REPORTE DE VENTAS",
        105,
        16,
        { align:"center" }
      );

      doc.setFont("helvetica","normal");
      doc.setFontSize(10);

      doc.text(
        "Sistema de Gestión de Inventario",
        105,
        24,
        { align:"center" }
      );

      doc.text(
        `${fechaGeneracion.fecha} • ${fechaGeneracion.hora}`,
        105,
        31,
        { align:"center" }
      );

    // ==========================
    // RESUMEN EJECUTIVO
    // ==========================

    doc.setTextColor(12, 74, 110);
      doc.setFont('helvetica', 'bold');
      doc.setFontSize(16);

      doc.text(
          'Resumen Ejecutivo',
          14,
          50
      );

      // Línea decorativa
      doc.setDrawColor(12, 74, 110);
      doc.setLineWidth(0.4);
      doc.line(14, 52, 196, 52);

    const cards = [
      [
        'Registros',
        sales.length.toString()
      ],

      [
        'Total Vendido',
        `$${totalVentas.toLocaleString(
          'es-CO'
        )}`
      ],

      [
        'Promedio Venta',
        `$${Math.round(
          promedioVenta
        ).toLocaleString('es-CO')}`
      ]
    ];

    let x = 14;

    cards.forEach(([titulo, valor]) => {
      // ---------- Sombra ----------
      doc.setFillColor(228, 232, 236);

      doc.roundedRect(
        x + 1,
        59,
        56,
        26,
        3,
        3,
        'F'
      );

      // ---------- Tarjeta ----------
      doc.setFillColor(250, 250, 250);

      doc.roundedRect(
        x,
        58,
        56,
        26,
        3,
        3,
        'F'
      );

      // ---------- Barra superior ----------
      doc.setFillColor(12, 74, 110);

      doc.roundedRect(
        x,
        58,
        56,
        3,
        3,
        3,
        'F'
      );

      // ---------- Título ----------
      doc.setTextColor(130, 130, 130);

      doc.setFont('helvetica', 'normal');
      doc.setFontSize(9);

      doc.text(
        titulo,
        x + 4,
        68
      );

      // ---------- Valor ----------
      doc.setTextColor(12, 74, 110);

      doc.setFont('helvetica', 'bold');
      doc.setFontSize(18);

      doc.text(
        valor,
        x + 4,
        79
      );

      x += 62;
    });

    // ==========================
    // INDICADOR PRINCIPAL
    // ==========================

    doc.setTextColor(12,74,110);
    doc.setFont('helvetica','bold');
    doc.setFontSize(15);

    doc.text(
        'Indicador Principal',
        14,
        98
    );

    doc.setDrawColor(12,74,110);
    doc.setLineWidth(.4);
    doc.line(14,100,196,100);

    // Sombra
    doc.setFillColor(228,232,236);
    doc.roundedRect(
        15,
        105,
        182,
        22,
        3,
        3,
        'F'
    );

    // Tarjeta
    doc.setFillColor(250,250,250);
    doc.roundedRect(
        14,
        104,
        182,
        22,
        3,
        3,
        'F'
    );

    // Barra azul
    doc.setFillColor(12,74,110);
    doc.roundedRect(
        14,
        104,
        182,
        2.5,
        3,
        3,
        'F'
    );

    if (ventaMasAlta) {

        doc.setTextColor(120,120,120);
        doc.setFont('helvetica','normal');
        doc.setFontSize(8);

        doc.text(
            'Venta con mayor valor registrada',
            18,
            112
        );

        doc.setTextColor(35,35,35);
        doc.setFont('helvetica','bold');
        doc.setFontSize(12);

        doc.text(
            String(ventaMasAlta.producto).slice(0,40),
            18,
            120
        );

        doc.setTextColor(12,74,110);
        doc.setFont('helvetica','bold');
        doc.setFontSize(18);

        doc.text(
            `$${Number(ventaMasAlta.precio).toLocaleString('es-CO')}`,
            190,
            119,
            {
                align:'right'
            }
        );

        doc.setTextColor(120,120,120);
        doc.setFont('helvetica','normal');
        doc.setFontSize(8);

        doc.text(
            'Valor de la venta',
            190,
            124,
            {
                align:'right'
            }
        );

    }
    // ==========================
    // TABLA
    // ==========================

    autoTable(doc, {
      startY: 130,

      head: [[
        'Producto',
        'Precio',
        'Método',
        'Vendedor',
        'Fecha'
      ]],

    body: sales.map((sale) => {
      const fecha = formatearFechaHora(
        new Date(sale.fecha.replace(' ', 'T'))
      );

      return [
        sale.producto,
        `$${Number(
          sale.precio || 0
        ).toLocaleString('es-CO')}`,
        sale.metodo,
        sale.vendedor,
        `${fecha.fecha}\n${fecha.hora}`
      ];
    }),

      headStyles: {
        fillColor: [12, 74, 110],
        textColor: [255, 255, 255],
        fontStyle: 'bold',
        fontSize: 10,
        halign: 'center'
      },

      styles: {
        font: 'helvetica',
        fontSize: 9,
        cellPadding: 4,
        lineColor: [225, 230, 235],
        lineWidth: 0.2,
        valign: 'middle'
      },

    });

    // ==========================
    // FOOTER
    // ==========================

    const pages = doc.getNumberOfPages();

    for (let i = 1; i <= pages; i++) {
      doc.setPage(i);

      // Línea superior del footer
      doc.setDrawColor(220, 220, 220);
      doc.setLineWidth(0.3);
      doc.line(14, 285, 196, 285);

      // Texto
      doc.setFont('helvetica', 'normal');
      doc.setFontSize(8);
      doc.setTextColor(120, 120, 120);

      // Lado izquierdo
      doc.text(
          'Sistema de Gestión de Inventario',
          14,
          290
      );

      // Centro
      doc.text(
          `${fechaGeneracion.fecha} • ${fechaGeneracion.hora}`,
          105,
          290,
          {
              align: 'center'
          }
      );

      // Lado derecho
      doc.text(
          `Página ${i} de ${pages}`,
          196,
          290,
          {
              align: 'right'
          }
      );
    }

    // ==========================
    // GUARDAR
    // ==========================

    doc.save(
      `reporte-ventas-${new Date()
        .toISOString()
        .split('T')[0]}.pdf`
    );
  }


  // ==========================
	//  Actualiza la información de la tabla aplicando los filtros seleccionados
	// ==========================
  export async function actualizarTabla(
    fechaInicio: string,
    fechaFin: string,
    metodoPago = 'todos',
    vendedor = 'todos',
    mostrarToast = false
  ) {
    loading = true;
    error = '';
    currentPage = 1;

    try {
      const ventas = await listarVentas();

      const resultado: any[] = [];

      for (const venta of ventas) {

        const fechaVenta = venta.fecha.split(' ')[0];

        if (
          fechaVenta < fechaInicio ||
          fechaVenta > fechaFin
        ) {
          continue;
        }

        if (
          vendedor !== 'todos' &&
          String(venta.id_usuario) !== String(vendedor)
        ) {
          continue;
        }

        const detalles = await listarDetalleVenta(
          venta.id_venta
        );

        for (const detalle of detalles) {

          if (
            metodoPago !== 'todos' &&
            detalle.nombre_metodo_pago.toLowerCase() !== metodoPago
          ) {
            continue;
          }

          resultado.push({
            id: `${venta.id_venta}-${detalle.id_detalle}`,
            producto: detalle.nombre_producto,
            precio: detalle.subtotal,
            metodo: detalle.nombre_metodo_pago,
            vendedor: venta.nombre_usuario,
            fecha: venta.fecha
          });

        }

      }

      sales = resultado;

      if (mostrarToast && sales.length === 0) {
        toast.warning(
          'No se encontraron resultados',
          {
            id: 'sin-resultados-tabla',
            description:
              'No existen ventas para los filtros seleccionados.'
          }
        );
      }

    } catch (err) {
      console.error(err);

      toast.error(
        'Error al cargar el historial de ventas'
      );

      error =
        'Error al cargar el historial de ventas';
    } finally {
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
      filtros.vendedor,
      true
    );
  }
</script>

<div class="bg-white border border-slate-200 rounded-2xl overflow-hidden dark:bg-[#1E293B] dark:border-[#334156]">

  <div class="flex items-start justify-between px-6 py-4 border-b border-slate-200 dark:border-[#334156]">

    <div class="flex flex-col items-start gap-0">

      <div class="flex items-center gap-2">

        <ShoppingCart class="w-5 h-5 text-slate-700 dark:text-[#39BDF8]" />
        
        <h3 class="text-lg font-semibold text-slate-800 dark:text-white">
          Historial de Ventas
        </h3>

      </div>

     <p class="text-sm text-slate-500 dark:text-slate-400">
        {sales.length} ventas encontradas
      </p>

    </div>

    <button
      on:click={descargarPDF}
      disabled={loading || sales.length === 0}
      class="h-11 w-11 rounded-xl border  border-slate-200 flex items-center justify-center hover:bg-slate-50 disabled:opacity-50 disabled:cursor-not-allowed transition-colors dark:border-[#334156] dark:hover:bg-[#0F172A]"
      title="Descargar reporte"
    >
      <Download class="w-5 h-5 text-slate-700 dark:text-[#39BDF8] " />
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
   <div class="overflow-x-auto px-6 py-6 dark:bg-[#1E293B]">

      <div class="border border-slate-200 rounded-xl overflow-hidden dark:border-[#475569]">

        <table class="w-full dark:border-b-[#475569]">

          <thead class="bg-[#26557c] dark:bg-[#334156] border-b-2 border-slate-300 dark:border-b-[#475569]">

            <tr class="text-left ">

             <th class="px-6 py-4 text-sm font-medium text-[#dee6eb] dark:text-white">
                Producto
              </th>

             <th class="px-6 py-4 text-sm font-medium text-[#dee6eb] dark:text-white">
                Precio
              </th>

             <th class="px-6 py-4 text-sm font-medium text-[#dee6eb] dark:text-white">
                Método
              </th>

             <th class="px-6 py-4 text-sm font-medium text-[#dee6eb] dark:text-white">
                Vendedor
              </th>

             <th class="px-6 py-4 text-sm font-medium text-[#dee6eb] dark:text-white">
                Fecha y Hora
              </th>

            </tr>

          </thead>

          <tbody class="divide-y divide-slate-200 dark:divide-[#334156]">

            {#if paginatedSales.length > 0}

              {#each paginatedSales as sale (sale.id)}
                {@const metodoEstilo = getMetodoEstilo(sale.metodo)}
                {@const formattedDate = formatearFechaHora(
                  new Date(sale.fecha.replace(' ', 'T'))
                )}

                <tr class="border-t border-slate-100 dark:border-[#334156] transition hover:bg-slate-50 dark:hover:bg-[#0F172A]">

                  <td class="px-6 py-5">
                    <span class="text-sm text-slate-600 dark:text-[#E2E8F0]">
                      {sale.producto}
                    </span>
                  </td>

                  <td class="px-6 py-5">
                    <span class="text-sm text-slate-600 dark:text-[#E2E8F0]">
                     ${formatCurrency(sale.precio)}
                    </span>
                  </td>

                  <td class="px-6 py-5 ">
                    <div class="inline-flex items-center gap-2  rounded-full  bg-[#1c5476]/10 px-3 py-1 text-xs font-medium text-[#1c5476] dark:bg-[#39BDF8]/20 dark:text-[#39BDF8]">
                      {#if metodoEstilo.icon}
                        <svelte:component this={metodoEstilo.icon} class="w-4 h-4 {metodoEstilo.text}" />
                      {/if}
                      <span class="text-sm font-medium">
                        {sale.metodo}
                      </span>
                    </div>
                  </td>

                  <td class="px-6 py-5">
                    <span class="text-sm text-slate-600 dark:text-[#E2E8F0]">
                      {sale.vendedor}
                    </span>
                  </td>

                  <td class="px-6 py-5">
                    <div class="flex flex-col">
                      <span class="text-sm font-semibold text-slate-800 dark:text-white">
                        {formattedDate.fecha}
                      </span>

                      <span class="text-xs text-slate-500 dark:text-slate-400">
                        {formattedDate.hora}
                      </span>
                    </div>
                  </td>

                </tr>

              {/each}

            {:else}

              <tr>

                <td colspan="5" class="px-6 py-16 text-center">

                  <div class="flex flex-col items-center justify-center">

                   <Search class="mb-3 h-10 w-10 text-slate-300 dark:text-slate-600" />

                    <h3 class="text-base font-semibold text-slate-700 dark:text-slate-300">
                      No se encontraron resultados
                    </h3>

                    <p class="mt-1 text-sm text-slate-500 dark:text-slate-400">
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
    <div class="px-6 pb-6 dark:bg-[#1E293B]">

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