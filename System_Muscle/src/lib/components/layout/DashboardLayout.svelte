<script lang="ts">
	import Sidebar from './Sidebar.svelte';
	import Header from './Header.svelte';
</script>

<div class="min-h-screen bg-[#f9fafb] dark:bg-[#020617]">

	<Sidebar />

	<div class="ml-[280px]">

		<Header />

		<main class="p-6">
			<slot />
		</main>
	</div>
</div>