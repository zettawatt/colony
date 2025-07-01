<script type="ts">
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


  let searchInput = "";
  let tableSearchResults = [];
  let activeRow = {};
  let showLogin = false;
  $: transfers = Object.values($transferManager);

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

  async function simpleSearch() {
    try {
      if (searchInput === "") return;
      const request = {
        query: {
          // Put your search parameters here, e.g.:
            "type": "text",
            "text": searchInput,
            "limit": 200
        },
      };
      // const request = {query: "beg"}
      const response = await invoke('search', { request });
      const parsedResults = parseSparqlResults(response.results)
      console.log(parsedResults)
      tableSearchResults = parsedResults;
    } catch (error) {
      console.log(error)
    }
  }

  function parseSparqlResults(results) {
    try {
      const aggregate = {};
      const searchResults = {
        metadata: {
          pods_found: results.pods_found,
          result_count: results.result_count,
          search_timestamp: results.search_timestamp
        },
        variables: results.sparql_results.head.vars,
        bindings: results.sparql_results.results.bindings,
      }

      for (let i = 0; i < searchResults.bindings.length; i++) {
        const binding = searchResults.bindings[i];
        if (!(binding.subject.value in aggregate)){
          aggregate[binding.subject.value] = {
            id: i+1,
            pod: binding.graph.value.startsWith("ant://") 
              ? binding.graph.value.slice(6) : binding.graph.value,
            address: binding.subject.value.startsWith("ant://") 
              ? binding.subject.value.slice(6) : binding.subject.value,
            depth: binding.depth?.value || undefined
          };
        }
        switch (binding.predicate.value) {
          case 'http://schema.org/name':
            aggregate[binding.subject.value].name = binding.object.value;
            break;
          case 'http://schema.org/description':
            aggregate[binding.subject.value].description = binding.object.value;
            break;
          case 'http://schema.org/contentSize':
            aggregate[binding.subject.value].size = formatFileSize(Number(binding.object.value));
            break;
          case 'http://www.w3.org/1999/02/22-rdf-syntax-ns#type':
            aggregate[binding.subject.value].type = binding.object.value;
            break;
        }
      }
      return Object.values(aggregate);
    } catch (error) {
      console.trace(error)
      return;
    }
  }
  
  // Function to update data (useful for real-time updates)
  // function updateTorrentData(newData) {
  //   torrentsData = newData; // Reactive update
  // }

  onMount(async () => {
    await transferManager.init();
    if (getPassword() === null) {
      showLogin = true;
    }
  })
</script>

{#if showLogin}
  <LoginModal/>
{/if}
<main class="search-container">
  <div class="tabs tabs-box">
    <input type="radio" name="my_tabs_2" class="tab" aria-label="Table" checked={true}/>
    <div class="tab-content border-base-300 bg-base-100 p-10">
      <TabulatorTable data={transfers} columns={statusColumns} />
    </div>

    <input type="radio" name="my_tabs_2" class="tab" aria-label="Search" />
    <div class="tab-content border-base-300 bg-base-100 p-10 pt-3">
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
          <input type="search" required placeholder="Search" bind:value={searchInput}/>
        </label>
        <button class="btn btn-sof btn-warning" onclick={()=>simpleSearch()}>Search</button>
      </div>
      <TabulatorTable data={tableSearchResults} columns={searchColumns} rowMenu={rowMenu} />
    </div>
  </div>
  <dialog id="fileMetadataModal" class="modal">
    <div class="modal-box w-10/12 max-w-5xl max-h-lg">
      <h3 class="text-lg font-bold">File Metadata: {activeRow?.name}</h3>
      <div class="py-2" style="justify-content: center;">
          <table class="table table-xs">
            {#if activeRow}
              {#each Object.entries(activeRow) as [key, value]}
                <tr>
                  <th>{key}</th>
                  <td>{value}</td>
                </tr>
                <!-- {#if key === "size"}
                  <tr>
                    <th>{key}</th>
                    <td>{value}</td>
                  </tr>
                {:else if key !== "id"}
                  <tr>
                    <th>{key}</th>
                    <td>{value}</td>
                  </tr>
                {/if} -->
              {/each}
            {/if}
        </table>
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button class="btn btn-soft btn-error">Close</button>
        </form>
      </div>
    </div>
  </dialog>
  <!-- <TabulatorTable data={torrentsData} columns={torrentsColumns} /> -->
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
