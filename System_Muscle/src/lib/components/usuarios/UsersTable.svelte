<script lang="ts">
  import {
    Eye,
    Pencil,
    Users 
  } from 'lucide-svelte';

  import Pagination from './Pagination.svelte';
  import DisableUserModal from './DisableUserModal.svelte';
  import EnableUserModal from './EnableUserModal.svelte';
  import { toast } from 'svelte-sonner';



  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  import UserStatusSwitch
    from './UserStatusSwitch.svelte';

  import type { Usuario } from '$lib/services/api/users/users.types';

  import EditUserModal from './EditUserModal.svelte';

  import {
	habilitarUsuario,
	deshabilitarUsuario
} from '$lib/services/api/users';

let disableModalOpen = false;
let disableLoading = false;
let userToDisable: Usuario | null = null;

let enableModalOpen = false;
let enableLoading = false;
let userToEnable: Usuario | null = null;



async function toggleEstado(user: Usuario) {
	if (user.estado === 1) {
		userToDisable = user;
		disableModalOpen = true;
		return;
	}

	userToEnable = user;
	enableModalOpen = true;
}

  let editModalOpen = false;
  let selectedUser: Usuario | null = null;

  import ViewUserModal from './ViewUserModal.svelte';

    let viewModalOpen = false;

    function abrirVista(user: Usuario) {
        selectedUser = user;
        viewModalOpen = true;
    }

  export let users: Usuario[] = [];

  function abrirEdicion(user: Usuario) {
    selectedUser = user;
    editModalOpen = true;
  }

  async function guardarEdicion(
    data: {
      direccion?: string;
      telefono?: string;
      correo?: string;
    }
  ) {
    if (!selectedUser) return;

    dispatch('save', {
      id: selectedUser.id_usuario,
      ...data
    });

    editModalOpen = false;
    selectedUser = null;
  }

async function confirmarDeshabilitar() {
	if (!userToDisable) return;

	try {
		disableLoading = true;

		await deshabilitarUsuario(
			userToDisable.id_usuario
		);

		toast.success(
			`Usuario "${userToDisable.nombre_completo}" deshabilitado correctamente`
		);

		dispatch('reload');

		disableModalOpen = false;
		userToDisable = null;

	} catch (error) {

		toast.error(
			'No fue posible deshabilitar el usuario'
		);

		console.error(error);

	} finally {
		disableLoading = false;
	}
}

async function confirmarHabilitar() {
	if (!userToEnable) return;

	try {
		enableLoading = true;

		await habilitarUsuario(
			userToEnable.id_usuario
		);

		toast.success(
			`Usuario "${userToEnable.nombre_completo}" habilitado correctamente`
		);

		dispatch('reload');

		enableModalOpen = false;
		userToEnable = null;

	} catch (error) {

		toast.error(
			'No fue posible habilitar el usuario'
		);

		console.error(error);

	} finally {
		enableLoading = false;
	}
}

    // PAGINACIÓN
  let currentPage = 1;

  const itemsPerPage = 8;

  // TOTAL PÁGINAS
  $: totalPages = Math.ceil(
    users.length / itemsPerPage
  );

  // ELEMENTOS DE LA PÁGINA ACTUAL
  $: paginatedUsers = users.slice(
    (currentPage - 1) * itemsPerPage,
    currentPage * itemsPerPage
  );

  $: if (currentPage > totalPages && totalPages > 0) {
  currentPage = totalPages;
}
</script>

<div class="bg-white border border-slate-200 rounded-2xl overflow-hidden">

<!-- HEADER -->
<div
  class="border-b border-slate-200 px-6 py-4"
>

  <div class="flex items-center gap-3">

    <Users
      size={24}
      class="text-slate-600"
    />

    <h3
      class="text-xl font-bold text-slate-900"
    >
      Lista de Usuarios
    </h3>

  </div>

  <p
    class="mt-2 text-sm text-slate-500"
  >
    {users.length} usuarios encontrados
  </p>

</div>

  <!-- TABLA -->
  <div class="overflow-x-auto px-6 py-6">

    <div class="overflow-hidden rounded-xl border border-slate-200">

     <table class="w-full table-fixed">

<colgroup>
  <col class="w-[25%]" />
  <col class="w-[13%]" />
  <col class="w-[18%]" />
  <col class="w-[12%]" />
  <col class="w-[12%]" />
  <col class="w-[10%]" />
  <col class="w-[10%]" />
</colgroup>

  <thead class="bg-[#26557c]">

          <tr>

            <th class="px-6 py-4 text-left text-sm font-medium text-white">
              Usuario
            </th>

            <th class="px-6 py-4 text-left text-sm font-medium text-white">
              Documento
            </th>

            <th class="px-6 py-4 text-left text-sm font-medium text-white">
              Email
            </th>

            <th class="px-6 py-4 text-left text-sm font-medium text-white">
              Teléfono
              
            </th>

            <th class="px-6 py-4 text-center text-sm font-medium text-white">
              Rol
            </th>

            <th class="px-6 py-4 text-center text-sm font-medium text-white">
              Estado
            </th>

            <th class="px-6 py-4 text-center text-sm font-medium text-white">
              Acciones
            </th>

          </tr>

        </thead>

        <tbody class="divide-y divide-slate-200">

          {#each paginatedUsers as user (user.id_usuario)}

            <tr class="transition hover:bg-slate-50">

              <!-- USUARIO -->
              <td class="px-6 py-5 align-middle">

                <div class="flex items-center gap-3">

                  <div
                    class="flex h-11 w-11 shrink-0 items-center justify-center rounded-full bg-gradient-to-br from-[#0C4A6E] to-[#0284C7] text-sm font-semibold text-white"
                    >
                    {user.nombre_completo
                      .split(' ')
                      .map((n) => n[0])
                      .slice(0, 2)
                      .join('')}
                  </div>

                  <div class="min-w-0">

                    <p
                        class="line-clamp-2 text-[15px] font-medium leading-6 text-slate-800"
                        >
                        {user.nombre_completo}
                    </p>
                  </div>

                </div>

              </td>

              <!-- DOCUMENTO -->
             <td class="px-6 py-5 align-middle">

                <div class="flex flex-col">

                  <span class="text-xs text-slate-500">
                    {user.tipo_documento}
                  </span>

                  <span class="whitespace-nowrap font-medium text-slate-700">
                        {user.numero_documento}
                    </span>

                </div>

              </td>

              <!-- EMAIL -->
            <td class="px-6 py-5 align-middle">
            <span class="block truncate text-slate-600">
                {user.correo || '-'}
            </span>
            </td>
              <!-- TELEFONO -->
              <td class="px-6 py-5 align-middle">

                <span class="text-slate-600">
                  {user.telefono || '-'}
                </span>

              </td>

              <!-- ROL -->
              <td class="px-6 py-5 text-center align-middle">

                <span
                  class={`inline-flex rounded-full px-4 py-2 text-xs font-semibold ${
                    user.id_rol === 1
                      ? 'bg-[#1c5476]/10 text-[#1c5476]'
                      : 'bg-[#EFF0F3] text-[#ACAFB8]'
                  }`}
                >
                  {user.id_rol === 1
                    ? 'Administrador'
                    : 'Recepcionista'}
                </span>

              </td>

              <!-- ESTADO -->
              <td class="px-6 py-5 align-middle">

                <div class="flex justify-center">

                  <UserStatusSwitch
                    activo={user.estado === 1}
                    disabled={user.id_usuario === 1}
                    onToggle={() => toggleEstado(user)}
                  />

                </div>

              </td>

              <!-- ACCIONES -->
              <td class="px-6 py-5 align-middle">

                <div class="flex items-center justify-center gap-3">

                  <button
                    class="rounded-lg p-2 transition hover:bg-slate-100"
                    on:click={() => abrirVista(user)}
                  >
                    <Eye size={18} />
                  </button>

                  <button
                    class="rounded-lg p-2 transition hover:bg-slate-100"
                    on:click={() => abrirEdicion(user)}
                  >
                    <Pencil size={18} />
                  </button>

                </div>

              </td>

            </tr>

          {/each}

          {#if paginatedUsers.length === 0}

            <tr>

              <td
                colspan="7"
                class="px-6 py-16 text-center"
              >

                <div class="flex flex-col items-center justify-center">

                  <h3 class="text-base font-semibold text-slate-700">
                    No hay usuarios registrados
                  </h3>

                  <p class="mt-1 text-sm text-slate-500">
                    No se encontraron usuarios
                  </p>

                </div>

              </td>

            </tr>

          {/if}

        </tbody>

      </table>

    </div>

  </div>

  <!-- PAGINACIÓN -->
  <div class="px-6 pb-6">

    <Pagination
      {currentPage}
      {totalPages}
      onPageChange={(page) => {

        currentPage = page;

      }}
    />

  </div>

</div>


<EditUserModal
  open={editModalOpen}
  user={selectedUser}
  onSave={guardarEdicion}
  on:close={() => {
    editModalOpen = false;
    selectedUser = null;
  }}
/>

<ViewUserModal
	open={viewModalOpen}
	user={selectedUser}
	on:close={() => {
		viewModalOpen = false;
		selectedUser = null;
	}}
/>

<DisableUserModal
	open={disableModalOpen}
	userName={userToDisable?.nombre_completo ?? ''}
	loading={disableLoading}
	on:cancel={() => {
		disableModalOpen = false;
		userToDisable = null;
	}}
	on:confirm={confirmarDeshabilitar}
/>

<EnableUserModal
	open={enableModalOpen}
	userName={userToEnable?.nombre_completo ?? ''}
	loading={enableLoading}
	on:cancel={() => {
		enableModalOpen = false;
		userToEnable = null;
	}}
	on:confirm={confirmarHabilitar}
/>