<script lang="ts">
    import Package from 'lucide-svelte/icons/package';
    import Tag from 'lucide-svelte/icons/tag';
    import AlertTriangle from 'lucide-svelte/icons/alert-triangle';
    import LogIn from 'lucide-svelte/icons/log-in';

    export let totalIngresado = 2450;
    export let totalProductos = 12;
    export let stockBajo = 3;
    export let totalEntradas = 0;

    interface Stat {
        icon: 'Package' | 'Tag' | 'AlertTriangle' | 'LogIn';
        title: string;
        value: string | number;
        color: string;
    }

    const icons = {
        Package,
        Tag,
        AlertTriangle,
        LogIn
    };

    const stats: Stat[] = [
        {
            icon: 'Package',
            title: 'Total Ingresado',
            value: `${totalIngresado.toLocaleString()} Uni`,
            color: '#0C4A6E'
        },
        {
            icon: 'Tag',
            title: 'Productos',
            value: totalProductos,
            color: '#0C4A6E'
        },
        {
            icon: 'AlertTriangle',
            title: 'Stock Bajo',
            value: stockBajo,
            color: '#0C4A6E'
        },
        {
            icon: 'LogIn',
            title: 'Total Entradas',
            value: totalEntradas,
            color: '#0C4A6E'
        }
    ];
</script>

<div class="grid grid-cols-4 gap-4">

    {#each stats as stat}

        <div
            class="flex items-center gap-4 rounded-2xl border border-slate-200 bg-white p-4 transition-all duration-300 hover:shadow-md"
        >

            <div
                class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-slate-100"
            >
                <svelte:component
                    this={icons[stat.icon]}
                    class="h-5 w-5"
                    style="color: {stat.color}"
                />
            </div>

            <div class="min-w-0">
                <p
                    class="text-[11px] font-semibold uppercase tracking-wide text-slate-500"
                >
                    {stat.title}
                </p>

                <h3 class="mt-1 text-2xl font-bold text-slate-800">
                    {stat.value}
                </h3>
            </div>

        </div>

    {/each}

</div>