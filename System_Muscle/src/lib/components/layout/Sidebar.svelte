<script lang="ts">
    import { page } from '$app/stores';
    import {
        LayoutGrid,
        ShoppingCart,
        Box,
        TrendingUp,
        FileText,
        Clock,
        Users,
        LogOut
    } from 'lucide-svelte';

    const menu = [
        {
            label: 'Dashboard',
            icon: LayoutGrid,
            href: '/dashboard'
        },
        {
            label: 'Ventas',
            icon: ShoppingCart,
            href: '/ventas'
        },
        {
            label: 'Inventario',
            icon: Box,
            href: '/inventario'
        },
        {
            label: 'Movimientos',
            icon: TrendingUp,
            href: '/movimientos'
        },
        {
            label: 'Reportes',
            icon: FileText,
            href: '/reportes'
        },
        {
            label: 'Historial',
            icon: Clock,
            href: '/historial'
        },
        {
            label: 'Usuarios',
            icon: Users,
            href: '/usuarios'
        }
    ];
</script>

<aside
    class="fixed left-0 top-0 flex h-screen w-[280px] flex-col border-r border-zinc-200 bg-white dark:border-zinc-800 dark:bg-[#0f172a]"
>
    <div class="flex items-center gap-3 p-6 border-b border-zinc-200 dark:border-zinc-800">
        <div
            class="flex h-10 w-10 items-center justify-center rounded-lg bg-gradient-to-br from-[#0C4A6E] to-[#0284C7] font-bold text-white"
        >
            SM
        </div>

        <h1 class="text-lg font-semibold dark:text-white">
            System Muscle
        </h1>
    </div>

    <nav class="flex-1 px-4 py-4">
        <p
            class="mb-4 px-3 text-xs font-semibold uppercase tracking-wider text-zinc-500"
        >
            Menu
        </p>

        <div class="space-y-2">
            {#each menu as item}
                <a
                    href={item.href}
                    class={`flex w-full items-center gap-3 rounded-lg px-4 py-3 text-sm font-medium transition-all ${
                        $page.url.pathname === item.href
                            ? 'bg-[#0C4A6E] text-white'
                            : 'text-zinc-600 hover:bg-zinc-100 dark:text-zinc-300 dark:hover:bg-zinc-800'
                    }`}
                >
                    <item.icon size={20} />

                    {item.label}
                </a>
            {/each}
        </div>
    </nav>

    <div class="border-t border-zinc-200 p-4 dark:border-zinc-800">
        <div class="mb-4 flex items-center gap-3 rounded-lg bg-zinc-100 px-4 py-3 dark:bg-zinc-800">
            <div
                class="flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-full bg-gradient-to-br from-[#0C4A6E] to-[#0284C7] font-semibold text-white"
            >
                AD
            </div>

            <div>
                <p class="text-sm font-semibold text-zinc-900 dark:text-white">
                    Administrador
                </p>

                <p class="text-xs text-zinc-500 dark:text-zinc-400">
                    Administrador
                </p>
            </div>
        </div>

        <button
            class="flex w-full items-center gap-3 rounded-lg px-4 py-3 text-sm text-zinc-600 transition-all hover:bg-zinc-100 dark:text-zinc-300 dark:hover:bg-zinc-800"
        >
            <LogOut size={20} />

            Cerrar sesión
        </button>
    </div>
</aside>