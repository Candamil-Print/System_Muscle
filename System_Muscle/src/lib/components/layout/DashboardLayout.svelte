<script lang="ts">
	import { onMount } from 'svelte';

	import Sidebar from './Sidebar.svelte';
	import Header from './Header.svelte';

	import ModalAperturaCaja from '$lib/components/ventas/ModalAperturaCaja.svelte';

	import {
		obtenerCajaActiva,
		abrirCaja,
		obtenerTurnoActivoGeneral
	} from '$lib/services/api/shifts';

	let mostrarModalCaja = false;

	async function verificarCaja() {

		try {

			const caja =
				await obtenerCajaActiva();

			console.log(
				'Caja activa:',
				caja
			);

			if (!caja) {
				mostrarModalCaja = true;
			}

		} catch (error) {

			console.error(
				'Error verificando caja:',
				error
			);

			mostrarModalCaja = true;
		}
	}

	onMount(async () => {
		await verificarCaja();
	});

	async function confirmarApertura(
		monto: number
	) {

		try {

			const turno: any =
				await obtenerTurnoActivoGeneral();

			console.log(
				'Turno activo:',
				turno
			);

			if (!turno) {

				alert(
					'No hay un turno activo'
				);

				return;
			}

			await abrirCaja(
				turno.id_turno,
				monto
			);

			mostrarModalCaja = false;

		} catch (error) {

			console.error(
				'Error abriendo caja:',
				error
			);

			alert(
				'No fue posible abrir la caja'
			);
		}
	}
</script>

<div class="min-h-screen bg-[#f9fafb] dark:bg-[#111827]">

	<Sidebar />

	<div class="ml-[280px]">

		<Header />

		<main class="p-6">
			<slot />
		</main>

	</div>

	<ModalAperturaCaja
		open={mostrarModalCaja}
		onConfirm={confirmarApertura}
	/>

</div>