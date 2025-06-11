<script lang="ts">
    import { onMount } from "svelte";

  // import Welcome from "../pages/welcome.svelte";
  import Intro from "./welcome-intro/+page.svelte";
  import ps from "../stores/persistantStorage";


  let hasUserCompletedIntro: Boolean | undefined = undefined;
  let loading = true;

  async function checkIfUserIsNew() {
    try {
      console.log("hs")
      hasUserCompletedIntro = await ps.getUserCompletedIntro();
      console.log(hasUserCompletedIntro);
      if (hasUserCompletedIntro === undefined) {
        await ps.setUserCompletedIntro(false);
        hasUserCompletedIntro = false;
      } else if (hasUserCompletedIntro) {
        window.location.href = '/screens/search';
      }
    } catch (e) {
      console.log(e)
    } finally {
      loading = false;
    }
  }

  onMount(async () => {
    await ps.initStore();
    await checkIfUserIsNew();
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

