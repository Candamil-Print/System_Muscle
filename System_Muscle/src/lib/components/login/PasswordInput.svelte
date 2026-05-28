<script lang="ts">
	import {
		Lock,
		Eye,
		EyeOff
	} from 'lucide-svelte';

	export let value = '';

	let showPassword = false;
</script>

<div>
	<label
		class="mb-2 block text-sm font-medium text-zinc-700 dark:text-zinc-300"
	>
		Contraseña
	</label>

	<div class="relative">
		<div
			class="absolute left-4 top-1/2 -translate-y-1/2 text-zinc-400 dark:text-zinc-500"
		>
			<Lock size={18} />
		</div>

		<input
			bind:value
			type={showPassword ? 'text' : 'password'}
			placeholder="Tu contraseña"
			class="w-full rounded-2xl
			border border-zinc-200
			bg-zinc-100
			py-3 pl-12 pr-12
			text-sm text-zinc-900
			outline-none
			transition-all

			placeholder:text-zinc-400

			focus:border-sky-500
			focus:ring-4
			focus:ring-sky-500/10

			dark:border-white/10
			dark:bg-white/5
			dark:text-white
			dark:placeholder:text-zinc-500"
		/>

		<button
			type="button"
			class="absolute right-4 top-1/2 -translate-y-1/2 text-zinc-400 transition-colors hover:text-zinc-700 dark:text-zinc-500 dark:hover:text-white"
			on:click={() => showPassword = !showPassword}
		>
			{#if showPassword}
				<EyeOff size={18} />
			{:else}
				<Eye size={18} />
			{/if}
		</button>
	</div>
</div>