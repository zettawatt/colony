// src/stores/downloadManager.ts
import { writable } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';
import ps from './persistantStorage';

const store = await ps.getStore();

export type DownloadStatus = {
  id: string;
  path: string;
  estimated_size?: number;
  progress: number;
  complete: boolean;
  error?: string;
  elapsed?: string; // HH:MM:SS
};

type InternalDownloadStatus = DownloadStatus & {
  timer?: ReturnType<typeof setInterval>;
  secondsElapsed?: number;
};

function formatElapsed(seconds: number): string {
  const hrs = Math.floor(seconds / 3600)
    .toString()
    .padStart(2, '0');
  const mins = Math.floor((seconds % 3600) / 60)
    .toString()
    .padStart(2, '0');
  const secs = (seconds % 60).toString().padStart(2, '0');
  return `${hrs}:${mins}:${secs}`;
}

const { subscribe, update, set } = writable<Record<string, InternalDownloadStatus>>({});

// Load from store on init
(async () => {
  const saved = (await store.get<Record<string, DownloadStatus>>('downloadManager')) ?? {};
  const restored = Object.fromEntries(
    Object.entries(saved).map(([id, d]) => [
      id,
      {
        ...d,
        timer: d.complete ? undefined : startElapsedTimer(id),
      },
    ])
  );
  set(restored);
})();

// Save to store on change
subscribe(current => {
  const toSave = Object.fromEntries(
    Object.entries(current).map(([id, d]) => [
      id,
      {
        id: d.id,
        path: d.path,
        estimated_size: d.estimated_size,
        progress: d.progress,
        complete: d.complete,
        error: d.error,
      },
    ])
  );
  store.set('downloadManager', toSave);
  store.save(); // Writes to disk
});

function startElapsedTimer(id: string) {
  return setInterval(() => {
  update(downloads => {
    const d = downloads[id];
      if (!d || d.complete) return downloads;
      const secondsElapsed = (d.secondsElapsed ?? 0) + 1;
    return {
      ...downloads,
      [id]: {
        ...d,
          secondsElapsed,
          elapsed: formatElapsed(secondsElapsed),
        },
    };
  });
  }, 1000);
}
listen('download-started', event => {
  console.log('download-started', event);
  const { id, path, estimated_size } = event.payload as DownloadStatus;
  update(downloads => {
    const timer = startElapsedTimer(id);
    return {
      ...downloads,
      [id]: {
        id,
        path,
        estimated_size,
        progress: 0,
        complete: false,
        secondsElapsed: 0,
        elapsed: '00:00:00',
        timer,
      },
    };
  });
});

listen('download-complete', event => {
  console.log('download-complete', event);
  const { id } = event.payload as { id: string };

  update(downloads => {
    const d = downloads[id];
    if (d?.timer) clearInterval(d.timer);
    return {
      ...downloads,
      [id]: {
        ...d,
        progress: 100,
        complete: true,
        timer: undefined,
      },
};
  });
});

listen('download-error', event => {
  const { id, message } = event.payload as { id: string; message: string };

  update(downloads => {
    const d = downloads[id];
    if (d?.timer) clearInterval(d.timer);
    return {
      ...downloads,
      [id]: {
        ...d,
        error: message,
        complete: false,
        timer: undefined,
      },
    };
  });
});

export const downloadManager = {
  subscribe,
};
