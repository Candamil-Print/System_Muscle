import { invoke } from '@tauri-apps/api/core';

import type {
  ProductoConStock,
  NuevoProducto
} from './inventory.types';


// LISTAR PRODUCTOS
export async function listarProductos(): Promise<ProductoConStock[]> {
  return await invoke('listar_productos', {
    soloActivos: true
  });
}


// CREAR PRODUCTO
export async function crearProducto(
  producto: NuevoProducto
): Promise<number> {

  return await invoke('crear_producto', {
    nuevo: producto
  });
}

// MODIFICAR PRODUCTO
export async function modificarProducto(
  id: number,
  datos: any
): Promise<void> {

  return await invoke('modificar_producto', {
    id,
    datos
  });
}


// BUSCAR PRODUCTOS
export async function buscarProductos(
  termino: string
): Promise<ProductoConStock[]> {

  return await invoke('buscar_productos', {
    termino
  });
}


// OBTENER PRODUCTO
export async function obtenerProducto(
  id: number
): Promise<ProductoConStock> {

  return await invoke('obtener_producto', {
    id
  });
}

// ELIMINAR PRODUCTO
export async function eliminarProducto(
  id: number
): Promise<void> {

  return await invoke('eliminar_producto', {
    id
  });
}
