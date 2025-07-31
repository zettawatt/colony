<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { TabulatorFull as Tabulator } from 'tabulator-tables';
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { DateTime } from "luxon";
  import { globalTheme } from '../stores/globals';

  export let columns: any[];
  export let data: any[];
  export let rowMenu: any[];
  export let initialSort: any[];
  export let cacheKey: string = 'default';

  let tableComponent: HTMLElement;
  let tabulatorInstance: any = null;
  let unlisten: (() => void) | undefined;
  let tableReady: boolean = false;
  let tableHeight: number = 300;
  let resizeTimeout: ReturnType<typeof setTimeout> | undefined;
  let scrollSaveTimeout: ReturnType<typeof setTimeout> | undefined;
  let isRestoringFromCache: boolean = false;

  // Export the tabulator instance so parent components can access it
  export { tabulatorInstance };

  export function getTabulatorInstance() {
    return tabulatorInstance;
  }

  function setTableHeight() {
    if (window) {
      tableHeight = Math.max(300, Math.min(window.innerHeight * 0.75, 1500));
    }
  }

  function handleWindowResize() {
    clearTimeout(resizeTimeout);
    resizeTimeout = setTimeout(() => {
      setTableHeight();
      window.dispatchEvent(new CustomEvent('tabulator-resize-start'));
      
      setTimeout(() => {
        if (tabulatorInstance && tableReady) {
          tabulatorInstance.setColumns(columns);
          tabulatorInstance.redraw(true);
        }
      }, 10);
    }, 100);
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

  function saveToCache() {
    if (tabulatorInstance && tableReady && cacheKey) {
      try {
        const scrollContainer = tabulatorInstance.element.querySelector('.tabulator-tableholder');
        const scrollPosition = scrollContainer ? {
          x: scrollContainer.scrollLeft,
          y: scrollContainer.scrollTop
        } : { x: 0, y: 0 };

        const state = {
          data: tabulatorInstance.getData ? tabulatorInstance.getData() : (data || []),
          columns: columns || [],
          columnLayout: tabulatorInstance.getColumnLayout ? tabulatorInstance.getColumnLayout() : null,
          filters: tabulatorInstance.getFilters ? tabulatorInstance.getFilters() : [],
          sorters: tabulatorInstance.getSorters ? tabulatorInstance.getSorters() : [],
          scrollPosition,
          tableHeight,
          timestamp: Date.now()
        };

        localStorage.setItem(`colony-tabulator-cache-${cacheKey}`, JSON.stringify(state));
        console.log(`💾 Saved tabulator state for key: ${cacheKey}`);
      } catch (error) {
        console.warn('Failed to save tabulator state:', error);
      }
    }
  }

  function setupScrollListener() {
    if (tabulatorInstance) {
      const scrollContainer = tabulatorInstance.element.querySelector('.tabulator-tableholder');
      if (scrollContainer) {
        // Simple scroll listener without aggressive optimizations
        scrollContainer.addEventListener('scroll', () => {
          // Simple debounced cache saving
          clearTimeout(scrollSaveTimeout);
          scrollSaveTimeout = setTimeout(() => {
            saveToCache();
          }, 250);
        }, { passive: true });
      }
    }
  }

  function restoreFromCache() {
    if (!cacheKey || !tableComponent) return false;

    try {
      const savedState = localStorage.getItem(`colony-tabulator-cache-${cacheKey}`);
      if (!savedState) {
        return false;
      }

      const cached = JSON.parse(savedState);

      // Check if cache is recent (within 5 minutes)
      const fiveMinutesAgo = Date.now() - (5 * 60 * 1000);
      if (!cached.timestamp || cached.timestamp < fiveMinutesAgo) {
        console.log('⏰ Cache expired, creating new instance');
        localStorage.removeItem(`colony-tabulator-cache-${cacheKey}`);
        return false;
      }

      console.log('🚀 Restoring tabulator from cached state instantly');
      isRestoringFromCache = true;

      // Create new instance with cached data for instant display
      tabulatorInstance = new Tabulator(tableComponent, {
        columns: cached.columns,
        height: cached.tableHeight || tableHeight,
        minHeight: 300,
        data: cached.data, // Use cached data for instant display
        rowContextMenu: rowMenu,
        reactiveData: false,
        layout: 'fitDataStretch',
        initialSort: initialSort,
        // Disable virtual DOM to prevent lockups - use basic rendering
        renderVertical: "basic",
        // Disable features that can cause scroll jank
        movableRows: false,
        persistence: {
          sort: true,
          filter: true,
          headerFilter: true,
          columns: true,
          page: false
        },
        persistenceMode: "local",
        persistenceID: `colony-${cacheKey}-table`,
        // @ts-ignore - tableBuilt is a valid Tabulator option
        tableBuilt: function() {
          tableReady = true;

          // Restore additional state after table is built
          setTimeout(() => {
            try {
              // Restore column layout if available
              if (cached.columnLayout && tabulatorInstance.setColumnLayout) {
                tabulatorInstance.setColumnLayout(cached.columnLayout);
              }

              // Restore filters
              if (cached.filters && cached.filters.length > 0 && tabulatorInstance.setFilter) {
                cached.filters.forEach((filter: any) => {
                  tabulatorInstance.setFilter(filter.field, filter.type, filter.value);
                });
              }

              // Restore sorters
              if (cached.sorters && cached.sorters.length > 0 && tabulatorInstance.setSort) {
                tabulatorInstance.setSort(cached.sorters);
              }

              // Add scroll listener to save position changes
              setupScrollListener();

              console.log('✅ Tabulator restored from cache');

              // Check if current data is different from cached data and update if needed
              if (data && Array.isArray(data) && JSON.stringify(data) !== JSON.stringify(cached.data)) {
                console.log('🔄 Updating cached tabulator with new data');
                // Use replaceData instead of clearData + setData to prevent lockups
                try {
                  tabulatorInstance.replaceData(data);
                } catch (error) {
                  console.warn('Error replacing data:', error);
                }
              }

              // Mark restoration as complete so reactive statements can work
              isRestoringFromCache = false;

              // Restore scroll position smoothly after table is fully rendered
              setTimeout(() => {
                const scrollContainer = tabulatorInstance.element.querySelector('.tabulator-tableholder');
                if (scrollContainer && cached.scrollPosition) {
                  // Use requestAnimationFrame for smooth restoration
                  requestAnimationFrame(() => {
                    scrollContainer.scrollLeft = cached.scrollPosition.x;
                    scrollContainer.scrollTop = cached.scrollPosition.y;
                    console.log('📍 Restored scroll position:', cached.scrollPosition);
                  });
                }
              }, 100); // Reduced delay for faster restoration
            } catch (error) {
              console.warn('Error during state restoration:', error);
            }
          }, 100);
        }
      });



      return true;
    } catch (error) {
      console.error('Failed to restore from cache:', error);
      isRestoringFromCache = false;
      return false;
    }
  }



  function createNewTabulator() {
    console.log('🔧 Creating new tabulator instance');

    try {
      tabulatorInstance = new Tabulator(tableComponent, {
        columns,
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
        // Disable virtual DOM to prevent lockups - use basic rendering
        renderVertical: "basic",
        // Disable features that can cause scroll jank
        movableRows: false,
        persistence: {
          sort: true,
          filter: true,
          headerFilter: true,
          columns: true,
          page: false
        },
        persistenceMode: "local",
        persistenceID: `colony-${cacheKey}-table`,
        tableBuilt: function() {
          tableReady = true;
          setupScrollListener();

          // Add error recovery for virtual DOM issues
          if (tabulatorInstance.element) {
            const scrollContainer = tabulatorInstance.element.querySelector('.tabulator-tableholder');
            if (scrollContainer) {
              // Add error recovery listener
              scrollContainer.addEventListener('error', () => {
                console.warn('Scroll error detected, attempting recovery...');
                setTimeout(() => {
                  if (tabulatorInstance && tabulatorInstance.redraw) {
                    tabulatorInstance.redraw(true);
                  }
                }, 100);
              });
            }
          }

          // Cache the instance after it's built
          setTimeout(() => {
            saveToCache();
          }, 100);
        }
      });

      setTimeout(() => {
        if (!tableReady) {
          tableReady = true;
        }
        if (tabulatorInstance && Array.isArray(data) && data.length > 0) {
          tabulatorInstance.replaceData(data);
        }
      }, 1000);
    } catch (error) {
      console.error('Failed to create Tabulator instance:', error);
    }
  }

  onMount(async () => {
    setTableHeight();
    window.addEventListener('resize', handleWindowResize);

    // Try to restore from cache first
    if (!restoreFromCache()) {
      // If no cache available, create new instance
      createNewTabulator();
    }

    unlisten = await getCurrentWindow().onThemeChanged(({ payload: theme }) => {
      switchTabulatorTheme(theme);
    });
  });

  onDestroy(() => {
    // Always save state before destroying
    saveToCache();

    if (unlisten) unlisten();

    // Always destroy the tabulator instance since we're using state-based caching
    if (tabulatorInstance) {
      tabulatorInstance.destroy();
    }

    window.removeEventListener('resize', handleWindowResize);
    clearTimeout(resizeTimeout);
    clearTimeout(scrollSaveTimeout);
  });

  // Reactive statements for data and column updates
  $: if (tabulatorInstance && Array.isArray(data) && tableReady && !isRestoringFromCache) {
    console.log('🔄 Reactive update: replacing data with', data.length, 'items');

    // Safety check to prevent excessive updates
    const currentDataLength = tabulatorInstance.getData ? tabulatorInstance.getData().length : 0;
    if (currentDataLength !== data.length) {
      // Use replaceData instead of clearData + setData to prevent lockups
      try {
        tabulatorInstance.replaceData(data);
        // Save updated state with delay
        setTimeout(() => saveToCache(), 200);
      } catch (error) {
        console.warn('Error updating tabulator data:', error);
      }
    } else {
      console.log('⏭️ Skipping update - same data length');
    }
  }

  $: if (tabulatorInstance && columns && tableReady) {
    tabulatorInstance.setColumns(columns);
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
