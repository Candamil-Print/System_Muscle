<script lang="ts">

  export let open = false;

  let montoApertura = '';

  export let onConfirm: (
    monto: number
  ) => void = () => {};

  $: if (open) {
    montoApertura = '';
  }

  function confirmar() {

    const monto =
      Number(montoApertura);

    if (
      isNaN(monto) ||
      monto < 0
    ) {
      return;
    }

    onConfirm(monto);
  }

</script>

{#if open}

  <!-- OVERLAY -->
  <div
    class="fixed top-0 left-0 w-screen h-screen z-[9998] bg-black/50 backdrop-blur-md"
  ></div>

  <!-- MODAL -->
  <div
    class="fixed inset-0 z-[9999] flex items-center justify-center p-4"
  >

    <div
      class="animate-in fade-in zoom-in duration-200 w-full max-w-md rounded-3xl border border-slate-200 bg-white shadow-2xl dark:bg-[#1E293B] dark:border-[#334156]"
    >

      <!-- HEADER -->
      <div
        class="rounded-t-3xl border-b border-slate-100 bg-white p-6 dark:border-[#334156] dark:bg-[#1E293B]"
      >

        <h2
          class="text-2xl font-bold text-slate-800 dark:text-white"
        >
          Apertura de Caja
        </h2>

        <p
          class="mt-1 text-sm text-slate-500 dark:text-slate-400"
        >
          Ingresa el monto inicial con el que abrirás la caja para este turno.
        </p>

      </div>

      <!-- BODY -->
      <div class="space-y-5 p-6">

        <div>

          <label
            class="mb-2 block text-sm font-medium text-slate-700 dark:text-slate-300"
          >
            Monto de Apertura
          </label>

          <input
            type="number"
            min="0"
            bind:value={montoApertura}
            placeholder="Ej: 100000"
            on:keydown={(e) => {
              if (e.key === 'Enter') {
                confirmar();
              }
            }}
            class="w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-slate-800 outline-none transition focus:border-[#0C4A6E] focus:ring-2 focus:ring-[#0C4A6E]/20 dark:border-[#334156] dark:bg-[#111827] dark:text-white"
          />

        </div>

        <div
          class="rounded-2xl bg-slate-50 p-4 dark:bg-[#111827]"
        >

          <p
            class="text-sm text-slate-600 dark:text-slate-400"
          >
            Este valor será registrado como saldo inicial de caja para el turno actual.
          </p>

        </div>

      </div>

      <!-- FOOTER -->
      <div
        class="rounded-b-3xl border-t border-slate-100 bg-white p-5 dark:border-[#334156] dark:bg-[#1E293B]"
      >

        <div class="flex justify-end">

          <button
            on:click={confirmar}
            disabled={
              montoApertura === '' ||
              Number(montoApertura) < 0
            }
            class="rounded-xl bg-[#0C4A6E] px-5 py-2.5 text-sm font-medium text-white transition hover:bg-[#0a3a52] disabled:cursor-not-allowed disabled:opacity-50"
          >
            Abrir Caja
          </button>

        </div>

      </div>

    </div>

  </div>

{/if}