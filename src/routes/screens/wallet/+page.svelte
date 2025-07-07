<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import ps from "../../../stores/persistantStorage";
  import { onMount } from "svelte";
  import { handleCopyAddress } from "../../../utils/copyAutonomiAddress";
  import { getPassword } from "../../../utils/password/session";

  const walletAddress = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
  const walletKey = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
  let storedWallets = $state([]);
  let activeWallet = $state<any>({
    name: "",
    privateKey: "",
    makePrimary: false,
  });
  let referenceWallet = $state<any>({
    name: "",
    privateKey: ""
  });


  // async function addWallet() {
  //   try {
  //     const response = await invoke("add_wallet", {
  //       request: {
  //         name: "Second Wallet",
  //         key: walletKey
  //       }
  //     })
  //     console.log("res", response);
  //   } catch (error) {
  //     console.error(error)
  //   }
  // }
  // Add a wallet
  export async function addWallet(name, key) {
    try {
      let pw = await getPassword();
      // request shape should match AddWalletRequest
      const response = await invoke('add_wallet', {
        request: { name, key }
      });
      await invoke("write_keystore_to_file", {password: pw})
      console.log(response); // "Wallet added"
      return response;
    } catch (error) {
      console.error('Error adding wallet:', error);
    }
  }

  // Remove a wallet by name
  export async function removeWallet(name) {
    try {
      let pw = await getPassword();
      const response = await invoke('remove_wallet', { name });
      await invoke("write_keystore_to_file", {password: pw})
      console.log(response); // "Wallet removed"
      return response;
    } catch (error) {
      console.error('Error removing wallet:', error);
    }
  }

  // List wallets (returns object of { walletName: privateKey } or similar)
  export async function listWallets() {
    try {
      const wallets = await invoke('list_wallets');
      console.log("wallets", wallets)
      const remappedWallets = [];
      for (const [key, value] of Object.entries(wallets)) {
        remappedWallets.push({
          name: key,
          privateKey: value
        })
      }
      return remappedWallets;
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
      console.log(response); // "Wallet switched"
      return response;
    } catch (error) {
      console.error('Error switching wallet:', error);
    }
  }
  
  async function updateWallet() {
    if (activeWallet.name !== referenceWallet.name) {
      const removeRes = await removeWallet(referenceWallet.name);
      console.log("removeWallet response:", removeRes);
      const addRes = await addWallet(activeWallet.name, activeWallet.privateKey);
      console.log("addWallet response:", addRes);
    } else if (activeWallet.privateKey !== referenceWallet.privateKey) {
      const addRes = await addWallet(activeWallet.name, activeWallet.privateKey);
      console.log("addWallet response:", addRes);
    }
    if (activeWallet.makePrimary) {
      const switchRes = await switchWallet(activeWallet.name);
      console.log("switchWallet response:", switchRes);
    }
    await loadTable();
  }

  async function loadTable() {
    try {
      storedWallets = await listWallets();
      console.log("storedWallets", storedWallets);
    } catch (error) {
      console.error(error)
    }
  }

  onMount(async () => {
    // await addWallet();
    await loadTable();
  })

</script>

<main class="wallet-container">
  <div class="row ps-5 pe-5 mb-3" style="display: flex; flex-direction: row; justify-content: space-between;">
    <h2 class="h2">Wallets</h2>
    <div class="utility-bar" style="display: flex;">
      <button 
        class="btn btn-neutral" 
        onclick={()=>{
          activeWallet = {
            name: "",
            privateKey: "",
            makePrimary: false,
          };
          referenceWallet = {
            name: "",
            privateKey: "",
            makePrimary: false,
          };
          addWalletModal.showModal()
        }}>Add New Wallet</button>
    </div>
  </div>
  <div class="row">
    <div class="card bg-base-100 w-96 shadow-lg card-xl" style="width: 95%;">
      <div class="card-body items-center text-center p-4">
        <!-- <h2 class="card-title h2">Your Pods</h2> -->
        <table class="table table-zebra">
          <thead>
            <tr>
              <th></th>
              <th>Wallet Name</th>
              <th>Wallet Key</th>
              <!-- <th>Balance</th> -->
              <th>Operations</th>
            </tr>
          </thead>
            <tbody>
              {#if storedWallets.length > 0}
                {#each storedWallets as wallet, idx}
                  <tr>
                    <th>{idx + 1}</th>
                    <td>{wallet.name}</td>
                    <td>
                      <div class="tooltip tooltip-warning" data-tip={wallet.privateKey}>
                        <button
                          class="address-tooltip"
                          data-address={wallet.privateKey}
                          onclick={handleCopyAddress}
                          tabindex="0"
                          style="cursor: pointer; font-style: italic; text-decoration: underline dotted;"
                        >wallet private key</button>
                      </div>
                    </td>
                    <td>
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
                        onclick={() => { activeWallet = wallet; deletePodModal.showModal(); }}>
                        <img src="/app-icons/trash-icon.svg" alt="trash icon" width="16" height="16" />
                      </button>
                    </td>
                  </tr>
                {/each}
              {:else}
                <tr>
                  <td colspan="6" style="text-align:center;">No wallets found</td>
                </tr>
              {/if}
            </tbody>
        </table>
      </div>
    </div>
  </div>

  <dialog id="editWalletModal" class="modal">
    <div class="modal-box max-h-lg">
      <h3 class="text-lg font-bold">Editing Wallet: {activeWallet?.name}</h3>
      <div class="pt-3 pb-3 flex flex-col items-start">
        <label class="label">New Name: </label>
        <input type="input" class="input" placeholder="New Wallet Name" bind:value={activeWallet.name}/>
        <!-- <p class="text-xs text-gray-400 mt-1">
          Empty values will result in no changes for the name.
        </p> -->
      </div>
      <div class="row pb-3 flex flex-col items-start">
        <label class="label">Private Key:</label>
        <input
          type="input" 
          class="input" 
          placeholder="New Private Key"
          bind:value={activeWallet.privateKey}
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
  <dialog id="addWalletModal" class="modal">
    <div class="modal-box max-h-lg">
      <h3 class="text-lg font-bold">New Wallet</h3>
      <div class="pt-3 pb-3 flex flex-col items-start">
        <label class="label">New Name: </label>
        <input type="input" class="input" placeholder="New Wallet Name" bind:value={activeWallet.name}/>
      </div>
      <div class="row pb-3 flex flex-col items-start">
        <label class="label">Private Key:</label>
        <input
          type="input" 
          class="input" 
          placeholder="New Private Key" 
          bind:value={activeWallet.privateKey}
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
          <button class="btn btn-primary" onclick={()=>{updateWallet()}}>Save</button>
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
.tooltip[data-tip]::before,
.tooltip.tooltip-open[data-tip]::before {
  max-width: 50rem !important;
  min-width: 16rem;
  white-space: pre-wrap !important;
  font-family: monospace !important;
}
.address-tooltip {
  transition: color 0.15s;
}
.address-tooltip:hover, .address-tooltip:focus {
  color: #009799;
  text-decoration-style: solid;
}
.row {
  display: flex;
  justify-content: center;
}

.input{
  width: 100%;
}

</style>

