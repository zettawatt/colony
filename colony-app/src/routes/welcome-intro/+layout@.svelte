<script>
  export const prerender = true;
  export const ssr = false;
  import "../../app.css";
</script>

<main class="container">
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
  }
</style>