<script lang="ts">
  import ThemeSwitcher from './themeSwitcher.svelte';
  import { isMobile } from '../utils/responsive.js';
</script>

<header class="">
  <div class="navbar bg-base-100 shadow-sm" class:mobile-header={$isMobile}>
    <div class="flex-1">
      <a href="/screens/app-info" class="btn btn-ghost text-xl">
        <span><img src="/logo.svg" class="logo svelte-kit" alt="Colony Logo" width="40" height="40"/></span>
        <span class="colony-logo-text" class:mobile-logo-text={$isMobile}>Colony</span>
      </a>
    </div>
{#if !$isMobile}
    <div class="flex-3 flex justify-center">
      <ul class="menu menu-horizontal px-1 flex justify-between w-full">
        <li>
          <a href="/screens/search" class="btn btn-ghost">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
            <span class="xl:inline hidden">Search</span>
          </a>
        </li>
        <li>
          <a href="/screens/status" class="btn btn-ghost">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
            </svg>
            <span class="xl:inline hidden">Status</span>
          </a>
        </li>
        <li>
          <a href="/screens/pod-management/downloads" class="btn btn-ghost">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2-2z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 5a2 2 0 012-2h4a2 2 0 012 2v2H8V5z" />
            </svg>
            <span class="xl:inline hidden">File Management</span>
          </a>
        </li>
        <!-- <li class="dropdown">
          <div tabindex="0" role="button" class="btn-ghost">File Management</div>
          <ul tabindex="0" class="menu dropdown-content bg-base-100 rounded-box w-52 shadow p-2 z-[1000]">
            <li><a href="/screens/pod-management/your-pods">Your Pods</a></li>
            <li><a href="/screens/pod-management/uploads">Uploads</a></li>
            <li><a href="/screens/pod-management/downloads">Downloads</a></li>
          </ul>
        </li> -->
        <!-- <li><a>Colonies</a></li> -->
        <li>
          <a href="/screens/wallet" class="btn btn-ghost">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 7h16a2 2 0 012 2v8a2 2 0 01-2 2H4a2 2 0 01-2-2V9a2 2 0 012-2z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 5h12a1 1 0 011 1" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 11h8M6 13h6" />
              <circle cx="17" cy="13" r="2" fill="none" stroke="currentColor" stroke-width="2"/>
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16.5 12.5h1" />
            </svg>
            <span class="xl:inline hidden">Wallet</span>
          </a>
        </li>
        <li>
          <a href="/screens/configuration" class="btn btn-ghost">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            <span class="xl:inline hidden">Configuration</span>
          </a>
        </li>
        <!-- <li><a href="/user-intro">Steps</a></li> -->
        <!-- <li><a href="/screens/test">Test</a></li> -->
      </ul>
    </div>
    {/if}
    <div class="flex-1 flex justify-end">
      <ThemeSwitcher />
    </div>
  </div>
</header>

<style>
  @font-face {
    font-family: "ColonyFont";
    src: url("/fonts/HiBlack-n3a1.otf") format("opentype");
    font-weight: normal;
    font-style: normal;
  }

  .colony-logo-text {
    font-family: "ColonyFont", sans-serif;
    font-weight: 800;
    font-size: 30px;
    color: #e28743;
  }

  /* Mobile-specific header styles */
  .mobile-header {
    padding: 0 16px;
    min-height: 56px;
  }

  .mobile-logo-text {
    display: inline !important;
    font-size: 24px;
  }

  /* Ensure header is properly sized on mobile */
  @media (max-width: 767px) {
    .navbar {
      min-height: 56px;
    }
  }


</style>