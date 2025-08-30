import { invoke } from "@tauri-apps/api/core";
import { setPassword } from "../password/session";
import { getPrimaryWallet } from "../wallet/getPrimaryWallet";
import { startDweb } from "../dweb/dwebCommands";
import { startAnttp } from "../anttp/anttpCommands";
import { addToast } from "../../stores/toast";

export async function initColony(password: string) {
  try {
    await invoke("open_keystore", { password: password });
    await setPassword(password);
    const primaryWallet = await getPrimaryWallet();
    const walletKey = primaryWallet?.privateKey;
    if (!walletKey) {
      throw new Error("No primary wallet key found");
    }
    await invoke("initialize_autonomi_client", { walletKey });
    await invoke("initialize_pod_manager");
    await startDweb(walletKey);
    await startAnttp(walletKey);
    addToast("Connected to Autonomi Network!", "success");
  } catch (error) {
    addToast("Encounted an error on start up, see logs...", "error");
    console.error(error)
    throw error;
  }
}

export async function initDatastore () {
  try {
    await invoke("initialize_datastore");
    await invoke("initialize_graph");
  } catch (error) {
    addToast("Failed to initialized Colony datastore, see logs...", "error");
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