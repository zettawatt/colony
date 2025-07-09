<script lang="ts">
  import { setTheme } from '@tauri-apps/api/app';
  import { onMount } from 'svelte';
  import ps from '../stores/persistantStorage';

  type Theme = 'auto' | 'light' | 'dark';
  let theme: Theme = 'auto';

  async function getUserTheme() {
    try {
      const userTheme = await ps.getTheme() as Theme;
      return userTheme
    } catch (error) {
      console.error(error)
      return 'auto'
    }
  }

  onMount(async() => {
    theme = await getUserTheme()
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
      await ps.setTheme(t);
      if (t === 'auto') {
        await setTheme(undefined);
      } else {
        await setTheme(t);
      }
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
    <img src="/app-icons/circle-arrow-icon.svg" alt="auto mode icon" width="18" height="18" />
  {:else if theme === 'light'}
    <img src="/app-icons/sun-icon.svg" alt="light mode icon" width="24" height="24" />
  {:else}
    <img src="/app-icons/moon-cloud-icon.svg" alt="dark mode icon" width="24" height="24" class="dark:invert" />
  {/if}
  <span>{themeLabel(theme)}</span>
</button>
