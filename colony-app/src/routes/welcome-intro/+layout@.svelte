<script>
  export const prerender = true;
  export const ssr = false;
  import "../../app.css";
</script>

<main class="">
  <slot />
</main>

<style>
  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
    overflow-y: scroll;
    /* Add these lines to prevent layout shift */
  }
</style>