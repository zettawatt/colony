<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import SeedPhrase from "../../components/seedPhrase.svelte";
  import ps from "../../stores/persistantStorage"; 
  import { getPassword, setPassword } from "../../utils/password/session";

  let seedPhraseRef: SeedPhrase;

  let newPassword = $state("");
  let confirmPassword = $state("");
  let passwordsMatch = $derived(newPassword && confirmPassword && newPassword === confirmPassword);
  let confirmClass = $derived(passwordsMatch ? 'input-success' : 'input-error');
  let words = "";

  function reroute(href:string) {
    window.location.href = href;
  }

  function handleGenerate() {
    words = seedPhraseRef.generateNewSeedPhrase();
    console.log(words)
  }

  async function initDatastore() {
    try {
      // const datastore = await invoke("initialize_datastore"); 
      console.log(words);
      await setPassword(confirmPassword);
      const pw = await getPassword();
      if (!pw) {
        console.error("password was null")
      }
      const keystore = await invoke("create_keystore_from_seed_phrase", {seedPhrase: words})
      const writtenKeystore = await invoke("write_keystore_to_file", {password: pw})
      await invoke("open_keystore", { password: pw });
      await ps.setUserCompletedIntro(true);
      reroute("/screens/search");
      return true;
    } catch (error) {
      console.error(error);
      return false;
    }
  }
</script>


<main class="intro-container">
  <h3 class="text-3xl font-extrabold dark:text-white">Welcome!</h3>

  <div class="row pt-3 pb-3">
    <img src="/splash_screen.png" alt="Colony logo" width="70%"/>
  </div>
  <div class="row pt-3 pb-3">
    <p>Before you get started, you must first create a password that will be used to encrypt and decrypt your seed phrase.</p>
    <!-- <button class="btn">Default</button> -->
  </div>
  <div class="row pt-3 pb-3">
    <label class="label">New Password: </label>
    <input bind:value={newPassword} type="password" class="input" placeholder="Password" />
  </div>
  <div class="row pt-3 pb-3">
    <label class="label">Confirm Password:</label>
    <input bind:value={confirmPassword} type="password" class="input {confirmClass}" placeholder="Password" />
  </div>

  <h3 class="text-3xl font-extrabold dark:text-white pt-3 pb-3">12 Word Seed Phrase</h3>

  <div class="row pt-3 pb-3">
    <p>If you have an existing 12 word seed phrase, please enter it here. <br> Otherwise, press the 'Generate' button to generate a new seed phrase.</p>
    <!-- <button class="btn">Default</button> -->
  </div>

  <div class="row pt-3 pb-3">
    <SeedPhrase bind:this={seedPhraseRef} />
  </div>
  <div class="row pt-3 pb-3">
    <button onclick={handleGenerate}>Generate</button>
  </div>
  <div>
    <button onclick={initDatastore}>done</button>
  </div>
</main>

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
