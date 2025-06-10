<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import toasts, { type Toast } from '../stores/toast';
  import { derived } from 'svelte/store';

  let items: Toast[] = [];

  const unsubscribe = toasts.subscribe(value => {
    items = value;
  });

  onMount(() => {
    return () => unsubscribe();
  });

  const toastClass = (type: string) => {
    switch (type) {
      case 'success': return 'alert-info';
      case 'error': return 'alert-error';
      case 'warning': return 'alert-warning';
      default: return 'alert-info';
    }
  };
</script>

<div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2 max-w-sm">
  {#each items as toast (toast.id)}
    <div
      class={`alert shadow-lg ${toastClass(toast.type)}`}
      in:fade={{ duration: 200 }}
      out:fade={{ duration: 200 }}
    >
      <span>{toast.message}</span>
    </div>
  {/each}
</div>
