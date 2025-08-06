<script lang="ts">
  import MobileHamburgerMenu from './MobileHamburgerMenu.svelte';
  import { isMobile } from '../utils/responsive.js';

  export let drawerId: string = "my-drawer-2"; // Allow custom IDs if needed
</script>

{#if $isMobile}
  <!-- Mobile layout with hamburger menu -->
  <div class="mobile-drawer" style="height: 100%; display: flex; flex-direction: column;">
    <div class="mobile-header" style="padding: 10px; border-bottom: 1px solid var(--fallback-bc, oklch(var(--bc) / 0.2));">
      <MobileHamburgerMenu />
    </div>
    <div class="mobile-content" style="flex: 1; min-height: 0; overflow: hidden;">
      <slot name="main" />
    </div>
  </div>
{:else}
  <!-- Desktop layout with sidebar -->
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
{/if}