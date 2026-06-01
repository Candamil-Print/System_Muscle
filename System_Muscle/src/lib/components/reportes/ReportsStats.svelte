<script lang="ts">
	import { onMount } from 'svelte';

	import DollarSign from 'lucide-svelte/icons/dollar-sign';
	import Moon from 'lucide-svelte/icons/moon';
	import ArrowRightLeft from 'lucide-svelte/icons/arrow-right-left';
	import Info from 'lucide-svelte/icons/info';
	import Briefcase from 'lucide-svelte/icons/briefcase';

	import {
		obtenerResumenVentasRango,
		obtenerReporteCajas
	} from '$lib/services/api/reports';

	interface Stat {
		icon: 'DollarSign' | 'Moon' | 'ArrowRightLeft' | 'Info' | 'Briefcase';
		title: string;
		value: string | number;
	}

	let stats: Stat[] = [];

	const icons = {
		DollarSign,
		Moon,
		ArrowRightLeft,
		Info,
		Briefcase
	};

	onMount(async () => {
		try {
			const hoy = new Date();

			const fechaInicio =
				`${hoy.getFullYear()}-${String(hoy.getMonth() + 1).padStart(2, '0')}-01`;

			const fechaFin =
				`${hoy.getFullYear()}-${String(hoy.getMonth() + 1).padStart(2, '0')}-${String(hoy.getDate()).padStart(2, '0')}`;

			const resumen = await obtenerResumenVentasRango(
				fechaInicio,
				fechaFin
			);

			const cajas = await obtenerReporteCajas(
				fechaInicio,
				fechaFin
			);

			const cajaInicial = cajas.reduce(
				(total, caja) => total + caja.monto_apertura,
				0
			);

			const cajaFinal = cajas.reduce(
				(total, caja) => total + (caja.monto_cierre || 0),
				0
			);

			stats = [
				{
					icon: 'Briefcase',
					title: 'TOTAL VENTAS',
					value: resumen.numero_ventas
				},
				{
					icon: 'DollarSign',
					title: 'TOTAL EFECTIVO',
					value: `$ ${resumen.total_efectivo.toLocaleString('es-CO')}`
				},
				{
					icon: 'ArrowRightLeft',
					title: 'TRANSFERENCIAS',
					value: `$ ${resumen.total_transferencia.toLocaleString('es-CO')}`
				},
				{
					icon: 'Info',
					title: 'CAJA INICIAL',
					value: `$ ${cajaInicial.toLocaleString('es-CO')}`
				},
				{
					icon: 'Moon',
					title: 'CAJA FINAL',
					value: `$ ${cajaFinal.toLocaleString('es-CO')}`
				},
				{
					icon: 'DollarSign',
					title: 'CAJA TOTAL',
					value: `$ ${(cajaFinal + resumen.total_transferencia).toLocaleString('es-CO')}`
				}
			];
		} catch (error) {
			console.error('Error cargando estadísticas:', error);
		}
	});
</script>

<div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-6">

	{#each stats as stat}

		<div
			class="flex items-center gap-4 rounded-2xl border border-slate-200 bg-white p-4 transition-all duration-300 hover:shadow-md"
		>

			<div
				class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-slate-100"
			>
				<svelte:component
					this={icons[stat.icon]}
					class="h-5 w-5 text-[#0C4A6E]"
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