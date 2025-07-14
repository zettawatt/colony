<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    greetMsg = await invoke("greet", { name });
  }
</script>

<main class="info-container">
  <h3 class="text-3xl font-extrabold dark:text-white">App Information</h3>
  <p>
    Colony App Built by
    <a href="https://www.linkedin.com/in/maxx-rodriguez/" target="_blank" class="profile-link">
      Maxx Rodriguez
      <svg class="linkedin-icon" width="16" height="16" viewBox="0 0 32 32" fill="currentColor" aria-label="LinkedIn">
        <path d="M27 0H5C2.2 0 0 2.2 0 5v22c0 2.8 2.2 5 5 5h22c2.8 0 5-2.2 5-5V5c0-2.8-2.2-5-5-5zM9.4
        27H5.3V12h4.2v15zM7.3 10.3c-1.3 0-2.3-1-2.3-2.3S6 5.8 7.3 5.8 9.6 6.8 9.6 8.1s-1 2.2-2.3 2.2zm20
        16.7h-4.2v-7.2c0-1.7-0.7-2.7-2.1-2.7-1.2 0-1.8 0.8-2.1 1.5-0.1 0.3-0.1 0.8-0.1 1.3V27h-4.2s0.1-13.6 0-15h4.2v2.1c0.6-1 1.7-2.4
        4-2.4 2.9 0 5 1.9 5 6v9.3z"/>
      </svg>
    </a>
  </p>
  <p>
    Colony Lib Built by Chuck McClish
  </p>
  <!-- <p>
    Colony Lib Built by
    <a href="https://www.linkedin.com/in/chuck-mcclish-9b227319/" target="_blank" class="profile-link">
      Chuck McClish
      <svg class="linkedin-icon" width="16" height="16" viewBox="0 0 32 32" fill="currentColor" aria-label="LinkedIn">
        <path d="M27 0H5C2.2 0 0 2.2 0 5v22c0 2.8 2.2 5 5 5h22c2.8 0 5-2.2 5-5V5c0-2.8-2.2-5-5-5zM9.4
        27H5.3V12h4.2v15zM7.3 10.3c-1.3 0-2.3-1-2.3-2.3S6 5.8 7.3 5.8 9.6 6.8 9.6 8.1s-1 2.2-2.3 2.2zm20
        16.7h-4.2v-7.2c0-1.7-0.7-2.7-2.1-2.7-1.2 0-1.8 0.8-2.1 1.5-0.1 0.3-0.1 0.8-0.1 1.3V27h-4.2s0.1-13.6 0-15h4.2v2.1c0.6-1 1.7-2.4
        4-2.4 2.9 0 5 1.9 5 6v9.3z"/>
      </svg>
    </a>
  </p> -->
  <p>Using the following technologies:</p>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://kit.svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />
    </a>
  </div>

  <div class="github-section">
    <h4>Source Code:</h4>
    <a href="https://github.com/zettawatt/colony" target="_blank">
      Colony App on GitHub
    </a>
    <br />
    <a href="https://github.com/zettawatt/colonylib" target="_blank">
      Colony Lib on GitHub
    </a>
  </div>

  <p>Version: 1.0.0</p>
  <p>Thank you for checking out Colony!</p>
</main>

<style>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.svelte-kit:hover {
  filter: drop-shadow(0 0 2em #ff3e00);
}

.info-container {
  padding-top: 10vh;
  text-align: center;
  overflow-y: auto;
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

#greet-input {
  margin-right: 5px;
}

.github-section {
  margin: 2em 0 1em 0;
}
.github-section a {
  display: inline-block;
  margin: 0.5em;
  color: #333;
  font-weight: 600;
}
.github-section a:hover {
  color: #24292f;
  text-decoration: underline;
}

.profile-link {
  color: #0072b1;
  text-decoration: underline;
  transition: color 0.2s;
  display: inline-flex;
  align-items: center;
  gap: 0.22em;
}
.profile-link:hover {
  color: #005582;
}
.linkedin-icon {
  vertical-align: middle;
  margin-left: 0.2em;
  color: #0072b1;
}
.profile-link:hover .linkedin-icon {
  color: #005582;
}
</style>
