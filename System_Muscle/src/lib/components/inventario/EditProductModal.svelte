<script lang="ts">
  import { Package, X, Image, ChevronDown, ChevronUp } from 'lucide-svelte';

  import { modificarProducto } from '$lib/services/api/inventory';

  export let open = false;

  export let product: {
    id: number;
    name: string;
    type: string;
    cost: number;
    sale: number;
    stock: number;
    image: string;
  };

  export let onClose: () => void = () => {};
  export let onUpdated: () => void = () => {};

  let loading = false;

  let nombre = '';
  let tipo = '';
  let precioCosto = '';
  let precioSugerido = '';
  let stock = '';

  let selectedImage: File | null = null;
  let previewImage = '';

  let fileInput: HTMLInputElement;
  let dragActive = false;

// llenar formulario SOLO cuando cambia el producto
let lastProductId: number | null = null;

$: if (open && product && product.id !== lastProductId) {
  nombre = product.name;
  tipo = product.type;
  precioCosto = formatNumber(String(product.cost));
  precioSugerido = formatNumber(String(product.sale));
  stock = formatNumber(String(product.stock));

  selectedImage = null;
  previewImage = product.image || '';

  lastProductId = product.id;
}

$: if (!open) {
  lastProductId = null;
}

function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement;

  if (target.files && target.files.length > 0) {
    processImage(target.files[0]);
  }
}

function processImage(file: File) {
  if (!file.type.startsWith('image/')) {
    alert('Solo se permiten imágenes');
    return;
  }

  selectedImage = file;

  if (previewImage?.startsWith('blob:')) {
    URL.revokeObjectURL(previewImage);
  }

  previewImage = URL.createObjectURL(file);

  console.log('Imagen seleccionada:', file.name);
  console.log('Preview:', previewImage);
}

function openImageSelector() {
  fileInput?.click();
}

  function handleDragEnter(e: DragEvent) {
    e.preventDefault();
    dragActive = true;
  }

  function handleDragLeave(e: DragEvent) {
    e.preventDefault();
    dragActive = false;
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
  }

function handleDrop(e: DragEvent) {
  e.preventDefault();
  dragActive = false;

  const files = e.dataTransfer?.files;

  if (!files || files.length === 0) return;

  const file = files[0];

  if (!file.type.startsWith('image/')) return;

  processImage(file);
}

  function increaseValue(field: 'precioCosto' | 'precioSugerido' | 'stock') {
    const current = Number(
      (field === 'precioCosto' ? precioCosto : field === 'precioSugerido' ? precioSugerido : stock).replace(/\./g, '')
    ) || 0;

    const newValue = formatNumber(String(current + 1));

    if (field === 'precioCosto') {
      precioCosto = newValue;
    } else if (field === 'precioSugerido') {
      precioSugerido = newValue;
    } else {
      stock = newValue;
    }
  }

  function decreaseValue(field: 'precioCosto' | 'precioSugerido' | 'stock') {
    const current = Number(
      (field === 'precioCosto' ? precioCosto : field === 'precioSugerido' ? precioSugerido : stock).replace(/\./g, '')
    ) || 0;

    if (current <= 0) return;

    const newValue = formatNumber(String(current - 1));

    if (field === 'precioCosto') {
      precioCosto = newValue;
    } else if (field === 'precioSugerido') {
      precioSugerido = newValue;
    } else {
      stock = newValue;
    }
  }

  function formatNumber(value: string) {
    const clean = value.replace(/\D/g, '');

    if (!clean) return '';

    return Number(clean).toLocaleString('es-CO');
  }

  function handleNumberInput(
    event: Event,
    field: 'precioCosto' | 'precioSugerido' | 'stock'
  ) {
    const target = event.target as HTMLInputElement;

    const newValue = formatNumber(target.value);

    if (field === 'precioCosto') {
      precioCosto = newValue;
    } else if (field === 'precioSugerido') {
      precioSugerido = newValue;
    } else {
      stock = newValue;
    }
  }

  function cleanNumber(value: string) {
    return Number(value.replace(/\./g, ''));
  }

  async function handleUpdate() {
    try {
      loading = true;

      await modificarProducto(product.id, {
        nombre,
        tipo_producto: tipo,
        precio_costo: cleanNumber(precioCosto),
        precio_sugerido: cleanNumber(precioSugerido),
        stock_maximo: cleanNumber(stock),
        imagen_url: previewImage
      });

      onUpdated();
      onClose();

    } catch (error) {
      console.error(error);
      alert('Error al actualizar producto');
    } finally {
      loading = false;
    }
  }
</script>

{#if open}
  <!-- Overlay -->
  <button
    class="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm"
    onclick={onClose}
  ></button>

  <!-- Container -->
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4">

    <!-- Modal -->
    <div
      class="w-full max-w-lg rounded-2xl bg-white shadow-2xl"
    >

      <!-- HEADER -->
      <div class="border-b border-slate-100 p-6">

        <div class="flex items-start justify-between">

          <div>
            <h2 class="text-xl font-bold text-slate-800">
              Editar Producto
            </h2>

            <p class="mt-1 text-sm text-slate-500">
              Modifica los datos del producto
            </p>
          </div>

          <button
            class="rounded-lg p-2 text-slate-400 transition hover:bg-slate-100"
            onclick={onClose}
          >
            <X size={18} />
          </button>

        </div>
      </div>

 
      <!-- BODY -->
      <div class="space-y-5 p-6">

        <!-- IMAGE UPLOAD -->
        <div>
          <p class="mb-2 block text-sm font-semibold text-slate-700">
            Imagen del producto
          </p>

          <!-- INPUT OCULTO -->
          <input
            bind:this={fileInput}
            type="file"
            accept="image/*"
            class="hidden"
            onchange={handleFileChange}
          />

          <!-- DRAG & DROP AREA -->
          <div
            class={`flex w-full cursor-pointer items-center gap-4 rounded-2xl border border-dashed p-4 transition duration-200
            ${
              dragActive
                ? 'border-[#0C4A6E] bg-sky-50'
                : 'border-slate-300 bg-slate-50 hover:border-[#0C4A6E] hover:bg-slate-100'
            }`}
            ondragenter={handleDragEnter}
            ondragleave={handleDragLeave}
            ondragover={handleDragOver}
            ondrop={handleDrop}
            onclick={openImageSelector}
          >
            <div class="flex h-14 w-14 items-center justify-center overflow-hidden rounded-2xl bg-slate-200">
              {#if previewImage}
                <img
                  src={previewImage}
                  alt={nombre}
                  class="h-full w-full object-cover"
                />
              {:else}
                <Image size={20} class="text-slate-500" />
              {/if}
            </div>

            <div class="flex-1 text-left">
              <p class="text-sm font-semibold text-slate-700">
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

        <!-- Nombre -->
        <div>
          <label class="mb-2 block text-sm font-semibold text-slate-700">
            Nombre
          </label>

          <input
            bind:value={nombre}
            type="text"
            class="w-full rounded-xl border border-slate-200 bg-white px-4 py-3 text-sm outline-none transition focus:border-[#0C4A6E] focus:ring-4 focus:ring-sky-100"
          />
        </div>

        <!-- Tipo -->
        <div>
          <label class="mb-2 block text-sm font-semibold text-slate-700">
            Tipo
          </label>

          <div class="relative">
            <select
              bind:value={tipo}
              class="w-full appearance-none rounded-xl border border-slate-200 bg-white px-4 py-3 pr-11 text-sm text-slate-700 outline-none transition duration-200 focus:border-[#0C4A6E] focus:ring-4 focus:ring-sky-100 focus:scale-[1.01]"
            >
              <option value="SNACKS">Snacks</option>
              <option value="SUPLEMENTOS">Suplementos</option>
              <option value="BEBIDAS">Bebidas</option>
            </select>

            <ChevronDown
              size={18}
              class="pointer-events-none absolute right-3 top-[0.9rem] text-slate-400"
            />
          </div>
        </div>

        <!-- Precios -->
        <div class="grid grid-cols-2 gap-3">
          <!-- Precio Costo -->
          <div>
            <label class="mb-1 block text-sm font-semibold text-slate-700">
              Precio Costo
            </label>

            <div
              class="group flex overflow-hidden rounded-xl border border-slate-200 bg-white transition duration-200 focus-within:border-[#0C4A6E] focus-within:ring-4 focus-within:ring-sky-100 focus-within:scale-[1.01]"
            >
              <!-- PREFIJO -->
              <div
                class="flex items-center border-r border-slate-200 bg-slate-50 px-4 text-sm font-medium text-slate-600"
              >
                $
              </div>

              <!-- INPUT -->
              <input
                value={precioCosto}
                oninput={(e) => handleNumberInput(e, 'precioCosto')}
                type="text"
                inputmode="numeric"
                placeholder="0"
                class="w-full bg-transparent px-1 py-3 text-sm outline-none"
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
            <label class="mb-1 block text-sm font-semibold text-slate-700">
              Precio Sugerido
            </label>

            <div
              class="group flex overflow-hidden rounded-xl border border-slate-200 bg-white transition duration-200 focus-within:border-[#0C4A6E] focus-within:ring-4 focus-within:ring-sky-100 focus-within:scale-[1.01]"
            >
              <!-- PREFIJO -->
              <div
                class="flex items-center border-r border-slate-200 bg-slate-50 px-4 text-sm font-medium text-slate-600"
              >
                $
              </div>

              <!-- INPUT -->
              <input
                value={precioSugerido}
                oninput={(e) => handleNumberInput(e, 'precioSugerido')}
                type="text"
                inputmode="numeric"
                placeholder="0"
                class="w-full bg-transparent px-1 py-3 text-sm outline-none"
              />

              <!-- BOTONES -->
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

        <!-- Stock Máximo -->
        <div>
          <label class="mb-2 block text-sm font-semibold text-slate-700">
            Stock Máximo
          </label>

          <div
            class="group flex overflow-hidden rounded-xl border border-slate-200 bg-white transition duration-200 focus-within:border-[#0C4A6E] focus-within:ring-4 focus-within:ring-sky-100 focus-within:scale-[1.01]"
          >
            <input
              value={stock}
              oninput={(e) => handleNumberInput(e, 'stock')}
              type="text"
              inputmode="numeric"
              placeholder="0"
              class="w-full bg-transparent px-4 py-3 text-sm outline-none"
            />

            <!-- BOTONES -->
            <div class="flex flex-col justify-center pr-2">
              <button
                type="button"
                class="flex h-5 w-5 items-center justify-center rounded-md text-slate-500 transition hover:bg-slate-100 hover:text-[#0C4A6E]"
                onclick={() => increaseValue('stock')}
              >
                <ChevronUp size={12} />
              </button>

              <button
                type="button"
                class="flex h-5 w-5 items-center justify-center rounded-md text-slate-500 transition hover:bg-slate-100 hover:text-red-500"
                onclick={() => decreaseValue('stock')}
              >
                <ChevronDown size={12} />
              </button>
            </div>
          </div>
        </div>

      </div>

      

      <!-- FOOTER -->
      <div class="flex justify-end gap-3 border-t border-slate-100 p-6">

        <button
          class="rounded-xl border border-slate-200 px-4 py-2 text-sm font-medium text-slate-700 transition hover:bg-slate-100"
          onclick={onClose}
        >
          Cancelar
        </button>

        <button
          class="rounded-xl bg-[#0C4A6E] px-4 py-2 text-sm font-medium text-white transition hover:bg-[#0a3a52] disabled:opacity-50"
          onclick={handleUpdate}
          disabled={loading}
        >
          {#if loading}
            Guardando...
          {:else}
            Guardar Cambios
          {/if}
        </button>

      </div>
    </div>
  </div>
{/if}