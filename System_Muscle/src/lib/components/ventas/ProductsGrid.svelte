<script lang="ts">
  import { onMount } from 'svelte';

  import ProductCard from './ProductCard.svelte';

  // IMPORTAR API
  import {
    listarProductos
  } from '$lib/services/api/inventory';

  // IMPORTAR TYPES
  import type {
    ProductoConStock
  } from '$lib/services/api/inventory/inventory.types';

  interface Producto 
  { 
    id_producto: number;
    nombre: string; 
    categoria: string; 
    precio: number; 
    stock: number; 
    stock_maximo: number;
    total: number; 
    imagen: string; 
  } 
    
  export let productos: Producto[] = [];

  let loading = false;

  // CARGAR PRODUCTOS
  onMount(async () => {

    try {

      loading = true;

      const response: ProductoConStock[] =
        await listarProductos();

      productos = response.map((product) => ({
        id_producto: product.id_producto,
        nombre: product.nombre,
        categoria: product.tipo_producto || 'Sin categoría',
        precio: product.precio_sugerido || 0,
        stock: product.stock_actual || 0,
        stock_maximo: product.stock_maximo || product.stock_actual || 0,
        total: (product.precio_sugerido || 0) * (product.stock_actual || 0),
        imagen: product.imagen_url || ''
      }));

    } catch (error) {

      console.error(error);

    } finally {

      loading = false;

    }

  });
</script>

<!-- LOADING -->
{#if loading}

  <div
    class="flex items-center justify-center py-20 text-slate-500"
  >
    Cargando productos...
  </div>

{:else}

  <!-- SIN RESULTADOS -->
  {#if productos.length === 0}

    <div
      class="flex items-center justify-center rounded-2xl border border-dashed border-slate-300 bg-white py-20 text-slate-500 dark:bg-[#1E293B] dark:border-[#334156] dark:text-slate-400"
    >
      No hay productos disponibles
    </div>

  {:else}

    <!-- GRID -->
    <div class="grid gap-5 sm:grid-cols-2 lg:grid-cols-3">

      {#each productos as producto} 
        <ProductCard {producto} /> 
      {/each}

    </div>

  {/if}

{/if}

