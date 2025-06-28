<script type="ts">
  import TabulatorTable from '../../../components/tabulator.svelte';
  import { torrentsColumns } from '../../../components/testCols';
  import { torrentsData } from '../../../components/testData';
  import { searchColumns } from '../../../utils/searchColumns';
  import { testDataSearch } from '../../../components/testDataSearch';
  import { invoke } from "@tauri-apps/api/core";

  let searchInput = $state("");
  let tableSearchResults = $state([])

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
            pod: binding.graph.value.startsWith("ant://") 
              ? binding.graph.value.slice(6) : binding.graph.value,
            address: binding.subject.value.startsWith("ant://") 
              ? binding.subject.value.slice(6) : binding.subject.value,
            depth: binding.depth.value
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
            aggregate[binding.subject.value].size = Number(binding.object.value);
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
</script>

<main class="search-container">
  <div class="tabs tabs-box">
    <input type="radio" name="my_tabs_2" class="tab" aria-label="Table" checked={true}/>
    <div class="tab-content border-base-300 bg-base-100 p-10">
      <TabulatorTable data={torrentsData} columns={torrentsColumns} />
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
      <TabulatorTable data={tableSearchResults} columns={searchColumns} />
    </div>
  </div>
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

</style>
