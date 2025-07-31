// src/stores/transferManager.ts
import { writable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';
import ps from './persistantStorage';
import { v4 as uuidv4 } from 'uuid';
import { formatFileSize } from '../utils/fileFormaters';

// The underlying store instance (initialized asynchronously)
let store: Awaited<ReturnType<typeof ps.getStore>> | null = null;

// Status type for transfer tasks (both upload & download)
export type TransferStatus = 
  | "Complete"
  | "Errored"
  | "Downloading"
  | "Uploading"
  | "Cancelled"
  | "Not Yet Uploaded"
  | "Pending";

// Kind of transfer
export type TransferType = 'download' | 'upload';

// Public-facing transfer info (persisted & exposed in store)
export type TransferInfo = {
  id: string;                       // Unique task identifier (UUID)
  name: string;                     // File name extracted from path
  type: TransferType;               // Upload or download
  path: string;                     // Local file path
  size?: string;                    // Bytes (optional, was estimated_size)
  progress: number;                 // 0-100 percent
  complete: boolean;                // Whether transfer finished
  error?: string;                   // Error message (if failed)
  status: TransferStatus;           // Current status
  startedDate: string;              // ISO date string when transfer started
};

// Internal transfer info (same as TransferInfo since we no longer need timers)
type InternalTransferInfo = TransferInfo;

// Utility: extract file name from path
function fileNameFromPath(filePath: string): string {
  return filePath.split(/[/\\]/).pop() || "";
}

// Svelte store for all tracked transfers (keyed by id)
const { subscribe, update, set } = writable<Record<string, InternalTransferInfo>>({});

// Internal state to prevent double-init and cleanup logic
let initialized = false;
let unsubStore: (() => void) | null = null;



// Set up Tauri event listeners to respond to transfer progress from backend
function connectListeners() {
  // Download events

  // On download start: add to store, status is "Downloading"
  listen('download-started', event => {
    const {id, address, path, size } = event.payload as { id: string; address: string; path: string; size?: number };
    const name = fileNameFromPath(path);
    update(transfers => {
      return {
        ...transfers,
        [id]: {
          id,
          name,
          type: 'download',
          path,
          size: formatFileSize(size) || undefined,
          progress: 0,
          complete: false,
          status: "Downloading",
          startedDate: new Date().toISOString()
        },
      };
    });
  });

  // On download complete: set as finished, mark status
  listen('download-complete', event => {
    console.log("download-complete");
    const { id } = event.payload as { id: string };
    update(transfers => {
      const t = transfers[id];
      return {
        ...transfers,
        [id]: {
          ...t,
          progress: 100,
          complete: true,
          status: "Complete"
        },
      };
    });
  });

  // On download error: mark status, add error message
  listen('download-error', event => {
    const { id, message } = event.payload as { id: string; message: string };
    update(transfers => {
      const t = transfers[id];
      return {
        ...transfers,
        [id]: {
          ...t,
          error: message,
          complete: false,
          status: "Errored"
        },
      };
    });
  });

  // Upload events follow same pattern:

  // On upload start: initialize new entry
  listen('upload-started', event => {
    console.log("upload-started");
    const { id, path, size } = event.payload as { id: string; path: string; size?: number };
    const name = fileNameFromPath(path);
    update(transfers => {
      return {
        ...transfers,
        [id]: {
          id,
          name,
          type: 'upload',
          path,
          size: formatFileSize(size) || undefined,
          progress: 0,
          complete: false,
          status: "Uploading",
          startedDate: new Date().toISOString()
        },
      };
    });
  });

  // On upload complete: mark as finished
  listen('upload-complete', event => {
    console.log("upload-complete");
    const { id } = event.payload as { id: string };
    update(transfers => {
      const t = transfers[id];
      return {
        ...transfers,
        [id]: {
          ...t,
          progress: 100,
          complete: true,
          status: "Complete"
        },
      };
    });
  });

  // On upload error: record error
  listen('upload-error', event => {
    console.log("upload-error");
    const { id, message } = event.payload as { id: string; message: string };
    update(transfers => {
      const t = transfers[id];
      return {
        ...transfers,
        [id]: {
          ...t,
          error: message,
          complete: false,
          status: "Errored"
        },
      };
    });
  });
}

// Initialize persistent store, restore prior transfers, set up listeners
async function init() {
  if (initialized) return;
  store = await ps.getStore();

  // Load persisted transfer info from storage (if available)
  const saved = (await store.get<Record<string, TransferInfo>>('transferManager')) ?? {};
  // Restore transfers, marking any in-progress transfers as errored since they can't continue
  const restored: Record<string, InternalTransferInfo> = Object.fromEntries(
    Object.entries(saved).map(([id, t]) => [
      id,
      {
        ...t,
        name: t.name ?? fileNameFromPath(t.path),
        startedDate: t.startedDate ?? new Date().toISOString(),
        status: t.status === "Uploading" || t.status === "Downloading" ? "Errored" : t.status
      },
    ])
  );
  set(restored);

  unsubStore = subscribe(current => {
    if (!store) return;
    const toSave = Object.fromEntries(
      Object.entries(current).map(([id, t]) => [
        id,
        {
          id: t.id,
          name: t.name,
          type: t.type,
          path: t.path,
          size: t.size,
          progress: t.progress,
          complete: t.complete,
          error: t.error,
          status: t.status,
          startedDate: t.startedDate
        },
      ])
    );
    store.set('transferManager', toSave);
    store.save();
  });

  connectListeners();  // Begin receiving backend events
  initialized = true;
  console.log("transfermanager - init");
}

// Clean up: unsubscribe from store and reset state
function cleanup() {
  if (unsubStore) unsubStore();
  initialized = false;
}

// The store as exported to Svelte UI
export const transferManager = {
  subscribe,   // Svelte store subscription
  update,      // Allow external updates to the store
  init,        // Call this on app/component mount
  cleanup,     // Call this on unmount
};