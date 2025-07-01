<script lang="ts">
  import { onMount } from 'svelte';
  let loginModal: HTMLDialogElement;
  let password;

  onMount(() => {
    // Open the modal when the component is mounted
    loginModal?.showModal();
  });

  function handleClose() {
    console.log(password)
    if (!password) return;
    loginModal?.close();
  }
</script>

<dialog id="my_modal_1" class="modal" bind:this={loginModal}>
  <div class="modal-box">
    <h3 class="text-lg font-bold">Login</h3>
    <p class="pt-4">Your password is needed for write access to your pod storage.</p>
    <input type="password" placeholder="Password" class="input input-bordered w-full my-4" bind:value={password}/>
    <div class="modal-action">
      <button class="btn" on:click={handleClose}>Authenticate</button>
    </div>
  </div>
</dialog>
