<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import Search from 'lucide-svelte/icons/search';
    import Download from 'lucide-svelte/icons/download';
    import { ChevronDown } from 'lucide-svelte';

    const dispatch = createEventDispatcher();

    let fechaInicio = '';
    let fechaFin = '';
    let tipoProducto = '';
    let proveedor = '';

    function aplicarFiltros() {
        dispatch('filter', {
            fechaInicio,
            fechaFin,
            tipoProducto,
            proveedor
        });
    }

    function exportarPDF() {
        dispatch('export');
    }
</script>

<div class="bg-white border border-slate-200 rounded-2xl p-5">

    <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-5 gap-4">

        <!-- Fecha Inicio -->
        <div>
            <label class="text-xs font-semibold text-slate-500 uppercase">
                Fecha Inicio
            </label>

            <input
                bind:value={fechaInicio}
                type="date"
                class="w-full mt-2 h-11 px-4 rounded-xl border border-slate-200"
            />
        </div>

        <!-- Fecha Fin -->
        <div>
            <label class="text-xs font-semibold text-slate-500 uppercase">
                Fecha Fin
            </label>

            <input
                bind:value={fechaFin}
                type="date"
                class="w-full mt-2 h-11 px-4 rounded-xl border border-slate-200"
            />
        </div>

        <!-- Tipo Producto -->
        <div class="relative">
            <label class="text-xs font-semibold text-slate-500 uppercase">
                Tipo de Producto
            </label>

            <select
                bind:value={tipoProducto}
                class="h-11 w-full rounded-xl border mt-2 border-slate-200 px-4 pr-10 text-sm outline-none focus:border-cyan-600 appearance-none bg-transparent"
            >
                <option value="">Todos</option>
                <option value="proteinas">Proteínas</option>
                <option value="suplementos">Suplementos</option>
                <option value="bebidas">Bebidas</option>
                <option value="snacks">Snacks</option>
            </select>

            <ChevronDown
                size={18}
                class="absolute right-3 top-[45px] pointer-events-none text-slate-400"
            />
        </div>

        <!-- Vendedor -->
        <div class="relative">
            <label class="text-xs font-semibold text-slate-500 uppercase">
                Vendedor
            </label>

            <select
                bind:value={proveedor}
                class="h-11 w-full rounded-xl border mt-2 border-slate-200 px-4 pr-10 text-sm outline-none focus:border-cyan-600 appearance-none bg-transparent"
            >
                <option value="">Todos</option>
                <option value="nutricion-pro">Persona 1</option>
                <option value="suplementos-elite">Persona 2</option>
                <option value="fitness-world">Persona 3</option>
            </select>

            <ChevronDown
                size={18}
                class="absolute right-3 top-[45px] pointer-events-none text-slate-400"
            />
        </div>

        <!-- Botones -->
        <div class="flex items-end gap-3">
            <button
                on:click={aplicarFiltros}
                class="h-11 px-5 rounded-xl bg-[#0C4A6E] text-white flex items-center gap-2 cursor-pointer hover:bg-sky-800"
            >
                <Search class="w-4 h-4" />
                Filtrar
            </button>

            <button
                on:click={exportarPDF}
                class="h-11 w-11 rounded-xl border border-slate-200 flex items-center justify-center hover:bg-slate-50 cursor-pointer"
            >
                <Download class="w-4 h-4" />
            </button>
        </div>

    </div>

</div>