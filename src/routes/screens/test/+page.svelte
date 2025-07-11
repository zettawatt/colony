<script lang="ts">
  import { onMount } from 'svelte';
  import { Sortable, MultiDrag } from 'sortablejs';


  let listOne = Array.from({ length: 10 }, (_, i) =>
    i === 0
      ? { id: `A${i}`, name: `Item A${i + 1}`, dragDisabled: true }
      : { id: `A${i}`, name: `Item A${i + 1}` }
  );
  let listTwo = Array.from({ length: 10 }, (_, i) =>
    i === 0
      ? { id: `B${i}`, name: `Item B${i + 1}`, dragDisabled: true }
      : { id: `B${i}`, name: `Item B${i + 1}` }
  );

  let listOneRef;
  let listTwoRef;

function syncArrays(evt, fromList, toList, fromRef, toRef) {
  if (evt.from === fromRef && evt.to === toRef) {
    const moved = fromList.splice(evt.oldIndex, 1);
    toList.splice(evt.newIndex, 0, ...moved);
  }
}

  onMount(() => {
document.body.style.userSelect = 'none';
    try {
      Sortable?.mount(new MultiDrag())
    } catch (error) {
      console.error(error)
    }

    const sharedGroup = 'shared';

    new Sortable(listOneRef, {
      group: sharedGroup,
      multiDrag: true,
      animation: 150,
      // forceFallback: true,
      selectedClass: "maxxtest",
      avoidImplicitDeselect: false,
    });

    new Sortable(listTwoRef, {
      group: sharedGroup,
      multiDrag: true,
      animation: 150,
      // forceFallback: true,
      selectedClass: "maxxtest",
      avoidImplicitDeselect: false,
      // fallbackTolerance: 1,
    });

  });
</script>

<style>
  .container {
    width: 45%;
    height: 300px;
    overflow-y: auto;
    border: 2px dashed #ccc;
    margin: 10px;
    padding: 10px;
    display: inline-block;
    vertical-align: top;
  }

  .item {
    padding: 10px;
    margin: 2px 0;
    background-color: #f2f2f2;
    border: 1px solid #ccc;
    cursor: grab;
    user-select: none;
  }

  .item:active {
    cursor: grabbing;
  }

  .item.disabled {
    color: #aaa;
    background-color: #eee;
    cursor: not-allowed;
  }
  .ghost {
    opacity: 0.3;
  }
  .chosen {
    background: #e4e4e4;
  }

  :global(.maxxtest) {
    background-color: #f9c7c8 !important;
    border: solid red 1px !important;
    z-index: 1 !important;
  }

</style>

<h1>SortableJS - Drag Between Lists</h1>
<div style="display: flex; justify-content: space-around;">
  <!-- List One -->
  <div class="container" bind:this={listOneRef}>
    {#each listOne as item (item.id)}
      <div class="item">{item.name}</div>
    {/each}
  </div>

  <!-- List Two -->
  <div class="container" bind:this={listTwoRef}>
    {#each listTwo as item (item.id)}
      <div class="item" on:click={() => console.log('clicked', item)}>{item.name}</div>
    {/each}
  </div>
</div>
