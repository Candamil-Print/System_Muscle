<script lang="ts">
	import LoginHeader from './LoginHeader.svelte';
	import LoginForm from './LoginForm.svelte';
</script>

<div
	class="rounded-3xl
	border border-zinc-200
	bg-white/80
	p-8
	shadow-2xl
	backdrop-blur-xl
	transition-all duration-500

	dark:border-white/10
	dark:bg-white/5
	dark:shadow-black/40"
>
	<LoginHeader />

	<LoginForm />
</div>