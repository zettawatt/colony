<script lang="ts">
  import TabulatorTable from '../../../components/tabulator.svelte';
  import { statusColumns } from '../../../utils/search/statusColumns';
  import { transferManager } from '../../../stores/transferManager';
  import { onMount } from 'svelte';
  import { getPassword } from "../../../utils/password/session";
  import LoginModal from '../../../components/login.svelte';
  import { addToast } from '../../../stores/toast';

  let showLogin = false;

  let statusInitialSort = [
    {column:"startedDate", dir:"desc"}
  ]

  let transfers = [];
  let statusTable; // Reference to the TabulatorTable component

  // Reactive statement to handle transfer updates
  $: {
    const values = Object.values($transferManager);
    transfers = values;
  }

  // Function to clear completed transfers
  function clearCompleted() {
    const instance = statusTable?.getTabulatorInstance();
    if (instance) {
      // Get all rows with "Complete" status
      const completedRows = instance.getRows().filter(row => {
        const data = row.getData();
        return data.status === "Complete";
      });

      // Delete the completed rows
      completedRows.forEach(row => row.delete());

      addToast(`Cleared ${completedRows.length} completed transfers`, "success");
    }
  }

  // Function to clear error transfers
  function clearErrors() {
    const instance = statusTable?.getTabulatorInstance();
    if (instance) {
      // Get all rows with "Errored" or "Cancelled" status
      const errorRows = instance.getRows().filter(row => {
        const data = row.getData();
        return data.status === "Errored" || data.status === "Cancelled";
      });

      // Delete the error rows
      errorRows.forEach(row => row.delete());

      addToast(`Cleared ${errorRows.length} error transfers`, "success");
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

    // Clear any existing column persistence data for status table to prevent column switching
    localStorage.removeItem('tabulator-colony-status-table-columns');
    localStorage.removeItem('tabulator-colony-search-table-columns'); // Also clear search table data that might interfere
  })
</script>

{#if showLogin}
  <LoginModal/>
{/if}
<main class="status-container" style="height: calc(100vh - 120px); display: flex; flex-direction: column; padding: 20px;">
  <div class="status-header" style="flex-shrink: 0; margin-bottom: 20px;">
    <div class="row mb-3">
      <h2 class="text-2xl font-bold">Transfer Status</h2>
      <div class="button-group">
        <button class="btn btn-sof btn-warning mr-2" onclick={clearCompleted}>
          Clear Completed
        </button>
        <button class="btn btn-sof btn-warning" onclick={clearErrors}>
          Clear Errors
        </button>
      </div>
    </div>
  </div>
  
  <div class="status-table-container" style="flex: 1; min-height: 0;">
    <TabulatorTable bind:this={statusTable} data={transfers} columns={statusColumns} rowMenu={[]} initialSort={statusInitialSort} persistenceID="colony-status-table" disableColumnPersistence={true} />
  </div>
</main>

<style>
.status-container {
  justify-content: flex-start;
  text-align: left;
  overflow: hidden;
}

.row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.button-group {
  display: flex;
  gap: 0.5rem;
}
</style>
