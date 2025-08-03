<script lang="ts">
  export let isOpen = false;
  export let ethBalance = 0;
  export let antBalance = 0;
  
  let dialog: HTMLDialogElement;
  
  $: if (dialog) {
    if (isOpen) {
      dialog.showModal();
    } else {
      dialog.close();
    }
  }
  
  function closeDialog() {
    isOpen = false;
  }
</script>

<dialog bind:this={dialog} class="modal" onclose={closeDialog}>
  <div class="modal-box">
    <h3 class="text-lg font-bold text-warning">⚠️ Insufficient Wallet Balance</h3>
    <div class="py-4">
      <p class="mb-4">
        Your active wallet must be funded with both ETH and AUTONOMI tokens before upload operations can occur.
      </p>
      
      <div class="bg-base-200 p-4 rounded-lg mb-4">
        <h4 class="font-semibold mb-2">Current Balance:</h4>
        <div class="space-y-1">
          <div class="flex justify-between">
            <span>ETH:</span>
            <span class={ethBalance > 0 ? 'text-success' : 'text-error'}>
              {ethBalance.toFixed(6)} ETH
            </span>
          </div>
          <div class="flex justify-between">
            <span>AUTONOMI:</span>
            <span class={antBalance > 0 ? 'text-success' : 'text-error'}>
              {antBalance.toFixed(6)} ANT
            </span>
          </div>
        </div>
      </div>
      
      <p class="text-sm text-base-content/70">
        Please fund your wallet with both ETH (for gas fees) and AUTONOMI tokens (for storage costs) before attempting to upload files or pods.
      </p>
    </div>
    
    <div class="modal-action">
      <button class="btn btn-primary" onclick={closeDialog}>
        OK
      </button>
    </div>
  </div>
</dialog>
