import { invoke } from '@tauri-apps/api/core';

// To start the dweb server
export async function startDweb(walletKey: string) {
  try {
    const message = await invoke('dweb_serve', { walletKey });
    console.log("DWEB", message); // e.g., "Started dweb serve with network: ..."
  } catch (error) {
    console.error('Could not start dweb:', error);
    console.error(error);
  }
}

// To open an address with dweb
export async function openDweb(address: string) {
  try {
    const message = await invoke('dweb_open', { address });
    console.log(message); // "Opened address with dweb"
  } catch (error) {
    console.error('Could not open address with dweb:', error);
  }
}