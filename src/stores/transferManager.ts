// src/stores/transferManager.ts
import { writable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';
import ps from './persistantStorage';

let store: Awaited<ReturnType<typeof ps.getStore>> | null = null;

export type TransferStatus = 
  | "Complete"
  | "Errored"
  | "Downloading"
  | "Uploading"
  | "Cancelled"
  | "Not Yet Uploaded";

export type TransferType = 'download' | 'upload';
export type TransferInfo = {
  id: string;
  type: TransferType;
  path: string;
  estimated_size?: number;
  progress: number;
  complete: boolean;
  error?: string;
  elapsed?: string; // HH:MM:SS
  status: TransferStatus
};

type InternalTransferInfo = TransferInfo & {
  timer?: ReturnType<typeof setInterval>;
  secondsElapsed?: number;
};

function formatElapsed(seconds: number): string {
  const hrs = Math.floor(seconds / 3600).toString().padStart(2, '0');
  const mins = Math.floor((seconds % 3600) / 60).toString().padStart(2, '0');
  const secs = (seconds % 60).toString().padStart(2, '0');
  return `${hrs}:${mins}:${secs}`;
}

const { subscribe, update, set } = writable<Record<string, InternalTransferInfo>>({});

let initialized = false;
let unsubStore: (() => void) | null = null;

function startElapsedTimer(id: string) {
  return setInterval(() => {
    update(transfers => {
      const t = transfers[id];
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

function connectListeners() {
  // Download listeners
  console.log("download-started")
  listen('download-started', event => {
    const { id, path, estimated_size } = event.payload as { id: string; path: string; estimated_size?: number };
    update(transfers => {
      const timer = startElapsedTimer(id);
      return {
        ...transfers,
        [id]: {
          id,
          type: 'download',
          path,
          estimated_size,
          progress: 0,
          complete: false,
          secondsElapsed: 0,
          elapsed: '00:00:00',
          timer,
          status: "Downloading"
        },
      };
    });
  });

  listen('download-complete', event => {
    console.log("download-complete")
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

  // Upload listeners
  listen('upload-started', event => {
    console.log("upload-started")
    const { id, path, estimated_size } = event.payload as { id: string; path: string; estimated_size?: number };
    update(transfers => {
      const timer = startElapsedTimer(id);
      return {
        ...transfers,
        [id]: {
          id,
          type: 'upload',
          path,
          estimated_size,
          progress: 0,
          complete: false,
          secondsElapsed: 0,
          elapsed: '00:00:00',
          timer,
          status: "Uploading"
        },
      };
    });
  });

  listen('upload-complete', event => {
    console.log("upload-complete")
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

  listen('upload-error', event => {
    console.log("upload-error")
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

async function init() {
  if (initialized) return;
  store = await ps.getStore();
  const saved = (await store.get<Record<string, TransferInfo>>('transferManager')) ?? {};
  const restored: Record<string, InternalTransferInfo> = Object.fromEntries(
    Object.entries(saved).map(([id, t]) => [
      id,
      {
        ...t,
        timer: t.complete ? undefined : startElapsedTimer(id),
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
          type: t.type,
          path: t.path,
          estimated_size: t.estimated_size,
          progress: t.progress,
          complete: t.complete,
          error: t.error,
          elapsed: t.elapsed,
          status: t.status
        },
      ])
    );
    store.set('transferManager', toSave);
    store.save();
  });

  connectListeners();
  initialized = true;
  console.log("transfermanager - init")
}

function cleanup() {
  if (unsubStore) unsubStore();
  initialized = false;
}

export const transferManager = {
  subscribe,
  init,
  cleanup,
};
