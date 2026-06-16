<script lang="ts">
    import Package from 'lucide-svelte/icons/package';
    import Tag from 'lucide-svelte/icons/tag';
    import AlertTriangle from 'lucide-svelte/icons/alert-triangle';
    import LogIn from 'lucide-svelte/icons/log-in';

    export let totalIngresado = 0;
    export let totalProductos = 0;
    export let stockBajo = 0;
    export let totalEntradas = 0;

    interface Stat {
        icon: 'Package' | 'Tag' | 'AlertTriangle' | 'LogIn';
        title: string;
        value: string | number;
    }

    const icons = {
        Package,
        Tag,
        AlertTriangle,
        LogIn
    };

    $: stats = [
        {
            icon: 'Package',
            title: 'Total Ingresado',
            value: `${totalIngresado.toLocaleString()} Uni`
        },
        {
            icon: 'Tag',
            title: 'Productos',
            value: totalProductos
        },
        {
            icon: 'AlertTriangle',
            title: 'Stock Bajo',
            value: stockBajo
        },
        {
            icon: 'LogIn',
            title: 'Total Entradas',
            value: totalEntradas
        }
    ];
</script>

<div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-4">

    {#each stats as stat}

        <div
            class="flex items-center gap-4 rounded-2xl border border-slate-200 bg-white p-4 transition-all duration-300 hover:shadow-md dark:border-[#334156] dark:bg-[#1E293B]"
        >

            <div
                class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-slate-100 dark:bg-[#39BDF8]/10"
            >
                <svelte:component
                    this={icons[stat.icon]}
                    class="h-5 w-5 text-[#0C4A6E] dark:text-[#39BDF8]"
                />
            </div>

            <div class="min-w-0">

                <p
                    class="text-[11px] font-semibold uppercase tracking-wide text-slate-500 dark:text-slate-400"
                >
                    {stat.title}
                </p>

                <h3
                    class="mt-1 text-2xl font-bold text-slate-800 dark:text-white"
                >
                    {stat.value}
                </h3>

            </div>

        </div>

    {/each}

</div>