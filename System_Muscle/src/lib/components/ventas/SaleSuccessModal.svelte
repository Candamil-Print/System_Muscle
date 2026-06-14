<script lang="ts">
  import CircleCheckBig from 'lucide-svelte/icons/circle-check-big';

  export let open = false;

  export let onClose: () => void = () => {};
</script>

{#if open}

  <!-- OVERLAY -->
<div
  class="fixed top-0 left-0 w-screen h-screen z-9998 bg-black/50 backdrop-blur-md"
></div>

  <!-- MODAL -->
  <div
    class="fixed inset-0 z-9999 flex items-center justify-center p-4"
  >

    <div
      class="w-full max-w-sm rounded-3xl border border-slate-200 bg-white p-8 text-center shadow-2xl dark:border-[#334156] dark:bg-[#1E293B]"
    >

      <!-- ICON -->
      <div class="mb-5 flex justify-center">

        <div
          class="flex h-20 w-20 items-center justify-center rounded-full  bg-[#0C4A6E]/10 dark:bg-[#0C4A6E]/10  "
        >
          <CircleCheckBig
            class="h-12 w-12 text-[#0C4A6E] dark:text-[#39BDF8]"
          />
        </div>

      </div>

      <!-- TITLE -->
      <h2
        class="text-2xl font-bold text-slate-800 dark:text-white"
      >
        ¡Venta Exitosa!
      </h2>

      <!-- DESCRIPTION -->
      <p
        class="mt-2 text-sm text-slate-500 dark:text-slate-400"
      >
        La venta ha sido registrada correctamente
      </p>

      <!-- BUTTON -->
      <button
        on:click={onClose}
        class="mt-6 w-full rounded-xl bg-[#0C4A6E] py-3 text-sm font-medium text-white transition hover:bg-[#0a3a52] dark:text-[#39BDF8]"
      >
        Continuar
      </button>

    </div>

  </div>

{/if}