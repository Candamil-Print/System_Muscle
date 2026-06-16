<script lang="ts">
  import CircleCheckBig from 'lucide-svelte/icons/circle-check-big';

  export let open = false;

  export let title = '¡Movimiento Exitoso!';
  export let description =
    'El movimiento ha sido registrado correctamente';

  function handleContinue() {
    open = false;
  }
</script>

{#if open}

  <!-- OVERLAY -->
  <div
    class="fixed left-0 top-0 z-[9998] h-dvh w-dvw bg-black/50 backdrop-blur-sm"
  ></div>

  <!-- MODAL CONTAINER -->
  <div
    class="fixed left-0 top-0 z-[9999] flex h-dvh w-dvw items-center justify-center p-4"
  >

    <!-- MODAL -->
    <div
      class="animate-in fade-in zoom-in duration-200 relative w-full max-w-md overflow-hidden rounded-3xl border border-slate-200 bg-white dark:bg-[#1E293B] dark:border-[#334156] p-8 shadow-2xl"
    >

      <!-- ICON -->
      <div
        class="mx-auto mb-5 flex h-20 w-20 items-center justify-center rounded-full bg-[#0C4A6E]/20"
      >

        <CircleCheckBig
          class="h-10 w-10 text-[#7B9DB0] dark:text-[#39BDF8]"
        />

      </div>

      <!-- CONTENT -->
      <div class="text-center">

        <h2
          class="text-2xl font-bold tracking-tight text-slate-900 dark:text-white"
        >
          {title}
        </h2>

        <p
          class="mt-3 text-sm leading-relaxed text-slate-500 dark:text-slate-400"
        >
          {description}
        </p>

      </div>

      <!-- BOTON -->
      <button
        on:click={handleContinue}
        class="mt-8 w-full rounded-xl bg-[#0C4A6E] px-5 py-2.5 text-sm font-medium text-white transition hover:bg-[#0a3a52] dark:text-[#39BDF8]"
      >
        Continuar
      </button>

    </div>

  </div>

{/if}