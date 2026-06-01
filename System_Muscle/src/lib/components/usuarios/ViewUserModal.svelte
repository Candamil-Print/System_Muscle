<script lang="ts">
	import { createEventDispatcher } from 'svelte';
    import { X } from 'lucide-svelte';
	import type { Usuario } from '$lib/services/api/users/users.types';

	export let open = false;
	export let user: Usuario | null = null;

	const dispatch = createEventDispatcher();

	function cerrar() {
		dispatch('close');
	}
</script>

{#if open && user}

<div
	class="fixed left-0 top-0 z-[9998] h-dvh w-dvw bg-black/50 backdrop-blur-sm"
></div>

<div
	class="fixed left-0 top-0 z-[9999] flex h-dvh w-dvw items-center justify-center p-4"
>
	<div
		class="animate-in fade-in zoom-in duration-200 relative w-full max-w-xl rounded-3xl border border-slate-200 bg-white shadow-2xl"
	>

    <div
        class="sticky top-0 rounded-t-3xl border-b border-slate-100 bg-white p-6"
    >
        <div class="flex items-start justify-between">

            <div>
                <h2 class="text-2xl font-bold text-slate-800">
                    Detalle del Usuario
                </h2>

                <p class="mt-1 text-sm text-slate-500">
                    Información completa del usuario
                </p>
            </div>

            <button
                type="button"
                class="rounded-xl p-2 text-slate-400 transition hover:bg-slate-100 hover:text-slate-700"
                on:click={cerrar}
            >
                <X size={20} />
            </button>

        </div>
    </div>

	<div class="max-h-[70vh] overflow-y-auto p-8">

		<div class="mb-6 flex flex-col items-center gap-4">

			<div
				class="flex h-20 w-20 items-center justify-center rounded-full bg-gradient-to-br from-[#0C4A6E] to-[#0284C7] font-semibold text-white text-2xl"
			>
				{user.nombre_completo
					.split(' ')
					.map((n: string) => n[0])
					.slice(0, 2)
					.join('')}
			</div>

			<div class="text-center">

				<h3 class="text-xl font-bold text-slate-800">
					{user.nombre_completo}
				</h3>

				<div class="mt-2 flex justify-center gap-2">

					<span
						class="rounded-full bg-[#1c5476]/10 text-[#1c5476] px-3 py-1 text-xs font-medium"
					>
						{user.id_rol === 1
							? 'Administrador'
							: 'Recepcionista'}
					</span>

					<span
						class={`rounded-full px-3 py-1 text-xs font-medium ${
							user.estado === 1
								? 'bg-[#85A4B6] text-white'
								: 'bg-slate-300 text-white'
						}`}
					>
						{user.estado === 1
							? 'Activo'
							: 'Inactivo'}
					</span>

				</div>

			</div>

		</div>

        <div
            class="mb-6 rounded-2xl border border-slate-200 bg-white p-5"
        >

			<div class="flex justify-between py-2">
				<span class="text-slate-500">
					Documento
				</span>

				<span class="font-medium">
					{user.tipo_documento}
					{user.numero_documento}
				</span>
			</div>

			<div class="flex justify-between py-2">
				<span class="text-slate-500">
					Correo
				</span>

				<span class="font-medium">
					{user.correo || '-'}
				</span>
			</div>

			<div class="flex justify-between py-2">
				<span class="text-slate-500">
					Teléfono
				</span>

				<span class="font-medium">
					{user.telefono || '-'}
				</span>
			</div>

			<div class="flex justify-between py-2">
				<span class="text-slate-500">
					Dirección
				</span>

				<span class="font-medium">
					{user.direccion || '-'}
				</span>
			</div>

		</div>

		<div class="grid grid-cols-3 gap-4">

			<div
				class="rounded-xl bg-slate-100 p-4 text-center"
			>
				<p class="text-xs text-slate-500">
					Tipo Sangre
				</p>

				<p class="text-lg font-bold">
					{user.tipo_sangre || '-'}
				</p>
			</div>

			<div
				class="rounded-xl bg-slate-100 p-4 text-center"
			>
				<p class="text-xs text-slate-500">
					EPS
				</p>

				<p class="text-lg font-bold">
					{user.eps || '-'}
				</p>
			</div>

			<div
				class="rounded-xl bg-slate-100 p-4 text-center"
			>
				<p class="text-xs text-slate-500">
					Género
				</p>

				<p class="text-lg font-bold">
					{user.genero || '-'}
				</p>
			</div>

		</div>

	</div>

	<div
	    class="flex justify-end gap-3 border-t border-slate-200 px-8 py-5"
    >

		<button
            on:click={cerrar}
            class="rounded-xl border border-slate-200 px-5 py-2.5 text-sm font-medium text-slate-700 transition hover:bg-slate-100"
        >
            Cerrar
        </button>

	</div>

</div>


</div>

{/if}
