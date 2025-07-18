<script lang="ts">
  import StepWelcome from '../../components/steps/StepWelcome.svelte';
  import StepSeedPhraseInit from '../../components/steps/StepSeedPhraseInit.svelte';
  import StepSeedPhraseConfirm from '../../components/steps/StepSeedPhraseConfirm.svelte';
  import StepWallet from '../../components/steps/StepWallet.svelte';
  import StepPreferences from '../../components/steps/StepPreferences.svelte';
  import StepFinish from '../../components/steps/StepFinish.svelte';
  import { onMount } from 'svelte';
  import * as bip39 from '@scure/bip39';
  import { wordlist } from '@scure/bip39/wordlists/english';
  import { getPassword, setPassword } from '../../utils/password/session';
  import { invoke } from '@tauri-apps/api/core';
  import ps from '../../stores/persistantStorage';
  import { startDweb } from '../../utils/dweb/dwebCommands';
  import { addToast } from '../../stores/toast';

  let currentStep = 0;
  let password = "";
  let isPasswordValid = false;
  let isPhraseValid = false;
  let showValidString = false;
  let parentSeedWords = [];
  let confirmSeedWords = [];
  let isSeedPhraseMatching = false;
  let showMatchingString = false;
  let walletPrivateKey = generateRandomPrivateKey();
  let initWalletName = "main";
  let genesisPodAddress = "aaa518a2cf8260f6bebc769c16b8147ea215adf569696497b7fc1f250823d89a49990187e83fc0f9ae1cf3d44afb7dce";
  let runSync = true;

  // Modal reference
  let syncingInProgressModal: HTMLDialogElement;

  $: {
    if (parentSeedWords.length == 12){
      isPhraseValid = validateSeedPhrase(parentSeedWords);
      showValidString = true;
    } else {
      showValidString = false;
      isPhraseValid = false;
    }
  }

  $: {
    if (confirmSeedWords.length == 12){
      isSeedPhraseMatching = parentSeedWords.every((value, index) => value === confirmSeedWords[index]);
      console.log("confirm", confirmSeedWords)
      console.log("parent", parentSeedWords)
      console.log(isSeedPhraseMatching)
      showMatchingString = true
    }
  }

  $: canGoNext =
  (currentStep === 0 && isPasswordValid)
  || (currentStep === 1 && isPhraseValid)
  || (currentStep === 2 && isSeedPhraseMatching)
  || (currentStep === 3 && walletPrivateKey) // Adjust this as needed
  || (currentStep === 4); // Allow "Finish"

  function onNext() {
    if (currentStep === steps.length - 1) {
      finishSteps();
      return;
    }
    if (canGoNext) {
      currentStep++;
    }
  }

  const steps = [
    { title: "Welcome", component: StepWelcome, valid: false },
    { title: "Seed Phrase", component: StepSeedPhraseInit, valid: false },
    { title: "Confirmation", component: StepSeedPhraseConfirm, valid: false},
    { title: "Wallet", component: StepWallet, valid: false},
    { title: "Finish", component: StepFinish, valid: false }
  ];

  function validatePassword(newPassword, confirmPassword) {
    if (newPassword && confirmPassword && newPassword === confirmPassword) {
      password = confirmPassword;
      isPasswordValid = true;
    } else {
      isPasswordValid = false;
    }
  }

  function validateSeedPhrase(words: string[]) {
    const wasPhraseValid = bip39.validateMnemonic(words.join(" "), wordlist)
    return wasPhraseValid;
  }

  function generateNewSeedPhrase() {
    const mn = bip39.generateMnemonic(wordlist);
    parentSeedWords = mn.split(" ");
    console.log(parentSeedWords)
    return parentSeedWords;
  }

  function generateRandomPrivateKey(): string {
    // Generate 32 random bytes for Ethereum private key
    const randomBytes = new Uint8Array(32);
    crypto.getRandomValues(randomBytes);

    // Convert to hexadecimal string
    return Array.from(randomBytes)
      .map(byte => byte.toString(16).padStart(2, '0'))
      .join('');
  }

  async function finishSteps() {
    // console.log({
    //   confirmSeedWords,
    //   password,
    //   walletPrivateKey
    // })
    addToast("Finalizing setup, please wait...", "info", 8000);
    await firstTimeSetup();
  }


  function reroute(href:string) {
    window.location.href = href;
  }

  // initColony but needs to be different
  async function firstTimeSetup() {
    try {
      // await invoke("initialize_datastore");
      // await invoke("initialize_graph");
      await setPassword(password);
      const pw = await getPassword();
      if (!pw) {
        console.error("password was null");
      }
      const keystore = await invoke("create_keystore_from_seed_phrase", {seedPhrase: confirmSeedWords.join(" ")})
      const addWallet = await invoke("add_wallet", {
        request: {
          name: initWalletName,
          key: walletPrivateKey
        }
      })
      console.log("addWallet", addWallet)
      await invoke("write_keystore_to_file", {password: pw})
      await ps.setUserCompletedIntro(true);
      await ps.setPrimaryWallet(initWalletName)
      await invoke('set_active_wallet', { name: initWalletName });
      const client = await invoke("initialize_autonomi_client", { walletKey: walletPrivateKey });
      const podManager = await invoke("initialize_pod_manager");
      // addToast("Connected to Autonomi Network!", "success");
      if (runSync) {
        // Show syncing modal
        syncingInProgressModal?.showModal?.();
        // Prevent closing via Escape key
        syncingInProgressModal.addEventListener('cancel', (e) => {
          e.preventDefault();
        });

        try {
          // Run sync to populate local cache
          await invoke("refresh_ref", { request: { depth: "0" } });
          // If there are no pods, create a default one
          const pods = await invoke("list_my_pods");
          if (pods.length === 0) {
            const newPod = await invoke('add_pod', { request: {name: "default"} }) as PodInfo;
            console.log('Pod created at address:', newPod.address);
            // Create a pod reference to the genesis pod
            const genesisPodRef = await invoke('add_pod_ref', {
              request: {
                pod_address: newPod.address,
                pod_ref_address: genesisPodAddress
              }
            });
            await invoke("refresh_ref", { request: { depth: "0" } });
          }
        } finally {
          // Hide syncing modal
          syncingInProgressModal?.close?.();
        }
      }
      await startDweb(walletPrivateKey);
      reroute("/screens/search");
      return true;
    } catch (error) {
      addToast("Encounted an error on start up, see logs...", "error");
      console.error(error);
      return false;
    }
  }
</script>

<main>
  <div class="p-6 max-w-5xl mx-auto">
    <ul class="steps w-full mb-6">
      {#each steps as step, index}
        <li class="step {index <= currentStep ? 'step-warning' : ''}">
          {step.title}
        </li>
      {/each}
    </ul>

    <div class="p-4 rounded shadow bg-base-200">
      {#if currentStep === 0}
        <StepWelcome {validatePassword} />
      {:else if currentStep === 1}
        <StepSeedPhraseInit
          {generateNewSeedPhrase}
          {validateSeedPhrase}
          bind:words={parentSeedWords}
          bind:isPhraseValid={isPhraseValid}
          bind:showValidString={showValidString}
        />
      {:else if currentStep === 2}
        <StepSeedPhraseConfirm
          bind:words={confirmSeedWords}
          bind:showMatchingString={showMatchingString}
          bind:isSeedPhraseMatching={isSeedPhraseMatching}
        />
      {:else if currentStep === 3}
        <StepWallet
          bind:walletPrivateKey={walletPrivateKey}
        />
      {:else if currentStep === 4}
        <StepFinish bind:runSync={runSync} />
      {/if}
    </div>


    <div class="mt-6 flex justify-between">
      <button class="btn btn-outline" onclick={() => currentStep--} disabled={currentStep === 0}>
        Back
      </button>
      <button
        class="btn btn-primary"
        onclick={onNext}
        disabled={!canGoNext}
      >
        {currentStep === steps.length - 1 ? 'Finish' : 'Next'}
      </button>
    </div>
  </div>

  <!-- Syncing Progress Modal -->
  <dialog id="syncingInProgressModal" class="modal" bind:this={syncingInProgressModal}>
    <div class="modal-box flex flex-col items-center">
      <h3 class="text-lg font-bold mb-2">Syncing Data</h3>
      <div class="my-4">
        <span class="loading loading-spinner loading-lg"></span>
      </div>
      <p class="mb-2 text-center">Syncing your search cache with the network. Please do not close or leave this page until syncing is complete.</p>
    </div>
  </dialog>
</main>
