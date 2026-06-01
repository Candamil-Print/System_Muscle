<script lang="ts">
    import { onMount } from 'svelte';
    import { listarHistorialConDetalle, type HistorialDetalle } from '$lib/services/api/history';
    import ClipboardClock from 'lucide-svelte/icons/clipboard-clock';
    import Pagination from '../movimientos/Pagination.svelte';

    // NUEVO: filtros recibidos desde el padre
    export let filtros = {
        search: '',
        turno: '',
        fecha: ''
    };

    let historial: HistorialDetalle[] = [];
    let historialOriginal: HistorialDetalle[] = [];

    let cargando = true;
    let error = '';
    let currentPage = 1;
    const itemsPerPage = 8;

    type HistorialTabla = {
        id_historial: number;
        accion: string;
        detalle: string;
        usuario: string;
        turno: string;
        fecha: string;
    };

    onMount(async () => {
        await cargarHistorial();
    });

    async function cargarHistorial() {
        try {
            cargando = true;
            error = '';

            historialOriginal = await listarHistorialConDetalle({});
            historial = [...historialOriginal];

            console.log('Historial cargado:', historial);
        } catch (err) {
            console.error('Error:', err);
            error = 'Error al cargar el historial';
        } finally {
            cargando = false;
        }
    }

    // NUEVO: aplicar filtros automáticamente
    $: if (historialOriginal.length >= 0) {
        const textoBusqueda = filtros.search?.toLowerCase() ?? '';

        historial = historialOriginal.filter((item) => {
            const matchSearch =
                !textoBusqueda ||
                item.accion?.toLowerCase().includes(textoBusqueda) ||
                item.descripcion?.toLowerCase().includes(textoBusqueda) ||
                item.usuario?.toLowerCase().includes(textoBusqueda);

            const matchTurno =
                !filtros.turno ||
                item.turno === filtros.turno;

            const matchFecha =
                !filtros.fecha ||
                item.fecha === filtros.fecha;

            return (
                matchSearch &&
                matchTurno &&
                matchFecha
            );
        });

        currentPage = 1;
    }

    function badgeClass(accion: string) {
        const classes: Record<string, string> = {
            "Venta": "bg-green-100 text-green-700",
            "Entrada Stock": "bg-green-100 text-green-700",
            "Inicio sesión": "bg-sky-100 text-sky-700",
            "Modificar Producto": "bg-amber-100 text-amber-700",
        };

        return classes[accion] || "bg-slate-100 text-slate-700";
    }

    function turnoClass(turno: string) {
        const classes: Record<string, string> = {
            "Mañana": "bg-amber-50 text-amber-600 border border-amber-200",
            "Tarde": "bg-blue-50 text-blue-600 border border-blue-200",
            "Único": "bg-purple-50 text-purple-600 border border-purple-200",
        };

        return classes[turno] || "bg-slate-50 text-slate-600 border border-slate-200";
    }

    // PAGINACIÓN
    $: totalPages = Math.ceil(historial.length / itemsPerPage);

    $: paginatedHistorial = historial.slice(
        (currentPage - 1) * itemsPerPage,
        currentPage * itemsPerPage
    );
</script>

{#if cargando}
    <div class="text-center py-8">
        Cargando historial...
    </div>

{:else if error}

    <div class="text-center py-8 text-red-500">
        {error}
    </div>

{:else}

    <div class="bg-white border border-slate-200 rounded-2xl overflow-hidden">

        <div class="flex items-start justify-between px-6 py-4 border-b border-slate-200">

            <div class="flex flex-col items-start gap-0">

                <div class="flex items-center gap-2">

                    <ClipboardClock class="w-5 h-5 text-slate-700" />

                    <h3 class="text-lg font-semibold text-slate-800">
                        Registro de Actividad
                    </h3>

                </div>

                <p class="text-sm text-slate-500">
                    {historial.length} acciones encontradas
                </p>

            </div>

        </div>

        {#if historial.length === 0}

            <div class="text-center py-16">
                <p class="text-slate-500">
                    No hay registros de actividad
                </p>
            </div>

        {:else}

            <div class="overflow-x-auto px-6 py-6">

                <div class="border border-slate-200 rounded-xl overflow-hidden">

                    <table class="w-full">

                        <thead class="bg-[#26557c]">

                            <tr class="text-left">

                                <th class="px-6 py-4 text-sm font-medium text-[#ffffff]">
                                    Acción
                                </th>

                                <th class="px-6 py-4 text-sm font-medium text-[#ffffff]">
                                    Detalles
                                </th>

                                <th class="px-6 py-4 text-sm font-medium text-[#ffffff]">
                                    Usuario
                                </th>

                                <th class="px-6 py-4 text-sm font-medium text-[#ffffff]">
                                    Turno
                                </th>

                                <th class="px-6 py-4 text-sm font-medium text-[#ffffff]">
                                    Fecha
                                </th>

                            </tr>

                        </thead>

                        <tbody class="divide-y divide-slate-200">

                            {#each paginatedHistorial as item}

                                <tr class="hover:bg-slate-50">

                                    <td class="px-6 py-4">

                                        <span
                                            class={`inline-flex rounded-full px-3 py-1 text-xs bg-[#1c5476]/10 text-[#1c5476] font-medium ${badgeClass(item.accion)}`}
                                        >
                                            {item.accion}
                                        </span>

                                    </td>

                                    <td class="px-6 py-4 text-sm text-slate-900">
                                        {item.descripcion}
                                    </td>

                                    <td class="px-6 py-4 text-sm text-slate-900">
                                        {item.usuario}
                                    </td>

                                    <td class="px-6 py-4">

                                        <span
                                            class={`inline-flex rounded-full px-3 py-1 text-xs font-medium bg-[#1c5476]/10 text-[#1c5476] ${turnoClass(item.turno)}`}
                                        >
                                            {item.turno}
                                        </span>

                                    </td>

                                    <td class="px-6 py-4 text-sm text-slate-500">
                                        {item.fecha} {item.hora}
                                    </td>

                                </tr>

                            {/each}

                        </tbody>

                    </table>

                </div>

            </div>

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

{/if}