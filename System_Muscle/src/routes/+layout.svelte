<script lang="ts">
	import '../app.css';

	import 'flatpickr/dist/flatpickr.min.css';

	import { browser } from '$app/environment';
	import { onMount } from 'svelte';
	import { emit } from '@tauri-apps/api/event';

	import { theme } from '$lib/stores/theme.store';

	onMount(async () => {

		if (!browser) return;

		// La clase "dark" ya se aplica de forma síncrona en app.html
		// (antes del primer paint) para evitar el flash de tema.
		// Aquí solo sincronizamos el store interno con lo que quedó aplicado.
		const savedTheme = localStorage.getItem('theme');

		if (savedTheme === 'dark') {
			theme.set('dark');
		}

		// Avisamos al backend de Tauri que el frontend ya está montado
		// y pintado, para que muestre la ventana (que arranca oculta).
		await new Promise((resolve) => requestAnimationFrame(resolve));
		await emit('frontend-ready');
	});
</script>

<slot />