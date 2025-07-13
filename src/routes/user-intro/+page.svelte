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

  let currentStep = 0;
  let password = "";
  let isPasswordValid = false;
  let isPhraseValid = false;
  let showValidString = false;
  let parentSeedWords = [];
  let confirmSeedWords = [];
  let isSeedPhraseMatching = false;
  let showMatchingString = false;
  let walletPrivateKey = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
  let initWalletName = "main";

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

  async function finishSteps() {
    console.log({
      confirmSeedWords,
      password,
      walletPrivateKey
    })
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
      await invoke('set_active_wallet', { initWalletName });
      const client = await invoke("initialize_autonomi_client", { walletKey: walletPrivateKey });
      const podManager = await invoke("initialize_pod_manager");
      reroute("/screens/search");
      return true;
    } catch (error) {
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
        <svelte:component this={steps[currentStep].component} {validatePassword} />
      {:else if currentStep === 1}
        <svelte:component 
          this={steps[currentStep].component} 
          {generateNewSeedPhrase}
          {validateSeedPhrase}
          bind:words={parentSeedWords}
          bind:isPhraseValid={isPhraseValid}
          bind:showValidString={showValidString}
        />
      {:else if currentStep === 2}
        <svelte:component 
          this={steps[currentStep].component} 
          bind:words={confirmSeedWords}
          bind:showMatchingString={showMatchingString}
          bind:isSeedPhraseMatching={isSeedPhraseMatching}
        />
      {:else if currentStep === 3}
        <svelte:component 
          this={steps[currentStep].component} 
          bind:walletPrivateKey={walletPrivateKey}
        />
      {:else}
        <svelte:component this={steps[currentStep].component} />
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
</main>