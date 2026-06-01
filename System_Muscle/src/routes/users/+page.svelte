<script lang="ts">

  import { Toaster } from 'svelte-sonner';
  import { onMount } from 'svelte';

  import {
    listarUsuarios,
    modificarUsuario
  } from '$lib/services/api/users';

  import Header from '$lib/components/layout/Header.svelte';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';

  import UsersStats
    from '$lib/components/usuarios/UsersStats.svelte';

  import UsersFilters
    from '$lib/components/usuarios/UsersFilters.svelte';

  import UsersTable
    from '$lib/components/usuarios/UsersTable.svelte';

  import UsersHeader
    from '$lib/components/usuarios/UsersHeader.svelte';

  import type { Usuario } from '$lib/services/api/users/users.types';

  let users: Usuario[] = [];

  let search = '';

  let currentUserId: number | null = null;

  async function loadUsuarios() {
    users = await listarUsuarios();
  }

  onMount(async () => {
    const sesion = localStorage.getItem('sesion');

    if (sesion) {
      const usuario = JSON.parse(sesion);

      console.log('Usuario logueado:', usuario);

      currentUserId = usuario.id_usuario;
    }

    await loadUsuarios();
  });

  // FILTRO GENERAL:
  // 1. Oculta el usuario logueado
  // 2. Aplica búsqueda
  $: usersFiltered = users.filter((user) => {

    // ocultar usuario actual
    if (user.id_usuario === currentUserId) {
      return false;
    }

    const term = search.toLowerCase().trim();

    if (!term) return true;

    return (
      user.nombre_completo?.toLowerCase().includes(term) ||
      user.numero_documento?.toString().includes(term) ||
      user.correo?.toLowerCase().includes(term)
    );
  });

  // ESTADÍSTICAS
  $: total = usersFiltered.length;

  $: activos = usersFiltered.filter(
    (u) => u.estado === 1
  ).length;

  $: administradores = usersFiltered.filter(
    (u) => u.id_rol === 1
  ).length;

  $: recepcionistas = usersFiltered.filter(
    (u) => u.id_rol === 2
  ).length;

  async function handleSaveUsuario(
    event: CustomEvent<{
      id: number;
      direccion?: string;
      telefono?: string;
      correo?: string;
    }>
  ) {
    const {
      id,
      direccion,
      telefono,
      correo
    } = event.detail;

    await modificarUsuario(
      id,
      {
        direccion,
        telefono,
        correo
      }
    );

    await loadUsuarios();
  }

</script>

<Toaster position="top-center" />

<div class="slex min-h-screen bg-slate-50">

  <Sidebar />

  <div class="ml-70 flex flex-1 flex-col">

    <Header />

    <main class="space-y-6 p-6">

      <UsersHeader
	      onReload={loadUsuarios}
      />

      <UsersStats
        {total}
        {activos}
        {administradores}
        {recepcionistas}
        />

      <UsersFilters
        bind:search
      />

      <UsersTable
        users={usersFiltered}
        on:save={handleSaveUsuario}
        on:reload={loadUsuarios}
    />

    </main>

  </div>

</div>
