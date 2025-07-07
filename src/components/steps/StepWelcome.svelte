<!-- src/components/StepWelcome.svelte -->
<script lang="ts">
  
  let newPassword = $state("");
  let confirmPassword = $state("");
  let passwordsMatch = $derived(newPassword && confirmPassword && newPassword === confirmPassword);
  let confirmClass = $derived(passwordsMatch ? 'input-success' : 'input-error');
  let { validatePassword } = $props();

</script>

<div>
  <h3 class="text-3xl font-extrabold dark:text-white" style="text-align: center;">Welcome!</h3>

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
    <input bind:value={confirmPassword} 
      type="password" 
      class="input {confirmClass}" 
      placeholder="Password" 
      oninput={()=>{validatePassword(newPassword, confirmPassword)}}
    />
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

</style>