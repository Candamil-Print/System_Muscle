<script lang="ts">
	import ChartCard from '$lib/components/reportes/ChartCard.svelte';

	const entradasPorDia = {
		labels: ['Lun', 'Mar', 'Mié', 'Jue', 'Vie', 'Sáb', 'Dom'],
		datasets: [
			{
				label: 'Entradas',
				data: [180, 220, 150, 280, 310, 200, 160],
				borderColor: '#0C4A6E',
				backgroundColor: 'rgba(12,74,110,0.15)',
				fill: true,
				tension: 0.4
			}
		]
	};

	const entradasPorTipo = {
		labels: ['Suplementos', 'Bebidas', 'Snacks'],
		datasets: [
			{
				data: [45, 30, 15],
				backgroundColor: [
					'#0C4A6E',
					'#0369A1',
					'#0284C7'
				]
			}
		]
	};

	const productosIngresados = {
		labels: [
			'Proteína Whey',
			'Creatina',
			'BCAA',
			'Multivitamínico',
			'Pre-entreno'
		],
		datasets: [
			{
				label: 'Unidades',
				data: [320, 250, 180, 290, 165],
				backgroundColor: '#0C4A6E'
			}
		]
	};

	const stockComparacion = {
		labels: [
			'Proteína Whey',
			'Creatina',
			'BCAA',
			'Multivitamínico',
			'Pre-entreno'
		],
		datasets: [
			{
				label: 'Stock Actual',
				data: [150, 120, 95, 200, 80],
				backgroundColor: '#0C4A6E'
			},
			{
				label: 'Stock Mínimo',
				data: [50, 50, 40, 60, 40],
				backgroundColor: '#94A3B8'
			}
		]
	};
</script>

<div class="grid grid-cols-1 gap-6 xl:grid-cols-2">

	<ChartCard
		title="Entradas por Día"
		type="line"
		data={entradasPorDia}
	/>

	<ChartCard
		title="Entradas por Tipo"
		type="pie"
		data={entradasPorTipo}
	/>

	<ChartCard
		title="Productos Ingresados"
		type="bar"
		data={productosIngresados}
	/>

	<ChartCard
		title="Stock Actual vs Stock Mínimo"
		type="bar"
		data={stockComparacion}
	/>

</div>