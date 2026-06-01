<script lang="ts">
	import { Toaster } from 'svelte-sonner';
	import ThemeToggle from '$lib/components/common/ThemeToggle.svelte';

	import LoginBackground from '$lib/components/login/LoginBackground.svelte';
	import LoginCard from '$lib/components/login/LoginCard.svelte';
</script>

<Toaster position="top-center" />
<div
	class="relative flex min-h-screen items-center justify-center overflow-hidden
	bg-gradient-to-br
	from-zinc-50
	to-sky-100
	px-4
	transition-all duration-500

	dark:from-[#020617]
	dark:via-[#071226]
	dark:to-[#0f2942]"
>
	<ThemeToggle />

	<LoginBackground />

	<div class="relative z-10 w-full max-w-md">
		<LoginCard />
	</div>
</div>