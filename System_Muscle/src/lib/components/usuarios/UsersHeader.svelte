<script lang="ts">
	import { UserPlus } from 'lucide-svelte';
	import CreateUserModal from '$lib/components/usuarios/CreateUserModal.svelte';

	export let onReload: () => void;

	let createModalOpen = false;
</script>

<div
	class="flex flex-col gap-4 md:flex-row md:items-start md:justify-between"
>
	<div>
		<h1 class="text-3xl font-bold text-slate-800 dark:text-white">
			Usuarios
		</h1>

		<p class="mt-1 text-sm text-slate-500">
			Gestión de usuarios del sistema
		</p>
	</div>

	<button
		on:click={() => (createModalOpen = true)}
		class=" flex items-center gap-2 rounded-lg bg-[#0C4A6E] dark:text-[#39BDF8] px-4 py-2.5 text-sm font-medium text-white hover:bg-[#0a3a52]"
	>
		<UserPlus size={16} />
		Nuevo Usuario
	</button>
</div>

<CreateUserModal
	open={createModalOpen}
	on:close={() => (createModalOpen = false)}
	on:created={onReload}
/>