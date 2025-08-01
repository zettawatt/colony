<script lang="ts">
  import { setTheme } from '@tauri-apps/api/app';
  import { onMount } from 'svelte';
  import ps from '../stores/persistantStorage';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { globalTheme } from '../stores/globals';

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
    const unlisten = await getCurrentWindow().onThemeChanged(async ({ payload: theme }) => {
      console.log(theme)
      await setAppTheme(theme);
    });
  });

  function cycleTheme() {
    if (theme === 'auto') theme = 'light';
    else if (theme === 'light') theme = 'dark';
    else theme = 'auto';
    setAppTheme(theme);
  }

  async function setAppTheme(t: Theme) {
    try {
      const preferredLightTheme = await ps.getPreferredLightTheme();
      const preferredDarkTheme = await ps.getPreferredDarkTheme();
      await ps.setTheme(t);
      if (t === 'auto') {
        await setTheme(undefined);
        const tauriTheme = await getCurrentWindow().theme();
        const preferredTheme = (tauriTheme === 'light') ? preferredLightTheme : preferredDarkTheme
        document.documentElement.setAttribute("data-theme", preferredTheme);
      } else if (t === 'light') {
        await setTheme(t);
        document.documentElement.setAttribute("data-theme", preferredLightTheme);
      } else {
        await setTheme(t);
        document.documentElement.setAttribute("data-theme", preferredDarkTheme);
      }

      // let currentTheme = await getCurrentWindow().theme();
      $globalTheme = (t !== undefined) ? t : 'light';
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
    <img src="/app-icons/circle-arrow-icon.svg" alt="auto mode icon" width="16" height="16" />
  {:else if theme === 'light'}
    <img src="/app-icons/sun-icon.svg" alt="light mode icon" width="24" height="24" />
  {:else}
    <img src="/app-icons/moon-cloud-icon.svg" alt="dark mode icon" width="24" height="24" class="dark:invert" />
  {/if}
  <span>{themeLabel(theme)}</span>
</button>
