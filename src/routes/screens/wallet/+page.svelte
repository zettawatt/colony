<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import ps from "../../../stores/persistantStorage";
  import { onMount } from "svelte";

  import { getPassword } from "../../../utils/password/session";
    import { addToast } from "../../../stores/toast";
  import AddressDisplay from "../../../components/AddressDisplay.svelte";
  import { isMobile } from '../../../utils/responsive.js';

  // Android detection
  const isAndroid = typeof window !== 'undefined' && /Android/i.test(navigator.userAgent);

  let primaryWalletName = $state("");
  let storedWallets = $state<WalletInfo[]>([]);
  let walletBalances = $state<Record<string, { ant_balance?: number; gas_balance?: number; loading: boolean }>>({});
  let activeWallet = $state<any>({
    name: "",
    key: "",
    address: "",
    makePrimary: false,
  });
  let referenceWallet = $state<any>({
    name: "",
    key: "",
    address: ""
  });

  // Modal references
  let editWalletModal: HTMLDialogElement;
  let addWalletModal: HTMLDialogElement;
  let deleteWalletModal: HTMLDialogElement;

  // Add a wallet
  export async function addWallet(name: string, key: string): Promise<void> {
    try {
      let pw = await getPassword();
      // request shape should match AddWalletRequest
      const response = await invoke('add_wallet', {
        request: { name, key }
      });
      await invoke("write_keystore_to_file", {password: pw})
      console.log(response); // "Wallet added"
    } catch (error) {
      console.error('Error adding wallet:', error);
    }
  }

  // Remove a wallet by name
  export async function removeWallet(name: string): Promise<void> {
    try {
      let pw = await getPassword();
      const response = await invoke('remove_wallet', { name });
      await invoke("write_keystore_to_file", {password: pw})
      console.log(response); // "Wallet removed"
    } catch (error) {
      console.error('Error removing wallet:', error);
    }
  }

  // Delete wallet handler for the modal
  async function deleteWalletHandler() {
    try {
      await getPassword();
      deleteWalletModal.close();
      await removeWallet(activeWallet.name);
      await loadTable();
      addToast("Wallet deleted!", "success");
    } catch (error) {
      console.error(error);
      addToast("Error removing wallet. Check logs...", "error");
    }
  }

  // List wallets (returns object of { walletName: key } or similar)
  export async function listWallets() {
    try {
      const wallets = await invoke('list_wallets');
      // console.log("wallets", wallets)
      return wallets;
    } catch (error) {
      console.error('Error listing wallets:', error);
      return [];
    }
  }

  // Switch to a wallet by name
  export async function switchWallet(name:string) {
    try {
      let pw = await getPassword();
      const response = await invoke('switch_wallet', { name });
      await invoke("write_keystore_to_file", {password: pw})
      await ps.setPrimaryWallet(name);
      await setActiveWallet(name);
      console.log(response); // "Wallet switched"
      addToast("Primary wallet has been switched!", "success");
      return response;
    } catch (error) {
      console.error('Error switching wallet:', error);
    }
  }
  
  async function addNewWallet(){
    const addRes = await addWallet(activeWallet.name, activeWallet.key);
    console.log("addNewWallet response:", addRes);
    if (activeWallet.makePrimary) {
      const switchRes = await switchWallet(activeWallet.name);
      console.log("switchWallet response:", switchRes);
    }
    await loadTable();
  }

  async function updateWallet() {
    if (activeWallet.name !== referenceWallet.name) {
      const removeRes = await removeWallet(referenceWallet.name);
      console.log("removeWallet response:", removeRes);
      const addRes = await addWallet(activeWallet.name, activeWallet.key);
      console.log("addWallet response:", addRes);
    } else if (activeWallet.key !== referenceWallet.key) {
      const addRes = await addWallet(activeWallet.name, activeWallet.key);
      console.log("addWallet response:", addRes);
    }
    if (activeWallet.makePrimary) {
      const switchRes = await switchWallet(activeWallet.name);
      console.log("switchWallet response:", switchRes);
    }
    await loadTable();
  }



  const setActiveWallet = async (name: string): Promise<void> => {
    try {
      const result = await invoke('set_active_wallet', { name }) as [string, string];
      const [walletName, address] = result;
      console.log('Set active wallet:', walletName, address);
    } catch (error) {
      console.error('Error:', error);
    }
  };

  // Function to fetch balance for a single wallet
  async function fetchWalletBalance(walletName: string, walletKey: string) {
    try {
      // Set loading state
      walletBalances[walletName] = { loading: true };

      // Fetch balance from Tauri command
      const result = await invoke('get_wallet_balance', { walletKey }) as [number, number];
      const [ant_balance, gas_balance] = result;

      // Update balance state
      walletBalances[walletName] = {
        ant_balance,
        gas_balance,
        loading: false
      };
    } catch (error) {
      console.error(`Error fetching balance for wallet ${walletName}:`, error);
      // Set error state
      walletBalances[walletName] = { loading: false };
    }
  }

  async function loadTable() {
    try {
      primaryWalletName = await ps.getPrimaryWallet();
      let wallets = await listWallets() as WalletInfo[];
      // Sort so primary wallet is first
      storedWallets = wallets.sort((a: WalletInfo, b: WalletInfo) => {
        if (a.name === primaryWalletName) return -1;
        if (b.name === primaryWalletName) return 1;
        return 0;
      });

      // Initialize balance loading states and start concurrent fetching
      walletBalances = {};
      for (const wallet of storedWallets) {
        // Start fetching balance concurrently (don't await)
        fetchWalletBalance(wallet.name, wallet.key);
      }

      // console.log("storedWallets", storedWallets);
    } catch (error) {
      console.error(error)
    }
  }


  onMount(async () => {
    // await addWallet();
    await loadTable();
  })

</script>

<main class="wallet-container" class:mobile-wallet={$isMobile}>
  {#if $isMobile}
    <!-- Mobile layout with active wallet balances at top -->
    <div class="mobile-wallet-header">
      {#if primaryWalletName}
        <div class="active-wallet-info">
          <h3 class="text-lg font-semibold mb-2">Active Wallet: {primaryWalletName}</h3>
          <div class="balance-display">
            <div class="balance-item">
              <span class="balance-label">ETH Balance:</span>
              {#if walletBalances[primaryWalletName]?.loading !== false}
                <span class="loading loading-spinner loading-sm"></span>
              {:else if walletBalances[primaryWalletName]?.gas_balance !== undefined}
                <span class="balance-value">{walletBalances[primaryWalletName]?.gas_balance?.toFixed(6)} ETH</span>
              {:else}
                <span class="balance-value">--</span>
              {/if}
            </div>
            <div class="balance-item">
              <span class="balance-label">AUTONOMI Balance:</span>
              {#if walletBalances[primaryWalletName]?.loading !== false}
                <span class="loading loading-spinner loading-sm"></span>
              {:else if walletBalances[primaryWalletName]?.ant_balance !== undefined}
                <span class="balance-value">{walletBalances[primaryWalletName]?.ant_balance?.toFixed(6)} AUTONOMI</span>
              {:else}
                <span class="balance-value">--</span>
              {/if}
            </div>
          </div>
        </div>
      {/if}
      <div class="mobile-add-wallet">
        <button
          class="btn btn-neutral"
          onclick={()=>{
            activeWallet = {
              name: "",
              key: "",
              makePrimary: false,
            };
            referenceWallet = activeWallet;
            addWalletModal.showModal()
          }}>Add Wallet</button>
      </div>
    </div>
  {:else}
    <!-- Desktop layout -->
    <div class="row ps-5 pe-5 mb-3" style="display: flex; flex-direction: row; justify-content: space-between;">
      <h2 class="h2">Wallets</h2>
      <div class="utility-bar" style="display: flex;">
        <button
          class="btn btn-neutral"
          onclick={()=>{
            activeWallet = {
              name: "",
              key: "",
              makePrimary: false,
            };
            referenceWallet = activeWallet;
            addWalletModal.showModal()
          }}>Add New Wallet</button>
      </div>
    </div>
  {/if}
  <div class="row">
    <div class="card bg-base-100 w-96 shadow-lg card-xl" style="width: 95%;">
      <div class="card-body items-center text-center p-4">
        <!-- <h2 class="card-title h2">Your Pods</h2> -->
        <table class="table table-zebra">
          <thead>
            <tr>
              {#if !$isMobile}
                <th></th>
              {/if}
              <th>Wallet Name</th>
              <th>Address</th>
              {#if !$isMobile}
                <th>ETH Balance</th>
                <th>AUTONOMI Balance</th>
              {/if}
              <th>Operations</th>
            </tr>
          </thead>
            <tbody>
              {#if storedWallets.length > 0}
                {#each storedWallets as wallet, idx}
                  <tr>
                    {#if !$isMobile}
                      <th>{idx + 1}</th>
                    {/if}
                    <td class:android-wallet-name={isAndroid}>
                      <span class="wallet-name-text" class:truncated={isAndroid}>
                        {wallet.name}
                      </span>
                      {#if wallet.name === primaryWalletName}
                        <span title="Primary Wallet" style="margin-left:4px; color: #ffc940; font-size: 1.1rem;">💰</span>
                      {/if}
                    </td>
                    <td>
                      <AddressDisplay address={wallet.address} />
                    </td>
                    {#if !$isMobile}
                      <td>
                        {#if walletBalances[wallet.name]?.loading !== false}
                          <span class="loading loading-spinner loading-sm"></span>
                        {:else if walletBalances[wallet.name]?.gas_balance !== undefined}
                          {walletBalances[wallet.name]?.gas_balance?.toFixed(6)} ETH
                        {:else}
                          --
                        {/if}
                      </td>
                      <td>
                        {#if walletBalances[wallet.name]?.loading !== false}
                          <span class="loading loading-spinner loading-sm"></span>
                        {:else if walletBalances[wallet.name]?.ant_balance !== undefined}
                          {walletBalances[wallet.name]?.ant_balance?.toFixed(6)} AUTONOMI
                        {:else}
                          --
                        {/if}
                      </td>
                    {/if}
                    <td>
                      <div class="operations-buttons" class:android-operations={isAndroid}>
                        <button
                          class="btn btn-warning btn-square"
                          onclick={() => {
                            activeWallet = JSON.parse(JSON.stringify(wallet));
                            referenceWallet = JSON.parse(JSON.stringify(wallet));
                            editWalletModal.showModal();
                          }}>
                          <img src="/app-icons/pencil-icon.svg" alt="edit icon" width="19" height="19" />
                        </button>
                        <button
                          class="btn btn-error btn-square"
                          onclick={() => { activeWallet = wallet; deleteWalletModal.showModal(); }}>
                          <img src="/app-icons/trash-icon.svg" alt="trash icon" width="16" height="16" />
                        </button>
                      </div>
                    </td>
                  </tr>
                {/each}
              {:else}
                <tr>
                  <td colspan={$isMobile ? 3 : 6} style="text-align:center;">No wallets found</td>
                </tr>
              {/if}
            </tbody>
        </table>
      </div>
    </div>
  </div>

  <dialog id="editWalletModal" class="modal" bind:this={editWalletModal}>
    <div class="modal-box max-h-lg">
      <h3 class="text-lg font-bold">Editing Wallet: {activeWallet?.name}</h3>
      <div class="pt-3 pb-3 flex flex-col items-start">
        <label class="label" for="edit-wallet-name">New Name: </label>
        <input id="edit-wallet-name" type="input" class="input" placeholder="New Wallet Name" bind:value={activeWallet.name}/>
        <!-- <p class="text-xs text-gray-400 mt-1">
          Empty values will result in no changes for the name.
        </p> -->
      </div>
      <div class="row pb-3 flex flex-col items-start">
        <label class="label" for="edit-wallet-key">Private Key:</label>
        <input
          id="edit-wallet-key"
          type="input"
          class="input"
          placeholder="New Private Key"
          bind:value={activeWallet.key}
        />
        <!-- <p class="text-xs text-gray-400 mt-1">
          Empty values will result in no changes for the private key.
        </p> -->
      </div>
      <div class="pb-3 flex flex-col">
        <label class="label font-bold">
          <input type="checkbox" class="checkbox" bind:checked={activeWallet.makePrimary} />
          Make Primary Wallet
        </label>
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button class="btn btn-primary" onclick={()=>{updateWallet()}}>Save</button>
          <button class="btn btn-soft btn-error">Cancel</button>
        </form>
      </div>
    </div>
  </dialog>
  <dialog id="addWalletModal" class="modal" bind:this={addWalletModal}>
    <div class="modal-box max-h-lg">
      <h3 class="text-lg font-bold">New Wallet</h3>
      <div class="pt-3 pb-3 flex flex-col items-start">
        <label class="label" for="new-wallet-name">New Name: </label>
        <input id="new-wallet-name" type="input" class="input" placeholder="New Wallet Name" bind:value={activeWallet.name}/>
      </div>
      <div class="row pb-3 flex flex-col items-start">
        <label class="label" for="new-wallet-key">Private Key:</label>
        <input
          id="new-wallet-key"
          type="input"
          class="input"
          placeholder="New Private Key"
          bind:value={activeWallet.key}
        />
      </div>
      <div class="pb-3 flex flex-col">
        <label class="label font-bold">
          <input type="checkbox" class="checkbox" bind:checked={activeWallet.makePrimary} />
          Make Primary Wallet
        </label>
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button class="btn btn-primary" onclick={()=>{addNewWallet()}}>Save</button>
          <button class="btn btn-soft btn-error">Cancel</button>
        </form>
      </div>
    </div>
  </dialog>
  <dialog id="deleteWalletModal" class="modal" bind:this={deleteWalletModal}>
    <div class="modal-box w-8/12 max-w-xl">
      <h3 class="text-lg font-bold">Wallet Deletion</h3>
      <div class="py-4" style="justify-content: center;">
        <p class="pb-3">Are you sure you want to delete the wallet "{activeWallet?.name}"?</p>
        <p class="text-sm text-gray-500">This action cannot be undone.</p>
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button class="btn btn-error" onclick={()=>{deleteWalletHandler()}}>Delete</button>
          <button class="btn btn-soft btn-error">Cancel</button>
        </form>
      </div>
    </div>
  </dialog>
</main>

<style>
.wallet-container {
  /* margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column; */
  padding-top: 5vh;
  justify-content: center;
  text-align: center;
  overflow-y: auto;
  /* Set a fixed max-width that accounts for scrollbar */
  /* max-width: 64rem;
  width: 100%;
  box-sizing: border-box; */
}

.row {
  display: flex;
  justify-content: center;
  padding-bottom: 2vh;
}

.input{
  width: 100%;
}

/* Mobile-specific styles */
.mobile-wallet {
  padding: 10px !important;
  padding-top: 20px !important;
}

.mobile-wallet-header {
  display: flex;
  flex-direction: column;
  gap: 20px;
  margin-bottom: 20px;
  padding: 0 10px;
}

.active-wallet-info {
  background: var(--fallback-b2, oklch(var(--b2)));
  border-radius: 8px;
  padding: 16px;
  text-align: center;
}

.balance-display {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-top: 12px;
}

.balance-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background: var(--fallback-b1, oklch(var(--b1)));
  border-radius: 6px;
}

.balance-label {
  font-weight: 600;
  color: var(--fallback-bc, oklch(var(--bc) / 0.8));
}

.balance-value {
  font-weight: 700;
  color: var(--fallback-bc, oklch(var(--bc)));
}

.mobile-add-wallet {
  display: flex;
  justify-content: center;
}

.mobile-add-wallet .btn {
  width: 200px;
}

/* Android-specific styles */
.android-wallet-name {
  max-width: 80px;
}

.wallet-name-text.truncated {
  display: block;
  max-width: 70px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.operations-buttons {
  display: flex;
  flex-direction: row;
  gap: 8px;
  justify-content: center;
}

.operations-buttons.android-operations {
  flex-direction: row;
  gap: 8px;
  justify-content: center;
}

.operations-buttons.android-operations .btn {
  min-width: 40px;
  padding: 8px;
}

@media (max-width: 767px) {
  .card {
    margin: 0 -10px; /* Extend table to screen edges */
  }
}

</style>

