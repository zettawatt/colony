<script>
  import { onMount, onDestroy } from 'svelte';
  import { TabulatorFull as Tabulator } from 'tabulator-tables';
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { DateTime } from "luxon";

  export let columns, data, rowMenu;

  let tableComponent;
  let tabulatorInstance;
  let unlisten;
  let currentTheme = 'light'; // default

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

    // Create Tabulator instance
    tabulatorInstance = new Tabulator(tableComponent, {
      columns,
      height: 500,
      data,
      rowContextMenu: rowMenu,
      reactiveData: false,
      layout: 'fitDataStretch',
      persistence:{
        sort:true,
        filter:true,
      },
      persistenceID:"tabulatorPersistance",
      dependencies:{
        DateTime:DateTime,
      }, 
    });

    // Listen for theme changes from Tauri
    unlisten = await getCurrentWindow().onThemeChanged(({ payload: theme }) => {
      switchTabulatorTheme(theme);
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
    if (tabulatorInstance) tabulatorInstance.destroy();
  });

  $: if (tabulatorInstance && Array.isArray(data)) {
    tabulatorInstance.setData(data);
  }
</script>

<div bind:this={tableComponent}></div>