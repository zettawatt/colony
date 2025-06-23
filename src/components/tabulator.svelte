<script>
  import {TabulatorFull as Tabulator} from 'tabulator-tables';
  import {onMount} from 'svelte';
  import 'tabulator-tables/dist/css/tabulator.min.css';

  export let data, columns;

  let tableComponent;

  onMount(() => {
    new Tabulator(tableComponent, {
      data: data, //link data to table
      reactiveData:true, //enable data reactivity
      columns: columns, //define table columns
      layout: 'fitDataStretch'
    });
  });
</script>

<div bind:this={tableComponent}></div>