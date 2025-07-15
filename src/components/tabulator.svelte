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
  let windowScroll = [0, 0]; // store X, Y

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

  function replaceTableDataWithScrollRestore(newData) {
    windowScroll = [window.scrollX, window.scrollY]; // save before replacing data
    tabulatorInstance.replaceData(newData); // update data
  }

  function handleRenderComplete() {
    window.scroll(windowScroll[0], windowScroll[1]);
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

    tabulatorInstance.on('renderComplete', handleRenderComplete);

    unlisten = await getCurrentWindow().onThemeChanged(({ payload: theme }) => {
      console.log("theme changed", theme)
      switchTabulatorTheme(theme);
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
    if (tabulatorInstance) {
      tabulatorInstance.off('renderComplete', handleRenderComplete);
      tabulatorInstance.destroy();
    }
    window.removeEventListener('resize', setTableHeight);
  });

  $: if (tabulatorInstance && Array.isArray(data)) {
    replaceTableDataWithScrollRestore(data);
  }

  $: if (typeof $globalTheme === 'string') {
    switchTabulatorTheme($globalTheme);
  }

  $: if (tabulatorInstance && tableReady && tableHeight) {
    tabulatorInstance.setHeight(tableHeight);
  }
</script>

<div bind:this={tableComponent}></div>