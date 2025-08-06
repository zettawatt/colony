<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import ps from "../../../stores/persistantStorage";
  import { onMount } from "svelte";
  import { appDataDir } from '@tauri-apps/api/path';
  import { app } from "@tauri-apps/api";
  import { open } from '@tauri-apps/plugin-dialog';
  import { addToast } from "../../../stores/toast";
  import { setPassword, getPassword } from "../../../utils/password/session";
  import { lightDaisyThemes, darkDaisyThemes } from "../../../utils/theme/daisyUIThemes";
  import { isMobile } from '../../../utils/responsive.js';

  let name = $state("");
  let greetMsg = $state("");
  let downloadPath = $state("");
  let appDataPath = $state("");
  let currentPassword = $state("");
  let newPassword = $state("");
  let confirmPassword = $state("");
  let passwordsMatch = $derived(newPassword && confirmPassword && newPassword === confirmPassword);
  let confirmClass = $derived(passwordsMatch ? 'input-success' : 'input-error');
  let preferredLightTheme = $state("light");
  let preferredDarkTheme = $state("dark");

  function previewTheme(theme: string) {
    document.documentElement.setAttribute('data-theme', theme);
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
      await ps.setPreferredLightTheme(preferredLightTheme);
      await ps.setPreferredDarkTheme(preferredDarkTheme);
      addToast("Saved config!", "success");
      addToast("Cycle through light and dark mode to reset themes if necessary...", "warning", 7000);
    } catch (error) {
      console.error(error)
      addToast("Could not save config", "error")
    }
  }

  async function updatePassword() {
    // TODO write to keystore with new password
    if (!passwordsMatch) {
      addToast("Passwords do not match!", "error")
      return;
    }
    try {
      const res = await invoke("open_keystore", { password: currentPassword });
      const writtenKeystore = await invoke("write_keystore_to_file", {password: confirmPassword})
      await setPassword(confirmPassword);
      addToast("Updated Password Successfully!", "success");
    } catch (error) {
      console.error(error)
      addToast("Could not update password. Check console for error....", "error");
    }
  }

  onMount(async() => {
    try {
      downloadPath = await ps.getDownloadDir();
      appDataPath = await appDataDir(); 
    } catch (error) {
      console.error(error)
      addToast("Failed to get app directories, see logs...", "error");
    }

    try {
      preferredLightTheme = await ps.getPreferredLightTheme();
      preferredDarkTheme = await ps.getPreferredDarkTheme();
    } catch (error) {
      console.error(error)
    }
  })
</script>

<main class="config-container" class:mobile-config={$isMobile}>
  <div class="row">
    {#if !$isMobile}
      <h2 class="h2">Configuration Settings</h2>
    {/if}
    <div class="card bg-base-100 w-96 shadow-lg card-xl" style="width: auto;">
      <div class="card-body p-4">
        <div class="flex-container" class:mobile-flex={$isMobile} style="display: flex;">
          <!-- Left: Existing content -->
          <div class="left-section" class:mobile-section={$isMobile} style="flex: 1;">
            <div class="row pt-3">
              <label for="download-path">Download Path:</label>
              <input
                id="download-path"
                type="text"
                value={downloadPath}
                class="input"
                style="min-width: 100%;"
                onclick={()=>{selectPath()}}
                readonly
              />
            </div>
            <div class="row pt-3">
              <label for="app-data-path">Colony Application Data Path:</label>
              <input id="app-data-path" bind:value={appDataPath} type="text" class="input appData w-full" disabled placeholder="/home/usr/downloads" />
            </div>
            <div class="row pt-3">
              <label for="light-theme">Preferred Light Theme</label>
              <select id="light-theme" class="select" bind:value={preferredLightTheme} onchange={() => previewTheme(preferredLightTheme)}>
                {#each lightDaisyThemes as theme}
                  <option value={theme}>{theme}</option>
                {/each}
              </select>
            </div>
            <div class="row pt-3">
              <label for="dark-theme">Preferred Dark Theme</label>
              <select id="dark-theme" class="select" bind:value={preferredDarkTheme} onchange={() => previewTheme(preferredDarkTheme)}>
                {#each darkDaisyThemes as theme}
                  <option value={theme}>{theme}</option>
                {/each}
              </select>
              <div class="mt-4">
                <button class="btn btn-primary" onclick={()=>(saveConfig())}>Save</button>
              </div>
            </div>
          </div>

          <!-- Vertical rule - hidden on mobile -->
          {#if !$isMobile}
            <div style="width:1px; background:#cdcfd1; margin: 0 2rem;"></div>
          {/if}

          <!-- Right: Change Password -->
          <div class="right-section" class:mobile-section={$isMobile} style="flex: 1;">
            <div class="row pt-3">
              <label class="label" for="current-password">Current Password: </label>
              <input id="current-password" bind:value={currentPassword} type="password" class="input w-full" placeholder="Password" />
            </div>
            <div class="row">
              <label class="label" for="new-password">New Password: </label>
              <input id="new-password" bind:value={newPassword} type="password" class="input w-full" placeholder="Password" />
            </div>
            <div class="row pb-3">
              <label class="label" for="confirm-password">Confirm Password:</label>
              <input id="confirm-password" bind:value={confirmPassword} type="password" class="input {confirmClass} w-full" placeholder="Password" />
              <button class="btn btn-error mt-4" onclick={()=>{updatePassword()}}>Update Password</button>
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

/* Mobile-specific styles */
.mobile-config {
  padding: 10px !important;
  padding-top: 20px !important;
}

.mobile-flex {
  flex-direction: column !important;
  gap: 20px;
}

.mobile-section {
  flex: none !important;
  width: 100%;
}

.mobile-config .card {
  margin: 0 -10px; /* Extend to screen edges */
}

.mobile-config .row {
  margin-left: 0;
  margin-right: 0;
  padding-bottom: 1vh;
}

@media (max-width: 767px) {
  .flex-container {
    overflow-y: auto;
    max-height: calc(100vh - 200px);
  }
}




</style>
