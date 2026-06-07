<script lang="ts">
	export let title = '';
	export let value = '';
	export let description = '';
	export let valueColor = 'text-zinc-900 dark:text-white';
</script>

<div
	class="rounded-2xl border border-zinc-200 bg-white p-5 shadow-sm dark:border-[#334156] dark:bg-[#1E293B]"
>
	<div class="mb-3">
		<p
			class="text-sm font-medium text-zinc-500 dark:text-zinc-400"
		>
			{title}
		</p>
	</div>

	<h2 class={`text-3xl font-bold ${valueColor}`}>
		{value}
	</h2>

	<p
		class="mt-1 text-sm text-zinc-500 dark:text-zinc-400"
	>
		{description}
	</p>
</div>