<script>
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