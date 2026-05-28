<script lang="ts">
	import { User } from 'lucide-svelte';

	import PasswordInput from './PasswordInput.svelte';

	let document = '';
	let password = '';

	const handleSubmit = () => {
		console.log({
			document,
			password
		});
	};
</script>

<form
	class="space-y-5"
	on:submit|preventDefault={handleSubmit}
>
	<div>
		<label
			class="mb-2 block text-sm font-medium text-zinc-700 dark:text-zinc-300"
		>
			Documento
		</label>

		<div class="relative">
			<div
				class="absolute left-4 top-1/2 -translate-y-1/2 text-zinc-400 dark:text-zinc-500"
			>
				<User size={18} />
			</div>

			<input
				bind:value={document}
				type="text"
				placeholder="Número de documento"
				class="w-full rounded-2xl
				border border-zinc-200
				bg-zinc-100
				py-3 pl-12 pr-4
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
		</div>
	</div>

	<PasswordInput bind:value={password} />

	<button
		type="submit"
		class="w-full rounded-2xl
		bg-gradient-to-r from-sky-900 to-sky-500
		py-3 text-sm font-semibold text-white

		shadow-lg shadow-sky-900/20
		transition-all duration-300

		hover:-translate-y-0.5

		dark:shadow-sky-500/10"
	>
		Ingresar
	</button>
</form>