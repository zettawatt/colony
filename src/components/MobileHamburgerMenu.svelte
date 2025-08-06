<script lang="ts">
  import { page } from '$app/stores';
  
  let isOpen = $state(false);
  
  // Get current route to highlight active item
  $: currentRoute = $page.url.pathname;
  
  function toggleMenu() {
    isOpen = !isOpen;
  }
  
  function closeMenu() {
    isOpen = false;
  }
  
  function isActive(route: string): boolean {
    return currentRoute.includes(route);
  }
  
  // Close menu when clicking outside
  function handleOutsideClick(event: MouseEvent) {
    const target = event.target as Element;
    if (!target.closest('.hamburger-menu')) {
      closeMenu();
    }
  }
</script>

<svelte:window on:click={handleOutsideClick} />

<div class="hamburger-menu">
  <button 
    class="hamburger-button" 
    onclick={toggleMenu}
    aria-label="Toggle navigation menu"
    aria-expanded={isOpen}
  >
    <svg 
      xmlns="http://www.w3.org/2000/svg" 
      width="24" 
      height="24" 
      fill="none" 
      viewBox="0 0 24 24" 
      stroke="currentColor"
      class:open={isOpen}
    >
      {#if isOpen}
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
      {:else}
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
      {/if}
    </svg>
  </button>
  
  {#if isOpen}
    <div class="menu-dropdown">
      <a 
        href="/screens/pod-management/your-pods" 
        class="menu-item" 
        class:active={isActive('your-pods')}
        onclick={closeMenu}
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2-2z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5a2 2 0 012-2h4a2 2 0 012 2v2H8V5z" />
        </svg>
        Your Pods
      </a>
      
      <a 
        href="/screens/pod-management/uploads" 
        class="menu-item" 
        class:active={isActive('uploads')}
        onclick={closeMenu}
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
        </svg>
        Uploads
      </a>
      
      <a 
        href="/screens/pod-management/downloads" 
        class="menu-item" 
        class:active={isActive('downloads')}
        onclick={closeMenu}
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 12l2 2 4-4" />
        </svg>
        Downloads
      </a>
    </div>
  {/if}
</div>

<style>
  .hamburger-menu {
    position: relative;
    display: inline-block;
  }

  .hamburger-button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 44px;
    height: 44px;
    border: none;
    background: var(--fallback-b2, oklch(var(--b2)));
    border-radius: 8px;
    color: var(--fallback-bc, oklch(var(--bc)));
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .hamburger-button:hover {
    background: var(--fallback-b3, oklch(var(--b3)));
  }

  .hamburger-button svg {
    transition: transform 0.2s ease;
  }

  .hamburger-button svg.open {
    transform: rotate(90deg);
  }

  .menu-dropdown {
    position: absolute;
    top: 100%;
    left: 0;
    margin-top: 8px;
    background: var(--fallback-b1, oklch(var(--b1)));
    border: 1px solid var(--fallback-bc, oklch(var(--bc) / 0.2));
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    z-index: 1000;
    min-width: 160px;
    overflow: hidden;
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    color: var(--fallback-bc, oklch(var(--bc)));
    text-decoration: none;
    transition: background-color 0.2s ease;
    font-size: 14px;
    font-weight: 500;
    border-bottom: 1px solid var(--fallback-bc, oklch(var(--bc) / 0.1));
  }

  .menu-item:last-child {
    border-bottom: none;
  }

  .menu-item:hover {
    background: var(--fallback-b2, oklch(var(--b2)));
  }

  .menu-item.active {
    background: var(--fallback-b2, oklch(var(--b2)));
    color: #e28743;
  }

  .menu-item svg {
    flex-shrink: 0;
  }
</style>
