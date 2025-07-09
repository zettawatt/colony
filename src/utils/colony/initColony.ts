import { invoke } from "@tauri-apps/api/core";
import { setPassword } from "../password/session";
import { getPrimaryWallet } from "../wallet/getPrimaryWallet";

export async function initColony(password: string) {
  try {
    await invoke("open_keystore", { password: password });
    await setPassword(password);
    const primaryWallet = await getPrimaryWallet();
    const walletKey = primaryWallet?.privateKey
    console.log("primaryWallet", primaryWallet)
    const client = await invoke("initialize_autonomi_client", { walletKey });
    const podManager = await invoke("initialize_pod_manager");
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