<!-- src/components/StepWelcome.svelte -->
<script lang="ts">
  
  export let walletPrivateKey: string = '';
  $: isValidPrivateKey = /^([a-fA-F0-9]{64})$/.test(walletPrivateKey || '');
</script>

<div>
  <h3 class="text-3xl font-extrabold dark:text-white" style="text-align: center;">Wallet</h3>
  <div class="row pt-3 pb-3">
    <p>
      Welcome! 🚀 If you plan on uploading content to the network, please enter your wallet's private key below.<br>
      <strong>You may <u>skip this step</u> by clicking Next if you just want to browse and download content from the network.</strong>
    </p>    <!-- <button class="btn">Default</button> -->
  </div>
  <div class="row pt-3 pb-3">
    <label class="label" for="wallet-private-key">Wallet Private Key: </label>
    <input
      id="wallet-private-key"
      bind:value={walletPrivateKey}
      type="text"
      class="input"
      maxlength="64"
      spellcheck="false"
      autocomplete="off"
      placeholder="wallet private key" />
  </div>
  {#if walletPrivateKey && !isValidPrivateKey}
    <div class="row" style="color: red;">
      Private key must be exactly 64 hexadecimal characters (0-9, a-f).
    </div>
  {/if}
</div>

<style>
  .label {
    /* display: inline-block; */
    width: 140px; /* Fixed width for both labels */
    text-align: right;
    margin-right: 10px;
  }
  .row {
    display: flex;
    justify-content: center;
  }

</style>