<script lang="ts">
  import ChevronLeft from 'lucide-svelte/icons/chevron-left';
  import ChevronRight from 'lucide-svelte/icons/chevron-right';

  export let currentPage = 1;

  export let totalPages = 1;

  export let onPageChange: (page: number) => void;

  function goToPage(page: number) {

    if (page < 1 || page > totalPages) return;

    onPageChange(page);

  }
</script>

{#if totalPages > 1}

  <div class="mt-6 flex items-center justify-center gap-2">

    <!-- PREV -->
    <button
      on:click={() => goToPage(currentPage - 1)}
      disabled={currentPage === 1}
      class="flex h-10 w-10 items-center justify-center rounded-xl border border-slate-200 bg-white text-slate-600 transition hover:bg-slate-100 disabled:cursor-not-allowed disabled:opacity-40"
    >

      <ChevronLeft class="h-5 w-5" />

    </button>

    <!-- PÁGINAS -->
    {#each Array(totalPages) as _, index}

      <button
        on:click={() => goToPage(index + 1)}
        class={`flex h-10 w-10 items-center justify-center rounded-xl text-sm font-medium transition ${
          currentPage === index + 1
            ? 'bg-[#0C4A6E] text-white'
            : 'border border-slate-200 bg-white text-slate-700 hover:bg-slate-100'
        }`}
      >

        {index + 1}

      </button>

    {/each}

    <!-- NEXT -->
    <button
      on:click={() => goToPage(currentPage + 1)}
      disabled={currentPage === totalPages}
      class="flex h-10 w-10 items-center justify-center rounded-xl border border-slate-200 bg-white text-slate-600 transition hover:bg-slate-100 disabled:cursor-not-allowed disabled:opacity-40"
    >

      <ChevronRight class="h-5 w-5" />

    </button>

  </div>

{/if}

