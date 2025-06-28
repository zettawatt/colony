<script>
  import { onMount } from 'svelte';
  import { TabulatorFull as Tabulator } from 'tabulator-tables';
  import 'tabulator-tables/dist/css/tabulator.min.css';

  export let columns, data;

  let tableComponent;
  let tabulatorInstance;

  onMount(() => {
    tabulatorInstance = new Tabulator(tableComponent, {
      columns: columns,
      data: data,
      reactiveData: true, // enables Tabulator's own reactivity
      layout: 'fitDataStretch'
    });
  });

  $: if (tabulatorInstance && Array.isArray(data)) {
    console.log("Tabulator updating data:", data);
    tabulatorInstance.setData(data);
  }
</script>

<div bind:this={tableComponent}></div>