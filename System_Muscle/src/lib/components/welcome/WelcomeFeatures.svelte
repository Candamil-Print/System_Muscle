<script lang="ts">
	import {
		Users,
		Package,
		BadgeDollarSign
	} from 'lucide-svelte';

	const features = [
		{
			icon: Users,
			text: 'Gestión de Clientes'
		},
		{
			icon: Package,
			text: 'Control de Inventario'
		},
		{
			icon: BadgeDollarSign,
			text: 'Ventas y Reportes'
		}
	];
</script>

<div class="mb-14 flex flex-wrap items-center justify-center gap-8">

	{#each features as feature}
		<div class="flex items-center gap-3 text-sm text-zinc-500">

			<div
				class="flex h-10 w-10 items-center justify-center rounded-xl bg-sky-900/10 text-sky-500"
			>
				<svelte:component this={feature.icon} size={18} />
			</div>

			<span>{feature.text}</span>
		</div>
	{/each}
</div>