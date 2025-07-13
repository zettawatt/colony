import { invoke } from "@tauri-apps/api/core";
import { setPassword } from "../password/session";
import { getPrimaryWallet } from "../wallet/getPrimaryWallet";
import { startDweb } from "../dweb/dwebCommands";

export async function initColony(password: string) {
  try {
    await invoke("open_keystore", { password: password });
    await setPassword(password);
    const primaryWallet = await getPrimaryWallet();
    const walletKey = primaryWallet?.privateKey
    const client = await invoke("initialize_autonomi_client", { walletKey });
    const podManager = await invoke("initialize_pod_manager");
    await startDweb(walletKey)
  } catch (error) {
    console.error(error)
    throw error;
  }
}

export async function initDatastore () {
  try {
    await invoke("initialize_datastore");
    await invoke("initialize_graph");
  } catch (error) {
    console.error(error);
    throw error;
  }
}

// export async function checkForExistingColonyStore () {
//   try {
//     await 
//   } catch (error) {
//     console.error(error);
//     throw error;
//   }
// }