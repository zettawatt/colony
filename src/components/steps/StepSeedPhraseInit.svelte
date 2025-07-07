<script lang="ts">
  import SeedPhrase from "../seedPhrase.svelte";

  let seedPhraseRef: SeedPhrase;

  // let showValidString = false;
  // let wasPhraseValid = false;

  // let { generateNewSeedPhrase, validateSeedPhrase, words } = $props();
  export let generateNewSeedPhrase, validateSeedPhrase, words, showValidString, isPhraseValid;

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
  .label {
    /* display: inline-block; */
    width: 140px; /* Fixed width for both labels */
    text-align: right;
    margin-right: 10px;
  }

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.svelte-kit:hover {
  filter: drop-shadow(0 0 2em #ff3e00);
}

.intro-container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  text-align: center;
  overflow-y: auto;
  width: 100%;
}

/* Remove the responsive breakpoints that cause the shift */
/* Keep the same max-width as the larger breakpoint */
/* @media (width >= 48rem) {
  .container {
    max-width: 64rem; 
  }
} */

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
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

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

</style>
