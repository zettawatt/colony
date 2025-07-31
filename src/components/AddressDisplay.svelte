<script lang="ts">
  import { addToast } from "../stores/toast";

  export let address: string;
  export let displayText: string = ""; // Optional custom display text (e.g., "pod address", "autonomi address")

  // Format address to show first 5 chars + ... + last 5 chars
  function formatAddress(addr: string): string {
    if (!addr || typeof addr !== 'string' || addr.length <= 13) return addr || '';
    
    const prefix = addr.substring(0, 5);
    const suffix = addr.substring(addr.length - 5);
    return `${prefix}...${suffix}`;
  }

  // Handle address click for clipboard copy
  async function handleAddressClick() {
    try {
      await navigator.clipboard.writeText(address);
      addToast(`Address ${address} copied!`, 'success');
    } catch (err) {
      console.error('Failed to copy address:', err);
      addToast('Failed to copy address', 'error');
    }
  }
</script>

<button
  class="address-display"
  onclick={handleAddressClick}
  tabindex="0"
  style="cursor: pointer; font-style: italic; text-decoration: underline dotted;"
>
  {displayText || formatAddress(address)}
</button>

<style>
  .address-display {
    background: none;
    border: none;
    padding: 0;
    color: inherit;
    font: inherit;
  }
  
  .address-display:hover {
    opacity: 0.8;
  }
</style>
