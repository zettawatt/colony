import { writable } from 'svelte/store';

export type ToastType = 'success' | 'error' | 'info' | 'warning';

export interface Toast {
  id: number;
  message: string;
  type: ToastType;
  duration?: number;
}

const toasts = writable<Toast[]>([]);
let id = 0;

export function addToast(message: string, type: ToastType = 'info', duration = 3000) {
  const toast: Toast = { id: id++, message, type, duration };
  toasts.update(t => [...t, toast]);

  setTimeout(() => {
    toasts.update(t => t.filter(to => to.id !== toast.id));
  }, duration);
}

export default toasts;
