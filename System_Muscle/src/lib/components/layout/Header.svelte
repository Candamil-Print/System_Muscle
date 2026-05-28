<script lang="ts">
	import { Bell, Clock, Moon } from 'lucide-svelte';
</script>

<header
	class="sticky top-0 z-40 flex h-16 items-center justify-between border-b border-zinc-200 bg-white px-6 dark:border-zinc-800 dark:bg-[#0f172a]"
>
	<h1 class="text-lg font-semibold dark:text-white">
		Steel Body Gym
	</h1>

	<div class="flex items-center gap-3">

		<button
			class="flex items-center gap-2 rounded-full border border-zinc-200 px-4 py-2 text-sm dark:border-zinc-700 dark:text-zinc-200"
		>
			<Clock size={18} />

			Turno:
		</button>

		<button
			class="relative flex h-10 w-10 items-center justify-center rounded-full border border-zinc-200 dark:border-zinc-700 dark:text-zinc-200"
		>
			<Bell size={18} />

			<span
				class="absolute right-0 top-0 flex h-5 min-w-[20px] items-center justify-center rounded-full bg-red-500 px-1 text-xs text-white"
			>
				2
			</span>
		</button>

		<button
			class="flex h-10 w-10 items-center justify-center rounded-full border border-zinc-200 dark:border-zinc-700 dark:text-zinc-200"
		>
			<Moon size={18} />
		</button>
	</div>
</header>