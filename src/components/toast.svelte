<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';
  import toasts, { type Toast } from '../stores/toast';
  let items: Toast[] = [];

  const unsubscribe = toasts.subscribe(value => {
    items = value;
  });

  onMount(() => {
    return () => unsubscribe();
  });
</script>

<div class="fixed bottom-4 right-4 z-50 flex flex-col gap-3">
  {#each items as toast (toast.id)}
    <div
      class={`flex items-center gap-3 p-4 rounded-lg shadow-lg border-l-4 transition-opacity duration-200
        ${toast.type === 'info'
          ? 'bg-blue-50 border-blue-400 text-blue-800 dark:bg-blue-900 dark:border-blue-300 dark:text-blue-100'
          : toast.type === 'error'
          ? 'bg-red-50 border-red-400 text-red-800 dark:bg-red-900 dark:border-red-300 dark:text-red-100'
          : toast.type === 'warning'
          ? 'bg-yellow-50 border-yellow-400 text-yellow-800 dark:bg-yellow-900 dark:border-yellow-300 dark:text-yellow-100'
          : toast.type === 'success'
          ? 'bg-green-50 border-green-400 text-green-800 dark:bg-green-900 dark:border-green-300 dark:text-green-100'
          : 'bg-gray-50 border-gray-300 text-gray-800 dark:bg-gray-800 dark:border-gray-500 dark:text-gray-100'
        }`}
      in:fade={{ duration: 200 }}
      out:fade={{ duration: 200 }}
    >
      <span>
        {#if toast.type === 'info'}
          <svg 
            class="h-6 w-6 text-blue-400 dark:text-blue-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path 
              stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
              d="M10 11h2v5m-2 0h4m-2.592-8.5h.01M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
            />
          </svg>
        {:else if toast.type === 'error'}
          <svg class="h-6 w-6 text-red-400 dark:text-red-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M12 8v4m0 4h.01M12 2a10 10 0 100 20 10 10 0 000-20z" />
          </svg>
        {:else if toast.type === 'warning'}
          <svg class="h-6 w-6 text-yellow-400 dark:text-yellow-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M12 8v4m0 4h.01M12 2a10 10 0 100 20 10 10 0 000-20z" />
          </svg>
        {:else if toast.type === 'success'}
          <svg class="h-6 w-6 text-green-400 dark:text-green-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
              d="M9 12l2 2l4-4m6 2a9 9 0 11-18 0a9 9 0 0118 0z"/>
          </svg>
        {:else}
          <svg class="h-6 w-6 text-gray-400 dark:text-gray-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <circle cx="12" cy="12" r="10" />
          </svg>
        {/if}
      </span>
      <span class="flex-1 text-sm">{toast.message}</span>
    </div>
  {/each}
</div>
