<script lang="ts">
  import "../../app.css";
  import Header from "../../components/header.svelte";
  import Footer from "../../components/footer.svelte";
</script>

<div class="layout-container">
  <Header />
  
  <main class="app-content-container">
    <slot />
  </main>
  
  <Footer />
</div>

<style>
  .layout-container {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }

  .app-content-container {
    flex: 1;
    /* padding-top: 5vh; */
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
  /* .container {
    margin: 0;
    flex: 1;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
    overflow-y: auto;
    max-width: 64rem;
    width: 100%;
    box-sizing: border-box;
  } */

  /* @media (width >= 48rem) {
    .container {
      max-width: 64rem;
    }
  } */
  
</style>

<svelte:head>
  <link id="tabulator-theme" rel="stylesheet" href="/css/tabulator.min.css">
</svelte:head>