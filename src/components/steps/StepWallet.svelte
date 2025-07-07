<!-- src/components/StepWelcome.svelte -->
<script lang="ts">
  
  export let walletPrivateKey;
</script>

<div>
  <h3 class="text-3xl font-extrabold dark:text-white" style="text-align: center;">Wallet</h3>
  <div class="row pt-3 pb-3">
    <p>If you'd like to upload content to the network, you must input your wallet's private key. You may skip this step if you want to browse the network.</p>
    <!-- <button class="btn">Default</button> -->
  </div>
  <div class="row pt-3 pb-3">
    <label class="label">Wallet Private Key: </label>
    <input bind:value={walletPrivateKey} type="text" class="input" placeholder="0x0000000000000000000000000000000000000000000000000000000000000000" />
  </div>
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