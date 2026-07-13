<script lang="ts">
    import { onMount } from 'svelte';
    import { toast } from 'svelte-sonner';
    import ShoppingCart from 'lucide-svelte/icons/shopping-cart';
    import Download from 'lucide-svelte/icons/download';

    import jsPDF from 'jspdf';
    import autoTable from 'jspdf-autotable';

    import { listarMovimientosEntrada } from '$lib/services/api/reports/entries/entries.service';

   
    // Propiedades recibidas desde el componente padre
    export let entradas: any[] = [];

    export let filtrosEntradas = {
        fechaInicio: '',
        fechaFin: '',
        tipoProducto: 'todos',
        vendedorEntrada: 'todos'
    }

    // Variables de control del componente
    let loading = true;
    let error = '';
    let currentPage = 1;
   

    // ==========================
    // Retorna los estilos visuales
    // según el tipo de producto
    // ==========================

    function getTipoEstilo(tipo: string) {
        const estilos: Record<string, { bg: string; text: string; border: string }> = {
            'Proteína': { bg: 'bg-blue-50', text: 'text-blue-700', border: 'border-blue-200' },
            'Suplemento': { bg: 'bg-purple-50', text: 'text-purple-700', border: 'border-purple-200' },
            'Accesorio': { bg: 'bg-green-50', text: 'text-green-700', border: 'border-green-200' }
        };
        return estilos[tipo] || { bg: 'bg-slate-50', text: 'text-slate-700', border: 'border-slate-200' };
    }

    function descargarPDF() {
        if (entradas.length === 0) {
            alert('No hay datos para descargar');
            return;
        }

        const doc = new jsPDF('p', 'mm', 'a4');

        const fechaGeneracion = formatearFechaHora(
            new Date().toISOString()
        );

        // ==========================
        // CALCULOS
        // ==========================

        const totalEntradas = entradas.reduce(
            (acc, entrada) =>
                acc + Number(
                    String(entrada.cantidad).replace(' Uni', '')
                ),
            0
        );

        const promedio =
            entradas.length > 0
                ? totalEntradas / entradas.length
                : 0;

        const mayorEntrada =
            entradas.length > 0
                ? [...entradas].sort(
                    (a, b) =>
                        Number(b.cantidad.replace(' Uni', '')) -
                        Number(a.cantidad.replace(' Uni', ''))
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
            "REPORTE DE ENTRADAS",
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
            {
                titulo: 'Registros',
                valor: entradas.length.toString()
            },
            {
                titulo: 'Unidades',
                valor: totalEntradas.toLocaleString('es-CO')
            },
            {
                titulo: 'Promedio',
                valor: promedio.toFixed(1)
            }
        ];

        let x = 14;

        cards.forEach(({ titulo, valor }) => {

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
        // INDICADOR CLAVE
        // ==========================

        // Título de la sección
        doc.setTextColor(12, 74, 110);
        doc.setFont('helvetica', 'bold');
        doc.setFontSize(15);

        doc.text(
            'Indicador Principal',
            14,
            98
        );

        // Línea decorativa
        doc.setDrawColor(12, 74, 110);
        doc.setLineWidth(0.4);
        doc.line(14, 100, 196, 100);

        // Sombra
        doc.setFillColor(228, 232, 236);
        doc.roundedRect(
            15,
            105,
            182,
            22,
            3,
            3,
            'F'
        );

        // Tarjeta principal
        doc.setFillColor(250, 250, 250);
        doc.roundedRect(
            14,
            104,
            182,
            22,
            3,
            3,
            'F'
        );

        // Barra superior azul
        doc.setFillColor(12, 74, 110);
        doc.roundedRect(
            14,
            104,
            182,
            2.5,
            3,
            3,
            'F'
        );

        if (mayorEntrada) {

            // Etiqueta
            doc.setTextColor(120, 120, 120);
            doc.setFont('helvetica', 'normal');
            doc.setFontSize(8);

            doc.text(
                'Producto con mayor cantidad registrada',
                18,
                112
            );

            // Nombre del producto
            doc.setTextColor(35, 35, 35);
            doc.setFont('helvetica', 'bold');
            doc.setFontSize(12);

            doc.text(
                String(mayorEntrada.producto).slice(0, 40),
                18,
                120
            );

            // Cantidad grande
            doc.setTextColor(12, 74, 110);
            doc.setFont('helvetica', 'bold');
            doc.setFontSize(18);

            doc.text(
                mayorEntrada.cantidad,
                190,
                119,
                {
                    align: 'right'
                }
            );

            // Texto debajo de la cantidad
            doc.setTextColor(120, 120, 120);
            doc.setFont('helvetica', 'normal');
            doc.setFontSize(8);

            doc.text(
                'Unidades',
                190,
                124,
                {
                    align: 'right'
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
                'Cantidad',
                'Tipo',
                'Quién recibe',
                'Fecha'
            ]],

            body: entradas.map((entrada) => {
                const fecha =
                    formatearFechaHora(
                        entrada.fecha
                    );

                return [
                    entrada.producto,
                    entrada.cantidad,
                    entrada.tipo,
                    entrada.recibe,
                    `${fecha.fecha}\n${fecha.hora}`
                ];
            }),

            headStyles:{
                fillColor:[12,74,110],
                textColor:[255,255,255],
                fontStyle:"bold",
                fontSize:10,
                halign:"center"
            },

            alternateRowStyles:{
                fillColor:[248,249,250]
            },

            styles:{
                font:"helvetica",
                fontSize:9,
                cellPadding:4,
                lineColor:[225,230,235],
                lineWidth:.2,
                valign:"middle"
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
            `reporte-entradas-${
                new Date()
                    .toISOString()
                    .split('T')[0]
            }.pdf`
        );
    }

    // Convierte la fecha recibida
    function formatearFechaHora(fechaTexto: string) {
        if (!fechaTexto) {
            return {
                fecha: "-",
                hora: "-"
            };
        }

        const fecha = new Date(fechaTexto);

        return {
            fecha: fecha.toLocaleDateString("es-CO", {
                day: "numeric",
                month: "long",
                year: "numeric"
            }),
            hora: fecha.toLocaleTimeString("es-CO", {
                hour: "numeric",
                minute: "2-digit"
            })
        };
    }

    // ==========================
    //  Actualiza la información de la tabla aplicando los filtros seleccionados
    // ==========================
    export async function actualizarTablaEntrada(
        fechaInicio: string,
        fechaFin: string,
        tipoProducto = 'todos',
        vendedorEntrada = 'todos',
        mostrarToast = false
    ) {
        loading = true;
        error = '';
        currentPage = 1;

        try {
            // Obtener todos los movimientos registrados
            const historialEntradas = await listarMovimientosEntrada();

            console.log("Filtros:", {
                fechaInicio,
                fechaFin,
                tipoProducto,
                vendedorEntrada
            });

            // Aplicar filtros seleccionados
            const resultado = historialEntradas.filter((entrada) => {

                const fechaEntrada = entrada.fecha.split(' ')[0];

                // Filtro por fecha
                if (
                    fechaEntrada < fechaInicio ||
                    fechaEntrada > fechaFin
                ) {
                    return false;
                }

                // Filtro por vendedor
                if (
                    vendedorEntrada !== 'todos' &&
                    String(entrada.id_usuario) !== String(vendedorEntrada)
                ) {
                    return false;
                }

                // Filtro por tipo
                if (
                    tipoProducto !== 'todos' &&
                    entrada.tipo_producto.toLowerCase() !== tipoProducto.toLowerCase()
                ) {
                    return false;
                }

                return true;
            }).map((entrada) => ({
                id: `${entrada.id_movimiento}-${entrada.id_producto}`,
                producto: entrada.nombre_producto,
                cantidad: `${entrada.cantidad} Uni`,
                fecha: entrada.fecha,
                recibe: entrada.nombre_usuario,
                tipo: entrada.tipo_producto
            }));

            entradas = resultado;

            console.log("Entradas dentro de EntriesTable:", entradas.length);

            if (mostrarToast && resultado.length === 0) {
                toast.warning(
                    'No se encontraron resultados',
                    {
                        id: 'sin-resultados-tabla',
                        description:
                            'No existen entradas para los filtros seleccionados.'
                    }
                );
            }

        } catch (err) {
            console.error(err);
            error = String(err);
        } finally {
            loading = false;
        }
    }
    

    // ==========================
    // Carga el historial completo 
    // ==========================
    onMount(async () => {
        loading = true;

        const historial = await listarMovimientosEntrada();

        entradas = historial.map(entrada => ({
            id: `${entrada.id_movimiento}-${entrada.id_producto}`,
            producto: entrada.nombre_producto,
            cantidad: `${entrada.cantidad} Uni`,
            fecha: entrada.fecha,
            recibe: entrada.nombre_usuario,
            tipo: entrada.tipo_producto
        }));

        loading = false;
    });


    // =====================
    // Actualiza la tabla cuando cambian los filtros seleccionados
    // =====================
    $: if (
        filtrosEntradas?.fechaInicio &&
        filtrosEntradas?.fechaFin
    ) {
        actualizarTablaEntrada(
        filtrosEntradas.fechaInicio,
        filtrosEntradas.fechaFin,
        filtrosEntradas.tipoProducto,
        filtrosEntradas.vendedorEntrada,
        true
        );
    }
</script>

<div class="bg-white border border-slate-200 dark:bg-[#1E293B] dark:border-[#334156] rounded-2xl overflow-hidden">

    <div class="flex items-center justify-between px-6 py-4 border-b border-slate-200 dark:border-[#334156] ">

        <div class="flex flex-col items-start gap-0">

            <div class="flex items-center gap-2">

                <ShoppingCart class="w-5 h-5 text-slate-700 dark:text-[#39BDF8]" />

                <h3 class="text-lg font-semibold text-slate-800 dark:text-white">
                    Historial de Entradas
                </h3>

            </div>

            <p class="text-sm text-slate-500">
                {entradas.length} entradas encontradas
            </p>

        </div>

        <button
            on:click={descargarPDF}
            disabled={entradas.length === 0}
            class="h-11 w-11 rounded-xl border border-slate-200 dark:border-[#334156] flex items-center justify-center hover:bg-slate-50 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            title="Descargar reporte"
        >
            <Download class="w-5 h-5 text-slate-700 dark:text-[#39BDF8]" />
        </button>
    </div>

    <div class="overflow-x-auto px-6 py-6">

        <div class="border border-slate-200 dark:border-[#334156] rounded-xl overflow-hidden">

            <table class="w-full">

                <thead class="bg-[#26557c] dark:bg-[#334156] ">

                    <tr class="text-left">

                        <th class="px-6 py-4 text-sm font-medium text-[#ffffff]">
                            ¿Qué entró?
                        </th>

                        <th class="px-6 py-4 text-sm font-medium text-[#ffffff]">
                            ¿Cuánto?
                        </th>

                        <th class="px-6 py-4 text-sm font-medium text-[#ffffff]">
                            Fecha y Hora
                        </th>

                        <th class="px-6 py-4 text-sm font-medium text-[#ffffff]">
                            ¿Quién recibe?
                        </th>

                        <th class="px-6 py-4 text-sm font-medium text-[#ffffff]">
                            Tipo
                        </th>

                    </tr>

                </thead>

                <tbody class="divide-y divide-slate-200 dark:divide-[#334156]">

                    {#if entradas.length > 0}

                        {#each entradas as entrada (entrada.id)}
                        
                            {@const tipoEstilo = getTipoEstilo(entrada.tipo)}
                            

                            <tr class="bg-white dark:bg-[#1E293B] hover:bg-slate-50 dark:hover:bg-[#162033] transition-colors">

                                <td class="px-6 py-5">
                                    <span class="font-medium text-slate-700 dark:text-white">
                                        {entrada.producto}
                                    </span>
                                </td>

                                <td class="px-6 py-5">
                                    <span class="text-slate-700 dark:text-white">
                                        {entrada.cantidad}
                                    </span>
                                </td>

                                <td class="px-6 py-5">
                                    <div class="flex flex-col leading-tight">
                                        <span class="text-base font-bold text-slate-800 dark:text-white">
                                            {formatearFechaHora(entrada.fecha).fecha}
                                        </span>

                                        <span class="mt-1 text-sm font-medium text-slate-500">
                                            {formatearFechaHora(entrada.fecha).hora}
                                        </span>
                                    </div>
                                </td>

                                <td class="px-6 py-5">
                                    <span class="text-slate-700 dark:text-white">
                                        {entrada.recibe}
                                    </span>
                                </td>

                                <td class="px-6 py-5">
                                    <span
                                        class="inline-flex px-3 py-2 rounded-full text-sm font-medium bg-[#1c5476]/10 px-3 py-1 text-xs font-medium text-[#1c5476] dark:bg-[#39BDF8]/20 dark:text-[#39BDF8]"
                                    >
                                        {entrada.tipo}
                                    </span>
                                </td>

                            </tr>

                        {/each}

                    {:else}

                        <tr>

                            <td colspan="5" class="px-6 py-16 text-center">

                                <div class="flex flex-col items-center justify-center">

                                    <h3 class="text-base font-semibold text-slate-700">
                                        No se encontraron resultados
                                    </h3>

                                    <p class="mt-1 text-sm text-slate-500">
                                        No hay entradas registradas
                                    </p>

                                </div>

                            </td>

                        </tr>

                    {/if}

                </tbody>

            </table>

        </div>
    </div>
</div>