import { invoke } from '@tauri-apps/api/core';

// To start the anttp server
export async function startAnttp(walletKey: string) {
  try {
    const message = await invoke('anttp_start', { walletKey });
    console.log("ANTTP", message); // e.g., "Started anttp server with network: ..."
  } catch (error) {
    console.error('Could not start anttp:', error);
    console.error(error);
  }
}

// To stop the anttp server
export async function stopAnttp() {
  try {
    const message = await invoke('anttp_stop');
    console.log("ANTTP", message); // e.g., "Stopped anttp server"
  } catch (error) {
    console.error('Could not stop anttp:', error);
    console.error(error);
  }
}
