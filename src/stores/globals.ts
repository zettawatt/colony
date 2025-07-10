import { writable } from "svelte/store";

export const podsSyncing = writable(false);
export const allPodsUploading = writable(false);
export const globalTheme = writable("light");