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

export async function checkActiveWalletBalance(): Promise<{ hasBalance: boolean; ethBalance: number; antBalance: number }> {
  try {
    // Get the active wallet
    const [walletName, walletAddress] = await invoke('get_active_wallet') as [string, string];

    // Get the wallet key
    const walletKey = await invoke('get_wallet', { name: walletName }) as string;

    // Get the wallet balance
    const [antBalance, ethBalance] = await invoke('get_wallet_balance', { walletKey }) as [number, number];

    // Check if both balances are greater than zero
    const hasBalance = ethBalance > 0 && antBalance > 0;

    return {
      hasBalance,
      ethBalance,
      antBalance
    };
  } catch (error) {
    console.error('Error checking wallet balance:', error);
    return {
      hasBalance: false,
      ethBalance: 0,
      antBalance: 0
    };
  }
}