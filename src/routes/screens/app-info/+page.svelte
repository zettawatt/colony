<script lang="ts">
  import { addToast } from '../../../stores/toast';
  import QRCode from 'qrcode';
  import { onMount } from 'svelte';

  let selectedCrypto = $state<string | null>(null);
  let qrCodeDataUrl = $state<string | null>(null);

  // Crypto addresses
  const cryptoAddresses = {
    bitcoin: 'bc1qp005au38ktl2zmhetsv223gld0sn3w456lkavw',
    ethereum: '0xc6e3a7a770656B8473DedCc3d4565b6D507afACE',
    usdc: '0xc6e3a7a770656B8473DedCc3d4565b6D507afACE',
    autonomi: '0xc6e3a7a770656B8473DedCc3d4565b6D507afACE'
  };

  async function selectCrypto(crypto: string) {
    selectedCrypto = crypto;
    const address = cryptoAddresses[crypto as keyof typeof cryptoAddresses];

    // Copy to clipboard
    try {
      await navigator.clipboard.writeText(address);
      addToast(`${crypto.toUpperCase()} address copied to clipboard!`, 'success');
    } catch (error) {
      console.error('Failed to copy to clipboard:', error);
      addToast('Failed to copy address to clipboard', 'error');
    }

    // Generate QR code
    await generateQRCode(address);
  }

  async function generateQRCode(text: string) {
    try {
      const qrDataUrl = await QRCode.toDataURL(text, {
        width: 200,
        margin: 2,
        color: {
          dark: '#000000',
          light: '#FFFFFF'
        },
        errorCorrectionLevel: 'M'
      });
      qrCodeDataUrl = qrDataUrl;
    } catch (error) {
      console.error('Failed to generate QR code:', error);
      addToast('Failed to generate QR code', 'error');
    }
  }

  function clearSelection() {
    selectedCrypto = null;
    qrCodeDataUrl = null;
  }

  // Close QR code when clicking outside
  function handleOutsideClick(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (!target.closest('.qr-container') && !target.closest('.crypto-icon')) {
      clearSelection();
    }
  }

  // Handle keyboard events for accessibility
  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      clearSelection();
    }
  }

  // Set up global event listeners
  onMount(() => {
    const handleGlobalClick = (event: MouseEvent) => {
      handleOutsideClick(event);
    };

    const handleGlobalKeyDown = (event: KeyboardEvent) => {
      handleKeyDown(event);
    };

    document.addEventListener('click', handleGlobalClick);
    document.addEventListener('keydown', handleGlobalKeyDown);

    return () => {
      document.removeEventListener('click', handleGlobalClick);
      document.removeEventListener('keydown', handleGlobalKeyDown);
    };
  });
</script>

<main class="info-container">
  <p>
    Colony App Built by
    <a href="https://www.linkedin.com/in/maxx-rodriguez/" target="_blank" class="profile-link">
      Maxx Rodriguez
      <svg class="linkedin-icon" width="16" height="16" viewBox="0 0 32 32" fill="currentColor" aria-label="LinkedIn">
        <path d="M27 0H5C2.2 0 0 2.2 0 5v22c0 2.8 2.2 5 5 5h22c2.8 0 5-2.2 5-5V5c0-2.8-2.2-5-5-5zM9.4
        27H5.3V12h4.2v15zM7.3 10.3c-1.3 0-2.3-1-2.3-2.3S6 5.8 7.3 5.8 9.6 6.8 9.6 8.1s-1 2.2-2.3 2.2zm20
        16.7h-4.2v-7.2c0-1.7-0.7-2.7-2.1-2.7-1.2 0-1.8 0.8-2.1 1.5-0.1 0.3-0.1 0.8-0.1 1.3V27h-4.2s0.1-13.6 0-15h4.2v2.1c0.6-1 1.7-2.4
        4-2.4 2.9 0 5 1.9 5 6v9.3z"/>
      </svg>
    </a>
  </p>
  <p>
    Colony Lib Built by Chuck McClish
  </p>
  <!-- <p>
    Colony Lib Built by
    <a href="https://www.linkedin.com/in/chuck-mcclish-9b227319/" target="_blank" class="profile-link">
      Chuck McClish
      <svg class="linkedin-icon" width="16" height="16" viewBox="0 0 32 32" fill="currentColor" aria-label="LinkedIn">
        <path d="M27 0H5C2.2 0 0 2.2 0 5v22c0 2.8 2.2 5 5 5h22c2.8 0 5-2.2 5-5V5c0-2.8-2.2-5-5-5zM9.4
        27H5.3V12h4.2v15zM7.3 10.3c-1.3 0-2.3-1-2.3-2.3S6 5.8 7.3 5.8 9.6 6.8 9.6 8.1s-1 2.2-2.3 2.2zm20
        16.7h-4.2v-7.2c0-1.7-0.7-2.7-2.1-2.7-1.2 0-1.8 0.8-2.1 1.5-0.1 0.3-0.1 0.8-0.1 1.3V27h-4.2s0.1-13.6 0-15h4.2v2.1c0.6-1 1.7-2.4
        4-2.4 2.9 0 5 1.9 5 6v9.3z"/>
      </svg>
    </a>
  </p> -->
  <p>Using the following technologies:</p>

  <div class="row">
    <a href="https://autonomi.com/" target="_blank">
      <img src="/autonomi.svg" class="logo autonomi" alt="Autonomi Logo" />
    </a>
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://kit.svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />
    </a>
  </div>

  <div class="github-section">
    <h4>Source Code:</h4>
    <a href="https://github.com/zettawatt/colony" target="_blank">
      Colony App on GitHub
    </a>
    <br />
    <a href="https://github.com/zettawatt/colonylib" target="_blank">
      Colony Lib on GitHub
    </a>
  </div>

  <p>Version: 1.1.5</p>
  <p>Like Colony? Donate to support future development:</p>

  <!-- Crypto donation section -->
  <div class="crypto-donation-section">
    <div class="crypto-icons-row">
      <button
        class="crypto-icon {selectedCrypto === 'bitcoin' ? 'active' : ''}"
        onclick={(e) => { e.stopPropagation(); selectCrypto('bitcoin'); }}
        aria-label="Donate BTC">
        <svg width="32" height="32" viewBox="0 0 32 32" fill="currentColor">
          <path d="M16 32C7.163 32 0 24.837 0 16S7.163 0 16 0s16 7.163 16 16-7.163 16-16 16zm7.189-17.98c.314-2.096-1.283-3.223-3.465-3.975l.708-2.84-1.728-.43-.69 2.765c-.454-.114-.92-.22-1.385-.326l.695-2.783L15.596 6l-.708 2.839c-.376-.086-.746-.17-1.104-.26l.002-.009-2.384-.595-.46 1.846s1.283.294 1.256.312c.7.175.826.638.805 1.006l-.806 3.235c.048.012.11.03.18.057l-.183-.045-1.13 4.532c-.086.212-.303.531-.793.41.018.025-1.256-.313-1.256-.313l-.858 1.978 2.25.561c.418.105.828.215 1.231.318l-.715 2.872 1.727.43.708-2.84c.472.127.93.245 1.378.357l-.706 2.828 1.728.43.715-2.866c2.948.558 5.164.333 6.097-2.333.752-2.146-.037-3.385-1.588-4.192 1.13-.26 1.98-1.003 2.207-2.538zm-3.95 5.538c-.533 2.147-4.148.986-5.32.695l.95-3.805c1.172.293 4.929.872 4.37 3.11zm.535-5.569c-.487 1.953-3.495.96-4.47.717l.86-3.45c.975.243 4.118.696 3.61 2.733z"/>
        </svg>
        <span>Bitcoin</span>
      </button>

      <button
        class="crypto-icon {selectedCrypto === 'ethereum' ? 'active' : ''}"
        onclick={(e) => { e.stopPropagation(); selectCrypto('ethereum'); }}
        aria-label="Donate ETH">
        <svg width="32" height="32" viewBox="0 0 32 32" fill="currentColor">
          <path d="M15.927 23.959l-9.823-5.797 9.817 13.839 9.828-13.839-9.828 5.797zM16.073 0l-9.819 16.297 9.819 5.807 9.823-5.801z"/>
        </svg>
        <span>Ethereum</span>
      </button>

      <button
        class="crypto-icon {selectedCrypto === 'usdc' ? 'active' : ''}"
        onclick={(e) => { e.stopPropagation(); selectCrypto('usdc'); }}
        aria-label="Donate USDC">
        <svg width="32" height="32" viewBox="0 0 2000 2000" xmlns="http://www.w3.org/2000/svg" fill="currentColor">
          <path d="M1000 2000c554.17 0 1000-445.83 1000-1000S1554.17 0 1000 0 0 445.83 0 1000s445.83 1000 1000 1000z" fill="#2775ca"/>
          <path d="M1275 1158.33c0-145.83-87.5-195.83-262.5-216.66-125-16.67-150-50-150-108.34s41.67-95.83 125-95.83c75 0 116.67 25 137.5 87.5 4.17 12.5 16.67 20.83 29.17 20.83h66.66c16.67 0 29.17-12.5 29.17-29.16v-4.17c-16.67-91.67-91.67-162.5-187.5-170.83v-100c0-16.67-12.5-29.17-33.33-33.34h-62.5c-16.67 0-29.17 12.5-33.34 33.34v95.83c-125 16.67-204.16 100-204.16 204.17 0 137.5 83.33 191.66 258.33 212.5 116.67 20.83 154.17 45.83 154.17 112.5s-58.34 112.5-137.5 112.5c-108.34 0-145.84-45.84-158.34-108.34-4.16-16.66-16.66-25-29.16-25h-70.84c-16.66 0-29.16 12.5-29.16 29.17v4.17c16.66 104.16 83.33 179.16 220.83 200v100c0 16.66 12.5 29.16 33.33 33.33h62.5c16.67 0 29.17-12.5 33.34-33.33v-100c125-20.84 208.33-108.34 208.33-220.84z" fill="#fff"/>
          <path d="M787.5 1595.83c-325-116.66-491.67-479.16-370.83-800 62.5-175 200-308.33 370.83-370.83 16.67-8.33 25-20.83 25-41.67V325c0-16.67-8.33-29.17-25-33.33-4.17 0-12.5 0-16.67 4.16-395.83 125-612.5 545.84-487.5 941.67 75 233.33 254.17 412.5 487.5 487.5 16.67 8.33 33.34 0 37.5-16.67 4.17-4.16 4.17-8.33 4.17-16.66v-58.34c0-12.5-12.5-29.16-25-37.5zM1229.17 295.83c-16.67-8.33-33.34 0-37.5 16.67-4.17 4.17-4.17 8.33-4.17 16.67v58.33c0 16.67 12.5 33.33 25 41.67 325 116.66 491.67 479.16 370.83 800-62.5 175-200 308.33-370.83 370.83-16.67 8.33-25 20.83-25 41.67V1700c0 16.67 8.33 29.17 25 33.33 4.17 0 12.5 0 16.67-4.16 395.83-125 612.5-545.84 487.5-941.67-75-237.5-258.34-416.67-487.5-491.67z" fill="#fff"/>
        </svg>
        <span>USDC</span>
      </button>

      <button
        class="crypto-icon {selectedCrypto === 'autonomi' ? 'active' : ''}"
        onclick={(e) => { e.stopPropagation(); selectCrypto('autonomi'); }}
        aria-label="Donate AUTONOMI">
        <img src="/autonomi.svg" alt="AUTONOMI" width="32" height="32" />
        <span>AUTONOMI</span>
      </button>
    </div>

    {#if selectedCrypto && qrCodeDataUrl}
      <div class="qr-container">
        <div class="qr-code">
          <img src={qrCodeDataUrl} alt="QR Code for {selectedCrypto} address" />
        </div>
        <div class="crypto-address">
          <p class="address-label">{selectedCrypto.toUpperCase()} Address:</p>
          <p class="address-text">{cryptoAddresses[selectedCrypto as keyof typeof cryptoAddresses]}</p>
        </div>
      </div>
    {/if}
  </div>
</main>

<style>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.svelte-kit:hover {
  filter: drop-shadow(0 0 2em #ff3e00);
}

.info-container {
  padding-top: 10vh;
  text-align: center;
  overflow-y: auto;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

.github-section {
  margin: 2em 0 1em 0;
}
.github-section a {
  display: inline-block;
  margin: 0.5em;
  color: #333;
  font-weight: 600;
}
.github-section a:hover {
  color: #24292f;
  text-decoration: underline;
}

.profile-link {
  color: #0072b1;
  text-decoration: underline;
  transition: color 0.2s;
  display: inline-flex;
  align-items: center;
  gap: 0.22em;
}
.profile-link:hover {
  color: #005582;
}
.linkedin-icon {
  vertical-align: middle;
  margin-left: 0.2em;
  color: #0072b1;
}
.profile-link:hover .linkedin-icon {
  color: #005582;
}

/* Crypto donation section styling */
.crypto-donation-section {
  margin-top: 2rem;
  padding: 1.5rem;
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.05);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.crypto-icons-row {
  display: flex;
  justify-content: center;
  gap: 1.5rem;
  margin-bottom: 1rem;
  flex-wrap: wrap;
}

.crypto-icon {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  padding: 1rem;
  border: 2px solid transparent;
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.1);
  color: currentColor;
  cursor: pointer;
  transition: all 0.3s ease;
  min-width: 80px;
}

.crypto-icon:hover {
  background: rgba(255, 255, 255, 0.2);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

.crypto-icon.active {
  border-color: #3b82f6;
  background: rgba(59, 130, 246, 0.2);
  box-shadow: 0 0 20px rgba(59, 130, 246, 0.3);
}

.crypto-icon svg,
.crypto-icon img {
  width: 32px;
  height: 32px;
  filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.2));
}

/* Specific crypto icon colors */
.crypto-icon[aria-label="Donate BTC"] svg {
  color: #f7931a; /* Bitcoin orange */
}

.crypto-icon[aria-label="Donate ETH"] svg {
  color: #627eea; /* Ethereum blue */
}

.crypto-icon[aria-label="Donate USDC"] svg {
  color: #2775ca; /* USDC blue */
}

/* Autonomi icon keeps its original colors from the SVG file */

.crypto-icon span {
  font-size: 0.875rem;
  font-weight: 500;
  text-align: center;
}

.qr-container {
  margin-top: 1.5rem;
  padding: 1.5rem;
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  animation: fadeIn 0.3s ease-in-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.qr-code {
  padding: 1rem;
  background: white;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

.qr-code img {
  display: block;
  border-radius: 4px;
}

.crypto-address {
  text-align: center;
  max-width: 100%;
}

.address-label {
  font-weight: 600;
  margin-bottom: 0.5rem;
  color: #3b82f6;
}

.address-text {
  font-family: 'Courier New', monospace;
  font-size: 0.875rem;
  word-break: break-all;
  padding: 0.75rem;
  background: rgba(0, 0, 0, 0.2);
  border-radius: 6px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  margin: 0;
}

/* Responsive design */
@media (max-width: 768px) {
  .crypto-icons-row {
    gap: 1rem;
  }

  .crypto-icon {
    min-width: 70px;
    padding: 0.75rem;
  }

  .crypto-icon svg,
  .crypto-icon img {
    width: 28px;
    height: 28px;
  }

  .crypto-icon span {
    font-size: 0.75rem;
  }

  .address-text {
    font-size: 0.75rem;
    padding: 0.5rem;
  }
}
</style>
