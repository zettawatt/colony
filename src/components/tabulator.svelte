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
  let currentTheme = $globalTheme;
  let tableReady = false;
  let tableHeight = 300;

  function setTableHeight() {
    if (window) {
      tableHeight = Math.max(300, Math.min(window.innerHeight * 0.75, 1500));
    }
  }

  function switchTabulatorTheme(theme) {
    const link = document.getElementById('tabulator-theme');
    if (link) {
      link.href = theme === 'dark'
        ? '/css/tabulator_midnight.min.css'
        : '/css/tabulator.min.css';
      if (tabulatorInstance) tabulatorInstance.redraw(true);
    }
  }

  onMount(async () => {
    setTableHeight();
    window.addEventListener('resize', setTableHeight);

    tabulatorInstance = new Tabulator(tableComponent, {
      columns,
      height: tableHeight,
      minHeight: 300,
      data,
      rowContextMenu: rowMenu,
      reactiveData: false,
      layout: 'fitDataStretch',
      dependencies: {
        DateTime: DateTime,
      }, 
      initialSort: initialSort,
      tableBuilt: function() {
        tableReady = true;
      }
    });

    unlisten = await getCurrentWindow().onThemeChanged(({ payload: theme }) => {
      console.log("theme changed", theme)
      switchTabulatorTheme(theme);
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
    if (tabulatorInstance) tabulatorInstance.destroy();
    window.removeEventListener('resize', setTableHeight);
  });

  $: if (tabulatorInstance && Array.isArray(data)) {
    tabulatorInstance.replaceData(data);
  }

  $: if (typeof $globalTheme === 'string') {
    switchTabulatorTheme($globalTheme);
  }

  $: if (tabulatorInstance && tableReady && tableHeight) {
    tabulatorInstance.setHeight(tableHeight);
  }
</script>

<div bind:this={tableComponent}></div>