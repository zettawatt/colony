<script lang="ts">
  import { SiteHeader } from "$lib/components/docs/index.ts"
  import { ModeWatcher } from "mode-watcher";
  import '../app.css';
	
  let { children } = $props();
</script>

<ModeWatcher />
<SiteHeader />
<div class="flex-1">
  {@render children()}
</div>
