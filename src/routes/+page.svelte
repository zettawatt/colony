<script lang="ts">
  import { onMount } from "svelte";

  // import Welcome from "../pages/welcome.svelte";
  import Intro from "./welcome-intro/+page.svelte";
  import UserIntro from "./user-intro/+page.svelte";
  import ps from "../stores/persistantStorage";
  import { initDatastore } from "../utils/colony/initColony";

  let hasUserCompletedIntro: Boolean | undefined = $state(undefined);
  let loading = $state(true);

  async function checkIfUserIsNew() {
    let wasUserNew = false;
    try {
      hasUserCompletedIntro = await ps.getUserCompletedIntro();
      if (hasUserCompletedIntro === undefined || hasUserCompletedIntro === false) {
        await ps.initStore();
        hasUserCompletedIntro = false;
        wasUserNew = true;
      } else if (hasUserCompletedIntro) {
        wasUserNew = false;
        // window.location.href = '/screens/search';
      }
    } catch (e) {
      console.error(e)
    } finally {
      loading = false;
      return wasUserNew;
    }
  }

  // App startup
  onMount(async () => {
    try {
      await initDatastore();
      const wasUserNew = await checkIfUserIsNew();
      if (!wasUserNew) {
        window.location.href = '/screens/search';
      } 
    } catch (error) {
      console.error(error)
      throw error;
    }
  })

</script>

<main>
  <div class="">
    {#if !loading && hasUserCompletedIntro === false}
      <div class="">
        <!-- <Intro/> -->
         <UserIntro/>
      </div>
    {/if}
  </div>
</main>

