<script lang="ts">
  import TabulatorTable from '../../../components/tabulator.svelte';
  import { torrentsColumns } from '../../../components/testCols';
  import { torrentsData } from '../../../components/testData';
  import { searchColumns } from '../../../utils/search/searchColumns';
  import { invoke } from "@tauri-apps/api/core";
  import { formatFileSize } from '../../../utils/fileFormaters';
  import { transferManager } from '../../../stores/transferManager';
  import { onMount } from 'svelte';
  import { statusColumns } from '../../../utils/search/statusColumns';
  import { statusTestData } from '../../../utils/search/statusTestData';
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
  let statusInitialSort = [
    {column:"startedDate", dir:"desc"}
  ]
  
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

  // set cellClick function for info column
  searchColumns[0].cellClick = function(e, cell) {
    activeRow = cell.getRow().getData();
    fileMetadataModal.showModal()
  }

  // set cellClick function for download column
  searchColumns[1].cellClick = function(e, cell) {
    activeRow = cell.getRow().getData();

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

  // set cellClick function for name column (show modal)
  searchColumns[2].cellClick = function(e, cell) {
    activeRow = cell.getRow().getData();
    fileMetadataModal.showModal()
  }

  // set cellClick function for size column (show modal)
  searchColumns[3].cellClick = function(e, cell) {
    activeRow = cell.getRow().getData();
    fileMetadataModal.showModal()
  }

  // Note: Address column (searchColumns[4]) intentionally has no click handler

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
      isSearching = false;
      tableSearchResults = parsedResults;
    } catch (error) {
      console.error(error)
      isSearching = false;
    }
  }

  async function browseSearch() {
    isSearching = true;
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
      isSearching = false;
      tableSearchResults = parsedResults;
    } catch (error) {
      console.error(error)
      isSearching = false;
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
  })
</script>

{#if showLogin}
  <LoginModal/>
{/if}
<main class="search-container">
  <div class="tabs tabs-box">
    <input type="radio" name="my_tabs_2" class="tab" aria-label="Status"/>
    <div class="tab-content border-base-300 bg-base-100 p-10" style="height: 100%;">
      <TabulatorTable data={transfers} columns={statusColumns} rowMenu={[]} initialSort={statusInitialSort} />
    </div>

    <input type="radio" name="my_tabs_2" class="tab" aria-label="Search" checked={true}/>
    <div class="tab-content border-base-300 bg-base-100 p-10 pt-3" style="height: 100%;">
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
      <TabulatorTable data={tableSearchResults} columns={searchColumns} rowMenu={rowMenu} initialSort={[]} />
    </div>
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
  justify-content: center;
  text-align: center;
  overflow-y: auto;
}

.tabs {
  /* display: flex; */
  justify-content: center;
  align-items: center;
  /* gap: 1rem;
  margin: 0 auto;
  width: fit-content; */
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
