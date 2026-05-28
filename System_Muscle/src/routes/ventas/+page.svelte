<script>
  import Header from '$lib/components/layout/Header.svelte';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';

  import SalesFilters from '$lib/components/ventas/SalesFilters.svelte';
  import ProductsGrid from '$lib/components/ventas/ProductsGrid.svelte';
  import Cart from '$lib/components/ventas/Cart.svelte';
  import SummaryCard from '$lib/components/ventas/SummaryCard.svelte';
  import ShiftSummary from '$lib/components/ventas/ShiftSummary.svelte';

  const productos = [
    {
      nombre: 'Proteína Whey',
      categoria: 'Suplementos',
      precio: 85000,
      stock: 45,
      total: 50,
      imagen: '/images/logo.jpeg'
    },
    {
      nombre: 'Creatina',
      categoria: 'Suplementos',
      precio: 65000,
      stock: 10,
      total: 30,
      imagen: '/images/logo.jpeg'
    },
    {
      nombre: 'Barra Proteica',
      categoria: 'Snacks',
      precio: 12000,
      stock: 25,
      total: 40,
      imagen: '/images/logo.jpeg'
    }
  ];

  const carrito = [
    {
      nombre: 'Proteína Whey',
      precio: 85000,
      cantidad: 1
    },
    {
      nombre: 'Barra Proteica',
      precio: 12000,
      cantidad: 2
    }
  ];
</script>

<div class="flex min-h-screen bg-slate-50">
  <Sidebar />

  <div class="flex flex-1 flex-col ml-[280px]">
    <Header />

    <main class="flex-1 p-6">
      <!-- encabezado -->
      <div class="mb-6 flex items-center justify-between">
        <div>
          <h1 class="text-3xl font-bold text-slate-800">
            Ventas
          </h1>

          <p class="mt-1 text-sm text-slate-500">
            Registra ventas de productos
          </p>
        </div>

        <SummaryCard />
      </div>

      <!-- contenido -->
      <div class="grid gap-6 xl:grid-cols-[2fr_1fr]">
        <section class="space-y-6">
            <SalesFilters />

            <ProductsGrid {productos} />
        </section>

        <div class="space-y-6">
            <Cart {carrito} />

            <ShiftSummary />
        </div>
        </div>
    </main>
  </div>
</div>