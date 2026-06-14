<script lang="ts">

  import {
    Users,
    UserCheck,
    Shield,
    User
  } from 'lucide-svelte';

  export let total = 0;
  export let activos = 0;
  export let administradores = 0;
  export let recepcionistas = 0;

</script>

<div
  class="grid gap-4 md:grid-cols-2 xl:grid-cols-4"
>

  <div
    class="rounded-2xl border border-slate-200 bg-white p-5 shadow-sm dark:bg-[#1E293B] dark:border-[#334156]"
  >


<div
  class="mb-3 flex items-center justify-between"
>

  <span
    class="text-sm font-medium text-slate-500"
  >
    Total Usuarios
  </span>

  <Users
    size={20}
    class="text-[#0C4A6E] dark:text-[#39BDF8]"
  />

</div>

<p
  class="text-3xl font-bold text-slate-800 dark:text-white"
>
  {total}
</p>

<p
  class="mt-1 text-sm text-slate-500"
>
  usuarios registrados
</p>


  </div>

  <div
    class="rounded-2xl border border-slate-200 bg-white dark:bg-[#1E293B] dark:border-[#334156] p-5 shadow-sm"
  >


<div
  class="mb-3 flex items-center justify-between"
>

  <span
    class="text-sm font-medium text-slate-500"
  >
    Usuarios Activos
  </span>

  <UserCheck
    size={20}
    class="text-[#0C4A6E] dark:text-[#39BDF8]"
  />

</div>

<p
  class="text-3xl font-bold text-[#0C4A6E] dark:text-[#39BDF8]"
>
  {activos}
</p>

<p
  class="mt-1 text-sm text-slate-500"
>
  usuarios habilitados
</p>


  </div>

  <div
    class="rounded-2xl border border-slate-200 bg-white dark:bg-[#1E293B] dark:border-[#334156] p-5 shadow-sm"
  >


<div
  class="mb-3 flex items-center justify-between"
>

  <span
    class="text-sm font-medium text-slate-500"
  >
    Administradores
  </span>

  <Shield
    size={20}
    class="text-[#0C4A6E] dark:text-[#39BDF8]"
  />

</div>

<p
  class="text-3xl font-bold text-slate-800 dark:text-white"
>
  {administradores}
</p>

<p
  class="mt-1 text-sm text-slate-500"
>
  administradores
</p>


  </div>

  <div
    class="rounded-2xl border border-slate-200 bg-white dark:bg-[#1E293B] dark:border-[#334156] p-5 shadow-sm"
  >


<div
  class="mb-3 flex items-center justify-between"
>

  <span
    class="text-sm font-medium text-slate-500"
  >
    Recepcionistas
  </span>

  <User
    size={20}
    class="text-[#0C4A6E] dark:text-[#39BDF8]"
  />

</div>

<p
  class="text-3xl font-bold text-slate-800 dark:text-white"
>
  {recepcionistas}
</p>

<p
  class="mt-1 text-sm text-slate-500"
>
  recepcionistas
</p>


  </div>

</div>
