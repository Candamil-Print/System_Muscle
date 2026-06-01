<script lang="ts">
	import { CheckCircle2, X } from 'lucide-svelte';

	export let open = false;
	export let onClose: () => void = () => {};
</script>

{#if open}
	<!-- Overlay -->
    <div
		class="fixed left-0 top-0 z-[9998] h-dvh w-dvw bg-black/50 backdrop-blur-sm"
	></div>

	<!-- Modal -->
	<div
		class="fixed inset-0 z-[99999] flex items-center justify-center p-4"
	>
		<div
	        class="relative w-full max-w-md rounded-2xl bg-white px-6 pb-6 pt-12 text-center shadow-2xl"
        >
            <!-- BOTÓN CERRAR -->
            <button
                type="button"
                on:click={onClose}
                class="absolute right-4 top-4 rounded-xl p-2 text-slate-400 transition hover:bg-slate-100 hover:text-slate-700"
            >
                <X size={20} />
            </button>

            <div class="mb-4 flex justify-center">
                <CheckCircle2
                    size={70}
                    class="text-[#0C4A6E]"
                    strokeWidth={2}
                />
            </div>

            <h2 class="mb-2 text-2xl font-bold text-slate-900">
                ¡Usuario creado!
            </h2>

            <p class="mb-6 text-sm text-slate-500">
                El usuario ha sido registrado correctamente en el sistema.
            </p>

            <button
                on:click={onClose}
                class="w-full rounded-xl bg-[#0C4A6E] px-5 py-3 text-sm font-medium text-white transition hover:bg-[#0a3a52]"
            >
                Continuar
            </button>
        </div>
	</div>
{/if}