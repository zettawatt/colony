<script lang="ts">
  import { Icons } from "$lib/components/docs/index.ts";
  import { buttonVariants } from "$lib/components/ui/button/index.js";
  import { siteConfig } from "$lib/config/site.ts";
  import { cn } from "$lib/utils.js";
</script>

<a href={siteConfig.links.github} target="_blank" rel="noopener noreferrer">
  <div
    class={cn(
    buttonVariants({
    size: "sm",
    variant: "ghost",
    }),
    "h-8 w-8 px-0"
    )}
    >
    <Icons.gitHub class="h-4 w-4" />
    <span class="sr-only">GitHub</span>
  </div>
</a>
