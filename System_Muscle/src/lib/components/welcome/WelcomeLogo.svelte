<script lang="ts">
	import { Dumbbell } from 'lucide-svelte';
</script>

<div class="relative mx-auto mb-8 h-36 w-36">

	<div
		class="absolute inset-0 animate-spin rounded-full border-[3px] border-transparent border-r-sky-500 border-t-sky-900"
	></div>

	<div
		class="absolute inset-2 animate-spin rounded-full border-[2px] border-transparent border-b-sky-500 border-l-sky-900 [animation-direction:reverse]"
	></div>

	<div
		class="absolute inset-4 flex items-center justify-center rounded-full bg-gradient-to-br from-sky-900 via-sky-600 to-sky-400 shadow-2xl shadow-sky-900/30"
	>
		<Dumbbell size={56} class="text-white" />
	</div>
</div>