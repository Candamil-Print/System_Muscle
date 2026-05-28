<script lang="ts">
  import { onMount } from 'svelte';
  import Chart from 'chart.js/auto';

  export let title = '';
  export let type = 'bar';
  export let data = {};

  let canvas: HTMLCanvasElement;
  let chart;

  onMount(() => {
    chart = new Chart(canvas, {
      type,
      data,

      options: {
        responsive: true,
        maintainAspectRatio: false,

        plugins: {
          legend: {
            labels: {
              color: '#64748b'
            }
          }
        },

        scales:
          type !== 'pie'
            ? {
                x: {
                  ticks: {
                    color: '#64748b'
                  },
                  grid: {
                    display: false
                  }
                },

                y: {
                  ticks: {
                    color: '#64748b'
                  },
                  grid: {
                    color: '#e2e8f0'
                  }
                }
              }
            : {}
      }
    });

    return () => chart?.destroy();
  });
</script>

<div
  class="rounded-2xl border border-slate-200 bg-white p-5"
>
  <div class="mb-5 flex items-center justify-between">
    <h3 class="text-xl font-semibold text-slate-800">
      {title}
    </h3>
  </div>

  <div class="h-[300px]">
    <canvas bind:this={canvas}></canvas>
  </div>
</div>