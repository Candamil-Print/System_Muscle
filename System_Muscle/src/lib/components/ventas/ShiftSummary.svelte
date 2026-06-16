<script lang="ts">

  import { onMount } from 'svelte';

  import { turnoStore } from '$lib/stores/shifts/turnoStore';

  let turnoActual: any = null;

  turnoStore.subscribe((turno) => {
    turnoActual = turno;
  });

  import type {
    VentaDetallePorTurno
  } from '$lib/services/api/sale/sale.types';

  import {
  ventasPorTurno
} from '$lib/services/api/sale';

  let ventas: VentaDetallePorTurno[] = [];

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
  async function cargarVentasTurno() {

    if (!turnoActual?.id_turno) return;

    try {

      loading = true;

      ventas = await ventasPorTurno(
        turnoActual.id_turno
      );

      total = ventas.reduce(
        (acc, venta) => acc + venta.subtotal,
        0
      );

      efectivo = ventas
        .filter(
          (v) =>
            v.metodo_pago.toLowerCase() ===
            'efectivo'
        )
        .reduce(
          (acc, venta) =>
            acc + venta.subtotal,
          0
        );

      transferencia = ventas
        .filter(
          (v) =>
            v.metodo_pago
              .toLowerCase()
              .includes('transfer')
        )
        .reduce(
          (acc, venta) =>
            acc + venta.subtotal,
          0
        );

    } catch (error) {

      console.error(error);

    } finally {

      loading = false;

    }

  }

  $: if (turnoActual?.id_turno) {
    cargarVentasTurno();
  }

</script>

<div
  class="rounded-2xl border border-slate-200 bg-white p-5 shadow-sm dark:bg-[#1E293B] dark:border-[#334156]"
>

  <div class="mb-5">

    <h3
      class="text-lg font-semibold text-slate-800 dark:text-white"
    >
      Resumen del Turno
    </h3>

    <p
      class="mt-1 text-sm text-slate-500 dark:text-slate-400"
    >
      Ventas registradas hoy
    </p>

  </div>

  {#if loading}

    <div
      class="py-10 text-center text-sm text-slate-500 dark:text-slate-400"
    >
      Cargando resumen...
    </div>

  {:else}

    <div class="space-y-4 ">

      <!-- EFECTIVO -->
      <div
        class="flex items-center justify-between"
      >

        <span
          class="text-sm text-slate-500 dark:text-slate-400"
        >
          Efectivo
        </span>

        <span
          class="font-semibold text-slate-800 dark:text-white"
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
          class="text-sm text-slate-500 dark:text-slate-400"
        >
          Transferencia
        </span>

        <span
          class="font-semibold text-slate-800 dark:text-white"
        >
          $
          {formatear(transferencia)}
        </span>

      </div>

      <!-- TOTAL -->
      <div
        class="border-t border-slate-200 dark:border-[#334156] pt-4"
      >

        <div
          class="flex items-center justify-between"
        >

          <span
            class="font-medium text-slate-700 dark:text-slate-300"
          >
            Total
          </span>

          <span
            class="text-xl font-bold text-[#0C4A6E] dark:text-[#39BDF8]"
          >
            $
            {formatear(total)}
          </span>

        </div>

      </div>

    </div>

  {/if}

</div>

