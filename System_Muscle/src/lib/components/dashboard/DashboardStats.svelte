<script lang="ts">
    import { Package, AlertTriangle, ShoppingCart, DollarSign } from 'lucide-svelte';
</script>

<div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-4">

    <div class="rounded-lg border border-zinc-200 bg-white p-6 dark:border-zinc-800 dark:bg-zinc-900">
        <div class="flex items-start justify-between">
            <div>
                <p class="text-sm text-zinc-600 dark:text-zinc-400">Total Productos</p>
                <p class="mt-2 text-3xl font-bold text-zinc-900 dark:text-white">4</p>
                <p class="mt-1 text-xs text-zinc-500">productos registrados</p>
            </div>
            <Package class="text-zinc-400" size={18} />
        </div>
    </div>

    <div class="rounded-lg border border-zinc-200 bg-white p-6 dark:border-zinc-800 dark:bg-zinc-900">
        <div class="flex items-start justify-between">
            <div>
                <p class="text-sm text-zinc-600 dark:text-zinc-400">Stock Bajo</p>
                <p class="mt-2 text-3xl font-bold">2</p>
                <p class="mt-1 text-xs text-zinc-500">requieren reposición</p>
            </div>
            <AlertTriangle class="text-zinc-400" size={18} />
        </div>
    </div>

    <div class="rounded-lg border border-zinc-200 bg-white p-6 dark:border-zinc-800 dark:bg-zinc-900">
        <div class="flex items-start justify-between">
            <div>
                <p class="text-sm text-zinc-600 dark:text-zinc-400">Ventas Hoy</p>
                <p class="mt-2 text-3xl font-bold text-zinc-900 dark:text-white">0</p>
                <p class="mt-1 text-xs text-zinc-500">transacciones realizadas</p>
            </div>
            <ShoppingCart class="text-zinc-400" size={18} />
        </div>
    </div>

    <div class="rounded-lg border border-zinc-200 bg-white p-6 dark:border-zinc-800 dark:bg-zinc-900">
        <div class="flex items-start justify-between">
            <div>
                <p class="text-sm text-zinc-600 dark:text-zinc-400">Ingresos Hoy</p>
                <p class="mt-2 text-3xl font-bold text-[#0C4A6E]">$ 0</p>
                <p class="mt-1 text-xs text-zinc-500">en ventas del día</p>
            </div>
            <DollarSign class="text-[#0C4A6E]" size={18} />
        </div>
    </div>

</div>