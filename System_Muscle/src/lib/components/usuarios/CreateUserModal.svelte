<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { crearUsuario } from '$lib/services/api/users'
    import { toast } from 'svelte-sonner';
    import { X } from 'lucide-svelte';
    import { Eye, EyeOff } from 'lucide-svelte';


    import UserCreatedSuccessModal
	from './UserCreatedSuccessModal.svelte';

    let successModalOpen = false;
    let showCreateModal = true;

    let mostrarPassword = false;
    let telefonoVisual = '';

	export let open = false;
    let loading = false; 

	const dispatch = createEventDispatcher();

    let form = {
        nombre_completo: '',
        tipo_documento: 'CC',
        numero_documento: '',
        direccion: '',
        tipo_sangre: 'O+',
        eps: '',
        genero: 'MASCULINO',
        correo: '',
        telefono: '',
        password: ''
    };
    
    let documentoVisual = '';

    const initialForm = {
        nombre_completo: '',
        tipo_documento: 'CC',
        numero_documento: '',
        direccion: '',
        tipo_sangre: 'O+',
        eps: '',
        genero: 'MASCULINO',
        correo: '',
        telefono: '',
        password: ''
    };

    function resetForm() {
        form = { ...initialForm };
        documentoVisual = '';
        telefonoVisual = '';
    }

    function cerrarModal() {
        resetForm();
        dispatch('close');
    }

    async function guardarUsuario() {

        if (!form.nombre_completo.trim()) {
            toast.error('El nombre es obligatorio');
            return;
        }

        // Nombre mínimo 3 caracteres
        if (form.nombre_completo.trim().length < 3) {
            toast.error('El nombre debe tener mínimo 3 caracteres');
            return;
        }

        // Documento
        if (!form.numero_documento.trim()) {
            toast.error('El número de documento es obligatorio');
            return;
        }


        if (form.numero_documento.length < 6) {
            toast.error('El número de documento debe tener mínimo 6 dígitos');
            return;
        }

        if (!form.direccion.trim()) {
            toast.error('La dirección es obligatoria');
            return;
        }

        // Dirección mínima 5 caracteres
        if (form.direccion.trim().length < 5) {
            toast.error('La dirección debe tener mínimo 5 caracteres');
            return;
        }

        // EPS obligatoria
        if (!form.eps) {
            toast.error('Selecciona una EPS');
            return;
        }

        // Teléfono colombiano
        if (form.telefono.length !== 10) {
            toast.error('Ingresa un número de teléfono válido');
            return;
        }

        // Correo obligatorio
        if (!form.correo.trim()) {
            toast.error('El correo es obligatorio');
            return;
        }

        // Validación básica de correo
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;

        if (!emailRegex.test(form.correo)) {
            toast.error('Ingresa un correo válido. Ej: ejemplo@correo.com');
            return;
        }

        // Contraseña obligatoria
        if (!form.password.trim()) {
            toast.error('La contraseña es obligatoria');
            return;
        }

        // Contraseña segura
        const passwordRegex =
            /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&.#_-])[A-Za-z\d@$!%*?&.#_-]{8,}$/;

        if (!passwordRegex.test(form.password)) {
            toast.error(
                'La contraseña debe tener mínimo 8 caracteres, una mayúscula, una minúscula, un número y un carácter especial'
            );
            return;
        }

        try {
            loading = true;

            const id = await crearUsuario(form);

            console.log('Usuario creado:', id);

            // ocultar modal crear
            showCreateModal = false;

            // mostrar modal éxito
            successModalOpen = true;

            dispatch('created');
        } catch (error) {
            console.error(error);

            toast.error(
                typeof error === 'string'
                    ? error
                    : 'Error al crear usuario'
            );
        } finally {
            loading = false;
        }
    }
    
    function cerrarSuccessModal() {
        successModalOpen = false;

        resetForm();

        dispatch('close');
    }

    function formatearDocumento(valor: string) {
	const numeros = valor.replace(/\D/g, '');

	return numeros.replace(/\B(?=(\d{3})+(?!\d))/g, '.');
}

    function manejarDocumento(event: Event) {
        const input = event.target as HTMLInputElement;

        // Guardar SOLO números en el form
        form.numero_documento = input.value.replace(/\D/g, '');

        // Mostrar formateado en pantalla
        input.value = formatearDocumento(form.numero_documento);
    }

    function formatearTelefonoColombia(valor: string) {
	const numeros = valor.replace(/\D/g, '').slice(0, 10);

	if (numeros.length <= 3) return numeros;
	if (numeros.length <= 6)
		return `${numeros.slice(0, 3)} ${numeros.slice(3)}`;

        return `${numeros.slice(0, 3)} ${numeros.slice(3, 6)} ${numeros.slice(6)}`;
    }

    function manejarTelefono(event: Event) {
        const input = event.target as HTMLInputElement;

        // Solo guardar números
        form.telefono = input.value.replace(/\D/g, '').slice(0, 10);

        // Mostrar formateado
        telefonoVisual = formatearTelefonoColombia(form.telefono);
    }

</script>

{#if open && showCreateModal}

	<!-- OVERLAY -->
	<div
		class="fixed left-0 top-0 z-[9998] h-dvh w-dvw bg-black/50 backdrop-blur-sm"
	></div>

	<!-- MODAL -->
	<div
		class="fixed left-0 top-0 z-[9999] flex h-dvh w-dvw items-center justify-center p-4"
	>

		<div
			class="animate-in fade-in zoom-in duration-200 relative w-full max-w-xl rounded-3xl border border-slate-200 bg-white dark:bg-[#1E293B] dark:border-[#334156] shadow-2xl"
		>

			<!-- HEADER -->
			<div class="sticky top-0 rounded-t-3xl border-b border-slate-100 bg-white dark:bg-[#1E293B] dark:border-[#334156] p-6">

                <div class="flex items-start justify-between">

                    <div>

                    <h2 class="text-2xl font-bold text-slate-800 dark:text-white">
                        Crear Usuario
                    </h2>

                    <p class="mt-1 text-sm text-slate-500">
                        Ingresa los datos del nuevo usuario
                    </p>

                    </div>

                    <button
                    type="button"
                    class="rounded-xl p-2 text-slate-400 transition hover:bg-slate-100 hover:text-slate-700 dark:hover:bg-[#162033]"
                    on:click={cerrarModal}
                    >
                    <X size={20} />
                    </button>

                </div>

            </div>

			<!-- BODY -->
			<div class="max-h-[70vh] overflow-y-auto p-8">

				<div class="grid gap-6 md:grid-cols-2">

					<!-- NOMBRE -->
					<div class="md:col-span-2">

						<label class="mb-2 block text-sm font-semibold text-slate-700 dark:text-white">
							Nombre Completo
						</label>

						<input
							bind:value={form.nombre_completo}
							placeholder="Nombre completo"
							class="w-full rounded-xl border border-slate-200 px-4 py-3 text-sm outline-none transition focus:border-[#0C4A6E] focus:ring-4 focus:ring-sky-100 dark:border-[#334156] dark:text-[#64748B] dark:focus:border-[#39BDF8] dark:focus:ring-4 dark:focus:ring-[#39BDF8]/20"
						/>

					</div>

					<!-- TIPO DOCUMENTO -->
                    <div class="relative ">

                        <label class="mb-2 block text-sm font-semibold text-slate-700 dark:text-white">
                                                Tipo Documento
                                            </label>


                    <select
                        bind:value={form.tipo_documento}
                        class="w-full appearance-none rounded-xl border border-slate-200 bg-white dark:bg-[#1E293B] dark:border-[#334156] dark:text-[#64748B] px-4 py-3 pr-11 text-sm outline-none transition duration-200 focus:border-[#0C4A6E] focus:ring-4 focus:ring-sky-100 focus:scale-[1.01] dark:focus:border-[#39BDF8] dark:focus:ring-4 dark:focus:ring-[#39BDF8]/20"
                    >
                        <option value="CC">CC</option>
                        <option value="TI">TI</option>
                        <option value="CE">CE</option>
                        <option value="Pasaporte">Pasaporte</option>
                    </select>

                    <div
                        class="pointer-events-none absolute inset-y-0 right-4 top-7 flex items-center"
                    >
                        <svg
                        class="h-4 w-4 text-slate-500"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                        >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M19 9l-7 7-7-7"
                        />
                        </svg>
                    </div>

                    </div>

					<!-- DOCUMENTO -->
					<div>

						<label class="mb-2 block text-sm font-semibold text-slate-700 dark:text-white">
							Número Documento
						</label>
                            <input
                                bind:value={documentoVisual}
                                on:input={manejarDocumento}
                                inputmode="numeric"
                                placeholder="Número"
                                class="w-full rounded-xl border border-slate-200 px-4 py-3 text-sm outline-none dark:border-[#334156] dark:text-[#64748B] transition focus:border-[#0C4A6E] focus:ring-4 focus:ring-sky-100 dark:focus:border-[#39BDF8] dark:focus:ring-4 dark:focus:ring-[#39BDF8]/20"
                            />

					</div>

					<!-- DIRECCION -->
					<div class="md:col-span-2">

						<label class="mb-2 block text-sm font-semibold text-slate-700 dark:text-white">
							Dirección
						</label>

						<input
							bind:value={form.direccion}
                            placeholder="Dirección"
							class="w-full rounded-xl border border-slate-200 px-4 py-3 text-sm outline-none transition focus:border-[#0C4A6E] dark:text-[#64748B] dark:border-[#334156] focus:ring-4 focus:ring-sky-100 dark:focus:border-[#39BDF8] dark:focus:ring-4 dark:focus:ring-[#39BDF8]/20"
						/>

					</div>

					<!-- SANGRE -->
                    <div class="relative">

                        <label class="mb-2 block text-sm font-semibold text-slate-700 dark:text-white">
                            Tipo de Sangre
                        </label>

                        <select
                        bind:value={form.tipo_sangre}
                        class="w-full appearance-none rounded-xl border border-slate-200 bg-white dark:bg-[#1E293B] dark:border-[#334156] dark:text-[#64748B] px-4 py-3 pr-11 text-sm outline-none transition duration-200 focus:border-[#0C4A6E] focus:ring-4 focus:ring-sky-100 focus:scale-[1.01] dark:focus:border-[#39BDF8] dark:focus:ring-4 dark:focus:ring-[#39BDF8]/20"
                    >
                            <option>A+</option>
                            <option>A-</option>
                            <option>B+</option>
                            <option>B-</option>
                            <option>AB+</option>
                            <option>AB-</option>
                            <option>O+</option>
                            <option>O-</option>
                        </select>

                    <div
                        class="pointer-events-none absolute inset-y-0 right-4 top-7 flex items-center"
                    >
                        <svg
                        class="h-4 w-4 text-slate-500"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                        >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M19 9l-7 7-7-7"
                        />
                        </svg>
                    </div>

                    </div>

					<!-- EPS -->
                    <div class="relative">

                        <label class="mb-2 block text-sm font-semibold text-slate-700 dark:text-white">
                            EPS
                        </label>


                    <select
                        bind:value={form.eps}
                        class="w-full appearance-none rounded-xl border border-slate-200 bg-white dark:bg-[#1E293B] dark:border-[#334156] dark:text-[#64748B] px-4 py-3 pr-11 text-sm outline-none transition duration-200 focus:border-[#0C4A6E] focus:ring-4 focus:ring-sky-100 focus:scale-[1.01] dark:focus:border-[#39BDF8] dark:focus:ring-4 dark:focus:ring-[#39BDF8]/20"
                    >
                         <option value="">Selecciona una EPS</option>

                        <option value="SURA">
                            SURA
                        </option>

                        <option value="SANITAS">
                            SANITAS
                        </option>

                        <option value="COMPENSAR">
                            COMPENSAR
                        </option>

                        <option value="NUEVA_EPS">
                            NUEVA EPS
                        </option>

                        <option value="FAMISANAR">
                            FAMISANAR
                        </option>

                        <option value="COOMEVA">
                            COOMEVA
                        </option>

                        <option value="SALUD_TOTAL">
                            SALUD TOTAL
                        </option>
                    </select>

                    <div
                        class="pointer-events-none absolute inset-y-0 right-4 top-7 flex items-center"
                    >
                        <svg
                        class="h-4 w-4 text-slate-500"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                        >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M19 9l-7 7-7-7"
                        />
                        </svg>
                    </div>

                    </div>

					<!-- GENERO -->
                    <div class="relative">

                        <label class="mb-2 block text-sm font-semibold text-slate-700 dark:text-white">
                            Género
                        </label>


                    <select
                        bind:value={form.genero}
                        class="w-full appearance-none rounded-xl border border-slate-200 bg-white dark:bg-[#1E293B] dark:border-[#334156] dark:text-[#64748B] px-4 py-3 pr-11 text-sm outline-none transition duration-200 focus:border-[#0C4A6E] focus:ring-4 focus:ring-sky-100 focus:scale-[1.01] dark:focus:border-[#39BDF8] dark:focus:ring-4 dark:focus:ring-[#39BDF8]/20"
                    >
                        <option value="MASCULINO">Masculino</option>
                        <option value="FEMENINO">Femenino</option>
                        <option value="OTRO">Otro</option>
                        <option value="PREFIERO_NO_DECIR">
                            Prefiero no decir
                        </option>
                    </select>

                    <div
                        class="pointer-events-none absolute inset-y-0 right-4 top-6 flex items-center"
                    >
                        <svg
                        class="h-4 w-4 text-slate-500"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                        >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M19 9l-7 7-7-7"
                        />
                        </svg>
                    </div>

                    </div>

                    <!-- TELEFONO -->
                    <div>
                        <label class="mb-2 block text-sm font-semibold text-slate-700 dark:text-white">
                            Teléfono
                        </label>

                        <div
                            class="flex overflow-hidden rounded-xl border border-slate-200 dark:border-[#334156] dark:text-[#64748B]
                            focus-within:border-[#0C4A6E]
                            focus-within:ring-4
                            focus-within:ring-sky-100
                            dark:focus-within:border-[#39BDF8]
                            dark:focus-within:ring-4
                            dark:focus-within:ring-[#39BDF8]/20"
                        >
                            <div
                                class="flex items-center border-r border-slate-200 bg-slate-50 px-4 text-sm font-medium text-slate-600 dark:bg-[#1E293B] dark:border-[#334156] dark:text-[#64748B]"
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

						<label class="mb-2 block text-sm font-semibold text-slate-700 dark:text-white">
							Correo
						</label>

						<input
							type="email"
                            placeholder="email@ejemplo.com"
							bind:value={form.correo}
							class="w-full rounded-xl border border-slate-200  dark:border-[#334156] dark:text-[#64748B] px-4 py-3 text-sm outline-none transition focus:border-[#0C4A6E] focus:ring-4 focus:ring-sky-100 dark:focus:border-[#39BDF8] dark:focus:ring-4 dark:focus:ring-[#39BDF8]/20"
						/>

					</div>

					<!-- PASSWORD -->
					<div>

						<label class="mb-2 block text-sm font-semibold text-slate-700 dark:text-white">
							Contraseña
						</label>

                        <div class="relative">
                            <input
                                type={mostrarPassword ? 'text' : 'password'}
                                placeholder="••••••••"
                                bind:value={form.password}
                                class="w-full rounded-xl border border-slate-200 dark:border-[#334156] dark:text-[#64748B] px-4 py-3 pr-12 text-sm outline-none transition focus:border-[#0C4A6E] focus:ring-4 focus:ring-sky-100 dark:focus:border-[#39BDF8] dark:focus:ring-4 dark:focus:ring-[#39BDF8]/20"
                            />

                            <button
                                type="button"
                                class="absolute right-3 top-1/2 -translate-y-1/2 text-slate-500 hover:text-slate-700"
                                on:click={() => (mostrarPassword = !mostrarPassword)}
                            >
                                {#if mostrarPassword}
                                    <EyeOff size={18} />
                                {:else}
                                    <Eye size={18} />
                                {/if}
                            </button>
                        </div>

                        <p class="mt-2 text-xs text-slate-500">
                            Debe contener mínimo 8 caracteres, una mayúscula,
                            una minúscula, un número y un carácter especial.
                        </p>

					</div>

				</div>

			</div>

			<!-- FOOTER -->
			<div
				class="flex justify-end gap-3 border-t border-slate-200 dark:border-[#334156]  px-8 py-5"
			>

				<button
					on:click={cerrarModal}
					class="rounded-xl border border-slate-200 dark:border-[#334156] dark:text-white px-5 py-2.5 text-sm font-medium text-slate-700 transition hover:bg-slate-100 dark:hover:bg-[#162033]"
				>
					Cancelar
				</button>

				<button
					on:click={guardarUsuario}
					disabled={loading}
					class="rounded-xl bg-[#0C4A6E] px-5 py-2.5 text-sm font-medium text-white dark:text-[#39BDF8] transition hover:bg-[#0a3a52] disabled:opacity-50"
				>

					{#if loading}
						Creando...
					{:else}
						Crear Usuario
					{/if}

				</button>

			</div>

		</div>

	</div>

{/if}

<UserCreatedSuccessModal
	open={successModalOpen}
	onClose={cerrarSuccessModal}
/>
