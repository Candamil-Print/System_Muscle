<script lang="ts">
    import { AlertTriangle } from 'lucide-svelte';

    const alerts = [
        {
            name: 'Bebida Energética',
            category: 'Bebidas',
            current: 8,
            total: 80
        },
        {
            name: 'Creatina Monohidrato',
            category: 'Suplementos',
            current: 5,
            total: 30
        },
        {
            name: 'Creatina Monohidrato',
            category: 'Suplementos',
            current: 5,
            total: 30
        }
    ];
</script>

<div
	class="rounded-lg border border-zinc-200 bg-white p-6 dark:border-zinc-800 dark:bg-zinc-900"
>
	<div class="flex items-center gap-2 mb-2">
		<AlertTriangle size={20} class="text-[#0C4A6E] dark:text-zinc-300" />
		<h3
			class="text-lg font-semibold text-[#0C4A6E] dark:text-white"
		>
			Alertas de Stock
		</h3>
	</div>

	<p
		class="mb-6 text-sm text-zinc-500 dark:text-zinc-400"
	>
		Productos que requieren reposición
	</p>

	<div class="space-y-3">
		{#each alerts as alert}
			<div
				class="flex items-center justify-between rounded-lg bg-[#0C4A6E]/10 p-4 dark:bg-zinc-800"
			>
				<div>
					<h4
						class="font-semibold text-[#0C4A6E] dark:text-white"
					>
						{alert.name}
					</h4>

					<p
						class="text-sm text-zinc-500 dark:text-zinc-400"
					>
						{alert.category}
					</p>
				</div>

				<div class="text-right">
					<p class="font-semibold text-[#0C4A6E] dark:text-white">
						{alert.current} / {alert.total}
					</p>

					<p
						class="text-xs text-zinc-500 dark:text-zinc-400"
					>
						unidades
					</p>
				</div>
			</div>
		{/each}
	</div>
</div>