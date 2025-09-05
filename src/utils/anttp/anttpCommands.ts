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

// To open an address with anttp
export async function openAnttp(address: string) {
  try {
    const message = await invoke('anttp_open', { address });
    console.log(message); // "Opened anttp address in browser: ..."
  } catch (error) {
    console.error('Could not open address with anttp:', error);
  }
}
