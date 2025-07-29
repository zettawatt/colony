<script lang="ts">
  import TabulatorTable from '../../../components/tabulator.svelte';
  import { statusColumns } from '../../../utils/search/statusColumns';
  import { transferManager } from '../../../stores/transferManager';
  import { onMount, onDestroy } from 'svelte';
  import { getPassword } from "../../../utils/password/session";
  import LoginModal from '../../../components/login.svelte';
  import { addToast } from '../../../stores/toast';

  let showLogin = false;
  let tabulatorTable;

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
  let lastTransferCount = 0;
  let lastTransferIds = new Set();

  // Function to update only the elapsed time cells without redrawing the entire table
  function updateElapsedTimes(newTransfers) {
    if (!tabulatorTable?.updateRowData) return;

    // Update each row's elapsed time individually using the new method
    newTransfers.forEach(transfer => {
      if (transfer.elapsed) {
        tabulatorTable.updateRowData(transfer.id, { elapsed: transfer.elapsed });
      }
    });
  }

  // Check if only elapsed times have changed (no structural changes)
  function onlyElapsedChanged(oldTransfers, newTransfers) {
    if (oldTransfers.length !== newTransfers.length) return false;

    for (let i = 0; i < newTransfers.length; i++) {
      const oldT = oldTransfers[i];
      const newT = newTransfers[i];

      if (!oldT || !newT) return false;
      if (oldT.id !== newT.id) return false;

      // Check if anything other than elapsed has changed
      const oldWithoutElapsed = { ...oldT };
      const newWithoutElapsed = { ...newT };
      delete oldWithoutElapsed.elapsed;
      delete newWithoutElapsed.elapsed;

      if (JSON.stringify(oldWithoutElapsed) !== JSON.stringify(newWithoutElapsed)) {
        return false;
      }
    }

    return true;
  }

  $: {
    const values = Object.values($transferManager);
    const currentTransferCount = values.length;
    const currentTransferIds = new Set(values.map(t => t.id));

    // Check if this is just an elapsed time update
    if (transfers.length > 0 &&
        currentTransferCount === lastTransferCount &&
        currentTransferIds.size === lastTransferIds.size &&
        [...currentTransferIds].every(id => lastTransferIds.has(id)) &&
        onlyElapsedChanged(transfers, values)) {

      // Only update elapsed times, don't trigger full table redraw
      updateElapsedTimes(values);

      // Update our local copy for next comparison
      transfers = values;
    } else {
      // Structural change - need full table update
      if (!shallowEqualArrays(transfers, values)) {
        transfers = values;
      }
    }

    lastTransferCount = currentTransferCount;
    lastTransferIds = currentTransferIds;
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
    <h2 class="text-2xl font-bold">Transfer Status</h2>
  </div>
  
  <div class="status-table-container" style="flex: 1; min-height: 0;">
    <TabulatorTable bind:this={tabulatorTable} data={transfers} columns={statusColumns} rowMenu={[]} initialSort={statusInitialSort} persistenceID="colony-status-table" disableColumnPersistence={true} />
  </div>
</main>

<style>
.status-container {
  justify-content: flex-start;
  text-align: left;
  overflow: hidden;
}
</style>
