<script lang="ts">

  export let activo = false;

  export let disabled = false;

  export let onToggle:
    (() => void) | null = null;

  function toggle() {

    if (disabled) return;

    onToggle?.();

  }

</script>

<div class="flex items-center gap-4">
	<button
		on:click={toggle}
		disabled={disabled}
		class={`relative flex h-6 w-11 items-center rounded-full transition-colors duration-200 ${
			activo
				? 'bg-[#85A4B6]'
				: 'bg-slate-300'
		} ${
			disabled
				? 'cursor-not-allowed opacity-60'
				: 'cursor-pointer'
		}`}
	>
		<span
			class={`absolute h-5 w-5 rounded-full bg-white shadow-md transition-transform duration-200 ${
				activo
					? 'translate-x-5'
					: 'translate-x-0.5'
			}`}
		/>
	</button>


</div>
