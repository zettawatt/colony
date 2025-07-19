<script lang="ts">
  export let drawerId: string = "my-drawer-2"; // Allow custom IDs if needed
</script>

<div class="drawer drawer-open" style="height: 100%; display: flex; flex-direction: row;">
  <input id={drawerId} type="checkbox" class="drawer-toggle" />
  <div class="drawer-side" style="height: 100%; flex-shrink: 0;">
    <label for={drawerId} aria-label="close sidebar" class="drawer-overlay"></label>
    <slot name="sidebar" />
  </div>
  <div class="drawer-content flex flex-col" style="flex: 1; min-height: 0; overflow: hidden;">
    <slot name="main" />
  </div>
</div>