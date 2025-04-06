<script lang="ts">
  import { Icons } from "$lib/components/docs/index.ts";
  import { page } from "$app/stores";
  import { siteConfig } from "$lib/config/site.js";
  import { cn } from "$lib/utils.js";
</script>

<div class="mr-4 hidden md:flex">
  <a href="/" class="mr-6 flex items-center space-x-2">
    <Icons.logo class="h-6 w-6" />
    <span class="hidden font-bold xl:inline-block">
      {siteConfig.name}
    </span>
  </a>
  <nav class="flex items-center gap-6 text-sm">
    <a
      href="/library"
      class={cn(
      "hover:text-foreground/80 transition-colors",
      $page.url.pathname.startsWith("/library") ? "text-foreground" : "text-foreground/60"
      )}
      >
      Library
    </a>
    <a
      href="/pods"
      class={cn(
      "hover:text-foreground/80 transition-colors",
      $page.url.pathname.startsWith("/library") ? "text-foreground" : "text-foreground/60"
      )}
      >
      Pods
    </a>
    <a
      href="/upload"
      class={cn(
      "hover:text-foreground/80 transition-colors",
      $page.url.pathname.startsWith("/upload") ? "text-foreground" : "text-foreground/60"
      )}
      >
      Upload
    </a>
    <a
      href="/profile"
      class={cn(
      "hover:text-foreground/80 transition-colors",
      $page.url.pathname.startsWith("/profile") ? "text-foreground" : "text-foreground/60"
      )}
      >
      Profile
    </a>
  </nav>
</div>
