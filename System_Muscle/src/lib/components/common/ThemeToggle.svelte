<script lang="ts">
	import { Moon, Sun } from 'lucide-svelte';

	import { theme } from '$lib/stores/theme.store';

	let currentTheme: 'light' | 'dark' = 'light';

	theme.subscribe(value => {
		currentTheme = value;
	});

	const toggleTheme = () => {

		const isDark = document.documentElement.classList.contains('dark');

		if (isDark) {

			document.documentElement.classList.remove('dark');

			localStorage.setItem('theme', 'light');

			theme.set('light');

		} else {

			document.documentElement.classList.add('dark');

			localStorage.setItem('theme', 'dark');

			theme.set('dark');
		}
	};
</script>

<button
	on:click={toggleTheme}
	class="absolute right-6 top-6 z-50 flex h-12 w-12 items-center justify-center rounded-full border border-zinc-200 bg-white text-zinc-900 transition-all hover:scale-105 dark:border-zinc-700 dark:bg-zinc-900 dark:text-white"
>

	{#if currentTheme === 'dark'}
		<Sun size={20} />
	{:else}
		<Moon size={20} />
	{/if}

</button>