<script lang="ts">
  import DollarSign from 'lucide-svelte/icons/dollar-sign';
  import Moon from 'lucide-svelte/icons/moon';
  import ArrowRightLeft from 'lucide-svelte/icons/arrow-right-left';
  import Info from 'lucide-svelte/icons/info';
  import Briefcase from 'lucide-svelte/icons/briefcase';

  interface Stat {
    icon: 'DollarSign' | 'Moon' | 'ArrowRightLeft' | 'Info' | 'Briefcase';
    title: string;
    value: string | number;
  }

  export let stats: Stat[] = [];

  const icons = {
    DollarSign,
    Moon,
    ArrowRightLeft,
    Info,
    Briefcase
  };
</script>

<div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-6">

  {#each stats as stat}

    <div
      class="flex items-center gap-4 rounded-2xl border border-slate-200 bg-white p-4 transition-all duration-300 hover:shadow-md"
    >

      <!-- Icono -->
      <div
        class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-slate-100"
      >
        <svelte:component
          this={icons[stat.icon]}
          class="h-5 w-5 text-[#0C4A6E]"
        />
      </div>

      <!-- Texto -->
      <div class="min-w-0">
        <p
          class="text-[11px] font-semibold uppercase tracking-wide text-slate-500"
        >
          {stat.title}
        </p>

        <h3 class="mt-1 text-2xl font-bold text-slate-800">
          {stat.value}
        </h3>
      </div>

    </div>

  {/each}

</div>