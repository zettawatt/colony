<script>
  // Tauri doesn't have a Node.js server to do proper SSR
  // so we will use adapter-static to prerender the app (SSG)
  // See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
  export const prerender = true;
  export const ssr = false;

  import "../app.css";
  import Header from "../components/header.svelte";
  import Footer from "../components/footer.svelte";
</script>

<Header></Header>

<main>
  <slot></slot>
</main>