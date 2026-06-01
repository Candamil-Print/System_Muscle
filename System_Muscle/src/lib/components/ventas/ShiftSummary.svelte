<script lang="ts">

  import { onMount } from 'svelte';

  // API
  import {
    ventasPorUsuario
  } from '$lib/services/api/sale';

  // TYPES
  import type {
    VentaResumen
  } from '$lib/services/api/sale/sale.types';

  let ventas: VentaResumen[] = [];

  let loading = false;

  // TOTALES
  let total = 0;

  // TEMPORALES
  let efectivo = 0;

  let transferencia = 0;

  // FORMATEAR
  const formatear = (
    valor: number
  ): string => {

    return new Intl.NumberFormat(
      'es-CO'
    ).format(valor);

  };

  // CARGAR VENTAS
  onMount(async () => {

    try {

      loading = true;

      ventas = await ventasPorUsuario(1);

      // TOTAL GENERAL
      total = ventas.reduce(
        (acc, venta) => {

          return acc + venta.total;

        },
        0
      );

      /**
       * TEMPORAL:
       * mientras backend no mande
       * total por método de pago
       */

      efectivo = total * 0.6;

      transferencia = total * 0.4;

    } catch (error) {

      console.error(error);

    } finally {

      loading = false;

    }

  });

</script>

<div
  class="rounded-2xl border border-slate-200 bg-white p-5 shadow-sm"
>

  <div class="mb-5">

    <h3
      class="text-lg font-semibold text-slate-800"
    >
      Resumen del Turno
    </h3>

    <p
      class="mt-1 text-sm text-slate-500"
    >
      Ventas registradas hoy
    </p>

  </div>

  {#if loading}

    <div
      class="py-10 text-center text-sm text-slate-500"
    >
      Cargando resumen...
    </div>

  {:else}

    <div class="space-y-4">

      <!-- EFECTIVO -->
      <div
        class="flex items-center justify-between"
      >

        <span
          class="text-sm text-slate-500"
        >
          Efectivo
        </span>

        <span
          class="font-semibold text-slate-800"
        >
          $
          {formatear(efectivo)}
        </span>

      </div>

      <!-- TRANSFERENCIA -->
      <div
        class="flex items-center justify-between"
      >

        <span
          class="text-sm text-slate-500"
        >
          Transferencia
        </span>

        <span
          class="font-semibold text-slate-800"
        >
          $
          {formatear(transferencia)}
        </span>

      </div>

      <!-- TOTAL -->
      <div
        class="border-t border-slate-200 pt-4"
      >

        <div
          class="flex items-center justify-between"
        >

          <span
            class="font-medium text-slate-700"
          >
            Total
          </span>

          <span
            class="text-xl font-bold text-[#0C4A6E]"
          >
            $
            {formatear(total)}
          </span>

        </div>

      </div>

    </div>

  {/if}

</div>

