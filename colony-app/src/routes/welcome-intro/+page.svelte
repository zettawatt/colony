<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import SeedPhrase from "../../components/seedPhrase.svelte";

  let seedPhraseRef: SeedPhrase;

  let newPassword = $state("");
  let confirmPassword = $state("");
  let passwordsMatch = $derived(newPassword && confirmPassword && newPassword === confirmPassword);
  let confirmClass = $derived(passwordsMatch ? 'input-success' : 'input-error');

  let name = $state("");
  let greetMsg = $state("");

  function reroute(href:string) {
    window.location.href = href;
  }

  async function handleGenerate() {
    // Call the generate function from the SeedPhrase component
    await seedPhraseRef.generate(new Event('click'));
  }

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>


<main class="container">
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
    <button onclick={() => reroute("/screens/search")}>done</button>
  </div>

  <!-- <form class="row" onsubmit={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p> -->
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

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
  overflow-y: auto;
  /* Set a fixed max-width that accounts for scrollbar */
  max-width: 64rem;
  width: 100%;
  box-sizing: border-box;
}

/* Remove the responsive breakpoints that cause the shift */
@media (width >= 48rem) {
  .container {
    max-width: 64rem; /* Keep the same max-width as the larger breakpoint */
  }
}

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

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
