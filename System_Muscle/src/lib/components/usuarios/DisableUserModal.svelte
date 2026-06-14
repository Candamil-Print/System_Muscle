<script lang="ts">
	import { createEventDispatcher } from 'svelte';
    import { X } from 'lucide-svelte';

	export let open = false;
	export let userName = '';
	export let loading = false;

	const dispatch = createEventDispatcher();
</script>

{#if open}

	<!-- OVERLAY -->
	<div
		class="fixed left-0 top-0 z-[9998] h-dvh w-dvw bg-black/50 backdrop-blur-sm"
	></div>

	<!-- MODAL -->
	<div
		class="fixed left-0 top-0 z-[9999] flex h-dvh w-dvw items-center justify-center p-4"
	>
		<div
			class="animate-in fade-in zoom-in duration-200 relative w-full max-w-md rounded-3xl border border-slate-200 bg-white dark:border-[#334156] dark:bg-[#1E293B]  shadow-2xl"
		>

			<!-- HEADER -->
			<div
				class="sticky top-0 rounded-t-3xl border-b border-slate-100 bg-white dark:border-[#334156] dark:bg-[#1E293B] p-6"
			>
				<div class="flex items-start justify-between">

					<div>
						<h2
							class="text-xl font-bold text-slate-800 dark:text-white"
						>
							¿Deshabilitar usuario?
						</h2>

						<p
							class="mt-2 text-sm leading-relaxed text-slate-500"
						>
							El usuario
							<strong class="text-slate-700 dark:text-[#39BDF8]">
								"{userName}"
							</strong>
							podrá acceder nuevamente al sistema.
						</p>
					</div>

					<button
						type="button"
						class="rounded-xl p-2 text-slate-400 transition hover:bg-slate-100 hover:text-slate-700 dark:hover:bg-[#162033]"
						on:click={() => dispatch('cancel')}
					>
						<X size={20} />
					</button>

				</div>
			</div>

			<!-- FOOTER -->
			<div
                class="flex justify-end gap-3 border-t border-slate-200 dark:border-[#334156] px-6 py-4"
            >
                <button
                    on:click={() => dispatch('cancel')}
                    class="rounded-xl border border-slate-200 px-4 py-2 text-sm font-medium text-slate-700 transition hover:bg-slate-100 dark:border-[#334156] dark:text-white dark:hover:bg-[#162033] "
                >
                    Cancelar
                </button>

                <button
                    on:click={() => dispatch('confirm')}
                    disabled={loading}
                    class="rounded-xl bg-[#0C4A6E] px-4 py-2 text-sm font-medium text-white dark:text-[#39BDF8] transition hover:bg-[#0a3a52] disabled:opacity-50"
                >
                    {loading ? 'Deshabilitando...' : 'Deshabilitar'}
                </button>
            </div>
		</div>
	</div>

{/if}