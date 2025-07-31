import { invoke } from "@tauri-apps/api/core";
import ps from "../../stores/persistantStorage";
import { getPassword } from "../password/session";

export async function getPrimaryWallet(): Promise<WalletInfo | null> {
  try {
    await getPassword();
    let primaryWallet = await ps.getPrimaryWallet();
    console.log("Primary Wallet", primaryWallet)
    const wallets = await invoke('list_wallets') as WalletInfo[];
    for (const wallet of wallets) {
      if (wallet.name === primaryWallet) {
        return {
          name: primaryWallet,
          privateKey: wallet.key,
          address: wallet.address,
          key: wallet.key
        };
      }
    }
    return null;
  } catch (error) {
    console.error(error);
    return null;
  }
}