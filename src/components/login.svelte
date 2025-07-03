<script lang="ts">
  import { onMount } from 'svelte';
  import { getPassword, setPassword } from '../utils/password/session';
  import { invoke } from '@tauri-apps/api/core';
  import { tick } from 'svelte';
  let loginModal: HTMLDialogElement;
  let password: string;
  let wasPasswordInvalid = false;
  let checkingAuth = false;

  onMount(() => {
    if (loginModal) {
      loginModal.showModal();

      // Prevent closing via Escape key
      loginModal.addEventListener('cancel', (e) => {
        e.preventDefault();
      });
    }
  });

  async function handleClose() {
    if (!password) return;
    checkingAuth = true;
    wasPasswordInvalid = false;
    await tick();
    try {
      await invoke("open_keystore", { password: password });
      setPassword(password);
      loginModal?.close(); 
    } catch (error) {
      console.trace(error);
      wasPasswordInvalid = true;
      checkingAuth = false;
    }
  }

  async function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      await handleClose();
    }
  }

</script>

<dialog id="my_modal_1" class="modal backdrop-blur-sm" bind:this={loginModal} onkeydown={handleKeydown}>
  <div class="modal-box">
    <h3 class="text-lg font-bold">Login</h3>
    <p class="pt-4">Your password is needed for write access to your pod storage.</p>
    <input type="password" placeholder="Password" class="input w-full my-4 input-{wasPasswordInvalid ? "error": "bordered"}" bind:value={password}/>
    {#if wasPasswordInvalid}
      <p class="text-red-500">Password was invalid</p>
    {/if}
    <div class="modal-action">
      <button class="btn btn-error" onclick={()=>{handleClose()}}>
        {#if checkingAuth}
          <span class="loading loading-spinner"></span>
        {:else}
          Authenticate
        {/if}
      </button>
    </div>
  </div>
</dialog>
