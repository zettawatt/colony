<script>
  // Tauri doesn't have a Node.js server to do proper SSR
  // so we will use adapter-static to prerender the app (SSG)
  // See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
  export const prerender = true;
  export const ssr = false;

  import "../app.css";
    import Toast from '../components/toast.svelte';
</script>

<div class="">
  <slot/>
  <Toast />
</div>

<style>
  /* .layout-container {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }

  .content {
    flex: 1;
  } */
</style>