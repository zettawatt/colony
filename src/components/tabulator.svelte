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

  // Functions to save and restore scroll position
  export function saveScrollPosition() {
    if (tabulatorInstance) {
      const scrollContainer = tabulatorInstance.element.querySelector('.tabulator-tableholder');
      if (scrollContainer) {
        const scrollData = {
          x: scrollContainer.scrollLeft,
          y: scrollContainer.scrollTop
        };
        localStorage.setItem('colony-search-table-scroll', JSON.stringify(scrollData));
        return scrollData;
      }
    }
    return null;
  }

  export function restoreScrollPosition() {
    if (tabulatorInstance) {
      const savedScroll = localStorage.getItem('colony-search-table-scroll');
      if (savedScroll) {
        try {
          const scrollData = JSON.parse(savedScroll);
          const scrollContainer = tabulatorInstance.element.querySelector('.tabulator-tableholder');
          if (scrollContainer && scrollData) {
            // Restore scroll position after a small delay to ensure table is fully rendered
            setTimeout(() => {
              scrollContainer.scrollLeft = scrollData.x;
              scrollContainer.scrollTop = scrollData.y;
            }, 50);
          }
        } catch (error) {
          console.warn('Failed to restore scroll position:', error);
        }
      }
    }
  }

  // Enhanced functions to save and restore complete table state
  export function saveCompleteTableState() {
    if (tabulatorInstance) {
      try {
        const scrollContainer = tabulatorInstance.element.querySelector('.tabulator-tableholder');
        const state = {
          // Save scroll position
          scrollPosition: scrollContainer ? {
            x: scrollContainer.scrollLeft,
            y: scrollContainer.scrollTop
          } : { x: 0, y: 0 },
          // Save column layout
          columnLayout: tabulatorInstance.getColumnLayout ? tabulatorInstance.getColumnLayout() : null,
          // Save current data
          data: tabulatorInstance.getData ? tabulatorInstance.getData() : [],
          // Save table dimensions
          tableHeight: tableHeight,
          // Save any active filters
          filters: tabulatorInstance.getFilters ? tabulatorInstance.getFilters() : [],
          // Save current sort
          sort: tabulatorInstance.getSorters ? tabulatorInstance.getSorters() : [],
          timestamp: Date.now()
        };

        localStorage.setItem('colony-search-table-complete-state', JSON.stringify(state));
        return state;
      } catch (error) {
        console.warn('Failed to save complete table state:', error);
        return null;
      }
    }
    return null;
  }

  export function restoreCompleteTableState() {
    try {
      const savedState = localStorage.getItem('colony-search-table-complete-state');
      if (savedState) {
        const state = JSON.parse(savedState);

        // Check if state is recent (within last 5 minutes)
        const fiveMinutesAgo = Date.now() - (5 * 60 * 1000);
        if (state.timestamp && state.timestamp > fiveMinutesAgo) {
          return state;
        }
      }
    } catch (error) {
      console.warn('Failed to restore complete table state:', error);
    }
    return null;
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
  let scrollSaveTimeout;

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
        // Enable persistence for instant restoration
        persistence: {
          sort: true,
          filter: true,
          headerFilter: true,
          columns: true,
          page: false // We'll handle scroll position separately
        },
        persistenceMode: "local", // Use localStorage for better performance
        persistenceID: "colony-search-table", // Unique ID for this table
        tableBuilt: function() {
          tableReady = true;
          // Restore scroll position after table is built and persistence is applied
          setTimeout(() => {
            restoreScrollPosition();
            // Add scroll listener to save position on scroll
            const scrollContainer = tabulatorInstance?.element?.querySelector('.tabulator-tableholder');
            if (scrollContainer) {
              scrollContainer.addEventListener('scroll', () => {
                // Debounce scroll saving to avoid excessive localStorage writes
                clearTimeout(scrollSaveTimeout);
                scrollSaveTimeout = setTimeout(() => {
                  saveScrollPosition();
                }, 250);
              });
            }
          }, 100);
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
    clearTimeout(scrollSaveTimeout);
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