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
    <TabulatorTable data={transfers} columns={statusColumns} rowMenu={[]} initialSort={statusInitialSort} persistenceID="colony-status-table" disableColumnPersistence={true} />
  </div>
</main>

<style>
.status-container {
  justify-content: flex-start;
  text-align: left;
  overflow: hidden;
}
</style>
