<script lang="ts">
    import { onMount } from "svelte";

  // import Welcome from "../pages/welcome.svelte";
  import Intro from "./welcome-intro/+page.svelte";
  import ps from "../stores/persistantStorage";
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");
  let statusMessage = $state("");
  let isLoading = $state(false);


  let hasUserCompletedIntro: Boolean | undefined = $state(undefined);
  let loading = $state(true);

  async function checkIfUserIsNew() {
    let wasUserNew = false;
    try {
      hasUserCompletedIntro = await ps.getUserCompletedIntro();
      if (hasUserCompletedIntro === undefined) {
        await ps.initStore();
        hasUserCompletedIntro = false;
        wasUserNew = true;
      } else if (hasUserCompletedIntro) {
        wasUserNew = false;
        // window.location.href = '/screens/search';
      }
    } catch (e) {
      console.log(e)
    } finally {
      loading = false;
      return wasUserNew;
    }
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
  
  async function initPodManager(wasUserNew: boolean) {
    try {
      await invoke("initialize_datastore");
      if (!wasUserNew) {await invoke("open_keystore", { password: "maxx" });}
      await invoke("initialize_graph");
      const result = await invoke("initialize_pod_manager");

    } catch (error) {
      console.log(error);
    }
  }

  onMount(async () => {
    console.log("here maxx")
    const wasUserNew = await checkIfUserIsNew();
    await initializeAutonomiClient();
    await initPodManager(wasUserNew);
    if (!wasUserNew) {
      window.location.href = '/screens/search';
    }
  })

</script>

<main>
  <div class="">
    {#if !loading && hasUserCompletedIntro === false}
      <div class="">
        <Intro/>
      </div>
    {/if}
  </div>
</main>

