<script>
  import { onMount, onDestroy, createEventDispatcher } from 'svelte';
  import { TabulatorFull as Tabulator } from 'tabulator-tables';
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { DateTime } from "luxon";
  import { globalTheme } from '../stores/globals';

  export let columns, data, rowMenu, initialSort;

  const dispatch = createEventDispatcher();

  let tableComponent;
  let tabulatorInstance = null;

  // Export the tabulator instance so parent components can access it
  export { tabulatorInstance };

  // Also create a getter function to access the instance
  export function getTabulatorInstance() {
    return tabulatorInstance;
  }
  let unlisten;
  let currentTheme = $globalTheme;
  let tableReady = false;
  let tableHeight = 300;

  function setTableHeight() {
    if (window) {
      tableHeight = Math.max(300, Math.min(window.innerHeight * 0.75, 1500));
    }
  }

  let resizeTimeout;

  function handleWindowResize() {
    // Debounce resize events to avoid excessive redraws
    clearTimeout(resizeTimeout);
    resizeTimeout = setTimeout(() => {
      setTableHeight();

      // Dispatch a custom event to notify parent components about resize
      // This allows parent components to update column widths before table redraw
      window.dispatchEvent(new CustomEvent('tabulator-resize-start'));

      // Small delay to allow parent components to update columns
      setTimeout(() => {
        if (tabulatorInstance && tableReady) {
          // Update columns first (this will trigger the reactive statement)
          tabulatorInstance.setColumns(columns);
          // Then redraw the table to handle layout changes
          tabulatorInstance.redraw(true);
        }
      }, 10);
    }, 100); // 100ms debounce
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
    window.addEventListener('resize', handleWindowResize);

    try {
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

      // Force tableReady after a timeout if tableBuilt doesn't fire
      setTimeout(() => {
        if (!tableReady) {
          tableReady = true;
        }

        // Also try to replace data if we have it
        if (tabulatorInstance && Array.isArray(data) && data.length > 0) {
          tabulatorInstance.replaceData(data);
        }
      }, 1000);
    } catch (error) {
      console.error('Failed to create Tabulator instance:', error);
    }

    unlisten = await getCurrentWindow().onThemeChanged(({ payload: theme }) => {
      switchTabulatorTheme(theme);
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
    if (tabulatorInstance) tabulatorInstance.destroy();
    window.removeEventListener('resize', handleWindowResize);
    clearTimeout(resizeTimeout);
  });

  $: if (tabulatorInstance && Array.isArray(data) && tableReady) {
    tabulatorInstance.replaceData(data);
  }

  // Update columns when they change
  $: if (tabulatorInstance && columns && tableReady) {
    tabulatorInstance.setColumns(columns);
    // Force redraw after setting new columns
    setTimeout(() => {
      if (tabulatorInstance) {
        tabulatorInstance.redraw(true);
      }
    }, 0);
  }

  $: if (typeof $globalTheme === 'string') {
    switchTabulatorTheme($globalTheme);
  }

  $: if (tabulatorInstance && tableReady && tableHeight) {
    tabulatorInstance.setHeight(tableHeight);
  }
</script>

<div bind:this={tableComponent}></div>