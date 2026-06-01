<script lang="ts">
	import { onMount } from 'svelte';

	import {
		Package,
		AlertTriangle,
		ShoppingCart,
		DollarSign
	} from 'lucide-svelte';

	import {
		obtenerDashboardResumen,
		type DashboardResumen
	} from '$lib/services/api/dashboard';

	import {
		listarProductos,
		type ProductoConStock
	} from '$lib/services/api/inventory';

	let resumen: DashboardResumen | null = null;
	let productos: ProductoConStock[] = [];

	let cargando = true;

	onMount(async () => {
		try {
			const [dashboardData, productosData] =
				await Promise.all([
					obtenerDashboardResumen(),
					listarProductos()
				]);

			resumen = dashboardData;
			productos = productosData;
		} catch (error) {
			console.error(
				'Error cargando dashboard:',
				error
			);
		} finally {
			cargando = false;
		}
	});
</script>

<div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-4">

	<!-- TOTAL PRODUCTOS -->
	<div
		class="rounded-lg border border-zinc-200 bg-white p-6 dark:border-zinc-800 dark:bg-zinc-900"
	>
		<div class="flex items-start justify-between">

			<div>
				<p class="text-sm text-zinc-600 dark:text-zinc-400">
					Total Productos
				</p>

				<p
					class="mt-2 text-3xl font-bold text-zinc-900 dark:text-white"
				>
					{cargando ? '...' : productos.length}
				</p>

				<p class="mt-1 text-xs text-zinc-500">
					productos registrados
				</p>
			</div>

			<Package
				class="text-zinc-400"
				size={18}
			/>

		</div>
	</div>

	<!-- STOCK BAJO -->
	<div
		class="rounded-lg border border-zinc-200 bg-white p-6 dark:border-zinc-800 dark:bg-zinc-900"
	>
		<div class="flex items-start justify-between">

			<div>
				<p class="text-sm text-zinc-600 dark:text-zinc-400">
					Stock Bajo
				</p>

				<p class="mt-2 text-3xl font-bold">
					{cargando
						? '...'
						: resumen?.productos_stock_bajo ?? 0}
				</p>

				<p class="mt-1 text-xs text-zinc-500">
					requieren reposición
				</p>
			</div>

			<AlertTriangle
				class="text-zinc-400"
				size={18}
			/>

		</div>
	</div>

	<!-- VENTAS HOY -->
	<div
		class="rounded-lg border border-zinc-200 bg-white p-6 dark:border-zinc-800 dark:bg-zinc-900"
	>
		<div class="flex items-start justify-between">

			<div>
				<p class="text-sm text-zinc-600 dark:text-zinc-400">
					Ventas Hoy
				</p>

				<p
					class="mt-2 text-3xl font-bold text-zinc-900 dark:text-white"
				>
					{cargando
						? '...'
						: resumen?.ventas_hoy ?? 0}
				</p>

				<p class="mt-1 text-xs text-zinc-500">
					transacciones realizadas
				</p>
			</div>

			<ShoppingCart
				class="text-zinc-400"
				size={18}
			/>

		</div>
	</div>

	<!-- INGRESOS HOY -->
	<div
		class="rounded-lg border border-zinc-200 bg-white p-6 dark:border-zinc-800 dark:bg-zinc-900"
	>
		<div class="flex items-start justify-between">

			<div>
				<p class="text-sm text-zinc-600 dark:text-zinc-400">
					Ingresos Hoy
				</p>

				<p
					class="mt-2 text-3xl font-bold text-[#0C4A6E]"
				>
					$
					{cargando
						? '...'
						: (resumen?.total_vendido_hoy ?? 0).toLocaleString(
								'es-CO'
							)}
				</p>

				<p class="mt-1 text-xs text-zinc-500">
					en ventas del día
				</p>
			</div>

			<DollarSign
				class="text-[#0C4A6E]"
				size={18}
			/>

		</div>
	</div>

</div>