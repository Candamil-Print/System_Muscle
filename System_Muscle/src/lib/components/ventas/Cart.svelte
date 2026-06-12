<script lang="ts">
  import ConfirmSaleModal from './ConfirmSaleModal.svelte';
  import SaleSuccessModal from './SaleSuccessModal.svelte';
  import { registrarVenta } from '$lib/services/api/sale';
  import { obtenerCajaActiva, abrirCaja } from '$lib/services/api/shifts';
  import { turnoStore } from '$lib/stores/shifts/turnoStore';
  import {
    ShoppingCart,
    Wallet,
    Landmark,
    AlertCircle
  } from 'lucide-svelte';

  // STORE
  import { cart } from '$lib/stores/sales/cart';

  let metodoPago: string = 'efectivo';
  let confirmOpen = false;
  let successOpen = false;
  let cajaActiva: any = null;
  let errorCaja = '';
  let procesando = false;
  let abriendo = false;
  let turnoActual: { id_turno: number; nombre: string; horario: string } | null = null;

  // Suscribirse al store del turno
  turnoStore.subscribe(turno => {
    turnoActual = turno;
  });

  const sesion = JSON.parse(localStorage.getItem('sesion') ?? '{}');

  $: total = $cart.reduce(
    (acc, item) => acc + item.precio * item.cantidad,
    0
  );

  const formatear = (valor: number): string =>
    new Intl.NumberFormat('es-CO').format(valor);

  async function cargarCajaActiva() {
    try {
      errorCaja = '';
      cajaActiva = await obtenerCajaActiva();
      
      if (!cajaActiva) {
        errorCaja = 'No hay una caja abierta. Por favor, abre una caja antes de realizar ventas.';
      }
    } catch (error) {
      console.error('Error al cargar caja activa:', error);
      errorCaja = 'Error al verificar caja activa';
    }
  }

  async function abrirCajaParaVenta() {
    try {
      abriendo = true;
      errorCaja = '';
      
      // Abre una nueva caja
      const resultado = await abrirCaja(turnoActual.id_turno);
      console.log('Caja abierta:', resultado);
      
      // Recarga la caja activa
      await cargarCajaActiva();
      
    } catch (error) {
      console.error('Error al abrir caja:', error);
      errorCaja = `Error: ${(error as Error).message}`;
    } finally {
      abriendo = false;
    }
  }

  import { onMount } from 'svelte';
  onMount(async () => {
    try {
      await turnoStore.inicializar();
    } catch (error: any) {
      // Si el error es "ya tiene un turno abierto", obtén el turno activo
      if (error.message?.includes('ya tiene un turno abierto')) {
        console.log('Turno previo detectado, cargando...');
      }
    }
    await cargarCajaActiva();
    
    // Si no hay caja, intenta abrir una
    if (!cajaActiva) {
      console.log('Sin caja, abriendo automáticamente...');
      await abrirCajaParaVenta();
    }
  });

  function aumentarCantidad(index: number) {
    cart.update((items) => {
      items[index].cantidad += 1;
      return [...items];
    });
  }

  function disminuirCantidad(index: number) {
    cart.update((items) => {
      if (items[index].cantidad > 1) {
        items[index].cantidad -= 1;
      } else {
        items.splice(index, 1);
      }
      return [...items];
    });
  }

  async function procesarVenta() {
    if (!cajaActiva) {
      errorCaja = 'No hay una caja activa. No se puede procesar la venta.';
      return;
    }

    if (!turnoActual || !turnoActual.id_turno) {
      alert('Por favor, selecciona un turno antes de procesar la venta.');
      return;
    }

    try {
      procesando = true;
      const sesion = JSON.parse(localStorage.getItem('sesion') ?? '{}');

      const venta = {
        id_usuario: sesion.id_usuario,
        id_caja: cajaActiva.id_caja,
        id_turno: turnoActual.id_turno,
        lineas: $cart.map((item) => ({
          id_producto: item.id_producto,
          cantidad: item.cantidad,
          precio_unitario: item.precio,
          metodo_pago: metodoPago === 'efectivo' ? 1 : 2
        }))
      };

      console.log("VENTA A ENVIAR", venta);
      const idVenta = await registrarVenta(venta);
      console.log('Venta registrada:', idVenta);

      cart.set([]);
      confirmOpen = false;
      successOpen = true;
    } catch (error) {
      console.error('Error al procesar venta:', error);
      alert('Error al procesar la venta: ' + (error as Error).message);
    } finally {
      procesando = false;
    }
  }

  // Obtén el turno activo
  const turnoGuardado = JSON.parse(localStorage.getItem('turnoSeleccionado'));
  console.log('Turno guardado:', turnoGuardado);

  // Copia esto en la consola del navegador
  console.log('Caja activa:', localStorage.getItem('cajaActiva'));
  console.log('Turno actual:', localStorage.getItem('turnoActual'));
  console.log('Carrito:', JSON.parse(localStorage.getItem('cart') ?? '[]').length);
</script>

<aside class="space-y-6">
  <div class="sticky top-24 rounded-2xl border border-slate-200 bg-white shadow-sm dark:bg-[#1E293B] dark:border-[#334156]">
    <div class="border-b border-slate-200 dark:border-[#334156] p-5">
      <div class="flex items-center gap-2">
        <ShoppingCart size={20} class="text-[#0C4A6E] dark:text-[#39BDF8]" />
        <h2 class="text-lg font-semibold text-slate-800 dark:text-white">Carrito</h2>
      </div>
      <p class="mt-1 text-sm text-slate-500">{$cart.length} productos</p>
    </div>

    <div class="space-y-3 p-5">
      {#if $cart.length === 0}
        <div class="flex items-center justify-center rounded-xl border border-dashed border-slate-300 dark:border-[#334156] py-10 text-sm text-slate-500 dark:text-slate-400">
          No hay productos en el carrito
        </div>
      {:else}
        {#each $cart as item, index}
          <div class="rounded-xl bg-[#F3F4F6] dark:bg-[#334156] p-4">
            <div class="flex items-center justify-between">
              <div>
                <h3 class="font-medium text-slate-800 dark:text-white">{item.nombre}</h3>
                <p class="text-sm text-slate-500 dark:text-slate-400">$ {formatear(item.precio)}</p>
              </div>
              <div class="flex items-center gap-3">
                <button
                  on:click={() => disminuirCantidad(index)}
                  class="flex h-8 w-8 items-center justify-center rounded-lg border border-[#E5E7EB] dark:border-[#475569] bg-white dark:bg-[#1E293B] hover:bg-slate-100 dark:hover:bg-[#0F172A] text-slate-800 dark:text-white"
                >
                  −
                </button>
                <span class="w-6 text-center font-semibold text-slate-800 dark:text-white">{item.cantidad}</span>
                <button
                  on:click={() => aumentarCantidad(index)}
                  class="flex h-8 w-8 items-center justify-center rounded-lg border border-[#E5E7EB] dark:border-[#475569] bg-white dark:bg-[#1E293B] hover:bg-slate-100 dark:hover:bg-[#0F172A] text-slate-800 dark:text-white"
                >
                  +
                </button>
              </div>
            </div>
          </div>
        {/each}
      {/if}
    </div>

    <div class="border-t border-slate-200 dark:border-[#334156] p-5">
      <div class="mb-5 flex items-center justify-between">
        <span class="text-slate-600">Total</span>
        <span class="text-3xl font-bold text-[#0C4A6E] dark:text-[#39BDF8]">$ {formatear(total)}</span>
      </div>

      <div class="mb-5 space-y-3">
        <h3 class="text-sm font-semibold text-slate-800">Método de pago</h3>
        <div class="flex gap-3">
          <button
            on:click={() => (metodoPago = 'efectivo')}
            class={`flex-1 flex items-center justify-center gap-2 rounded-lg px-4 py-3 text-sm font-medium transition ${
              metodoPago === 'efectivo'
                ? 'bg-[#0C4A6E] dark:text-[#39BDF8] text-white'
                : 'border border-slate-300 text-slate-700 hover:bg-slate-50'
            }`}
          >
            <Wallet size={18} />
            Efectivo
          </button>
          <button
            on:click={() => (metodoPago = 'transferencia')}
            class={`flex-1 flex items-center justify-center gap-2 rounded-lg px-4 py-3 text-sm font-medium transition ${
              metodoPago === 'transferencia'
                ? 'bg-[#0C4A6E] dark:text-[#39BDF8] text-white'
                : 'border border-slate-300 text-slate-700 hover:bg-slate-50'
            }`}
          >
            <Landmark size={18} />
            Transferencia
          </button>
        </div>
      </div>

      <button
        on:click={() => {
          if (!cajaActiva) {
            errorCaja = 'No hay una caja activa. No se puede procesar la venta.';
            return;
          }
          if (!turnoActual) {
            alert('Por favor, selecciona un turno');
            return;
          }
          confirmOpen = true;
        }}
        disabled={!cajaActiva || !turnoActual || $cart.length === 0 || procesando}
        class="w-full rounded-lg bg-[#0C4A6E] py-3.5 text-sm font-medium text-white transition hover:bg-[#0a3a52] disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {procesando ? 'Procesando...' : 'Procesar Venta'}
      </button>
    </div>
  </div>
</aside>

<ConfirmSaleModal
  open={confirmOpen}
  carrito={$cart}
  {total}
  metodoPago={metodoPago}
  onClose={() => {
    confirmOpen = false;
  }}
  onConfirm={procesarVenta}
/>

<SaleSuccessModal
  open={successOpen}
  onClose={() => {
    successOpen = false;
  }}
/>