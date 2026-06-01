<script lang="ts">
  import { onMount } from 'svelte';
  import Search from 'lucide-svelte/icons/search';
  import Download from 'lucide-svelte/icons/download';
  import { obtenerVentasPorUsuario } from '$lib/services/api/reports/reports.service';
  import type { VentasPorUsuario } from '$lib/services/api/reports/reports.types';

  export let onFilter: (filters: { fechaInicio: string; fechaFin: string; metodoPago: string; vendedor: string }) => void = () => {};
  export let loading = false;

  let fechaInicio = '';
  let fechaFin = '';
  let metodoPago = 'todos';
  let vendedor = 'todos';
  let vendedores: VentasPorUsuario[] = [];

  async function cargarVendedores() {
    try {
      const today = new Date();
      const treintaDiasAgo = new Date(today.getTime() - 30 * 24 * 60 * 60 * 1000);
      
      const inicio = treintaDiasAgo.toISOString().split('T')[0];
      const fin = today.toISOString().split('T')[0];

      vendedores = await obtenerVentasPorUsuario(inicio, fin);
    } catch (err) {
      console.error('Error cargando vendedores:', err);
    }
  }

  function inicializarFechas() {
    const today = new Date();
    const sevenDaysAgo = new Date(today.getTime() - 7 * 24 * 60 * 60 * 1000);
    
    fechaFin = today.toISOString().split('T')[0];
    fechaInicio = sevenDaysAgo.toISOString().split('T')[0];
  }

  function handleFilter() {
    if (!fechaInicio || !fechaFin) {
      alert('Por favor selecciona ambas fechas');
      return;
    }

    onFilter({
      fechaInicio,
      fechaFin,
      metodoPago,
      vendedor
    });
  }

  function handleDownload() {
    console.log('Descargar reporte');
  }

  onMount(() => {
    inicializarFechas();
    cargarVendedores();
  });
</script>

<div class="bg-white border border-slate-200 rounded-2xl p-5">

  <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-5 gap-4">

    <div>
      <label class="text-xs font-semibold text-slate-500 uppercase">
        Fecha Inicio
      </label>

      <input
        type="date"
        bind:value={fechaInicio}
        class="w-full mt-2 h-11 px-4 rounded-xl border border-slate-200"
      />
    </div>

    <div>
      <label class="text-xs font-semibold text-slate-500 uppercase">
        Fecha Fin
      </label>

      <input
        type="date"
        bind:value={fechaFin}
        class="w-full mt-2 h-11 px-4 rounded-xl border border-slate-200"
      />
    </div>

    <div>
      <label class="text-xs font-semibold text-slate-500 uppercase">
        Método Pago
      </label>

      <select
        bind:value={metodoPago}
        class="w-full mt-2 h-11 px-4 rounded-xl border border-slate-200"
      >
        <option value="todos">Todos</option>
        <option value="efectivo">Efectivo</option>
        <option value="transferencia">Transferencia</option>
      </select>
    </div>

    <div>
      <label class="text-xs font-semibold text-slate-500 uppercase">
        Vendedor
      </label>

      <select
        bind:value={vendedor}
        class="w-full mt-2 h-11 px-4 rounded-xl border border-slate-200"
      >
        <option value="todos">Todos</option>
        {#each vendedores as v (v.id_usuario)}
          <option value={v.id_usuario}>
            {v.nombre_usuario}
          </option>
        {/each}
      </select>
    </div>

    <div class="flex items-end gap-3">

      <button
        on:click={handleFilter}
        disabled={loading}
        class="h-11 px-5 rounded-xl bg-[#0C4A6E] text-white flex items-center gap-2 disabled:opacity-50 cursor-pointer"
      >
        <Search class="w-4 h-4" />
        {loading ? 'Cargando...' : 'Filtrar'}
      </button>

      <button
        on:click={handleDownload}
        class="h-11 w-11 rounded-xl border border-slate-200 flex items-center justify-center hover:bg-slate-50 cursor-pointer"
      >
        <Download class="w-4 h-4" />
      </button>

    </div>

  </div>

</div>