<script lang="ts">
  import {
    MainNav,
    ModeToggle,
    GitHubIcon,
  } from "$lib/components/docs/index.js";
</script>

<header
  class="border-border/40 bg-background/95 supports-[backdrop-filter]:bg-background/60 sticky top-0 z-50 w-full border-b backdrop-blur"
>
  <div class="container flex h-14 max-w-screen-2xl items-center">
    <MainNav />
    <div class="flex flex-1 items-center justify-between space-x-2 md:justify-end">
      <nav class="flex items-center">
        <GitHubIcon />
        <ModeToggle />
      </nav>
    </div>
  </div>
</header>
