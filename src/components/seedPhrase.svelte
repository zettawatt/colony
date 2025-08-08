<script lang="ts">
  import * as bip39 from '@scure/bip39';
  import { wordlist } from '@scure/bip39/wordlists/english';

  export let seedWords: string[] = Array(12).fill("");

  // Android detection
  const isAndroid = typeof window !== 'undefined' && /Android/i.test(navigator.userAgent);

  export function generateNewSeedPhrase() {
    const mn = bip39.generateMnemonic(wordlist);
    seedWords = mn.split(" ");
    return mn;
  }
</script>

{#if isAndroid}
  <!-- Android: Vertical stack layout -->
  <div class="android-seed-phrase-container">
    {#each seedWords as _, index}
      <div class="android-word-input">
        <input
          type="text"
          class="input input-primary"
          bind:value={seedWords[index]}
          id="{(index + 1).toString()}"
          autocapitalize="off"
          autocomplete="off"
          autocorrect="off"
          spellcheck="false"
          placeholder="Word {index + 1}"
        />
      </div>
    {/each}
  </div>
{:else}
  <!-- Desktop: Table layout -->
  <div class="overflow-x-auto rounded-box border border-base-content/5 bg-base-100">
    <table class="table">
      <tbody>
        <tr>
          <td><input type="text" class="input input-primary" bind:value={seedWords[0]} id="1" autocapitalize="off" autocomplete="off" autocorrect="off" spellcheck="false" placeholder="Word 1" /></td>
          <td><input type="text" class="input input-primary" bind:value={seedWords[1]} id="2" autocapitalize="off" autocomplete="off" autocorrect="off" spellcheck="false" placeholder="Word 2" /></td>
          <td><input type="text" class="input input-primary" bind:value={seedWords[2]} id="3" autocapitalize="off" autocomplete="off" autocorrect="off" spellcheck="false" placeholder="Word 3" /></td>
          <td><input type="text" class="input input-primary" bind:value={seedWords[3]} id="4" autocapitalize="off" autocomplete="off" autocorrect="off" spellcheck="false" placeholder="Word 4" /></td>
        </tr>
        <tr>
          <td><input type="text" class="input input-primary" bind:value={seedWords[4]} id="5" autocapitalize="off" autocomplete="off" autocorrect="off" spellcheck="false" placeholder="Word 5" /></td>
          <td><input type="text" class="input input-primary" bind:value={seedWords[5]} id="6" autocapitalize="off" autocomplete="off" autocorrect="off" spellcheck="false" placeholder="Word 6" /></td>
          <td><input type="text" class="input input-primary" bind:value={seedWords[6]} id="7" autocapitalize="off" autocomplete="off" autocorrect="off" spellcheck="false" placeholder="Word 7" /></td>
          <td><input type="text" class="input input-primary" bind:value={seedWords[7]} id="8" autocapitalize="off" autocomplete="off" autocorrect="off" spellcheck="false" placeholder="Word 8" /></td>
        </tr>
        <tr>
          <td><input type="text" class="input input-primary" bind:value={seedWords[8]} id="9" autocapitalize="off" autocomplete="off" autocorrect="off" spellcheck="false" placeholder="Word 9" /></td>
          <td><input type="text" class="input input-primary" bind:value={seedWords[9]} id="10" autocapitalize="off" autocomplete="off" autocorrect="off" spellcheck="false" placeholder="Word 10" /></td>
          <td><input type="text" class="input input-primary" bind:value={seedWords[10]} id="11" autocapitalize="off" autocomplete="off" autocorrect="off" spellcheck="false" placeholder="Word 11" /></td>
          <td><input type="text" class="input input-primary" bind:value={seedWords[11]} id="12" autocapitalize="off" autocomplete="off" autocorrect="off" spellcheck="false" placeholder="Word 12" /></td>
        </tr>
      </tbody>
    </table>
  </div>
{/if}

<style>
  .android-seed-phrase-container {
    display: flex;
    flex-direction: column;
    gap: 8px;
    max-width: 400px;
    margin: 0 auto;
  }

  .android-word-input {
    width: 100%;
  }

  .android-word-input input {
    width: 100%;
  }
</style>