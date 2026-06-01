<script lang="ts">
	import { User } from 'lucide-svelte';
	import { toast } from 'svelte-sonner';
	import { goto } from '$app/navigation';

	import PasswordInput from './PasswordInput.svelte';

	import { login } from '$lib/services/api/login';

	let document = '';
	let password = '';
	let loading = false;

	const handleSubmit = async () => {
		// Documento vacío
		if (!document.trim()) {
			toast.warning(
				'Debes ingresar tu número de documento'
			);
			return;
		}

		// Solo números
		if (!/^\d+$/.test(document.trim())) {
			toast.error(
				'El documento solo puede contener números'
			);
			return;
		}

		// Longitud mínima
		if (document.trim().length < 5) {
			toast.error(
				'El número de documento no es válido'
			);
			return;
		}

		// Contraseña vacía
		if (!password.trim()) {
			toast.warning(
				'Debes ingresar tu contraseña'
			);
			return;
		}

		// Contraseña muy corta
		if (password.length < 4) {
			toast.error(
				'La contraseña es demasiado corta'
			);
			return;
		}

		try {
			loading = true;

			const sesion = await login({
				documento: document.trim(),
				password
			});

			// Login inválido
			if (!sesion) {
				toast.error(
					'Documento o contraseña incorrectos'
				);
				return;
			}

			// Usuario deshabilitado
			if (sesion.estado !== 1) {
				toast.error(
					'Tu usuario se encuentra deshabilitado'
				);
				return;
			}

			localStorage.setItem(
				'sesion',
				JSON.stringify(sesion)
			);

			toast.success(
				`Bienvenido ${sesion.nombre_completo}`
			);

			await goto('/dashboard');

		} catch (error: any) {
			console.error(error);

			const message =
				typeof error === 'string'
					? error
					: error?.toString?.() ?? '';

			// Mensajes específicos
			if (
				message.includes(
					'El número de documento es obligatorio'
				)
			) {
				toast.error(
					'Debes ingresar tu documento'
				);
			}
			else if (
				message.includes(
					'La contraseña es obligatoria'
				)
			) {
				toast.error(
					'Debes ingresar tu contraseña'
				);
			}
			else if (
				message.includes('database')
			) {
				toast.error(
					'Error de conexión con la base de datos'
				);
			}
			else {
				toast.error(
					'No fue posible iniciar sesión'
				);
			}
		} finally {
			loading = false;
		}
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
		disabled={loading}
		class="w-full rounded-2xl
		bg-gradient-to-r from-sky-900 to-sky-500
		py-3 text-sm font-semibold text-white

		shadow-lg shadow-sky-900/20
		transition-all duration-300

		hover:-translate-y-0.5

		disabled:cursor-not-allowed
		disabled:opacity-50

		dark:shadow-sky-500/10"
	>
		{#if loading}
		Verificando credenciales...
		{:else}
			Ingresar
		{/if}
	</button>
</form>