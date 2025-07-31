<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { TabulatorFull as Tabulator } from 'tabulator-tables';
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { DateTime } from "luxon";
  import { globalTheme } from '../stores/globals';

  export let columns: TabulatorColumn[];
  export let data: any[];
  export let rowMenu: any[];
  export let initialSort: any[];
  export let persistenceID: string = "colony-search-table";
  export let disableColumnPersistence: boolean = false;

  let tableComponent: HTMLElement;
  let tabulatorInstance: any = null;

  // Export the tabulator instance so parent components can access it
  export { tabulatorInstance };

  // Also create a getter function to access the instance
  export function getTabulatorInstance(): any {
    return tabulatorInstance;
  }

  // Method to update specific rows without full redraw
  export function updateRowData(rowId: any, newData: any): boolean {
    if (tabulatorInstance && tableReady) {
      try {
        // Use updateData method which preserves scroll position by design
        // Create the update object with the row ID and new data
        const updateObject = { ...newData };
        updateObject.id = rowId; // Ensure the ID is set for the update

        tabulatorInstance.updateData([updateObject]);
        return true;
      } catch (error) {
        console.debug('Failed to update row:', error);
      }
    }
    return false;
  }

  // Functions to save and restore scroll position
  export function saveScrollPosition(): { x: number; y: number } | null {
    if (tabulatorInstance) {
      const scrollContainer = tabulatorInstance.element.querySelector('.tabulator-tableholder') as HTMLElement;
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

  export function restoreScrollPosition(): void {
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
  let unlisten: (() => void) | undefined;
  let tableReady: boolean = false;
  let tableHeight: number = 300;

  function setTableHeight() {
    if (window) {
      tableHeight = Math.max(300, Math.min(window.innerHeight * 0.75, 1500));
    }
  }

  let resizeTimeout: ReturnType<typeof setTimeout> | undefined;
  let scrollSaveTimeout: ReturnType<typeof setTimeout> | undefined;

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

  function switchTabulatorTheme(theme: Theme): void {
    const link = document.getElementById('tabulator-theme') as HTMLLinkElement;
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
        columns: columns as any,
        height: tableHeight,
        minHeight: 300,
        data,
        rowContextMenu: rowMenu,
        reactiveData: false,
        layout: 'fitDataStretch',
        // @ts-ignore - dependencies is a valid Tabulator option
        dependencies: {
          DateTime: DateTime,
        },
        initialSort: initialSort,
        // Set index field for row identification (important for row updates)
        index: "id",
        // Enable virtual DOM for scroll position preservation during updates
        // Note: Virtual DOM is required for updateData to maintain scroll position
        renderVertical: "virtual",
        // Disable features that can cause scroll jank
        movableRows: false,
        // Enable persistence for instant restoration
        persistence: {
          sort: true,
          filter: true,
          headerFilter: true,
          columns: !disableColumnPersistence, // Allow disabling column persistence
          page: false // We'll handle scroll position separately
        },
        persistenceMode: "local", // Use localStorage for better performance
        persistenceID: persistenceID, // Unique ID for this table
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

  // Track previous data to avoid unnecessary updates
  let previousData: any[] = [];
  let lastDataUpdateTime: number = 0;

  $: if (tabulatorInstance && Array.isArray(data) && tableReady) {
    const now = Date.now();

    // Throttle data updates to prevent excessive redraws during rapid timer updates
    if (now - lastDataUpdateTime >= 100) {
      // Check if data has actually changed structurally
      const dataChanged = data.length !== previousData.length ||
                         data.some((item, index) => {
                           const prevItem = previousData[index];
                           if (!prevItem) return true;

                           // Compare all fields except elapsed (which changes frequently)
                           const currentWithoutElapsed = { ...item };
                           const prevWithoutElapsed = { ...prevItem };
                           delete currentWithoutElapsed.elapsed;
                           delete prevWithoutElapsed.elapsed;

                           return JSON.stringify(currentWithoutElapsed) !== JSON.stringify(prevWithoutElapsed);
                         });

      if (dataChanged) {
        tabulatorInstance.replaceData(data);
        previousData = [...data];
        lastDataUpdateTime = now;
      }
    }
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
    switchTabulatorTheme($globalTheme as Theme);
  }

  $: if (tabulatorInstance && tableReady && tableHeight) {
    tabulatorInstance.setHeight(tableHeight);
  }
</script>

<div bind:this={tableComponent} class="smooth-scroll-table"></div>

<style>
  /* Basic scrollbar styling only */
  :global(.smooth-scroll-table .tabulator-tableholder::-webkit-scrollbar) {
    width: 12px;
  }

  :global(.smooth-scroll-table .tabulator-tableholder::-webkit-scrollbar-track) {
    background: transparent;
  }

  :global(.smooth-scroll-table .tabulator-tableholder::-webkit-scrollbar-thumb) {
    background-color: rgba(0, 0, 0, 0.3);
    border-radius: 6px;
    border: 2px solid transparent;
    background-clip: content-box;
  }

  :global(.smooth-scroll-table .tabulator-tableholder::-webkit-scrollbar-thumb:hover) {
    background-color: rgba(0, 0, 0, 0.5);
  }
</style>