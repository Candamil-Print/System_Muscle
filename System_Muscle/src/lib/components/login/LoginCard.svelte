<script lang="ts">
	import LoginHeader from './LoginHeader.svelte';
	import LoginForm from './LoginForm.svelte';
</script>

<div
	class="rounded-3xl
	border border-zinc-200
	bg-white/95
	p-8
	shadow-2xl
	transition-all duration-500

	dark:border-white/10
	dark:bg-[#0F172A]/95
	dark:shadow-black/40"
>
	<LoginHeader />

	<LoginForm />
</div>