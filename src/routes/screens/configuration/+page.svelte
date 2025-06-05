<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';


  let name = $state("");
  let greetMsg = $state("");
  let downloadPath = $state("");

  async function toast() {
    let permissionGranted = await isPermissionGranted();
    console.log('here', permissionGranted)
    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === 'granted';
    }
    if (permissionGranted) {
      sendNotification('Tauri is awesome!');
      sendNotification({ title: 'TAURI', body: 'Tauri is awesome!' });
    }
  }

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<main>
  <div class="row">
    <h4 class="h4">Configuration Settings</h4>
  </div>
  <div class="row">
    <div class="row pt-3">
      <label class="">Download Path:</label>
      <input bind:value={downloadPath} type="text" class="input" placeholder="/home/usr/downloads" />
    </div>
    <div class="row pt-3">
      <label class="">Colony Application Data Path:</label>
      <input bind:value={downloadPath} type="text" class="input" disabled placeholder="/home/usr/downloads" />
    </div>
    <div class="row pt-3">
      <label class="">Auto-Lock Timeout (Minutes):</label>
      <input bind:value={downloadPath} type="text" class="input" placeholder=10 />
      <div style="" class="pt-3">
        <button class="btn btn-primary" onclick={toast}>Save</button>
      </div>
    </div>
  </div>

</main>

<style>
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
  flex-direction: column;
  margin-left: 2%;
  /* justify-content: center; */
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
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
