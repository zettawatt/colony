<header class="">
  <div class="navbar bg-base-100 shadow-sm">
    <div class="flex-1">
      <a href="/screens/app-info" class="btn btn-ghost text-xl">
        <span><img src="/logo.svg" class="logo svelte-kit" alt="Colony Logo" width="24" height="24"/></span>
        Colony
      </a>
    </div>
    <div class="flex-3 flex justifiy-center">
      <ul class="menu menu-horizontal px-1 flex justify-between w-full">
        <li><a href="/screens/search">Search</a></li>
        <li>
          <details>
            <summary>Uploads</summary>
            <ul class="bg-base-100 rounded-t-none p-2">
              <li><a>Link 1</a></li>
              <li><a>Link 2</a></li>
            </ul>
          </details>
        </li>
        <li><a>Downloads</a></li>
        <li><a>Colonies</a></li>
        <li><a>Wallet</a></li>
        <li><a href="/welcome-intro">Configuration</a></li>
      </ul>
    </div>
    <div class="flex-1"></div>  <!-- Added this empty div for balance -->
  </div>
</header>