<script lang="ts">
	import DashboardStatCard from './DashboardStatCard.svelte';
</script>

<div class="grid grid-cols-1 gap-4 lg:grid-cols-2">

	<DashboardStatCard
		title="Total Usuarios"
		value="2"
		description="usuarios registrados"
	/>

	<DashboardStatCard
		title="Usuarios Activos"
		value="2"
		description="usuarios habilitados"
		valueColor="text-[#0C4A6E]"
	/>

</div>