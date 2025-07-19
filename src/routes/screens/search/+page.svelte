<script lang="ts">
  import TabulatorTable from '../../../components/tabulator.svelte';
  import { torrentsColumns } from '../../../components/testCols';
  import { torrentsData } from '../../../components/testData';
  import { searchColumns } from '../../../utils/search/searchColumns';
  import { invoke } from "@tauri-apps/api/core";
  import { formatFileSize } from '../../../utils/fileFormaters';
  import { transferManager } from '../../../stores/transferManager';
  import { onMount, onDestroy } from 'svelte';

  import { getPassword } from "../../../utils/password/session";
  import LoginModal from '../../../components/login.svelte';
  import { downloadFile } from '../../../utils/file/download';
  import { parseBrowseSparqlResults, parseTextSparqlResults } from '../../../utils/search/parseSparql';
  import { openDweb } from '../../../utils/dweb/dwebCommands';
  import { addToast } from '../../../stores/toast';


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
      if (tabulatorTable?.tabulatorInstance) {
        // Update columns with new widths
        tabulatorTable.tabulatorInstance.setColumns(searchColumns);
        // Force redraw and recalculate column widths
        tabulatorTable.tabulatorInstance.redraw(true);
        tabulatorTable.tabulatorInstance.recalcColumnWidths();
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
    fileMetadataModal.showModal()
  }

  // set cellClick function for description column (show modal) - column 2
  searchColumns[2].cellClick = function(e, cell) {
    activeRow = cell.getRow().getData();
    fileMetadataModal.showModal()
  }

  // set cellClick function for type column (show modal) - column 3
  searchColumns[3].cellClick = function(e, cell) {
    activeRow = cell.getRow().getData();
    fileMetadataModal.showModal()
  }

  // set cellClick function for size column (show modal) - column 4
  searchColumns[4].cellClick = function(e, cell) {
    activeRow = cell.getRow().getData();
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
    } catch (error) {
      console.error(error)
      isSearching = false;
      searchMetrics = {
        itemCount: 0,
        searchTime: 0,
        hasSearched: true
      };
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
      const parsedResults = parseBrowseSparqlResults(response.results)
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
    } catch (error) {
      console.error(error)
      isSearching = false;
      searchMetrics = {
        itemCount: 0,
        searchTime: 0,
        hasSearched: true
      };
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

  let handleTabulatorResize;
  let handleWindowResize;
  let resizeTimeout;

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

    window.addEventListener('tabulator-resize-start', handleTabulatorResize);
    window.addEventListener('resize', handleWindowResize);
  })

  onDestroy(() => {
    if (handleTabulatorResize) {
      window.removeEventListener('tabulator-resize-start', handleTabulatorResize);
    }
    if (handleWindowResize) {
      window.removeEventListener('resize', handleWindowResize);
    }
    if (resizeTimeout) {
      clearTimeout(resizeTimeout);
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
        <input type="search" required placeholder="Press Browse to see what's on the network..." bind:value={searchInput} onkeydown={handleKeydown}/>
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
/* ---- Added styles for bordered rows in fileMetadataModal ---- */
#fileMetadataModal table tr {
  border-bottom: 1px solid #d1d5db; /* Tailwind slate-300 */
}

#fileMetadataModal table th, 
#fileMetadataModal table td {
  border: 1px solid #d1d5db;
  padding: 0.4em 0.6em;
}

#fileMetadataModal table {
  border-collapse: collapse;
  width: 100%;
}
</style>
