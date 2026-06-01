<script lang="ts">

  import CircleCheckBig from 'lucide-svelte/icons/circle-check-big';

  export let open = false;

  export let onClose: () => void = () => {};

</script>

{#if open}

  <!-- OVERLAY -->
  <div
    class="fixed inset-0 z-[9998] bg-black/70 backdrop-blur-sm"
  ></div>

  <!-- MODAL -->
  <div
    class="fixed inset-0 z-[9999] flex items-center justify-center p-4"
  >

    <div
      class="w-full max-w-sm rounded-2xl border border-slate-700 bg-slate-800 p-8 text-center shadow-2xl"
    >

      <!-- ICON -->
      <div
        class="mb-5 flex justify-center"
      >

        <CircleCheckBig
          class="h-16 w-16 text-green-500"
        />

      </div>

      <!-- TITLE -->
      <h2
        class="text-2xl font-semibold text-slate-100"
      >
        ¡Venta Exitosa!
      </h2>

      <!-- DESCRIPTION -->
      <p
        class="mt-2 text-sm text-slate-400"
      >
        La venta ha sido registrada correctamente
      </p>

      <!-- BUTTON -->
      <button
        on:click={onClose}
        class="mt-6 w-full rounded-lg bg-sky-500 py-3 text-sm font-medium text-white transition hover:bg-sky-600"
      >
        Continuar
      </button>

    </div>

  </div>

{/if}

