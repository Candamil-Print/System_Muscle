<script lang="ts">
    import ShoppingCart from 'lucide-svelte/icons/shopping-cart';
    import Download from 'lucide-svelte/icons/download';

    export let entradas: any[] = [];
   

    function getTipoEstilo(tipo: string) {
        const estilos: Record<string, { bg: string; text: string; border: string }> = {
            'Proteína': { bg: 'bg-blue-50', text: 'text-blue-700', border: 'border-blue-200' },
            'Suplemento': { bg: 'bg-purple-50', text: 'text-purple-700', border: 'border-purple-200' },
            'Accesorio': { bg: 'bg-green-50', text: 'text-green-700', border: 'border-green-200' }
        };
        return estilos[tipo] || { bg: 'bg-slate-50', text: 'text-slate-700', border: 'border-slate-200' };
    }

    function descargarCSV() {
        if (entradas.length === 0) {
            alert('No hay datos para descargar');
            return;
        }

        const headers = ['Producto', 'Cantidad', 'Fecha y Hora', 'Quién Recibe', 'Tipo'];

        const rows = entradas.map(entrada => [
            entrada.producto,
            entrada.cantidad,
            entrada.fecha,
            entrada.recibe,
            entrada.tipo
        ]);

        const csvContent = [
            headers.join(','),
            ...rows.map(row => row.map(cell => `"${cell}"`).join(','))
        ].join('\n');

        const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });
        const link = document.createElement('a');
        const url = URL.createObjectURL(blob);

        link.setAttribute('href', url);
        link.setAttribute('download', `reporte-entradas-${new Date().toISOString().split('T')[0]}.csv`);
        link.style.visibility = 'hidden';

        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
    }

    function formatearFechaHora(fechaTexto: string) {
    if (!fechaTexto) {
        return {
            fecha: '-',
            hora: '-'
        };
    }

    const [fechaParte, horaParte] = fechaTexto.split(',');

    const [dia, mes, anioCorto] = fechaParte.trim().split('/');

    const meses = [
        'enero',
        'febrero',
        'marzo',
        'abril',
        'mayo',
        'junio',
        'julio',
        'agosto',
        'septiembre',
        'octubre',
        'noviembre',
        'diciembre'
    ];

    const anio = `20${anioCorto}`;

    return {
        fecha: `${Number(dia)} de ${meses[Number(mes) - 1]} de ${anio}`,
        hora: horaParte.trim()
    };
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
            on:click={descargarCSV}
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