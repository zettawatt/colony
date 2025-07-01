<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import ps from "../../../stores/persistantStorage";
  import { onMount } from "svelte";
  import { appDataDir } from '@tauri-apps/api/path';
  import { app } from "@tauri-apps/api";
  import { open } from '@tauri-apps/plugin-dialog';
  import { addToast } from "../../../stores/toast";

  let name = $state("");
  let greetMsg = $state("");
  let downloadPath = $state("");
  let appDataPath = $state("");
  let newPassword = $state("");
  let confirmPassword = $state("");
  let passwordsMatch = $derived(newPassword && confirmPassword && newPassword === confirmPassword);
  let confirmClass = $derived(passwordsMatch ? 'input-success' : 'input-error');

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

  async function selectPath() {
    const newDownloadPath = await open({ multiple: false, directory: true });
    if (newDownloadPath) {
      downloadPath = newDownloadPath;
    }
  }

  async function saveConfig() {
    try {
      if (!downloadPath) return;
      const newDownloadPath = await ps.setDownloadDir(downloadPath);
      addToast("Saved config!", "success");
    } catch (error) {
      console.trace(error)
      addToast("Could not save config", "error")
    }
  }

  onMount(async() => {
    downloadPath = await ps.getDownloadDir();
    appDataPath = await appDataDir();
  })
</script>

<main class="config-container">
  <div class="row">
    <h2 class="h2">Configuration Settings</h2>
    <div class="card bg-base-100 w-96 shadow-lg card-xl" style="width: auto;">
      <div class="card-body p-4">
        <div class="flex-container" style="display: flex;">
          <!-- Left: Existing content -->
          <div class="left-section" style="flex: 1;">
            <div class="row pt-3">
              <label>Download Path:</label>
              <input 
                type="text" 
                value={downloadPath} 
                class="input" 
                style="min-width: 100%;"
                onclick={()=>{selectPath()}}
                readonly
              />
            </div>
            <div class="row pt-3">
              <label>Colony Application Data Path:</label>
              <input bind:value={appDataPath} type="text" class="input appData w-full" disabled placeholder="/home/usr/downloads" />
              <div class="mt-4">
                <button class="btn btn-primary" onclick={()=>(saveConfig())}>Save</button>
              </div>
            </div>
          </div>

          <!-- Vertical rule -->
          <div style="width:1px; background:#cdcfd1; margin: 0 2rem;"></div>

          <!-- Right: Change Password -->
          <div class="right-section" style="flex: 1;">
            <div class="row pt-3 pb-3">
              <label class="label">New Password: </label>
              <input bind:value={newPassword} type="password" class="input w-full" placeholder="Password" />
            </div>
            <div class="row pt-3 pb-3">
              <label class="label">Confirm Password:</label>
              <input bind:value={confirmPassword} type="password" class="input {confirmClass} w-full" placeholder="Password" />
              <button class="btn btn-error mt-4">Update Password</button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
  <!-- <div class="row">
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
  </div> -->

</main>

<style>

.appData::placeholder{
  color:dimgray;
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.svelte-kit:hover {
  filter: drop-shadow(0 0 2em #ff3e00);
}

.config-container {
  /* margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column; */
  padding-top: 3vh;
  justify-content: center;
  overflow-y: auto;
  /* Set a fixed max-width that accounts for scrollbar */
  /* max-width: 64rem;
  width: 100%;
  box-sizing: border-box; */
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
  margin-right: 2%;
  padding-bottom: 2vh;
  /* justify-content: center; */
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}


</style>
