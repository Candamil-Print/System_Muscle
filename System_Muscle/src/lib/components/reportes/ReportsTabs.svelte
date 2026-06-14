<script>
	import DollarSign from 'lucide-svelte/icons/dollar-sign';
	import ArrowDownCircle from 'lucide-svelte/icons/arrow-down-circle';

	export let activeTab = 'ventas';
</script>

<div class="flex gap-2 border-b border-slate-200">

	<button
		on:click={() => activeTab = 'ventas'}
		class={`flex items-center gap-2 px-5 py-3 font-semibold border-b-2 transition-colors ${
			activeTab === 'ventas'
				? 'border-[#0C4A6E] text-[#0C4A6E] dark:text-[#39BDF8] '
				: 'border-transparent text-slate-500 hover:text-slate-700 dark:text-slate-400 dark:boder-[#263244]'
		}`}
	>
		<DollarSign class="w-4 h-4" />
		Reporte Ventas
	</button>

	<button
		on:click={() => activeTab = 'entradas'}
		class={`flex items-center gap-2 px-5 py-3 font-semibold border-b-2 transition-colors ${
			activeTab === 'entradas'
				? 'border-[#0C4A6E] text-[#0C4A6E] dark:text-[#39BDF8] '
				: 'border-transparent text-slate-500 hover:text-slate-700'
		}`}
	>
		<ArrowDownCircle class="w-4 h-4" />
		Reporte Entradas
	</button>

</div>