<script lang="ts">
    import { BarChart3 } from 'lucide-svelte';
    import { onMount } from 'svelte';
    import Chart from 'chart.js/auto';
    import type { ChartOptions } from 'chart.js';

    let chartContainer: HTMLCanvasElement;

    const data = {
        labels: ['Lun', 'Mar', 'Mié', 'Jue', 'Vie', 'Sab', 'Dom'],
        datasets: [
            {
                label: 'Ventas ($)',
                data: [400, 650, 500, 900, 750, 1200, 950],
                borderColor: '#0C4A6E',
                backgroundColor: 'rgba(12, 74, 110, 0.1)',
                fill: true,
                tension: 0.4,
                pointRadius: 5,
                pointBackgroundColor: '#0C4A6E'
            }
        ]
    };

    const options: ChartOptions<'line'> = {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
            legend: {
                display: true
            }
        }
    };

    onMount(() => {
        if (chartContainer) {
            const ctx = chartContainer.getContext('2d');
            if (ctx) {
                new Chart(ctx, {
                    type: 'line',
                    data,
                    options
                });
            }
        }
    });
</script>

<div
    class="rounded-lg border border-zinc-200 bg-white p-6 dark:border-zinc-800 dark:bg-zinc-900"
>
    <div class="flex items-center gap-2 mb-2">
        <BarChart3 size={20} class="text-zinc-700 dark:text-zinc-300" />
        <h3
            class="text-lg font-semibold text-zinc-900 dark:text-white"
        >
            Ventas Recientes
        </h3>
    </div>

    <p
        class="mb-6 text-sm text-zinc-500 dark:text-zinc-400"
    >
        Últimas transacciones realizadas
    </p>

    <div class="h-[300px]">
        <canvas bind:this={chartContainer}></canvas>
    </div>
</div>