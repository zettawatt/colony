<script lang="ts">
  import { page } from '$app/state';

  // Get current route to highlight active tab
  let currentRoute = $derived(page.url.pathname);

  function isActive(route: string): boolean {
    if (route === '/screens/pod-management' && currentRoute.includes('/screens/pod-management')) {
      return true;
    }
    return currentRoute === route;
  }
</script>

<nav class="mobile-bottom-nav">
  <a 
    href="/screens/search" 
    class="nav-item" 
    class:active={isActive('/screens/search')}
    aria-label="Search"
  >
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
    </svg>
  </a>

  <a 
    href="/screens/status" 
    class="nav-item" 
    class:active={isActive('/screens/status')}
    aria-label="Status"
  >
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
    </svg>
  </a>

  <a 
    href="/screens/pod-management/your-pods" 
    class="nav-item" 
    class:active={isActive('/screens/pod-management')}
    aria-label="File Management"
  >
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2-2z" />
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5a2 2 0 012-2h4a2 2 0 012 2v2H8V5z" />
    </svg>
  </a>

  <a 
    href="/screens/wallet" 
    class="nav-item" 
    class:active={isActive('/screens/wallet')}
    aria-label="Wallet"
  >
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
    </svg>
  </a>

  <a 
    href="/screens/configuration" 
    class="nav-item" 
    class:active={isActive('/screens/configuration')}
    aria-label="Configuration"
  >
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
    </svg>
  </a>
</nav>

<style>
  .mobile-bottom-nav {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    height: 60px;
    background: var(--fallback-b1, oklch(var(--b1)));
    border-top: 1px solid var(--fallback-bc, oklch(var(--bc) / 0.2));
    display: flex;
    justify-content: space-around;
    align-items: center;
    z-index: 1000;
    padding: 0 8px;
    box-shadow: 0 -2px 8px rgba(0, 0, 0, 0.1);
  }

  .nav-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 8px 12px;
    border-radius: 8px;
    color: var(--fallback-bc, oklch(var(--bc) / 0.6));
    text-decoration: none;
    transition: all 0.2s ease;
    min-width: 48px;
    min-height: 48px;
  }

  .nav-item:hover {
    background: var(--fallback-b2, oklch(var(--b2)));
    color: var(--fallback-bc, oklch(var(--bc)));
  }

  .nav-item.active {
    color: #e28743;
    background: var(--fallback-b2, oklch(var(--b2)));
  }

  .nav-item svg {
    width: 24px;
    height: 24px;
  }

  /* Ensure proper spacing on very small screens */
  @media (max-width: 360px) {
    .nav-item {
      padding: 6px 8px;
      min-width: 40px;
      min-height: 40px;
    }
    
    .nav-item svg {
      width: 20px;
      height: 20px;
    }
  }
</style>
