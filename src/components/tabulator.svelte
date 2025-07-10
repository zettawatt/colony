<script>
  import { onMount, onDestroy } from 'svelte';
  import { TabulatorFull as Tabulator } from 'tabulator-tables';
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { DateTime } from "luxon";
  import { globalTheme } from '../stores/globals';

  export let columns, data, rowMenu, initialSort;

  let tableComponent;
  let tabulatorInstance;
  let unlisten;
  let currentTheme = $globalTheme; // default

  function switchTabulatorTheme(theme) {
    const link = document.getElementById('tabulator-theme');
    if (link) {
      link.href = theme === 'dark'
        ? '/css/tabulator_midnight.min.css'
        : '/css/tabulator.min.css';
      // Redraw table in case dimensions changed
      if (tabulatorInstance) tabulatorInstance.redraw(true);
    }
  }

  onMount(async () => {
    // const currentTheme = await getCurrentWindow().theme();
    // Create Tabulator instance
    tabulatorInstance = new Tabulator(tableComponent, {
      columns,
      height: 300,
      maxHeight: 700,
      data,
      rowContextMenu: rowMenu,
      reactiveData: false,
      layout: 'fitDataStretch',
      dependencies:{
        DateTime:DateTime,
      }, 
      initialSort: initialSort,
    });

    // Listen for theme changes from Tauri
    unlisten = await getCurrentWindow().onThemeChanged(({ payload: theme }) => {
      console.log("theme changed", theme)
      switchTabulatorTheme(theme);
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
    if (tabulatorInstance) tabulatorInstance.destroy();
  });

  $: if (tabulatorInstance && Array.isArray(data)) {
    tabulatorInstance.replaceData(data);
  }

  $: if (typeof $globalTheme === 'string') {
    switchTabulatorTheme($globalTheme);
  }
</script>

<div bind:this={tableComponent}></div>