<script lang="ts">
  import ProductCard from './ProductCard.svelte';

  interface Producto {
    nombre: string;
    categoria: string;
    precio: number;
    stock: number;
    total: number;
    imagen: string;
  }

  export let productos: Producto[] = [];
</script>

<div class="grid gap-5 sm:grid-cols-2 xl:grid-cols-3">
  {#each productos as producto}
    <ProductCard {producto} />
  {/each}
</div>