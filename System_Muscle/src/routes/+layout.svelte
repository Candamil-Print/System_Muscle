<script lang="ts">
	import '../app.css';

	import { browser } from '$app/environment';
	import { onMount } from 'svelte';

	import { theme } from '$lib/stores/theme.store';

	onMount(() => {

		if (!browser) return;

		const savedTheme = localStorage.getItem('theme');

		if (savedTheme === 'dark') {

			document.documentElement.classList.add('dark');

			theme.set('dark');

		}
	});
</script>

<slot />