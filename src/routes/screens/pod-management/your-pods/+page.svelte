<script lang="ts">
  import Drawer from "../../../../components/drawer.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { addToast }  from '../../../../stores/toast';
  import { onMount } from "svelte";
  import { FileObj, type FileInfo } from "../../../../classes/FileObj";
  import ps from "../../../../stores/persistantStorage";
  import { handleCopyAddress } from "../../../../utils/copyAutonomiAddress";
  import { getPassword } from "../../../../utils/password/session";
  import { podsSyncing, allPodsUploading } from "../../../../stores/globals";
  import { v4 as uuidv4 } from 'uuid';
  import { parseSubjectData } from "../../../../utils/pod-management/parseSubjectData";

  let podListTemp = $state([
    {
      uuid: "1",
      name: "907f7857974fef55dcba7f73529790925e91738d5df54f021cd18b92f533e68946c1a416e144b00d869e68c080bda3ac",
      selected: false
    },
    {
      uuid: "2",
      name: "Fallout-4-Vault-Dweller's-Survival-Guuuide-Prima-Official-Game-Guuuide.pdf",
      selected: false
    },
    {
      uuid: "3",
      name: "music.mp3",
      selected: false
    },
    {
      uuid: "4",
      name: "report.pdf",
      selected: false
    },
    {
      uuid: "5",
      name: "another_document.docx",
      selected: false
    },
    {
      uuid: "6",
      name: "holuuiday-photo.png",
      selected: false
    },
    {
      uuid: "7",
      name: "presentation.pptx",
      selected: false
    },
    {
      uuid: "8",
      name: "important_data.bak",
      selected: false
    }
  ])

  type PodInfo = {
    address: string
  }

  type PodMetaData = {
    address: string;
    name?: string | undefined;
    creation?: string | undefined;
    modified?: string | undefined;
    depth?: string | undefined;
  };

  let isLoading = $state(false);
  let newPodName = $state("");
  let createdPods = $state<any[]>([]) as PodMetaData[];
  let activePod = $state<any>({ fileObjs: [] }); // Holds the pod for the currently active modal
  let uploadedFiles = $state<any[]>([]);
  let selectedFileName = $state(""); // <-- Track the filename selected for adding
  let activeFileType = $state("other");
  let availableTypes = $state(['audio', 'video', 'image', 'book', 'other']);
  let displayFields = $derived.by(() => {
      switch (activeFileType) {
        case 'audio':
          return ['Title', 'Artist', 'Album', 'Release Date', 'Comment'];
        case 'video':
          return ['Title', 'Director', 'Release Date', 'Duration', 'Comment'];
        case 'image':
          return ['Title', 'Description', 'Date Taken', 'Comment'];
        case 'book':
          return ['Title', 'Author', 'Publisher', 'Publication Date', 'Comment'];
        default:
          return ['Title', 'Description', 'Comment'];
      }
  })
  let userConfigPod = $state();
  let podRefAddress = $state(""); // address of the pod reference user wants to add
  let editingPodItem = $state();
  let editMetadataFields = $state({});
  let deletedPodItems = $state([]);

  $effect(()=> {
    console.log('activepod', activePod)
  })


  async function addPodRef(podAddress, podRefAddress) {
    try {
      const response = await invoke('add_pod_ref', {
        request: {
          pod_address: podAddress,
          pod_ref_address: podRefAddress
        }
      });
      console.log('Pod added:', response);
      // response.address is the pod's address (from your Rust PodInfo)
      return response;
    } catch (error) {
      console.error('Error adding pod ref:', error);
      throw error;
    }
  }

  async function removePodRef(podAddress, podRefAddress) {
    try {
      const response = await invoke('remove_pod_ref', {
        request: {
          pod_address: podAddress,
          pod_ref_address: podRefAddress,
        }
      });
      // response is an object like: { address: 'the pod address' }
      console.log('Removed pod ref:', response);
      return response;
    } catch (error) {
      console.error('Error removing pod ref:', error);
      throw error;
    }
  }

  async function putSubjectData(podAddress, subjectAddress, data) {
    try {
      let jsonData = JSON.stringify(data);
      const result = await invoke('put_subject_data', {
        request: {
          pod_address: podAddress,
          subject_address: subjectAddress,
          data: jsonData,
        }
      });
      // `result` will be a String: "Successfully put data for subject ... in pod ..."
      console.log(result);
      return result;
    } catch (error) {
      console.error('Error putting subject data:', error);
      throw error;
    }
  }

  async function savePod() {
    console.log("savePod", activePod)

    if (activePod.fileObjs.length > 0 || deletedPodItems.length > 0) {
      for (const file of activePod.fileObjs) {
        if (file.type === 'file' && file.modified === true){
          const metadataJson = generateFileMetaJson(file)
          // console.log({
          //   pod_address: activePod.address,
          //   subject_address: file.autonomiAddress,
          //   data: JSON.stringify(metadataJson)
          // })
          // const result = await invoke<string>('put_subject_data', {request: {
          //   pod_address: activePod.address,
          //   subject_address: file.autonomiAddress,
          //   // data: JSON.stringify({})
          //   data: JSON.stringify(metadataJson)
          // }});
          const result = await putSubjectData(activePod.address, file.autonomiAddress, metadataJson)
          addToast(`Successfilly added ${file.name} to pod!`, "success")
        } else if (file.type === 'pod-ref'){
          const result = await addPodRef(activePod.address, file.autonomiAddress)
          console.log(result);
        }
      }

      // remove any deleted items
      for (const file of deletedPodItems) {
        if (file.type === 'file'){
          const result = await putSubjectData(activePod.address, file.autonomiAddress, {})
          // addToast(`Successfilly added ${file.name} to pod!`, "success")
        } else if (file.type === 'pod-ref'){
          const result = await removePodRef(activePod.address, file.autonomiAddress)
          // do something else to add pod reference
        }
      }
    }
  }

  async function deletePod(podName: string) {
    try {
      const response = await invoke('remove_pod', {
        request: { name: podName }
      });
      // response will be { address: podName }
      console.log('Pod removed:', response);
      return response;
    } catch (error) {
      console.error('Error removing pod:', error);
    }
  }


  function generateFileMetaJson(file: any) {
    const fileMetaJson = {
      "@context": { "schema": "http://schema.org/" },
      "@type": "",
      "@id": `ant://${file.autonomiAddress}`,
      "schema:name": file.name,
      "schema:description": "",
      "schema:contentSize": file.fileSize
    };
    switch (file.metadata.type) {
      case 'audio':
        fileMetaJson["@type"] = "schema:MusicRecording";
        fileMetaJson["schema:alternateName"] = file.metadata["Title"];
        fileMetaJson["schema:byArtist"] = file.metadata["Artist"];
        fileMetaJson["schema:inAlbum"] = file.metadata["Album"];
        fileMetaJson["schema:datePublished"] = file.metadata["Release Date"];
        fileMetaJson["schema:comment"] = file.metadata["Comment"];
        break;
      case 'video':
        fileMetaJson["@type"] = "schema:VideoObject";
        fileMetaJson["schema:alternateName"] = file.metadata["Title"];
        fileMetaJson["schema:director"] = file.metadata["Director"];
        fileMetaJson["schema:datePublished"] = file.metadata["Release Date"];
        fileMetaJson["schema:duration"] = file.metadata["Duration"];
        fileMetaJson["schema:comment"] = file.metadata["Comment"];
        break;
      case 'image':
        fileMetaJson["@type"] = "schema:ImageObject";
        fileMetaJson["schema:alternateName"] = file.metadata["Title"];
        fileMetaJson["schema:description"] = file.metadata["Description"];
        fileMetaJson["schema:dateCreated"] = file.metadata["Date Taken"];
        fileMetaJson["schema:comment"] = file.metadata["Comment"];
        break;
      case 'book':
        fileMetaJson["@type"] = "schema:Book";
        fileMetaJson["schema:alternateName"] = file.metadata["Title"];
        fileMetaJson["schema:author"] = file.metadata["Author"];
        fileMetaJson["schema:publisher"] = file.metadata["Publisher"];
        fileMetaJson["schema:datePublished"] = file.metadata["Publication Date"];
        fileMetaJson["schema:comment"] = file.metadata["Comment"];
        break;
      default:
        fileMetaJson["@type"] = "schema:CreativeWork";
        fileMetaJson["schema:alternateName"] = file.metadata["Title"];
        fileMetaJson["schema:description"] = file.metadata["Description"];
        fileMetaJson["schema:comment"] = file.metadata["Comment"];
        break;
    }
    return fileMetaJson;
  }

  async function uploadAllPods() {
    try {
      allPodsUploading.set(true);
      const result = await invoke<string>('upload_all');
      addToast(result, "success");
      allPodsUploading.set(false);
      console.log(result); // "Successfully uploaded all updated pods to Autonomi"
    } catch (error) {
      console.error('Upload failed:', error);
      allPodsUploading.set(false);
    }
  }

  async function uploadSinglePod() {
    try {
      const podAddress = activePod.address;
      const result = await invoke('upload_pod', {
        request: { pod_address: podAddress }
      });
      console.log('Pod uploaded:', result);
      addToast(`Successfully uploaded pod ${activePod.name}`, "success");
    } catch (err) {
      console.error('Failed to upload pod:', err);
      addToast(err, "error");
    }
  }

  async function fetchPods() {
    try {
      const results = await invoke('list_my_pods');
      const regularPods = results.filter(pod => pod.name !== "User Configuration");
      userConfigPod = results.find(pod => pod.name === "User Configuration");
      // result will likely be { addresses: [ ..pod addresses.. ] }
      console.log('Pods:', results);
      console.log('user config pod', userConfigPod)
      return regularPods
    } catch (e) {
      console.error('Failed to fetch pods:', e);
    }
  }

  async function fetchPodSubjects(address) {
    try {
      // The name must exactly match your Rust function (snake_case)
      const result = await invoke('list_pod_subjects', { address });
      // result will be your AddressList Rust struct as a JS object
      console.log(result.addresses);
      return result.addresses;
    } catch (error) {
      // Handle error from Rust
      console.error("Error calling list_pod_subjects:", error);
      return null;
    }
  }

  async function fetchSubjectData(subjectAddress) {
    try {
      // Call the Tauri command "get_subject_data"
      const result = await invoke('get_subject_data', { request: { subject_address: subjectAddress } });
      const parsedResult = JSON.parse(result.data);
      // result will be your SubjectDataResult struct as a JS object, ex: { data: ... }
      return parsedResult;
    } catch (e) {
      console.error('Failed to get subject data:', e);
      return null;
    }
  }

  async function syncPods(depthValue: number) {
    try {
      podsSyncing.set(true);
      addToast("Refreshing pods....", "info");
      const response = await invoke('refresh_ref', {
        request: {
          depth: String(depthValue),
        }
      });
      console.log('Success:', response);
      podsSyncing.set(false);
      addToast("Pods have been synced", "info");
      await loadTable();
    } catch (e) {
      console.error('Failed to sync:', e);
      addToast("Failed to sync pods", "error");
      podsSyncing.set(false);
    }
  }

  async function createPod() {
    if (newPodName) {
      const podObj = {
        name: newPodName,
        createdDate: new Date().toISOString(),
        lastModifiedDate: new Date().toISOString(),
        address: "",
        fileObjs: []
      }
      try {
        isLoading = true;
        const pw = await getPassword();
        const podInfo = await invoke('add_pod', { request: {name: newPodName} }) as PodInfo;
        podObj["address"] = podInfo.address;
        addToast('Pod created at address:'+ podInfo.address, "info")
        await invoke("write_keystore_to_file", {password: pw})
        await loadTable();
        console.log('Pod created at address:', podInfo.address);
      } catch (err) {
        console.error('Failed to add pod:', err);
        addToast('Failed to add pod: ' + err, "error")
      } finally {
        const modal = document.getElementById('createNewPodModal') as HTMLDialogElement;
        if (modal) modal.close();
        isLoading = false;
      }
    }
  }

  function addFileToActivePod() {
    // console.log("activePod", activePod)
    if (!selectedFileName) return;
    if (!activePod.fileObjs) activePod.fileObjs = [];
    // Find FileObj in uploadedFiles by name
    const fileToAdd = uploadedFiles.find(f => f.name === selectedFileName);
    if (fileToAdd && !activePod?.fileObjs.some(f => f.name === fileToAdd.name)) {
      // Add only if not already present
      activePod.fileObjs = [...activePod.fileObjs, fileToAdd];
    }
    selectedFileName = ""; // Optionally reset selection
  }

  async function deletePodHandler() {
    try {
      const pw = await getPassword();
      deletePodModal.close();
      await deletePod(activePod.name);
      await invoke("write_keystore_to_file", {password: pw})
      await loadTable();
      addToast("Pod deleted!", "success");
    } catch (error) {
      console.error(error);
      addToast("Error removing pod. Check logs...", "error");
    }
  }

  async function loadTable() {
    // createdPods = await ps.getPodCache() as [];
    createdPods = await fetchPods();
    console.log("createdPods", createdPods)
  }

  function makeDateReadable(date: string | undefined) {
    if (date) {
      return new Date(date).toLocaleString();
    } else {
      return;
    }
  }

  function toggleSelection(list: any[], id: string) {
    console.log('here maxx');
    
    return list.map(item =>
      item.uuid === id ? {...item, selected: !item.selected} : item
    );
  }

  function transferItems(from: any[], to: any[]) {
    const selectedItems = from.filter(item => item.selected);

    // Filter out items whose uuid is already present in 'to'
    const toUuids = new Set(to.map(item => item.uuid));
    const newItems = selectedItems.filter(item => !toUuids.has(item.uuid));

    return {
      newFrom: from.map(item => ({ ...item, selected: false })),
      newTo: [
        ...to,
        ...newItems.map(item => ({
          ...item,
          selected: false,
          metadata: {},
          type: 'file',
          modified: true,
        }))
      ]
    };
  }

  function removeItems(from: any[]){
    const selectedItems = from.filter(item => item.selected);
    deletedPodItems = deletedPodItems.concat(selectedItems);
    return from.filter(item => !item.selected)
  }

  function addPodReference() {
    if (!podRefAddress) return;
    if (!activePod.fileObjs) activePod.fileObjs = [];
    activePod.fileObjs.push({
      autonomiAddress: podRefAddress,
      type: "pod-ref",
      uuid: uuidv4(),
    })
  }

  function saveMetaDataToItem() {
    if (editingPodItem) {
      editingPodItem.metadata = {...editMetadataFields}; // copy values
      editingPodItem.metadata["type"] = activeFileType;
      editingPodItem.modified = true;
      addToast('Metadata saved!', 'success');
    }
    console.log(editingPodItem)
    editFileMetadataModal.close();
  }

  function openEditMetadata(item) {
    editingPodItem = item;
    activeFileType = item.metadata.type;
    // Shallow copy to avoid direct binding unless you want live updating
    editMetadataFields = {...(item.metadata || {})};
    // If there are new fields, ensure they're in the object
    for (const field of displayFields) {
      if (!(field in editMetadataFields)) {
        editMetadataFields[field] = "";
      }
    }
    editFileMetadataModal.showModal();
  }

  async function openEditPod() {
    const subjects = await fetchPodSubjects(activePod.address)
    const tempPodItems = [];
    if (subjects.length > 0) {
      for (let subject of subjects) {
        // console.log("subject", subject)
        const data = await fetchSubjectData(subject)
        const item = parseSubjectData(data, activePod.address, subject)
        if ('type' in item && (item.type === 'pod-ref' || item.type === 'file')){
          tempPodItems.push(item);
        }
      }
    }
    console.log("tempPodItems", tempPodItems)
    activePod.fileObjs = tempPodItems;
    editPodModal.showModal();
  }

  function resetState() {
    deletedPodItems = [];
    activePod = { fileObjs: [] };
    editingPodItem = undefined;
    editMetadataFields = {};
    podRefAddress = "";
    activeFileType = "other";
    // Optionally deselect any files in uploadedFiles too:
    uploadedFiles = uploadedFiles.map(f => ({ ...f, selected: false }));
    // Close the modal if open for manual or programmatic triggers (defensive)
    editPodModal.close();
  }

  onMount(async () => {
    // await initPodManager();
    await loadTable();
    uploadedFiles = (await ps.getUploadedFilesArray()).map(file => ({
      ...file,
      disabled: false,
      selected: false
    }));
    console.log(uploadedFiles)
  })
</script>
<main>
  <Drawer>
    <div slot="main">
      <div class="row" style="display: flex; flex-direction: row; justify-content: space-between; padding-top:4vh;">
        <h2 class="h2">Your Pods</h2>
        <div class="utility-bar" style="display: flex;">
          <button class="btn btn-neutral btn-soft" onclick={() => syncPods(0)} disabled={$podsSyncing}>Sync Pods</button>
          <button class="btn btn-neutral" onclick={() => uploadAllPods()} disabled={$allPodsUploading}>Upload All Pods</button>
          <button class="btn btn-warning" onclick={createNewPodModal.showModal()}>Create New Pod</button>
        </div>
      </div>
      <div class="row">
        <div class="card bg-base-100 w-96 shadow-lg card-xl" style="width: auto;">
          <div class="card-body items-center text-center p-4">
            <table class="table table-zebra">
              <thead>
                <tr>
                  <th></th>
                  <th>Pod Name</th>
                  <th>Pod Address</th>
                  <th>Created Date</th>
                  <th>Last Modified</th>
                  <th>Operations</th>
                </tr>
              </thead>
              <tbody>
                {#if createdPods.length > 0}
                  {#each createdPods as pod, idx}
                    <tr>
                      <th>{idx + 1}</th>
                      <td>{pod.name}</td>
                      <td>
                        <div class="tooltip tooltip-warning" data-tip={pod.address}>
                          <button
                            class="address-tooltip"
                            data-address={pod.address}
                            onclick={handleCopyAddress}
                            tabindex="0"
                            style="cursor: pointer; font-style: italic; text-decoration: underline dotted;"
                          >pod address</button>
                        </div>
                      </td>
                      <td>{makeDateReadable(pod.creation)}</td>
                      <td>{makeDateReadable(pod.modified)}</td>
                      <td>
                        {#if pod.name !== "User Configuration"}
                          <!-- <button
                            class="btn btn-accent btn-square"
                            onclick={() => { activePod = pod; activePod.fileObjs = []; uploadSinglePod(); }}>
                            <img src="/app-icons/cloud-data-upload-icon.svg" alt="upload icon" width="24" height="24" />
                          </button> -->
                          <button 
                            class="btn btn-warning btn-square"
                            onclick={() => { activePod = pod; activePod.fileObjs = []; openEditPod(); }}>
                            <img src="/app-icons/pencil-icon.svg" alt="edit icon" width="19" height="19" />
                          </button>
                          <button 
                            class="btn btn-error btn-square"
                            onclick={() => { activePod = pod; activePod.fileObjs = []; deletePodModal.showModal(); }}>
                            <img src="/app-icons/trash-icon.svg" alt="trash icon" width="16" height="16" />
                          </button>
                        {/if}
                      </td>
                    </tr>
                  {/each}
                {:else}
                  <tr>
                    <td colspan="12" style="text-align:center;">No pods created yet</td>
                  </tr>
                {/if}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
    <ul slot="sidebar" class="menu bg-base-100 text-base-content min-h-full w-40 p-5">
      <li><a href="#" class="menu-active">Your Pods</a></li>
      <li><a href="/screens/pod-management/uploads">Uploads</a></li>
      <li><a href="/screens/pod-management/downloads">Downloads</a></li>
    </ul>
  </Drawer>
  <dialog id="createNewPodModal" class="modal">
    <div class="modal-box">
      <h3 class="text-lg font-bold">Create New Pod</h3>
      <div class="py-4">
        <input type="text" placeholder="Please enter a name for your pod..." class="input w-full" bind:value={newPodName} />
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button
            class="btn btn-primary"
            type= "button"
            onclick={createPod}
            disabled={!newPodName}
          >
            {#if isLoading}
              <span class="loading loading-spinner"></span>
            {:else}
              Create
            {/if}
          </button>
          <button class="btn btn-soft btn-error">Cancel</button>
        </form>
      </div>
    </div>
  </dialog>
  <dialog id="uploadPodModal" class="modal">
    <div class="modal-box w-8/12 max-w-xl">
      <h3 class="text-lg font-bold">Upload Preview</h3>
      <div class="py-4" style="justify-content: center;">
        <p class="pb-3">This is a preview of how much it might cost to upload your pod:</p>
        <table class="table">
          <tbody>
            <tr>
              <td>[pod name here]</td>
              <td><span class="loading loading-dots"></span> [some ant denomination here 100 ANT]</td>
            </tr>
          </tbody>
        </table>
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button class="btn btn-primary btn-disabled">
            <span class="loading loading-spinner"></span>
            Checking price
          </button>
          <button class="btn btn-soft btn-error">Cancel Upload</button>
        </form>
      </div>
    </div>
  </dialog>
  <dialog id="deletePodModal" class="modal">
    <div class="modal-box w-8/12 max-w-xl">
      <h3 class="text-lg font-bold">Pod Deletion</h3>
      <div class="py-4" style="justify-content: center;">
        <p class="pb-3">Are you use you want to delete your pod?</p>
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button class="btn btn-error" onclick={()=>{deletePodHandler()}}>Delete</button>
          <button class="btn btn-soft btn-error">Cancel</button>
        </form>
      </div>
    </div>
  </dialog>
  <dialog id="editPodModal" class="modal">
    <div class="modal-box w-10/12 max-w-5xl max-h-lg">
      <h3 class="text-lg font-bold">Editing Pod: {activePod?.name}</h3>
      <div class="py-2 flex items-center justify-center gap-x-1">
        <div class="flex flex-col items-center">
          <h4 class="text-center font-semibold">Pod Items</h4>
          <ul id="podItems" class="item-container flex flex-col mb-1">
            {#each activePod?.fileObjs as item (item.uuid)}
              <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              {#if item.type === 'pod-ref'}
                <li
                  class="flex item {item.selected ? 'item-selected' : ''} {item.disabled ? 'item-disabled' : ''}"
                  onclick={() => {
                    if (!item.disabled) {
                      activePod.fileObjs = toggleSelection(activePod?.fileObjs, item.uuid);
                    }
                  }}
                >
                  <span class="truncate">{item.autonomiAddress}</span>
                  <button
                    class="edit-button btn btn-sm"
                    onclick={() => {
                      editingPodItem = item;
                      podRefAddress = item.autonomiAddress;
                      console.log("editingPodItem", editingPodItem)
                      event.stopPropagation();
                      editFileMetadataModal.showModal();
                    }}
                  >
                    Edit
                  </button>
                </li>
              {:else}
                <li
                  class="flex item {item.selected ? 'item-selected' : ''} {item.disabled ? 'item-disabled' : ''}"
                  onclick={() => {
                    if (!item.disabled) {
                      activePod.fileObjs = toggleSelection(activePod?.fileObjs, item.uuid);
                    }
                  }}
                >
                  <span class="truncate">{item.name}</span>
                  <button
                    class="edit-button btn btn-sm"
                    onclick={() => {
                      event.stopPropagation();
                      openEditMetadata(item);
                      // editingPodItem = item;
                      // console.log("editingPodItem", editingPodItem)
                      // event.stopPropagation();
                      // editFileMetadataModal.showModal();
                    }}
                  >
                    Edit
                  </button>
                </li>
              {/if}
            {/each}
          </ul>
          <div class="w-full ml-5">
            <button
              class="btn btn-neutral btn-xs"
              onclick={() => {
                podRefAddress = "";
                addPodRefModal.showModal()
              }}
            >
              Add Pod Ref
            </button>
          </div>
        </div>

        <div class="mx-4 flex flex-col gap-2 items-center">
          <button 
            class="btn btn-error btn-sm w-full" 
            disabled={!activePod.fileObjs?.some(f => f.selected)}
            onclick={()=>{
              const result = removeItems(activePod?.fileObjs);
              activePod.fileObjs = result;
            }}  
          >
            Remove
          </button>
          <button 
            class="btn btn-primary btn-sm" 
            disabled={!uploadedFiles?.some(f => f.selected)}
            onclick={() => {
              const result = transferItems(uploadedFiles, activePod?.fileObjs);
              uploadedFiles = result.newFrom;
              activePod.fileObjs = result.newTo;
            }}
          >
            &larr;&nbsp;Transfer
          </button>
        </div>

        <div class="flex flex-col items-center">
          <h4 class="text-center font-semibold">Available Files</h4>
          <ul id="files" class="item-container flex flex-col mb-1">
            {#each uploadedFiles as item (item.uuid)}
              <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <li
                class="flex item {item.selected ? 'item-selected' : ''} {item.disabled ? 'item-disabled' : ''}"
                onclick={() => {
                  if (!item.disabled) {
                    uploadedFiles = toggleSelection(uploadedFiles, item.uuid);
                  }
                }}
              >
                <span class="truncate">{item.name}</span>
              </li>
            {/each}
          </ul>
          <div class="w-full invisible">
            <button class="btn btn-xs btn-outline">invisible</button>
          </div>
        </div>
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button class="btn btn-primary" onclick={() => savePod()}>Save Pod</button>
          <button class="btn btn-soft btn-error" onclick={() => resetState()}>Cancel</button>
        </form>
      </div>
    </div>
  </dialog>
  <!-- <dialog id="editPodModalOld" class="modal">
    <div class="modal-box w-10/12 max-w-3xl max-h-lg">
      <h3 class="text-lg font-bold">Editing Pod: {activePod?.name}</h3>
      <div class="py-2" style="justify-content: center;">
        <div class="join">
          <select class="select" bind:value={selectedFileName}>
            <option disabled selected>File Reference</option>
            {#if uploadedFiles.length > 0}
              {#each uploadedFiles as file}
                <option>{file.name}</option>
              {/each}
            {/if}
          </select>
          <button class="btn join-item" onclick={addFileToActivePod} disabled={!selectedFileName}>
            Add File To Pod
          </button>
        </div>
        <table class="table" id="pod">
          <thead>
            <tr>
              <th>File name</th>
              <th>File size</th>
              <th>File type</th>
              <th>File Metadata</th>
              <th>Operations</th>
            </tr>
          </thead>
          <tbody>
            {#if activePod?.fileObjs && activePod.fileObjs.length > 0}
              {#each activePod.fileObjs as file}
                <tr>
                  <td>{file.name}</td>
                  <td>{formatFileSize(file.fileSize)}</td>
                  <td>{file.extension}</td>
                  <td></td>
                  <td>
                  </td>
                </tr>
              {/each}
            {:else}
              <tr><td colspan="5" style="text-align:center;">No files in pod</td></tr>
            {/if}
          </tbody>
        </table>
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button class="btn btn-primary" onclick={() => addFilesToPod()}>Save Pod</button>
          <button class="btn btn-soft btn-error">Cancel</button>
        </form>
      </div>
    </div>
  </dialog> -->
  <dialog id="editFileMetadataModal" class="modal">
    <div class="modal-box w-5/12 max-w-xl">
      <h3 class="text-lg font-bold">Editing Metadata</h3>
      <div class="py-4" style="justify-content: center;">
        {#if editingPodItem?.type === "pod-ref"}
          <fieldset class="fieldset">
            <legend class="fieldset-legend">Pod Address</legend>
            <input type="text" class="input w-full" placeholder="some address" bind:value={podRefAddress}/>
          </fieldset>
        {:else}
          <fieldset class="fieldset">
            <legend class="fieldset-legend">File Type</legend>
            <select class="input" bind:value={activeFileType}>
              <option disabled selected value="">Select a file type</option>
              {#each availableTypes as type}
                <option value={type}>{type.charAt(0).toUpperCase() + type.slice(1)}</option>
              {/each}
            </select>

            {#each displayFields as field}
              <legend class="fieldset-legend">{field}</legend>
              <input
                type="text"
                class="input"
                placeholder={field}
                bind:value={editMetadataFields[field]}
              />
            {/each}
          </fieldset>
        {/if}
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button class="btn btn-primary" type="button" onclick={saveMetaDataToItem}>Save</button>
          <button class="btn btn-soft btn-error">Cancel</button>
        </form>
      </div>
    </div>
  </dialog>
  <dialog id="addPodRefModal" class="modal">
    <div class="modal-box w-5/12 max-w-xl">
      <h3 class="text-lg font-bold">Add Pod Reference</h3>
      <div class="py-4" style="justify-content: center;">
        <fieldset class="fieldset">
          <legend class="fieldset-legend">Pod Address</legend>
          <input type="text" class="input w-full" placeholder="some address" bind:value={podRefAddress}/>
        </fieldset>
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button class="btn btn-neutral" onclick={() => {addPodReference()}}>Add</button>
          <button class="btn btn-soft btn-error" onclick={()=>{podRefAddress=""}}>Cancel</button>
        </form>
      </div>
    </div>
  </dialog>
</main>

<style>
  .tooltip[data-tip]::before,
  .tooltip.tooltip-open[data-tip]::before {
    max-width: 75rem !important;
    min-width: 16rem;
    white-space: pre-wrap !important;
    font-family: monospace !important;
    z-index: 100;
  }
  .address-tooltip {
    transition: color 0.15s;
  }
  .address-tooltip:hover, .address-tooltip:focus {
    color: #009799;
    text-decoration-style: solid;
  }
.utility-bar {
  display: flex;
  align-items: center;
  gap: 5px;
}

.item {
  justify-content: space-between;
  padding: 3px;
  background-color: #f2f2f2;
  border: 1px solid #d9d9d9;
  cursor: grab;
  user-select: none;
  width: 100%;
}

.item-disabled {
  color: #aaa;
  background-color: #eee;
  cursor: not-allowed;
}

.item-selected {
  background-color: #f9c7c8 !important;
  border: solid red 1px !important;
  z-index: 1 !important;
}

.item-container {
  min-width: 300px;
  max-width: 100%; /* Allow it to expand based on available space */
  min-height: 300px;
  height: 300px;
  overflow-y: auto;
  overflow-x: scroll;
  border: 2px solid #ccc;
  margin: 10px;
}

@media (min-width: 768px) {
  .item-container {
    max-width: 300px; /* Adjust as necessary for wider screens */
  }
}

@media (min-width: 1024px) {
  .item-container {
    max-width: 400px; /* Further expand on even wider screens */
  }
}

.row {
  display: flex;
  flex-direction: column;
  margin-left: 2%;
  margin-right: 2%;
  /* padding-top: 2vh; */
  padding-bottom: 2vh;
  /* justify-content: center; */
}

.table th:last-child,
.table td:last-child {
  min-width: 170px;
}

@media (prefers-color-scheme: dark) {

  .item {
    background-color: #333;
    border-color: #666;
    color: #ddd;
  }

  .item-container {
    border-color: #666;
  }

  .modal-box {
    background-color: #1e1e1e;
    color: #f0f0f0;
  }

  .item-selected {
    background-color: #e0282b !important;
    color: black;
    border: solid red 1px !important;
    z-index: 1 !important;
  }
}

</style>
