<script lang="ts">
  import SeedPhrase from "../seedPhrase.svelte";



  // let showValidString = false;
  // let wasPhraseValid = false;

  // let { generateNewSeedPhrase, validateSeedPhrase, words } = $props();
  export let generateNewSeedPhrase: () => string[];
  export const validateSeedPhrase: (words: string[]) => boolean = () => false;
  export let words: string[];
  export let showValidString: boolean;
  export let isPhraseValid: boolean;

</script>


<div>
  <h3 class="text-3xl font-extrabold dark:text-white pt-3 pb-3" style="text-align: center;">12 Word Seed Phrase</h3>

  <div class="row pt-3 pb-3">
    <p>If you have an existing 12 word seed phrase, please enter it here. <br> Otherwise, press the 'Generate' button to generate a new seed phrase.</p>
    <!-- <button class="btn">Default</button> -->
  </div>

  <div class="row pt-3 pb-3">
    <SeedPhrase bind:seedWords={words}/>
  </div>
  <div class="row pt-3 pb-3">
    <button onclick={()=>{
      words = generateNewSeedPhrase();
    }}>Generate</button>
    <!-- <button class="ms-3" onclick={
      ()=>{
          wasPhraseValid = validateSeedPhrase(words)
          showValidString = true;
        }
      }
    >Validate</button> -->
  </div>
  <div style="text-align: center;">
    {#if showValidString}
      {#if isPhraseValid}
        <p class="text-green-700">Seed phrase is valid!</p>
      {:else}
        <p class="text-red-700">Seed phrase is invalid!</p>
      {/if}
    {/if}
  </div>
</div>

<style>


/* Remove the responsive breakpoints that cause the shift */
/* Keep the same max-width as the larger breakpoint */
/* @media (width >= 48rem) {
  .container {
    max-width: 64rem; 
  }
} */



.row {
  display: flex;
  justify-content: center;
}





button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

button {
  outline: none;
}

</style>
