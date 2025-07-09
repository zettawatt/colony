<script lang="ts">
  import { setTheme } from '@tauri-apps/api/app';
  import { onMount } from 'svelte';

  type Theme = 'auto' | 'light' | 'dark' | undefined | null;
  let theme: Theme = 'light';

  onMount(() => {
    setAppTheme(theme);
  });

  function cycleTheme() {
    if (theme === 'auto') theme = 'light';
    else if (theme === 'light') theme = 'dark';
    else theme = 'auto';
    setAppTheme(theme);
  }

  async function setAppTheme(t: Theme) {
    try {
      if (!t) {
        t = 'light';
      }
      await setTheme(t);
    } catch (error) {
      console.error(error);
    }
  }

  function themeLabel(t: Theme) {
    if (t === 'auto') return 'Auto';
    if (t === 'light') return 'Light';
    return 'Dark';
  }
</script>

<button
  class="btn btn-ghost"
  aria-label="Change theme"
  on:click={cycleTheme}
>
  {#if theme === 'auto'}
    <img src="/app-icons/operations-icon.svg" alt="auto mode icon" width="24" height="24" />
  {:else if theme === 'light'}
    <img src="/app-icons/sun-icon.svg" alt="light mode icon" width="24" height="24" />
  {:else}
    <img src="/app-icons/moon-cloud-icon.svg" alt="dark mode icon" width="24" height="24" class="dark:invert" />
  {/if}
  <span>{themeLabel(theme)}</span>
</button>
