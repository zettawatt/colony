<!-- src/components/StepFinish.svelte -->
<script lang="ts">
  export let runSync: boolean;
</script>

<div>
  <h2 class="text-xl font-bold">🎉 All Set!</h2>
  <p>You're ready to use the app. Click Finish to populate your search cache and start browsing Autonomi!</p>
  <p>Deselect this box if you want to skip the sync step.</p>
  <label class="label cursor-pointer">
    <input type="checkbox" class="checkbox" bind:checked={runSync} />
    Run Sync
  </label>
</div>
