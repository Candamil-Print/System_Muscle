import { writable } from 'svelte/store';

export interface CartItem {
  id_producto: number;
  nombre: string;
  precio: number;
  cantidad: number;
  imagen?: string;
}

export const cart = writable<CartItem[]>([]);

// AGREGAR PRODUCTO
export function addToCart(producto: CartItem) {

  cart.update((items) => {

    const existing = items.find(
      (item) => item.nombre === producto.nombre
    );

    // SI YA EXISTE
    if (existing) {

      existing.cantidad += 1;

      return [...items];

    }

    // NUEVO
    return [

      ...items,

      {
        ...producto,
        cantidad: 1
      }

    ];

  });

}

