<script lang="ts">
	import { onMount } from 'svelte';

	import DashboardStatCard from './DashboardStatCard.svelte';

	import { listarUsuarios } from '$lib/services/api/users';

	let totalUsuarios = 0;
	let usuariosActivos = 0;

	onMount(async () => {
		try {
			const usuarios = await listarUsuarios();

			totalUsuarios = usuarios.length;

			usuariosActivos = usuarios.filter(
				(usuario) => usuario.estado === 1
			).length;

		} catch (error) {
			console.error(
				'Error cargando usuarios:',
				error
			);
		}
	});
</script>

<div class="grid grid-cols-1 gap-4 lg:grid-cols-2">

	<DashboardStatCard
		title="Total Usuarios"
		value={String(totalUsuarios)}
		description="usuarios registrados"
	/>

	<DashboardStatCard
		title="Usuarios Activos"
		value={String(usuariosActivos)}
		description="usuarios habilitados"
		valueColor="text-[#0C4A6E] dark:text-[#39BDF8]"
	/>

</div>