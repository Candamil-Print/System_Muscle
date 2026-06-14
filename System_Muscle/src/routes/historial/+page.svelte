<script lang="ts">
	import { Toaster } from "svelte-sonner";
	import { onMount } from "svelte";

	import Header from "$lib/components/layout/Header.svelte";
	import Sidebar from "$lib/components/layout/Sidebar.svelte";

	import HistorialHeader from "$lib/components/historial/HistorialHeader.svelte";
	import HistorialStats from "$lib/components/historial/HistorialStats.svelte";
	import HistorialFilters from "$lib/components/historial/HistorialFilters.svelte";
	import HistorialTable from "$lib/components/historial/HistorialTable.svelte";
	import HistorialShiftInfo from "$lib/components/historial/HistorialShiftInfo.svelte";

	import {
		listarHistorialConDetalle
	} from "$lib/services/api/history";

	import type { HistorialDetalle } from "$lib/services/api/history/history.types";

	let filtros = {
		search: '',
		turno: '',
		fecha: ''
	};

	type HistorialTabla = {
		id_historial: number;
		accion: string;
		detalle: string;
		usuario: string;
		turno: string;
		fecha: string;
	};

	let historialOriginal: HistorialTabla[] = [];
	let historial: HistorialTabla[] = [];

	onMount(async () => {
		try {
			const data = await listarHistorialConDetalle({});

			historialOriginal = data.map((item) => ({
				id_historial: item.id_historial,
				accion: item.accion,
				detalle: item.descripcion ?? "",
				usuario:
					item.usuario ??
					`Usuario #${(item as any).id_usuario ?? "-"}`,
				turno:
					item.turno ??
					`Turno #${(item as any).id_turno ?? "-"}`,
				fecha: `${item.fecha} ${item.hora}`
			}));

			historial = [...historialOriginal];
		}
		catch (error) {
			console.error(
				"Error cargando historial:",
				error
			);
		}
	});

	function handleFilter(
		event: CustomEvent<{
			search: string;
			turno: string;
			fecha: string;
		}>
	) {
		filtros = event.detail;

		const {
			search,
			turno,
			fecha
		} = filtros;

		historial = historialOriginal.filter(
			(item) => {
				const textoBusqueda =
					search.toLowerCase();

				const matchSearch =
					!search ||
					item.accion
						.toLowerCase()
						.includes(textoBusqueda) ||
					item.detalle
						.toLowerCase()
						.includes(textoBusqueda) ||
					item.usuario
						.toLowerCase()
						.includes(textoBusqueda);

				const matchTurno =
					!turno ||
					item.turno === turno;

				const matchFecha =
					!fecha ||
					item.fecha.startsWith(
						fecha
					);

				return (
					matchSearch &&
					matchTurno &&
					matchFecha
				);
			}
		);
	}
</script>

<Toaster
	position="top-center"
	theme="dark"
	toastOptions={{
		class: '!bg-[#1E293B] !border !border-[#334156] !text-white'
	}}
/>

<div class="flex min-h-screen bg-slate-50 dark:bg-[#111827]">
	<Sidebar />

	<div class="ml-70 flex flex-1 flex-col">
		<Header />

		<main class="space-y-6 p-6">
			<HistorialHeader />

			<HistorialStats
				{filtros}
			/>

			<HistorialFilters
				on:filter={handleFilter}
			/>

			<HistorialTable
				{historial}
				{filtros}
			/>

			<HistorialShiftInfo />
		</main>
	</div>
</div>