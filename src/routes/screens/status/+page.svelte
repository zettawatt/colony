<script lang="ts">
  import TabulatorTable from '../../../components/tabulator.svelte';
  import { statusColumns as importedStatusColumns } from '../../../utils/search/statusColumns';
  import { transferManager } from '../../../stores/transferManager';
  import { onMount } from 'svelte';
  import { getPassword } from "../../../utils/password/session";
  import LoginModal from '../../../components/login.svelte';
  import { addToast } from '../../../stores/toast';
  import { isMobile } from '../../../utils/responsive.js';

  let showLogin = $state(false);

  let statusInitialSort = [
    {column:"startedDate", dir:"desc"}
  ]

  // Create mobile-specific columns (status icon, Name, Type only)
  const mobileStatusColumns = [
    importedStatusColumns[0], // Status icon
    importedStatusColumns[1], // Name
    importedStatusColumns[3], // Type
  ];

  // Use mobile or desktop columns based on screen size
  let statusColumns = $derived($isMobile ? mobileStatusColumns : importedStatusColumns);

  let transfers = $state<any[]>([]);
  let statusTable: any; // Reference to the TabulatorTable component

  // Reactive statement to handle transfer updates
  $effect(() => {
    const values = Object.values($transferManager);
    transfers = values;
  });

  // Function to clear completed transfers
  function clearCompleted() {
    const instance = statusTable?.getTabulatorInstance();
    if (instance) {
      // Get all rows with "Complete" status
      const completedRows = instance.getRows().filter((row: any) => {
        const data = row.getData();
        return data.status === "Complete";
      });

      // Get the IDs of completed transfers to remove from store
      const completedIds = completedRows.map((row: any) => row.getData().id);

      // Remove from the transfer manager store (this will also update persistent storage)
      transferManager.update((transfers: any) => {
        const updatedTransfers = { ...transfers };
        completedIds.forEach((id: string) => {
          delete updatedTransfers[id];
        });
        return updatedTransfers;
      });

      // Delete the completed rows from tabulator
      completedRows.forEach((row: any) => row.delete());

      addToast(`Cleared ${completedRows.length} completed transfers`, "success");
    }
  }

  // Function to clear error transfers
  function clearErrors() {
    const instance = statusTable?.getTabulatorInstance();
    if (instance) {
      // Get all rows with "Errored" or "Cancelled" status
      const errorRows = instance.getRows().filter((row: any) => {
        const data = row.getData();
        return data.status === "Errored" || data.status === "Cancelled";
      });

      // Get the IDs of error transfers to remove from store
      const errorIds = errorRows.map((row: any) => row.getData().id);

      // Remove from the transfer manager store (this will also update persistent storage)
      transferManager.update((transfers: any) => {
        const updatedTransfers = { ...transfers };
        errorIds.forEach((id: string) => {
          delete updatedTransfers[id];
        });
        return updatedTransfers;
      });

      // Delete the error rows from tabulator
      errorRows.forEach((row: any) => row.delete());

      addToast(`Cleared ${errorRows.length} error transfers`, "success");
    }
  }

  onMount(async () => {
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
<main class="status-container" class:mobile-status={$isMobile} style="height: calc(100vh - 120px); display: flex; flex-direction: column; padding: 20px;">
  <div class="status-header" style="flex-shrink: 0; margin-bottom: 20px;">
    <div class="row mb-3">
      {#if !$isMobile}
        <h2 class="text-2xl font-bold">Transfer Status</h2>
      {/if}
      <div class="button-group">
        <button class="btn btn-soft btn-warning mr-2" onclick={clearCompleted}>
          Clear Completed
        </button>
        <button class="btn btn-soft btn-warning" onclick={clearErrors}>
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

/* Mobile-specific styles */
.mobile-status {
  padding: 10px !important;
  height: calc(100vh - 116px) !important; /* Account for mobile header + bottom nav */
}

.mobile-status .status-header {
  margin-bottom: 10px !important;
}

.mobile-status .row {
  justify-content: center;
}

.mobile-status .button-group {
  width: 100%;
  justify-content: center;
}

@media (max-width: 767px) {
  .status-table-container {
    margin: 0 -10px; /* Extend table to screen edges */
  }
}
</style>
