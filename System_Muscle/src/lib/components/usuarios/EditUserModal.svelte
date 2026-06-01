<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { toast } from 'svelte-sonner';
	import { X, House, Mail } from 'lucide-svelte';

	import type { Usuario } from '$lib/services/api/users/users.types';

	export let open = false;
	export let user: Usuario | null = null;
	export let onSave: (data: {
		direccion?: string;
		telefono?: string;
		correo?: string;
	}) => Promise<void>;

	let loading = false;

    let telefonoVisual = '';

    let loaded = false;

	const dispatch = createEventDispatcher();

	let form = {
        nombre_completo: '',
        tipo_documento: '',
        numero_documento: '',
        direccion: '',
        tipo_sangre: '',
        eps: '',
        genero: '',
        telefono: '',
        correo: ''
    };


    $: if (open && user && !loaded) {
    loaded = true;

    form = {
        nombre_completo: user.nombre_completo ?? '',
        tipo_documento: user.tipo_documento ?? 'CC',
        numero_documento: user.numero_documento ?? '',
        direccion: user.direccion ?? '',
        tipo_sangre: user.tipo_sangre ?? 'O+',
        eps: user.eps ?? '',
        genero: user.genero ?? 'MASCULINO',
        telefono: (user.telefono ?? '').replace(/\D/g, ''),
        correo: user.correo ?? ''
    };

        telefonoVisual = formatearTelefonoColombia(form.telefono);
    }

    $: if (!open) {
        loaded = false;
    }



    async function guardarCambios() {

        // No hubo cambios
        const huboCambios =
            form.direccion !== (user?.direccion ?? '') ||
            form.telefono !== (user?.telefono ?? '') ||
            form.correo !== (user?.correo ?? '');

        if (!huboCambios) {
            toast.error('Debes modificar al menos un campo');
            return;
        }

        // Dirección obligatoria
        if (!form.direccion.trim()) {
            toast.error('La dirección es obligatoria');
            return;
        }

        // Dirección mínima
        if (form.direccion.trim().length < 5) {
            toast.error(
                'La dirección debe tener mínimo 5 caracteres'
            );
            return;
        }

        // Teléfono obligatorio
        if (!form.telefono.trim()) {
            toast.error('El teléfono es obligatorio');
            return;
        }

        // Solo números
        const telefonoLimpio =
            form.telefono.replace(/\D/g, '');

        if (telefonoLimpio.length !== 10) {
            toast.error(
                'Ingresa un número de teléfono válido'
            );
            return;
        }

        // Correo obligatorio
        if (!form.correo.trim()) {
            toast.error('El correo es obligatorio');
            return;
        }

        // Validación correo
        const emailRegex =
            /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

        if (!emailRegex.test(form.correo)) {
            toast.error(
                'Ingresa un correo válido. Ej: ejemplo@correo.com'
            );
            return;
        }

        try {
            loading = true;

            await onSave(form);

            toast.success(
                'Usuario actualizado correctamente'
            );

            dispatch('updated');
            dispatch('close');
        } catch (error) {
            console.error(error);

            toast.error(
                typeof error === 'string'
                    ? error
                    : 'Error al actualizar usuario'
            );
        } finally {
            loading = false;
        }
    }

    function formatearTelefonoColombia(valor: string) {
        const numeros = valor.replace(/\D/g, '').slice(0, 10);

        if (numeros.length <= 3) return numeros;

        if (numeros.length <= 6) {
            return `${numeros.slice(0, 3)} ${numeros.slice(3)}`;
        }

        return `${numeros.slice(0, 3)} ${numeros.slice(3, 6)} ${numeros.slice(6)}`;
    }

    function manejarTelefono(event: Event) {
        const input = event.target as HTMLInputElement;

        form.telefono = input.value.replace(/\D/g, '').slice(0, 10);

        telefonoVisual = formatearTelefonoColombia(form.telefono);
    }
</script>

{#if open && user}

	<!-- OVERLAY -->
	<div
		class="fixed left-0 top-0 z-[9998] h-dvh w-dvw bg-black/50 backdrop-blur-sm"
	></div>

	<!-- MODAL -->
	<div
		class="fixed left-0 top-0 z-[9999] flex h-dvh w-dvw items-center justify-center p-4"
	>
		<div
			class="animate-in fade-in zoom-in duration-200 relative w-full max-w-xl rounded-3xl border border-slate-200 bg-white shadow-2xl"
		>

			<!-- HEADER -->
			<div
				class="sticky top-0 rounded-t-3xl border-b border-slate-100 bg-white p-6"
			>
				<div class="flex items-start justify-between">

					<div>
						<h2 class="text-2xl font-bold text-slate-800">
							Editar Usuario
						</h2>

						<p class="mt-1 text-sm text-slate-500">
							Modifica la información de contacto del usuario
						</p>
					</div>

					<button
						type="button"
						class="rounded-xl p-2 text-slate-400 transition hover:bg-slate-100 hover:text-slate-700"
						on:click={() => dispatch('close')}
					>
						<X size={20} />
					</button>

				</div>
			</div>

			<!-- BODY -->
			<div class="max-h-[70vh] overflow-y-auto p-8">

				<!-- PREVIEW USUARIO -->
				<div
					class="mb-6 flex items-center gap-4 rounded-2xl border border-slate-200 bg-slate-50 p-5"
				>

					<div
						class="flex h-14 w-14 items-center justify-center rounded-full bg-[#0C4A6E] font-semibold text-white"
					>
						{user.nombre_completo
							.split(' ')
							.map((n) => n[0])
							.slice(0, 2)
							.join('')}
					</div>

					<div class="flex-1">

						<p class="font-medium text-slate-800">
							{user.nombre_completo}
						</p>

						<span
							class="mt-1 inline-block rounded-full bg-slate-200 px-2 py-1 text-xs font-medium text-slate-700"
						>
							{user.id_rol === 1
								? 'Administrador'
								: 'Recepcionista'}
						</span>

					</div>

				</div>

                <!-- DIRECCIÓN -->
                <div class="mb-5">

                    <label
                        class="mb-2 block text-sm font-semibold text-slate-700"
                    >
                        Dirección Domiciliaria
                    </label>

                    <div
                        class="flex overflow-hidden rounded-xl border border-slate-200 focus-within:border-[#0C4A6E] focus-within:ring-4 focus-within:ring-sky-100"
                    >

                        <div
                            class="flex items-center border-r border-slate-200 bg-slate-50 px-4 text-slate-600"
                        >
                            <House size={18} />
                        </div>

                        <input
                            bind:value={form.direccion}
                            placeholder="Ingrese dirección"
                            class="w-full px-4 py-3 text-sm outline-none"
                        />

                    </div>

                </div>


                <!-- TELÉFONO -->
                <div class="mb-5">

                    <label
                        class="mb-2 block text-sm font-semibold text-slate-700"
                    >
                        Teléfono
                    </label>

                    <div
                        class="flex overflow-hidden rounded-xl border border-slate-200 focus-within:border-[#0C4A6E] focus-within:ring-4 focus-within:ring-sky-100"
                    >

                        <div
                            class="flex items-center border-r border-slate-200 bg-slate-50 px-4 text-sm font-medium text-slate-600"
                        >
                            +57
                        </div>

                        <input
                            bind:value={telefonoVisual}
                            on:input={manejarTelefono}
                            inputmode="numeric"
                            placeholder="300 123 4567"
                            class="w-full px-4 py-3 text-sm outline-none"
                        />

                    </div>

                </div>

                <!-- CORREO -->
                <div>

                    <label
                        class="mb-2 block text-sm font-semibold text-slate-700"
                    >
                        Correo Electrónico
                    </label>

                    <div
                        class="flex overflow-hidden rounded-xl border border-slate-200 focus-within:border-[#0C4A6E] focus-within:ring-4 focus-within:ring-sky-100"
                    >

                        <div
                            class="flex items-center border-r border-slate-200 bg-slate-50 px-4 text-slate-600"
                        >
                            <Mail size={18} />
                        </div>

                        <input
                            type="email"
                            bind:value={form.correo}
                            placeholder="correo@ejemplo.com"
                            class="w-full px-4 py-3 text-sm outline-none"
                        />

                    </div>

                </div>

			</div>

			<!-- FOOTER -->
			<div
				class="flex justify-end gap-3 border-t border-slate-200 px-8 py-5"
			>

				<button
					on:click={() => dispatch('close')}
					class="rounded-xl border border-slate-200 px-5 py-2.5 text-sm font-medium text-slate-700 transition hover:bg-slate-100"
				>
					Cancelar
				</button>

				<button
					on:click={guardarCambios}
					disabled={loading}
					class="rounded-xl bg-[#0C4A6E] px-5 py-2.5 text-sm font-medium text-white transition hover:bg-[#0a3a52] disabled:opacity-50"
				>
					{#if loading}
						Guardando...
					{:else}
						Guardar Cambios
					{/if}
				</button>

			</div>

		</div>
	</div>

{/if}

