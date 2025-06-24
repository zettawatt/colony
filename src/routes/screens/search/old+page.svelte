<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");
  let statusMessage = $state("");
  let isLoading = $state(false);

  // Type definitions for Tauri commands
  interface CreatePodRequest {
    name: string;
  }

  interface PodInfo {
    name: string;
    address: string;
  }

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }

  async function initializeAutonomiClient() {
    // const walletKey = prompt("Enter wallet key:");
    // if (!walletKey) {
    //   statusMessage = "Wallet key is required";
    //   return;
    // }

    // const walletKey = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    const walletAddress = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
    const walletKey = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

    isLoading = true;
    statusMessage = "Initializing Autonomi client...";

    try {
      const result = await invoke("initialize_autonomi_client", { walletKey });
      statusMessage = `Success: ${result}`;
    } catch (error) {
      statusMessage = `Error: ${error}`;
    } finally {
      isLoading = false;
    }
    console.log(statusMessage)
  }

  async function handleInitializePodManager() {
    const password = prompt("Enter keystore password:");
    if (!password) {
      statusMessage = "Password is required";
      return;
    }

    isLoading = true;

    try {
      // Step 1: Initialize datastore
      statusMessage = "Initializing datastore...";
      await invoke("initialize_datastore");

      // Step 2: Open keystore
      statusMessage = "Opening keystore...";
      await invoke("open_keystore", { password });

      // Step 3: Initialize graph
      statusMessage = "Initializing graph...";
      await invoke("initialize_graph");

      // Step 4: Initialize pod manager
      statusMessage = "Initializing pod manager...";
      const result = await invoke("initialize_pod_manager");

      statusMessage = `Success: Pod Manager fully initialized - ${result}`;
    } catch (error) {
      statusMessage = `Error: ${error}`;
    } finally {
      isLoading = false;
    }
  }

  async function handleAddPod() {
    const podName = prompt("Enter pod name:");
    if (!podName) {
      statusMessage = "Pod name is required";
      return;
    }

    isLoading = true;
    statusMessage = "Adding pod...";

    try {
      const request: CreatePodRequest = { name: podName };
      const result: PodInfo = await invoke("add_pod", { request });
      statusMessage = `Success: Pod "${result.name}" created with address: ${result.address}`;
    } catch (error) {
      statusMessage = `Error: ${error}`;
    } finally {
      isLoading = false;
    }
  }
</script>

<main class="search-container" style="padding-top: 7vh;">
  <!-- <h1>Welcome to Colony</h1> -->

  <div class="row">
    <img src="/logo-192x192.png" alt="Colony logo"/>
    <!-- <img src="/splash_screen.png" alt="Colony logo" width="75%" class="p-5"/> -->
    <!-- <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a> -->
  </div>
  <div class="row">
    <label class="input">
      <svg class="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
        <g
          stroke-linejoin="round"
          stroke-linecap="round"
          stroke-width="2.5"
          fill="none"
          stroke="currentColor"
        >
          <circle cx="11" cy="11" r="8"></circle>
          <path d="m21 21-4.3-4.3"></path>
        </g>
      </svg>
      <input type="search" required placeholder="Search"/>
    </label>
  </div>

  <!-- Action buttons -->
  <!-- <div class="button-row">
    <button
      class="action-button"
      onclick={handleInitializeAutonomiClient}
      disabled={isLoading}
    >
      Initialize Autonomi Client
    </button>
    <button
      class="action-button"
      onclick={handleInitializePodManager}
      disabled={isLoading}
    >
      Initialize Pod Manager
    </button>
    <button
      class="action-button"
      onclick={handleAddPod}
      disabled={isLoading}
    >
      Add Pod
    </button>
  </div> -->

  <!-- Status message -->
  {#if statusMessage}
    <div class="status-message" class:loading={isLoading}>
      {statusMessage}
    </div>
  {/if}

  <!-- <form class="row" onsubmit={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p> -->
</main>

<style>
.search-container {
  /* margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column; */
  justify-content: center;
  text-align: center;
  overflow-y: auto;
  /* Set a fixed max-width that accounts for scrollbar */
  /* max-width: 64rem;
  width: 100%;
  box-sizing: border-box; */
}

.input {
  width: 60%;
  max-width: 600px;
}

.button-row {
  display: flex;
  gap: 1rem;
  justify-content: center;
  margin-top: 2rem;
  flex-wrap: wrap;
}

.action-button {
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
  cursor: pointer;
  min-width: 160px;
}

.action-button:hover:not(:disabled) {
  border-color: #396cd8;
}

.action-button:active:not(:disabled) {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

.action-button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.status-message {
  margin-top: 1.5rem;
  padding: 1rem;
  border-radius: 8px;
  max-width: 600px;
  margin-left: auto;
  margin-right: auto;
  text-align: center;
}

.status-message.loading {
  background-color: #e3f2fd;
  border-color: #2196f3;
  color: #1976d2;
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.svelte-kit:hover {
  filter: drop-shadow(0 0 2em #ff3e00);
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

/* .logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
} */

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

/* button {
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
} */

#greet-input {
  margin-right: 5px;
}

</style>
