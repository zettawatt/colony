<script lang="ts">
  import TabulatorTable from '../../../components/tabulator.svelte';
  import { searchColumns } from '../../../utils/search/searchColumns';
  import { invoke } from "@tauri-apps/api/core";
  import { transferManager } from '../../../stores/transferManager';
  import { onMount, onDestroy } from 'svelte';

  import { getPassword } from "../../../utils/password/session";
  import LoginModal from '../../../components/login.svelte';
  import { downloadFile } from '../../../utils/file/download';
  import { parseBrowseSparqlResults, parseTextSparqlResults } from '../../../utils/search/parseSparql';
  import { openDweb } from '../../../utils/dweb/dwebCommands';
  import { addToast } from '../../../stores/toast';
  import { searchState } from '../../../stores/searchState';

  // Local variables that will be synced with the store
  let searchInput = "";
  let tableSearchResults = [];
  let activeRow = {};
  let showLogin = false;
  let isSearching = false;
  let fileMetadataModal: HTMLDialogElement;
  let searchMetrics = {
    itemCount: 0,
    searchTime: 0,
    hasSearched: false
  };

  // Store subscription to keep local state in sync
  let storeUnsubscribe;

  let windowWidth = 0;
  let tabulatorTable; // Reference to the TabulatorTable component

  // Calculate optimal description column width based on window size
  function updateDescriptionColumnWidth() {
    if (typeof window !== 'undefined') {
      // Fixed column widths: download(40) + name(200) + type(120) + size(90) + address(130) = 580px
      // Add padding and margins: ~80px for table padding, scrollbars, etc.
      const fixedColumnsWidth = 580;
      const tablePadding = 100; // Account for table padding, borders, scrollbars
      const availableWidth = windowWidth - tablePadding;
      const descriptionWidth = Math.max(200, availableWidth - fixedColumnsWidth);

      // Update the description column width
      if (searchColumns[2]) {
        searchColumns[2].width = descriptionWidth;
      }

      // Force table to update columns and recalculate layout if table is ready
      const instance = tabulatorTable?.getTabulatorInstance?.() || tabulatorTable?.tabulatorInstance;
      if (instance) {
        // Update columns with new widths
        instance.setColumns(searchColumns);
        // Force redraw to recalculate layout
        instance.redraw(true);
      }
    }
  }

  // Update column width when window resizes
  $: if (windowWidth) {
    updateDescriptionColumnWidth();
  }

  
  function shallowEqualArrays(a, b) {
    if (a === b) return true;
    if (!a || !b) return false;
    if (a.length !== b.length) return false;
    for (let i = 0; i < a.length; i++) {
      if (a[i] !== b[i]) return false;
    }
    return true;
  }

  let transfers = [];

  $: {
    const values = Object.values($transferManager);
    if (!shallowEqualArrays(transfers, values)) {
      transfers = values;
    }
  }

  let rowMenu = [
    {
        label:"Download",
        action:function(e, row){
            row.update({name:"Steve Bobberson"});
        }
    },
    {
      label:"View Metadata",
      action:function(e, row){
        activeRow = row.getData();
        searchState.setActiveRow(activeRow);
        fileMetadataModal.showModal()
      }
    }
  ];

  // set cellClick function for download column (column 0)
  searchColumns[0].cellClick = function(e, cell) {
    activeRow = cell.getRow().getData();

    // Don't do anything if it's a pod or has no type (no icon shown)
    if (activeRow.type === 'ant://colonylib/v1/pod' || !activeRow.type || activeRow.type === '') {
      return;
    }

    if (activeRow.type === 'ant://dweb/v1/WebSite') {
      openDweb(activeRow.address)
    } else if(activeRow.type && activeRow.type.includes("directory")) {
      const request = {
        name: activeRow.name,
        address: activeRow.address,
        bytes: activeRow.bytes ?? 0
      }

      downloadFile(request, 'directory');
    } else {
      const request = {
        name: activeRow.name,
        address: activeRow.address,
        bytes: activeRow.bytes ?? 0
      }

      downloadFile(request, 'file');
    }
  }

  // set cellClick function for name column (show modal) - column 1
  searchColumns[1].cellClick = function(e, cell) {
    activeRow = cell.getRow().getData();
    searchState.setActiveRow(activeRow);
    fileMetadataModal.showModal()
  }

  // set cellClick function for description column (show modal) - column 2
  searchColumns[2].cellClick = function(e, cell) {
    activeRow = cell.getRow().getData();
    searchState.setActiveRow(activeRow);
    fileMetadataModal.showModal()
  }

  // set cellClick function for type column (show modal) - column 3
  searchColumns[3].cellClick = function(e, cell) {
    activeRow = cell.getRow().getData();
    searchState.setActiveRow(activeRow);
    fileMetadataModal.showModal()
  }

  // set cellClick function for size column (show modal) - column 4
  searchColumns[4].cellClick = function(e, cell) {
    activeRow = cell.getRow().getData();
    searchState.setActiveRow(activeRow);
    fileMetadataModal.showModal()
  }

  // set cellClick function for address column (copy to clipboard) - column 5
  searchColumns[5].cellClick = function(e, cell) {
    const rowData = cell.getRow().getData();
    const fullAddress = rowData.address;

    // Copy to clipboard and show toast
    navigator.clipboard.writeText(fullAddress).then(() => {
      addToast(`Address ${fullAddress} copied!`, 'success');
    }).catch(err => {
      console.error('Failed to copy address:', err);
      addToast('Failed to copy address', 'error');
    });
  }

  // Helper functions for modal download button
  function shouldShowDownloadButton(row: any): boolean {
    if (!row || !row.type) return false;

    // Only hide download button for pods or missing types
    // Websites should show "Open Site" button, so they return true
    if (row.type === 'ant://colonylib/v1/pod' || row.type === '') {
      return false;
    }

    return true;
  }

  function getDownloadButtonText(row: any): string {
    if (!row || !row.type) return 'Download';

    if (row.type === 'ant://dweb/v1/WebSite') {
      return 'Open Site';
    }

    return 'Download';
  }

  function handleModalDownload(row: any) {
    if (!row) return;

    if (row.type === 'ant://dweb/v1/WebSite') {
      // Open website using dweb
      openDweb(row.address);
    } else if (row.type && row.type.includes('directory')) {
      // Download directory
      const request = {
        name: row.name,
        address: row.address,
        bytes: row.bytes ?? 0
      };
      downloadFile(request, 'directory');
    } else {
      // Download file
      const request = {
        name: row.name,
        address: row.address,
        bytes: row.bytes ?? 0
      };
      downloadFile(request, 'file');
    }
  }

  async function simpleSearch() {
    isSearching = true;
    const startTime = performance.now();

    try {
      if (searchInput === "") return;
      const request = {
        query: {
          // Put your search parameters here, e.g.:
            "type": "text",
            "text": searchInput,
            "limit": 2000
        },
      };
      // const request = {query: "beg"}
      const response = await invoke('search', { request });
      // console.log(response)
      const parsedResults = parseTextSparqlResults(response.results)
      // console.log(parsedResults)

      const endTime = performance.now();
      const searchTime = Math.round(endTime - startTime);

      searchMetrics = {
        itemCount: parsedResults.length,
        searchTime: searchTime,
        hasSearched: true
      };

      isSearching = false;
      tableSearchResults = parsedResults;

      // Save state to store
      searchState.updateState({
        searchInput,
        tableSearchResults: parsedResults,
        searchMetrics,
        scrollPosition: { x: 0, y: 0 } // Reset scroll position for new search
      });
    } catch (error) {
      console.error(error)
      isSearching = false;
      searchMetrics = {
        itemCount: 0,
        searchTime: 0,
        hasSearched: true
      };

      // Save error state to store
      searchState.updateState({
        searchInput,
        tableSearchResults: [],
        searchMetrics,
        scrollPosition: { x: 0, y: 0 }
      });
    }
  }

  async function browseSearch() {
    isSearching = true;
    const startTime = performance.now();

    try {
      if (searchInput !== "") return;
      const request = {
        query: {
          // Put your search parameters here, e.g.:
            "type": "browse",
            "limit": 2000
        },
      };
      // const request = {query: "beg"}
      const response = await invoke('search', { request });
      console.log(response)
      const parsedResults = parseBrowseSparqlResults((response as any).results)
      console.log(parsedResults)

      const endTime = performance.now();
      const searchTime = Math.round(endTime - startTime);

      searchMetrics = {
        itemCount: parsedResults.length,
        searchTime: searchTime,
        hasSearched: true
      };

      isSearching = false;
      tableSearchResults = parsedResults;

      // Save state to store
      searchState.updateState({
        searchInput,
        tableSearchResults: parsedResults,
        searchMetrics,
        scrollPosition: { x: 0, y: 0 } // Reset scroll position for new search
      });
    } catch (error) {
      console.error(error)
      isSearching = false;
      searchMetrics = {
        itemCount: 0,
        searchTime: 0,
        hasSearched: true
      };

      // Save error state to store
      searchState.updateState({
        searchInput,
        tableSearchResults: [],
        searchMetrics,
        scrollPosition: { x: 0, y: 0 }
      });
    }
  }

  async function searchHandler() {
    if (searchInput === "") {
      await browseSearch();
    } else {
      await simpleSearch();
    }
  }

  
  async function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      await searchHandler();
    }
  }

  // Handle search input changes to save to store
  function handleSearchInputChange() {
    searchState.setSearchInput(searchInput);
  }

  let handleTabulatorResize;
  let handleWindowResize;
  let resizeTimeout;
  let scrollTimeout;
  let handleTableScroll;

  onMount(async () => {
    try {
      await transferManager.init();
    } catch (error) {
      addToast("Failed to init transfer manager, see logs...", "error");
      console.error(error);
    }
    const pw = await getPassword();
    if (pw === null) {
      console.error("password was null");
      showLogin = true;
    }

    // Initialize search state store and restore previous state
    searchState.init();

    // Subscribe to store changes and restore state
    storeUnsubscribe = searchState.subscribe((state) => {
      // Update local state carefully to avoid race conditions
      if (searchInput !== state.searchInput) {
        searchInput = state.searchInput;
      }
      if (searchMetrics !== state.searchMetrics) {
        searchMetrics = state.searchMetrics;
      }
      if (activeRow !== state.activeRow) {
        activeRow = state.activeRow;
      }

      // Only update table results if they're different and we have results
      if (state.tableSearchResults.length > 0 && tableSearchResults !== state.tableSearchResults) {
        tableSearchResults = state.tableSearchResults;
      }
    });



    // Listen for tabulator resize events to update column widths
    handleTabulatorResize = () => {
      updateDescriptionColumnWidth();
    };

    // Listen for window resize events directly with debouncing
    handleWindowResize = () => {
      clearTimeout(resizeTimeout);
      resizeTimeout = setTimeout(() => {
        updateDescriptionColumnWidth();
      }, 100); // 100ms debounce
    };

    // Define scroll event handler with better debouncing
    handleTableScroll = () => {
      clearTimeout(scrollTimeout);
      scrollTimeout = setTimeout(() => {
        const tableHolder = getTableScrollContainer();
        if (tableHolder) {
          const scrollX = tableHolder.scrollLeft;
          const scrollY = tableHolder.scrollTop;
          searchState.setScrollPosition(scrollX, scrollY);
        }
      }, 500); // Increased debounce time for better performance
    };

    window.addEventListener('tabulator-resize-start', handleTabulatorResize);
    window.addEventListener('resize', handleWindowResize);
  })

  // Track if scroll listener has been added
  let scrollListenerAdded = false;
  let scrollPositionRestored = false;

  // Function to get the tabulator scroll container
  function getTableScrollContainer() {
    const instance = tabulatorTable?.getTabulatorInstance?.() || tabulatorTable?.tabulatorInstance;
    if (!instance) return null;
    return instance.element?.querySelector('.tabulator-tableholder');
  }

  // Function to add scroll listener
  function addScrollListener() {
    const tableHolder = getTableScrollContainer();
    if (tableHolder && !scrollListenerAdded) {
      tableHolder.addEventListener('scroll', handleTableScroll);
      scrollListenerAdded = true;
    }
  }

  // Function to restore scroll position with retry logic
  function restoreScrollPosition() {
    if (scrollPositionRestored) return; // Only restore once

    const currentState = JSON.parse(sessionStorage.getItem('colony-search-state') || '{}');
    const tableHolder = getTableScrollContainer();

    if (currentState.scrollPosition && tableHolder) {
      const { x, y } = currentState.scrollPosition;
      if (x !== 0 || y !== 0) {
        // Try to restore scroll position with a small delay to ensure table is fully rendered
        setTimeout(() => {
          const tableHolder = getTableScrollContainer(); // Get fresh reference
          if (tableHolder) {
            tableHolder.scrollLeft = x;
            tableHolder.scrollTop = y;
            scrollPositionRestored = true;
            console.log('✅ Restored scroll position:', { x, y });

            // Verify the scroll position was actually set
            setTimeout(() => {
              if (tableHolder.scrollLeft !== x || tableHolder.scrollTop !== y) {
                console.warn('⚠️ Scroll position restoration may have failed. Expected:', { x, y }, 'Actual:', { x: tableHolder.scrollLeft, y: tableHolder.scrollTop });
              }
            }, 100);
          }
        }, 200);
      }
    }
  }

  // Function to setup table when it's ready
  function setupTableWhenReady() {
    const tableHolder = getTableScrollContainer();
    if (tableHolder && tableSearchResults.length > 0) {
      // Check if table actually has rendered rows
      const tableRows = tableHolder.querySelectorAll('.tabulator-row');
      console.log('🔧 Setting up table with', tableSearchResults.length, 'results,', tableRows.length, 'rendered rows');

      if (tableRows.length > 0) {
        addScrollListener();

        // Restore scroll position after table is confirmed to have content
        setTimeout(() => {
          restoreScrollPosition();
        }, 100);
      } else {
        console.log('⏳ Table not yet rendered, will retry...');
      }
    }
  }

  // Function to wait for tabulator to be ready and then setup
  function waitForTabulatorAndSetup() {
    const maxAttempts = 20; // 2 seconds max wait
    let attempts = 0;

    const checkTabulator = () => {
      attempts++;

      const instance = tabulatorTable?.getTabulatorInstance?.() || tabulatorTable?.tabulatorInstance;

      if (instance) {
        const tableHolder = getTableScrollContainer();

        if (tableHolder) {
          const tableRows = tableHolder.querySelectorAll('.tabulator-row');

          if (tableHolder && tableSearchResults.length > 0 && tableRows.length > 0) {
            setupTableWhenReady();
            return;
          }
        }
      }

      if (attempts < maxAttempts) {
        setTimeout(checkTabulator, 100);
      }
    };

    checkTabulator();
  }

  // Watch for table data changes with better timing
  $: if (tableSearchResults.length > 0) {
    // Reset flags when new data is loaded
    scrollPositionRestored = false;
    scrollListenerAdded = false;

    // Wait a bit for the reactive update to complete, then check tabulator
    setTimeout(waitForTabulatorAndSetup, 500);
  }

  onDestroy(() => {
    // Clean up store subscription
    if (storeUnsubscribe) {
      storeUnsubscribe();
    }

    // Save current scroll position before leaving
    const tableHolder = getTableScrollContainer();
    if (tableHolder) {
      const scrollX = tableHolder.scrollLeft;
      const scrollY = tableHolder.scrollTop;
      searchState.setScrollPosition(scrollX, scrollY);

      // Remove scroll event listener
      if (scrollListenerAdded) {
        tableHolder.removeEventListener('scroll', handleTableScroll);
        scrollListenerAdded = false;
      }
    }

    if (handleTabulatorResize) {
      window.removeEventListener('tabulator-resize-start', handleTabulatorResize);
    }
    if (handleWindowResize) {
      window.removeEventListener('resize', handleWindowResize);
    }
    if (resizeTimeout) {
      clearTimeout(resizeTimeout);
    }
    if (scrollTimeout) {
      clearTimeout(scrollTimeout);
    }
  })
</script>

<svelte:window bind:innerWidth={windowWidth} />

{#if showLogin}
  <LoginModal/>
{/if}
<main class="search-container" style="height: calc(100vh - 120px); display: flex; flex-direction: column; padding: 20px; overflow: hidden;">
  <div class="search-header" style="flex-shrink: 0; margin-bottom: 20px;">
    <div class="row mb-3">
      <label class="input mr-2">
        <svg class="h-[1em] opacity-50" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
          <g
            stroke-linejoin="round"
            stroke-linecap="round"
            stroke-width="2.5"
            fill="none"
            stroke="currentColor"
          >
            <circle cx="11" cy="11" r="8"></circle>
            <path d="m21 21-4.3-4.3"></path>
          </g>
        </svg>
        <input type="search" required placeholder="Press Browse to see what's on the network..." bind:value={searchInput} onkeydown={handleKeydown} oninput={handleSearchInputChange}/>
      </label>
      <button class="btn btn-sof btn-warning" onclick={()=>searchHandler()}>
        {#if isSearching}
          <span class="loading loading-spinner"></span>
        {:else if searchInput === ""}
          Browse
        {:else}
          Search
        {/if}
      </button>
    </div>

    <!-- Search Metrics -->
    {#if searchMetrics.hasSearched}
      <div class="text-center mb-3">
        <p class="text-sm italic text-gray-600">
          {searchMetrics.itemCount} items returned in {searchMetrics.searchTime}ms
        </p>
      </div>
    {/if}
  </div>

  <div class="search-table-container" style="flex: 1; min-height: 0;">
    <TabulatorTable bind:this={tabulatorTable} data={tableSearchResults} columns={searchColumns} rowMenu={rowMenu} initialSort={[]} />
  </div>
  <dialog id="fileMetadataModal" class="modal" bind:this={fileMetadataModal}>
    <div class="modal-box w-10/12 max-w-5xl max-h-[80vh]">
      <h3 class="text-lg font-bold">File Metadata: {activeRow?.name}</h3>
      <div class="py-2 max-h-[60vh] overflow-y-auto" style="justify-content: center;">
          <table class="table table-xs">
            <tbody>
              {#if activeRow}
                {#each Object.entries(activeRow) as [key, value]}
                  {#if value !== "" && value !== null && value !== undefined}
                    <tr>
                      <th class="w-1/4">{key}</th>
                      <td class="w-3/4 break-words whitespace-pre-wrap">{value}</td>
                    </tr>
                  {/if}
                {/each}
              {/if}
            </tbody>
        </table>
      </div>
      <div class="modal-action">
        {#if shouldShowDownloadButton(activeRow)}
          <button class="btn btn-primary" onclick={() => handleModalDownload(activeRow)}>
            {getDownloadButtonText(activeRow)}
          </button>
        {/if}
        <form method="dialog">
          <button class="btn btn-soft btn-error">Close</button>
        </form>
      </div>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button>close</button>
    </form>
  </dialog>
</main>

<style>
.search-container {
  justify-content: flex-start;
  text-align: left;
  overflow: hidden;
}

.input {
  width: 60%;
  max-width: 600px;
}

/* .logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
} */

.row {
  display: flex;
  justify-content: center;
}
/* ---- Modern styling for fileMetadataModal table ---- */
#fileMetadataModal table {
  border-collapse: separate;
  border-spacing: 0;
  width: 100%;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  font-size: 15px;
  background-color: hsl(var(--b1));
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 4px 12px hsl(var(--bc) / 0.1);
  border: 1px solid hsl(var(--bc) / 0.1);
}

#fileMetadataModal table tr {
  transition: background-color 0.15s ease;
  border-bottom: 1px solid hsl(var(--bc) / 0.1);
}

#fileMetadataModal table tr:last-child {
  border-bottom: none;
}

#fileMetadataModal table tr:nth-child(even) {
  background-color: hsl(var(--b2) / 0.5);
}

#fileMetadataModal table tr:hover {
  background-color: hsl(var(--b3) / 0.7);
}

/* Key column (th) styling with enhanced shading and bold text */
#fileMetadataModal table th {
  background: linear-gradient(135deg, hsl(var(--b2) / 0.8) 0%, hsl(var(--b3) / 0.9) 100%);
  border-right: 2px solid hsl(var(--bc) / 0.3);
  padding: 14px 18px;
  font-weight: 700;
  font-size: 15px;
  letter-spacing: 0.5px;
  text-align: left;
  vertical-align: middle;
  color: hsl(var(--bc) / 0.9);
  position: sticky;
  left: 0;
  z-index: 1;
  min-width: 105px;
  max-width: 150px;
  width: 25%;
  box-shadow: inset -2px 0 4px hsl(var(--bc) / 0.1);
}

/* Value column (td) styling */
#fileMetadataModal table td {
  border-right: 1px solid hsl(var(--bc) / 0.1);
  padding: 14px 18px;
  vertical-align: top;
  line-height: 1.5;
  color: hsl(var(--bc) / 0.85);
  word-wrap: break-word;
  overflow-wrap: break-word;
  hyphens: auto;
  font-size: 15px;
}

#fileMetadataModal table th:last-child,
#fileMetadataModal table td:last-child {
  border-right: none;
}
</style>
