<script lang="ts">
    import { onMount } from 'svelte';
    import Search from 'lucide-svelte/icons/search';
    import Download from 'lucide-svelte/icons/download';
    import { toast } from 'svelte-sonner';
    import CalendarDays from 'lucide-svelte/icons/calendar-days';
    import { ChevronDown } from 'lucide-svelte';

    import {  entradasPorUsuario, resumenEntradasPorProducto } from '$lib/services/api/reports/entries/entries.service';


    import flatpickr from 'flatpickr';
    import { Spanish } from 'flatpickr/dist/l10n/es.js';

    // ==========================
    // Referencias a los campos de fecha
    // ==========================
    let inicioInput: HTMLInputElement;
    let finInput: HTMLInputElement;

    let finPicker: flatpickr.Instance;

    // ==========================
    // Propiedades recibidas desde el componente padre
    // ==========================
    export let onFilter: (filters: { fechaInicio: string; fechaFin: string; tipoProducto: string; vendedor: string }) => void = () => {};

    export let loading = false;

    let fechaInicio = '';
    let fechaFin = '';
    let tipoProducto = 'todos';
    let vendedor = 'todos';

    let vendedoresEntrada: EntradasPorUsuario[] = [];

    let tiposProducto: string[] = [];

    // ==========================
    // Obtiene los tipos de producto registrados para llenar elnselector de filtros
    // ==========================
    async function cargarTiposProducto() {
    try {
        const today = new Date();
        const treintaDiasAgo = new Date(
            today.getTime() - 30 * 24 * 60 * 60 * 1000
        );

        const inicio = treintaDiasAgo.toISOString().split('T')[0];
        const fin = today.toISOString().split('T')[0];

        const resumen = await resumenEntradasPorProducto(
            inicio,
            fin
        );

        tiposProducto = [
            ...new Set(
                resumen.map(r => r.tipo_producto)
            )
        ].sort();

    } catch (err) {
        console.error('Error cargando tipos de producto:', err);
    }
}

    // ==========================
    // Obtiene los vendedores que tienen movimientos registrados para llenar el selector
    // ==========================
    async function cargarVendedores() {
        try {
        const today = new Date();
        const treintaDiasAgo = new Date(today.getTime() - 30 * 24 * 60 * 60 * 1000);
        
        const inicio = treintaDiasAgo.toISOString().split('T')[0];
        const fin = today.toISOString().split('T')[0];

        vendedoresEntrada = await  entradasPorUsuario(inicio, fin);
        console.log("Vendedores obtenidos filtro dos:", vendedoresEntrada);
        console.table(vendedoresEntrada);
        } catch (err) {
        console.error('Error cargando vendedores:', err);
        }
    }


    // ==========================
    // Valida los filtros y envía
    // ==========================
    function handleFilter() {
        if (!fechaInicio || !fechaFin) {
        toast.warning('Debes seleccionar ambas fechas');
        return;
        }
    
        onFilter({
        fechaInicio,
        fechaFin,
        tipoProducto,
        vendedor
        });
    }

    // ==========================
    // Inicializa los componentes
    // ==========================
    onMount(() => {
        cargarVendedores();
        cargarTiposProducto();

        // Configuración del selector de fecha inicial
        flatpickr(inicioInput, {
            locale: Spanish,
            dateFormat: 'Y-m-d',
            allowInput: true,

            onChange: (selectedDates) => {
                if (selectedDates[0]) {
                    fechaInicio = selectedDates[0]
                        .toISOString()
                        .split('T')[0];

                    finPicker?.set('minDate', fechaInicio);

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

        // Configuración del selector de fecha final
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

<div class="bg-white border border-slate-200 dark:bg-[#1E293B] dark:border-[#334156] rounded-2xl p-5">

    <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-5 gap-4">

        <!-- Fecha Inicio -->
        <div>
            <label class="text-xs font-semibold text-slate-500 dark:text-white uppercase">
                Fecha Inicio
            </label>

            <div class="relative mt-2">

                <CalendarDays
                class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400 pointer-events-none"
                />

                <input
                    bind:this={inicioInput}
                    placeholder="Seleccione fecha"
                    class="w-full h-11 pl-10 pr-4 rounded-xl border border-slate-200 bg-white text-slate-700 dark:bg-[#1E293B] dark:border-[#334156] dark:text-[#E2E8F0]"
                />

            </div>
        </div>

        <!-- Fecha Fin -->
        <div>
            <label class="text-xs font-semibold text-slate-500 dark:text-white uppercase">
                Fecha Fin
            </label>

            <div class="relative mt-2">

                <CalendarDays
                class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400 pointer-events-none"
                />

                <input
                    bind:this={finInput}
                    placeholder="Seleccione fecha"
                    class="w-full h-11 pl-10 pr-4 rounded-xl border border-slate-200 bg-white text-slate-700 dark:bg-[#1E293B] dark:border-[#334156] dark:text-[#E2E8F0]"
                />

            </div>
        </div>

        <!-- Tipo Producto -->
        <div class="relative">
            <label class="text-xs font-semibold text-slate-500 dark:text-white uppercase">
                Tipo de Producto
            </label>

            <select
                bind:value={tipoProducto}
                class="h-11 w-full rounded-xl border mt-2 border-slate-200 px-4 pr-10 text-sm outline-none focus:border-cyan-600 appearance-none bg-transparent dark:border-[#334156] dark:text-white"
            >
                <option value="todos">Todos</option>
                {#each tiposProducto as tipo}
                    <option value={tipo}>
                        {tipo.charAt(0) + tipo.slice(1).toLowerCase()}
                    </option>
                {/each}
            </select>

            <ChevronDown
                size={18}
                class="absolute right-3 top-[45px] pointer-events-none text-slate-400"
            />
        </div>

        <!-- Vendedor -->
        <div class="relative">
            <label class="text-xs font-semibold text-slate-500 dark:text-white uppercase">
                Vendedor
            </label>

            <select
                bind:value={vendedor}
                class="h-11 w-full rounded-xl border mt-2 border-slate-200 px-4 pr-10 text-sm outline-none focus:border-cyan-600 appearance-none bg-transparent dark:border-[#334156] dark:text-white"
            >
                <option value="todos">Todos</option>
                {#each vendedoresEntrada as v (v.id_usuario)}
                    <option value={v.id_usuario}>
                        {v.nombre_usuario}
                    </option>
                {/each}
            </select>

            <ChevronDown
                size={18}
                class="absolute right-3 top-[45px] pointer-events-none text-slate-400"
            />
        </div>

        <!-- Botones -->
        <div class="flex items-end gap-3">
            <button
                on:click={handleFilter}
                disabled={loading}
                class="h-11 px-5 rounded-xl bg-[#0C4A6E] text-white dark:text-[#39BDF8]  flex items-center gap-2 cursor-pointer hover:bg-sky-800"
            >
                <Search class="w-4 h-4 " />
                {loading ? 'Cargando...' : 'Filtrar'}
            </button>

            <button
                on:click={exportarPDF}
                class="h-11 w-11 rounded-xl border border-slate-200 flex items-center justify-center hover:bg-slate-50 dark:hover:bg-[#162033] cursor-pointer dark:border-[#334156] "
            >
                <Download class="w-4 h-4 dark:text-[#39BDF8]" />
            </button>
        </div>

    </div>

</div>