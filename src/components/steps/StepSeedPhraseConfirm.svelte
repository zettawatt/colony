<script lang="ts">
  import SeedPhrase from "../seedPhrase.svelte";



  // let showValidString = false;
  // let wasPhraseValid = false;

  // let { generateNewSeedPhrase, validateSeedPhrase, words } = $props();
  export let words: string[];
  export let showMatchingString: boolean;
  export let isSeedPhraseMatching: boolean;

</script>


<div>
  <h3 class="text-3xl font-extrabold dark:text-white pt-3 pb-3" style="text-align: center;">Confirm Seed Phrase</h3>

  <div class="row pt-3 pb-3">
    <p>Please confirm your seed phrase. This will be your last chance to write it down.</p>
    <!-- <button class="btn">Default</button> -->
  </div>

  <div class="row pt-3 pb-3">
    <SeedPhrase bind:seedWords={words}/>
  </div>
  <div style="text-align: center;">
    {#if showMatchingString}
      {#if isSeedPhraseMatching}
        <p class="text-green-700">Seed phrase is matching!</p>
      {:else}
        <p class="text-red-700">Seed phrase does not match!</p>
      {/if}
    {/if}
  </div>
</div>

<style>
.row {
  display: flex;
  justify-content: center;
}

</style>
