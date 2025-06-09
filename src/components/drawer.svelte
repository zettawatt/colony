<script lang="ts">
  export let drawerId: string = "my-drawer-2"; // Allow custom IDs if needed
</script>

<div class="drawer drawer-open" style="padding-top:1vh;">
  <input id={drawerId} type="checkbox" class="drawer-toggle" />
  <div class="drawer-content flex flex-col">
    <slot name="main" />
  </div>
  <div class="drawer-side" style="height: 90vh;">
    <label for={drawerId} aria-label="close sidebar" class="drawer-overlay"></label>
    <slot name="sidebar" />
  </div>
</div>