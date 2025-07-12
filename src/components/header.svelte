<script lang="ts">
  import { onMount } from 'svelte';
  import ThemeSwitcher from './themeSwitcher.svelte';
</script>

<header class="">
  <div class="navbar bg-base-100 shadow-sm">
    <div class="flex-1">
      <a href="/screens/app-info" class="btn btn-ghost text-xl">
        <span><img src="/logo.svg" class="logo svelte-kit" alt="Colony Logo" width="40" height="40"/></span>
        <span class="colony-logo-text">Colony</span>
      </a>
    </div>
    <div class="flex-3 flex justify-center">
      <ul class="menu menu-horizontal px-1 flex justify-between w-full">
        <li><a href="/screens/search">Search</a></li>
        <li class="dropdown">
          <div tabindex="0" role="button" class="btn-ghost">Pod Management</div>
          <ul tabindex="0" class="menu dropdown-content bg-base-100 rounded-box w-52 shadow p-2 z-[1000]">
            <li><a href="/screens/pod-management/your-pods">Your Pods</a></li>
            <li><a href="/screens/pod-management/uploads">Uploads</a></li>
            <li><a href="/screens/pod-management/downloads">Downloads</a></li>
          </ul>
        </li>
        <li><a>Colonies</a></li>
        <li><a href="/screens/wallet">Wallet</a></li>
        <li><a href="/screens/configuration">Configuration</a></li>
        <li><a href="/user-intro">Steps</a></li>
        <li><a href="/screens/test">Test</a></li>
      </ul>
    </div>
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
</style>