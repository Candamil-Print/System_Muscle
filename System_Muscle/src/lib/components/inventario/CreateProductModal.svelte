<script lang="ts">
  import { onMount } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { readFile } from '@tauri-apps/plugin-fs';
  import {
    X,
    Image,
    ChevronUp,
    ChevronDown
  } from 'lucide-svelte';

  import { toast } from 'svelte-sonner';

  import { crearProducto } from '$lib/services/api/inventory';

  export let open = false;
  export let onClose: () => void = () => {};
  export let onCreated: () => void = () => {};

  let isDragging = false;

  let loading = false;

  let product = {
    nombre: '',
    tipo: 'Snacks',
    precioCosto: '',
    precioSugerido: '',
    stockMaximo: ''
  };

  let dialogElement: HTMLDivElement;

  let selectedImage: File | null = null;
  let previewImage = '';
  let fileInput: HTMLInputElement;
  let imageBase64 = '';

  onMount(() => {
  let unlisten:
    | (() => void)
    | undefined;

  const init = async () => {
    const appWindow =
      getCurrentWindow();

    unlisten =
      await appWindow.onDragDropEvent(
        async (event) => {
          switch (
            event.payload.type
          ) {
            case 'enter':
              isDragging = true;
              break;

            case 'leave':
              isDragging = false;
              break;

            case 'drop':
              isDragging = false;

              if (
                !event.payload.paths
                  .length
              )
                return;

              await processImagePath(
                event.payload.paths[0]
              );

              break;
          }
        }
      );
  };

  init();

  return () => {
    unlisten?.();
  };
});

  function handleCloseModal() {
    // limpiar formulario
    product = {
      nombre: '',
      tipo: 'Snacks',
      precioCosto: '',
      precioSugerido: '',
      stockMaximo: ''
    };

    // limpiar imagen
    selectedImage = null;
    previewImage = '';
    imageBase64 = '';

    onClose();
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      handleCloseModal();
    }
  }

  

  async function handleCreateProduct() {
    // VALIDACIONES
    if (!product.nombre.trim()) {
      toast.error('El nombre es obligatorio');
      return;
    }

    if (!product.precioCosto) {
      toast.error('Ingresa el precio costo');
      return;
    }

    if (!product.precioSugerido) {
      toast.error('Ingresa el precio sugerido');
      return;
    }

    if (!product.stockMaximo) {
      toast.error('Ingresa el stock máximo');
      return;
    }

    try {
      loading = true;

    await crearProducto({
      nombre: product.nombre,
      tipo_producto: product.tipo.toUpperCase(),
      precio_costo: cleanNumber(product.precioCosto),
      precio_sugerido: cleanNumber(product.precioSugerido),
      imagen_url: imageBase64 || '',
      stock_maximo: cleanNumber(product.stockMaximo)
    });

      console.log('Producto creado');

      toast.success('Producto creado correctamente');

      await onCreated();

      handleCloseModal();

    } catch (error) {
      console.error(error);

      toast.error('Error al crear producto');
    } finally {
      loading = false;
    }
  }

  function handleFileChange(event: Event) {
    const target = event.target as HTMLInputElement;

    if (target.files && target.files.length > 0) {
      processImage(target.files[0]);
    }
  }

  function processImage(file: File) {
  selectedImage = file;

  previewImage = URL.createObjectURL(file);

  const reader = new FileReader();

  reader.onload = () => {
    imageBase64 = reader.result as string;

    toast.success('Imagen cargada correctamente');
  };

  reader.readAsDataURL(file);
}

async function processImagePath(path: string) {
  try {
    const bytes = await readFile(path);

    const extension =
      path.split('.').pop()?.toLowerCase() || '';

    if (
      !['png', 'jpg', 'jpeg', 'webp'].includes(
        extension
      )
    ) {
      toast.error('Solo se permiten imágenes');
      return;
    }

    const blob = new Blob([bytes]);

    const fileName =
      path.split(/[\\/]/).pop() || 'imagen';

    const file = new File(
      [blob],
      fileName,
      {
        type:
          extension === 'jpg'
            ? 'image/jpeg'
            : `image/${extension}`
      }
    );

    processImage(file);
  } catch (error) {
    console.error(error);

    toast.error(
      'No se pudo cargar la imagen'
    );
  }
}


  function increaseValue(field: keyof typeof product) {
    const current = Number(product[field].replace(/\./g, '')) || 0;

    product = {
      ...product,
      [field]: formatNumber(String(current + 1))
    };
  }

  function decreaseValue(field: keyof typeof product) {
    const current = Number(product[field].replace(/\./g, '')) || 0;

    if (current <= 0) return;

    product = {
      ...product,
      [field]: formatNumber(String(current - 1))
    };
  }

  // FORMATO MILES EN EL MISMO INPUT
  function formatNumber(value: string) {
    const clean = value.replace(/\D/g, '');

    if (!clean) return '';

    return Number(clean).toLocaleString('es-CO');
  }

  function handleNumberInput(
    event: Event,
    field: keyof typeof product
  ) {
    const target = event.target as HTMLInputElement;

    product = {
      ...product,
      [field]: formatNumber(target.value)
    };
  }

  // LIMPIAR PUNTOS PARA ENVIAR A API
  function cleanNumber(value: string) {
    return Number(value.replace(/\./g, ''));
  }
</script>

{#if open}
  <!-- Overlay -->
  <button
    type="button"
    aria-label="Cerrar modal"
    class="fixed inset-0 z-90 bg-black/50 backdrop-blur-sm"
    onclick={handleCloseModal}
  ></button>

  <!-- Modal Container -->
  <div
    class="fixed inset-0 z-100 flex items-center justify-center p-4"
    role="button"
    tabindex="0"
    aria-label="Cerrar modal (clic o tecla Espacio/Enter)"
    onclick={handleCloseModal}
    onkeydown={(e) => {
      if (e.key === ' ') {
        e.preventDefault();
      }

      if (
        e.key === 'Escape' ||
        e.key === 'Enter' ||
        e.key === ' '
      ) {
        handleCloseModal();
      }
    }}
  >
    <!-- Modal -->
    <div
      bind:this={dialogElement}
      class="animate-in fade-in zoom-in duration-200 my-8 w-full max-w-2xl rounded-3xl border border-slate-200 bg-white shadow-2xl dark:bg-[#1E293B] dark:border-[#334156]"
      style="max-height: calc(100vh - 4rem); overflow-y: auto;"
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      onclick={(e) => e.stopPropagation()}
      onkeydown={(e) => e.stopPropagation()}
    >
      <!-- Header -->
      <div class="sticky top-0 rounded-t-3xl border-b border-slate-100 bg-white dark:bg-[#1E293B] dark:border-[#334156] p-6">
        <div class="flex items-start justify-between">
          <div>
            <h2 class="text-2xl font-bold text-slate-800 dark:text-white">
              Crear Producto
            </h2>

            <p class="mt-1 text-sm text-slate-500">
              Ingresa los datos del nuevo producto
            </p>
          </div>

          <button
            type="button"
            class="rounded-xl p-2 text-slate-400 transition hover:bg-slate-100 hover:text-slate-700 dark:hover:bg-[#162033]"
            onclick={handleCloseModal}
            aria-label="Cerrar"
          >
            <X size={20} />
          </button>
        </div>
      </div>

      <!-- Body -->
      <div class="space-y-5 p-5">
        <!-- Nombre -->
        <div>
          <label
            for="nombre"
            class="mb-1 block text-sm font-semibold text-slate-700 dark:text-white"
          >
            Nombre
          </label>

          <input
            bind:value={product.nombre}
            id="nombre"
            type="text"
            placeholder="Nombre del producto"
            class="w-full rounded-xl border border-slate-200 bg-white dark:bg-[#111827] dark:border-[#334156] dark:text-[#64748B] px-4 py-3 text-sm outline-none transition duration-200 focus:border-[#0C4A6E] focus:ring-4 focus:ring-sky-100 focus:scale-[1.01] dark:focus:border-[#39BDF8] dark:focus:ring-4 dark:focus:ring-[#39BDF8]/20"
          />
        </div>

        <!-- Tipo -->
        <div>
          <label
            for="tipo"
            class="mb-1 block text-sm font-semibold text-slate-700 dark:text-white"
          >
            Tipo
          </label>

          <div class="relative">
            <select
              bind:value={product.tipo}
              id="tipo"
              class="w-full appearance-none rounded-xl border border-slate-200 bg-white dark:bg-[#111827] dark:border-[#334156] dark:text-[#64748B] px-4 py-3 pr-11 text-sm outline-none transition duration-200 focus:border-[#0C4A6E] focus:ring-4 focus:ring-sky-100 focus:scale-[1.01] dark:focus:border-[#39BDF8] dark:focus:ring-4 dark:focus:ring-[#39BDF8]/20 "
            >
              <option>Snacks</option>
              <option>Suplementos</option>
              <option>Bebidas</option>
            </select>

            <!-- FLECHA -->
            <div class="pointer-events-none absolute inset-y-0 right-4 flex items-center">
              <svg
                class="h-4 w-4 text-slate-500"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M19 9l-7 7-7-7"
                />
              </svg>
            </div>
          </div>
        </div>

        <!-- Precios -->
        <div class="grid grid-cols-2 gap-3">
          <!-- Precio Costo -->
          <div>
            <label
              for="precioCosto"
              class="mb-1 block text-sm font-semibold text-slate-700 dark:text-white "
            >
              Precio Costo
            </label>

            <div
              class="group flex overflow-hidden rounded-xl border border-slate-200 bg-white dark:bg-[#111827] dark:border-[#334156] dark:text-[#64748B] transition duration-200 focus-within:border-[#0C4A6E] focus-within:ring-4 focus-within:ring-sky-100 focus-within:scale-[1.01] dark:focus-within:border-[#39BDF8] dark:focus-within:ring-4 dark:focus-within:ring-[#39BDF8]/20"
            >
              <!-- PREFIJO -->
              <div
                class="flex items-center border-r border-slate-200 bg-slate-50 px-4 text-sm font-medium text-slate-600 dark:bg-[#111827] dark:border-[#334156] dark:text-[#64748B]"
              >
                $
              </div>

              <!-- INPUT -->
              <input
                value={product.precioCosto}
                oninput={(e) =>
                  handleNumberInput(e, 'precioCosto')
                }
                id="precioCosto"
                type="text"
                inputmode="numeric"
                placeholder="0"
                class="w-full bg-transparent px-1 py-3 text-sm outline-none dark:bg-[#111827] dark:border-[#334156] dark:text-[#64748B]"
              />

              <!-- BOTONES -->
              <div class="ml-2 flex flex-col justify-center pr-2">
                <button
                  type="button"
                  class="flex h-5 w-5 items-center justify-center rounded-md text-slate-500 transition hover:bg-slate-100 hover:text-[#0C4A6E]"
                  onclick={() => increaseValue('precioCosto')}
                >
                  <ChevronUp size={12} />
                </button>

                <button
                  type="button"
                  class="flex h-5 w-5 items-center justify-center rounded-md text-slate-500 transition hover:bg-slate-100 hover:text-red-500"
                  onclick={() => decreaseValue('precioCosto')}
                >
                  <ChevronDown size={12} />
                </button>
              </div>
            </div>
          </div>

          <!-- Precio Sugerido -->
          <div>
            <label
              for="precioSugerido"
              class="mb-1 block text-sm font-semibold text-slate-700 dark:text-white"
            >
              Precio Sugerido
            </label>

            <div
              class="group flex overflow-hidden rounded-xl border border-slate-200 bg-white transition duration-200 focus-within:border-[#0C4A6E] focus-within:ring-4 focus-within:ring-sky-100 focus-within:scale-[1.01] dark:bg-[#111827] dark:border-[#334156] dark:text-[#64748B] dark:focus-within:border-[#39BDF8] dark:focus-within:ring-4 dark:focus-within:ring-[#39BDF8]/20"
            >
              <!-- PREFIJO -->
              <div
                class="flex items-center border-r border-slate-200 bg-slate-50 px-4 text-sm font-medium text-slate-600 dark:bg-[#111827] dark:border-[#334156] dark:text-[#64748B]"
              >
                $
              </div>

              <!-- INPUT -->
              <input
                value={product.precioSugerido}
                oninput={(e) =>
                  handleNumberInput(e, 'precioSugerido')
                }
                id="precioSugerido"
                type="text"
                inputmode="numeric"
                placeholder="0"
                class="w-full bg-transparent px-1 py-3 text-sm outline-none"
              />

              <!-- CONTROLES -->
              <div class="ml-2 flex flex-col justify-center pr-2">
                <button
                  type="button"
                  class="flex h-5 w-5 items-center justify-center rounded-md text-slate-500 transition hover:bg-slate-100 hover:text-[#0C4A6E]"
                  onclick={() => increaseValue('precioSugerido')}
                >
                  <ChevronUp size={12} />
                </button>

                <button
                  type="button"
                  class="flex h-5 w-5 items-center justify-center rounded-md text-slate-500 transition hover:bg-slate-100 hover:text-red-500"
                  onclick={() => decreaseValue('precioSugerido')}
                >
                  <ChevronDown size={12} />
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- Stock -->
        <div>
          <label
            for="stockMaximo"
            class="mb-1 block text-sm font-semibold text-slate-700 dark:text-white"
          >
            Stock Máximo
          </label>

          <div
            class="group flex overflow-hidden rounded-xl border border-slate-200 bg-white transition duration-200 focus-within:border-[#0C4A6E] focus-within:ring-4 focus-within:ring-sky-100 focus-within:scale-[1.01] dark:bg-[#111827] dark:border-[#334156] dark:text-[#64748B] dark:focus-within:border-[#39BDF8] dark:focus-within:ring-4 dark:focus-within:ring-[#39BDF8]/20"
          >
            <input
              value={product.stockMaximo}
              oninput={(e) =>
                handleNumberInput(e, 'stockMaximo')
              }
              id="stockMaximo"
              type="text"
              inputmode="numeric"
              placeholder="0"
              class="w-full bg-transparent px-4 py-3 text-sm outline-none "
            />

            <!-- CONTROLES -->
            <div class="flex flex-col justify-center pr-2">
              <button
                type="button"
                class="flex h-5 w-5 items-center justify-center rounded-md text-slate-500 transition hover:bg-slate-100 hover:text-[#0C4A6E]"
                onclick={() => increaseValue('stockMaximo')}
              >
                <ChevronUp size={12} />
              </button>

              <button
                type="button"
                class="flex h-5 w-5 items-center justify-center rounded-md text-slate-500 transition hover:bg-slate-100 hover:text-red-500"
                onclick={() => decreaseValue('stockMaximo')}
              >
                <ChevronDown size={12} />
              </button>
            </div>
          </div>
        </div>

        <!-- Upload -->
        <div>
          <p class="mb-1 block text-sm font-semibold text-slate-700 dark:text-white ">
            Imagen del producto
          </p>

          <!-- INPUT OCULTO -->
          <input
            bind:this={fileInput}
            type="file"
            accept="image/*"
            class="hidden "
            onchange={handleFileChange}
          />

          <!-- BOTON -->
          <div
            role="button"
            tabindex="0"
            class={`flex w-full cursor-pointer items-center gap-4 rounded-2xl border border-dashed p-4 transition duration-200 dark:bg-[#111827] dark:border-[#334156] dark:text-[#64748B] dark:hover:bg-[#162033]
            ${
              isDragging
                ? 'border-[#0C4A6E] bg-sky-50'
                : 'border-slate-300 bg-slate-50 hover:border-[#0C4A6E] hover:bg-slate-100'
            }`}
            onclick={() => fileInput.click()}
            onkeydown={(e) => {
              if (e.key === 'Enter' || e.key === ' ') {
                e.preventDefault();
                fileInput.click();
              }
            }}
            ondragenter={(e) => {
              e.preventDefault();
              e.stopPropagation();
              isDragging = true;
            }}
            ondragover={(e) => {
              e.preventDefault();
              e.stopPropagation();
              isDragging = true;
            }}
            ondragleave={(e) => {
              e.preventDefault();
              e.stopPropagation();
              isDragging = false;
            }}
          >
            <div class="flex h-14 w-14 items-center justify-center overflow-hidden rounded-2xl bg-slate-200 dark:border-[#334156] dark:bg-[#1E293B]  ">
              {#if previewImage}
                <img
                  src={previewImage}
                  alt="Preview"
                  class="h-full w-full object-cover"
                />
              {:else}
                <Image size={20} class="text-slate-500" />
              {/if}
            </div>

            <div class="text-left ">
              <p class="text-sm font-semibold text-slate-700 dark:text-white ">
                {#if selectedImage}
                  {selectedImage.name}
                {:else}
                  Imagen del producto
                {/if}
              </p>

              <p class="text-xs text-slate-500">
                {#if selectedImage}
                  Imagen seleccionada correctamente
                {:else}
                  Arrastra una imagen o haz clic para seleccionarla
                {/if}
              </p>
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="sticky bottom-0 rounded-b-3xl border-t border-slate-100 bg-white dark:bg-[#1E293B] dark:border-[#334156] p-5">
        <div class="flex justify-end gap-3">
          <button
            type="button"
            class="rounded-xl border border-slate-200 px-5 py-2.5 text-sm font-medium text-slate-700 dark:border-[#334156] dark:text-white transition hover:bg-slate-100 dark:hover:bg-[#162033]"
            onclick={handleCloseModal}
          >
            Cancelar
          </button>

          <button
            type="button"
            class="rounded-xl bg-[#0C4A6E] px-5 py-2.5 text-sm font-medium text-white dark:text-[#39BDF8] transition hover:bg-[#0a3a52] disabled:opacity-50"
            onclick={handleCreateProduct}
            disabled={loading}
          >
            {#if loading}
              Creando...
            {:else}
              Crear Producto
            {/if}
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}