<script lang="ts">
  import { onMount } from 'svelte';

  import Header from '$lib/components/layout/Header.svelte';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';

  import SalesFilters from '$lib/components/ventas/SalesFilters.svelte';
  import ProductsGrid from '$lib/components/ventas/ProductsGrid.svelte';
  import Cart from '$lib/components/ventas/Cart.svelte';
  import SummaryCard from '$lib/components/ventas/SummaryCard.svelte';
  import ShiftSummary from '$lib/components/ventas/ShiftSummary.svelte';

  // API
  import {
    listarProductos
  } from '$lib/services/api/inventory';

  import {
  listarVentas
} from '$lib/services/api/sale';

  // TYPES
  import type {
    ProductoVenta
  } from '$lib/services/api/sale';

  interface Producto {
    nombre: string;
    categoria: string;
    precio: number;
    stock: number;
    total: number;
    imagen: string;
  }

  // PRODUCTOS
  let productos: Producto[] = [];

  // LOADING
  let loading = false;

  let ventasHoy = 0;

  let totalVentas = 0;

  // CARGAR PRODUCTOS
  onMount(async () => {

    try {

      loading = true;

      const response =
        await listarProductos();

      productos = response.map((product) => ({
        id_producto: product.id_producto,
        nombre: product.nombre,
        categoria: product.tipo_producto || 'Sin categoría',
        precio: product.precio_sugerido || 0,
        stock: product.stock_actual || 0,
        total: (product.precio_sugerido || 0) * (product.stock_actual || 0),
        imagen: product.imagen_url || ''
      }));

      const ventas = await listarVentas();

      ventasHoy = ventas.length;

      totalVentas = ventas.reduce(
        (acc, venta) => acc + venta.total,
        0
      );

    } catch (error) {

      console.error(error);

    } finally {

      loading = false;

    }

  });
</script>

<div class="flex min-h-screen bg-slate-50">

  <Sidebar />

  <div class="ml-[280px] flex flex-1 flex-col">

    <Header />

    <main class="flex-1 p-6">

      <!-- ENCABEZADO -->
      <div class="mb-6 flex items-center justify-between">

        <div>

          <h1 class="text-3xl font-bold text-slate-800">
            Ventas
          </h1>

          <p class="mt-1 text-sm text-slate-500">
            Registra ventas de productos
          </p>

        </div>

        <SummaryCard
          {ventasHoy}
          {totalVentas}
        />
      </div>

      <!-- CONTENIDO -->
      <div class="grid gap-6 xl:grid-cols-[2fr_1fr]">

        <!-- PRODUCTOS -->
        <section class="space-y-6">

          <SalesFilters />

          {#if loading}

            <div
              class="flex items-center justify-center rounded-2xl border border-slate-200 bg-white py-20 text-slate-500"
            >
              Cargando productos...
            </div>

          {:else}

            <ProductsGrid {productos} />

          {/if}

        </section>

        <!-- SIDEBAR -->
        <div class="space-y-6">

          <!-- YA NO USA PROPS -->
          <Cart />

          <ShiftSummary />

        </div>

      </div>

    </main>

  </div>

</div>
```
