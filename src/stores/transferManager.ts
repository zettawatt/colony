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
  elapsed?: string;                 // Elapsed time as "HH:MM:SS"
  status: TransferStatus;           // Current status
  startedDate: string;              // ISO date string when transfer started
};

// Internal transfer info (adds timer/secondsElapsed, not persisted)
type InternalTransferInfo = TransferInfo & {
  timer?: ReturnType<typeof setInterval>; // Interval timer to update time
  secondsElapsed?: number;                // Seconds since started
};

// Utility: format seconds as "HH:MM:SS"
function formatElapsed(seconds: number): string {
  const hrs = Math.floor(seconds / 3600).toString().padStart(2, '0');
  const mins = Math.floor((seconds % 3600) / 60).toString().padStart(2, '0');
  const secs = (seconds % 60).toString().padStart(2, '0');
  return `${hrs}:${mins}:${secs}`;
}

// Utility: extract file name from path
function fileNameFromPath(filePath: string): string {
  return filePath.split(/[/\\]/).pop() || "";
}

// Svelte store for all tracked transfers (keyed by id)
const { subscribe, update, set } = writable<Record<string, InternalTransferInfo>>({});

// Internal state to prevent double-init and cleanup logic
let initialized = false;
let unsubStore: (() => void) | null = null;

// Start a timer that increments elapsed time for a transfer task
function startElapsedTimer(id: string) {
  // Fires every second; updates store with +1 elapsed second & updates elapsed string
  return setInterval(() => {
    update(transfers => {
      const t = transfers[id];
      // Don't increment if missing or already complete
      if (!t || t.complete) return transfers;
      const secondsElapsed = (t.secondsElapsed ?? 0) + 1;
      return {
        ...transfers,
        [id]: {
          ...t,
          secondsElapsed,
          elapsed: formatElapsed(secondsElapsed),
        },
      };
    });
  }, 1000);
}

// Set up Tauri event listeners to respond to transfer progress from backend
function connectListeners() {
  // Download events

  // On download start: add to store, begin timer, status is "Downloading"
  console.log("download-started");
  listen('download-started', event => {
    const {id, address, path, size } = event.payload as { id: string; address: string; path: string; size?: number };
    const name = fileNameFromPath(path);
    update(transfers => {
      const timer = startElapsedTimer(id);
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
          secondsElapsed: 0,
          elapsed: '00:00:00',
          timer,
          status: "Downloading",
          startedDate: new Date().toISOString()
        },
      };
    });
  });

  // On download complete: set as finished, stop timer, mark status
  listen('download-complete', event => {
    console.log("download-complete");
    const { id } = event.payload as { id: string };
    update(transfers => {
      const t = transfers[id];
      if (t?.timer) clearInterval(t.timer);
      return {
        ...transfers,
        [id]: {
          ...t,
          progress: 100,
          complete: true,
          timer: undefined,
          status: "Complete"
        },
      };
    });
  });

  // On download error: stop timer, mark status, add error message
  listen('download-error', event => {
    const { id, message } = event.payload as { id: string; message: string };
    update(transfers => {
      const t = transfers[id];
      if (t?.timer) clearInterval(t.timer);
      return {
        ...transfers,
        [id]: {
          ...t,
          error: message,
          complete: false,
          timer: undefined,
          status: "Errored"
        },
      };
    });
  });

  // Upload events follow same pattern:

  // On upload start: initialize new entry, start timer
  listen('upload-started', event => {
    console.log("upload-started");
    const { id, path, size } = event.payload as { id: string; path: string; size?: number };
    const name = fileNameFromPath(path);
    update(transfers => {
      const timer = startElapsedTimer(id);
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
          secondsElapsed: 0,
          elapsed: '00:00:00',
          timer,
          status: "Uploading",
          startedDate: new Date().toISOString()
        },
      };
    });
  });

  // On upload complete: mark as finished, stop timer
  listen('upload-complete', event => {
    console.log("upload-complete");
    const { id } = event.payload as { id: string };
    update(transfers => {
      const t = transfers[id];
      if (t?.timer) clearInterval(t.timer);
      return {
        ...transfers,
        [id]: {
          ...t,
          progress: 100,
          complete: true,
          timer: undefined,
          status: "Complete"
        },
      };
    });
  });

  // On upload error: record error, stop timer
  listen('upload-error', event => {
    console.log("upload-error");
    const { id, message } = event.payload as { id: string; message: string };
    update(transfers => {
      const t = transfers[id];
      if (t?.timer) clearInterval(t.timer);
      return {
        ...transfers,
        [id]: {
          ...t,
          error: message,
          complete: false,
          timer: undefined,
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
  // Note: timers are not restored for completed transfers; errored states TODO (see comment)
  const restored: Record<string, InternalTransferInfo> = Object.fromEntries(
    Object.entries(saved).map(([id, t]) => [
      id,
      {
        ...t,
        name: t.name ?? fileNameFromPath(t.path),
        timer: t.complete ? undefined : startElapsedTimer(id),
        // Parse elapsed string back to seconds, if available
        secondsElapsed: t.elapsed
          ? (t.elapsed.split(':').reduce((acc, v, idx) =>
            idx === 0
              ? acc + +v * 3600
              : idx === 1
              ? acc + +v * 60
              : acc + +v
            , 0)
          )
          : t.complete ? undefined : 0,
        startedDate: t.startedDate ?? new Date().toISOString()
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
          size: t.size, // Save as size, not estimated_size
          progress: t.progress,
          complete: t.complete,
          error: t.error,
          elapsed: t.elapsed,
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
  init,        // Call this on app/component mount
  cleanup,     // Call this on unmount
};