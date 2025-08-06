<script lang="ts">
  import "../../app.css";
  import Header from "../../components/header.svelte";
  import Footer from "../../components/footer.svelte";
  import MobileBottomNav from "../../components/MobileBottomNav.svelte";
  import { initMobileDetection, isMobile } from "../../utils/responsive.js";
  import { onMount, onDestroy } from "svelte";

  let cleanupMobileDetection: () => void;

  onMount(() => {
    cleanupMobileDetection = initMobileDetection();
  });

  onDestroy(() => {
    if (cleanupMobileDetection) {
      cleanupMobileDetection();
    }
  });
</script>

<div class="layout-container" class:mobile-layout={$isMobile}>
  <Header />

  <main class="app-content-container" class:mobile-content={$isMobile}>
    <slot />
  </main>

  {#if !$isMobile}
    <Footer />
  {/if}

  {#if $isMobile}
    <MobileBottomNav />
  {/if}
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

  /* Mobile-specific styles */
  .mobile-layout {
    /* Ensure mobile layout takes full height */
    height: 100vh;
    overflow: hidden;
  }

  .mobile-content {
    /* Account for bottom navigation bar height (60px) */
    padding-bottom: 60px;
    overflow-y: auto;
  }

  /* Hide scrollbars on mobile for cleaner look */
  @media (max-width: 767px) {
    .mobile-content::-webkit-scrollbar {
      display: none;
    }
    .mobile-content {
      -ms-overflow-style: none;
      scrollbar-width: none;
    }
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