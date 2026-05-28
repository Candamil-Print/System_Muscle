<script lang="ts">
	import DashboardStats from './DashboardStats.svelte';
	import DashboardUsers from './DashboardUsers.svelte';
	import DashboardChart from './DashboardChart.svelte';
	import DashboardAlerts from './DashboardAlerts.svelte';
</script>

<div class="space-y-6">

	<!-- PAGE HEADER -->
	<div
		class="flex flex-col gap-4 lg:flex-row lg:items-center lg:justify-between"
	>
		<div>
			<h1
				class="text-3xl font-bold text-zinc-900 dark:text-white"
			>
				Dashboard
			</h1>

			<p
				class="text-sm text-zinc-500 dark:text-zinc-400"
			>
				Bienvenido, Administrador
			</p>
		</div>

		<div
			class="rounded-full  bg-[#0C4A6E]/10 px-5 py-2.5 text-sm font-medium text-zinc-700 dark:border-zinc-800 dark:bg-zinc-900 dark:text-zinc-300"
		>
			<p class="text-[#0C4A6E]">Turno Tarde (1pm - 10pm)</p>
		</div>
	</div>

	<DashboardStats />

	<DashboardUsers />

	<div class="grid grid-cols-1 gap-4 xl:grid-cols-2">
		<DashboardChart />
		<DashboardAlerts />
	</div>

</div>