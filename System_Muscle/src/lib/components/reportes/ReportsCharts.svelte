<script>
  import ReportsChartCard from './ReportsChartCard.svelte';

  const salesByDay = {
    labels: ['Lun', 'Mar', 'Mié', 'Jue', 'Vie', 'Sab', 'Dom'],
    datasets: [
      {
        label: 'Ventas',
        data: [250000, 320000, 280000, 410000, 390000, 320000, 280000],
        borderColor: '#0C4A6E',
        backgroundColor: 'rgba(12,74,110,.15)',
        fill: true,
        tension: 0.4
      }
    ]
  };

  const profitMargin = {
    labels: ['Proteína', 'Red Bull', 'Barrita', 'Agua', 'BCAA'],
    datasets: [
      {
        label: 'Margen %',
        data: [35, 42, 28, 45, 38],
        backgroundColor: '#0C4A6E'
      }
    ]
  };

  const topProducts = {
    labels: ['Proteína', 'Red Bull', 'Barrita', 'Agua', 'BCAA'],
    datasets: [
      {
        label: 'Unidades',
        data: [245, 189, 156, 312, 98],
        backgroundColor: '#0284C7'
      }
    ]
  };

  const salesBySeller = {
    labels: ['Admin', 'Maria', 'Carlos'],
    datasets: [
      {
        data: [580000, 420000, 250000],
        backgroundColor: [
          '#0C4A6E',
          '#0284C7',
          '#38BDF8'
        ]
      }
    ]
  };
</script>

<div class="grid grid-cols-1 gap-5 xl:grid-cols-2">

  <ReportsChartCard
    title="Ventas por Día"
    type="line"
    data={salesByDay}
  />

  <ReportsChartCard
    title="Margen de Ganancias"
    type="bar"
    data={profitMargin}
  />

  <ReportsChartCard
    title="Top Productos"
    type="bar"
    data={topProducts}
  />

  <ReportsChartCard
    title="Ventas por Vendedor"
    type="pie"
    data={salesBySeller}
  />

</div>