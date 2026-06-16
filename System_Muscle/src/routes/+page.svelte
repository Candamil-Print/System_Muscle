<script lang="ts">
	import { goto } from '$app/navigation';


	import WelcomeBackground from '$lib/components/welcome/WelcomeBackground.svelte';
	import WelcomeLogo from '$lib/components/welcome/WelcomeLogo.svelte';
	import WelcomeFeatures from '$lib/components/welcome/WelcomeFeatures.svelte';
	import WelcomeFooter from '$lib/components/welcome/WelcomeFooter.svelte';
	import ThemeToggle from '$lib/components/common/ThemeToggle.svelte';

	import { ArrowRight } from 'lucide-svelte';

	const handleLoginRedirect = () => {
		goto('/login');
	};
</script>



<div
	class="relative flex min-h-screen flex-col items-center justify-center overflow-hidden bg-zinc-50 px-6 text-zinc-900 transition-all duration-500 dark:bg-[#020617] dark:text-white"
>
	<ThemeToggle />

	<WelcomeBackground />

	<div class="relative z-10 max-w-3xl text-center">
		<WelcomeLogo />

		<h1
			class="mb-3 bg-gradient-to-r from-zinc-900 to-sky-500 bg-clip-text text-6xl font-extrabold tracking-tight text-transparent dark:from-white dark:to-sky-400"
		>
			System Muscle
		</h1>

		<p
			class="mb-14 text-lg text-zinc-500 transition-colors duration-300 dark:text-zinc-400"
		>
			Sistema integral de gestión para gimnasios
		</p>

		<WelcomeFeatures />

		<button
			on:click={handleLoginRedirect}
			class="group inline-flex items-center gap-3 rounded-2xl bg-gradient-to-r from-sky-900 to-sky-500 px-10 py-4 font-semibold text-white shadow-2xl shadow-sky-900/20 transition-all duration-300 hover:-translate-y-1 dark:shadow-sky-500/10"
		>
			Iniciar Sesión

			<ArrowRight
				class="transition-transform duration-300 group-hover:translate-x-1"
				size={20}
			/>
		</button>
	</div>

	<WelcomeFooter />
</div>