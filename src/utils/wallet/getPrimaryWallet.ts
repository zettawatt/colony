import { invoke } from "@tauri-apps/api/core";
import ps from "../../stores/persistantStorage";
import { getPassword } from "../password/session";

export async function getPrimaryWallet() {
  try {
    let pw = await getPassword();
    let primaryWallet = await ps.getPrimaryWallet();
    console.log("Primary Wallet", primaryWallet)
    const wallets = await invoke('list_wallets');
    console.log(wallets)
    for (const [key, value] of Object.entries(wallets)) {
      if (key === primaryWallet) {
        console.log("hit")
        return {
          name: primaryWallet,
          privateKey: value
        };
      }
    }
  } catch (error) {
    console.error(error);
  }
}