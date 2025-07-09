<script lang="ts">
  import Drawer from "../../../../components/drawer.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { addToast }  from '../../../../stores/toast';
  import { onMount } from "svelte";
  import { FileObj, type FileInfo } from "../../../../classes/FileObj";
  import ps from "../../../../stores/persistantStorage";
  import { handleCopyAddress } from "../../../../utils/copyAutonomiAddress";
  import { getPassword } from "../../../../utils/password/session";
  import { podsSyncing } from "../../../../stores/globals";

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
  let activePod = $state<any>(null); // Holds the pod for the currently active modal
  let uploadedFiles = $state<FileObj[]>([]);
  let selectedFileName = $state(""); // <-- Track the filename selected for adding

  async function addFilesToPod() {
    const files = activePod.fileObjs as FileObj[];
    if (files) {
      try {
        for(let file of files) {
          const fileMetaJson = {
            "@context": { "schema": "http://schema.org/" },
            "@type": "schema:MediaObject",
            "@id": `ant://${file.autonomiAddress}`,
            "schema:name": file.name,
            "schema:description": "",
            "schema:contentSize": file.fileSize
          };
          console.log({
            pod_address: activePod.address,
            subject_address: file.autonomiAddress,
            data: JSON.stringify(fileMetaJson)
          })
          const result = await invoke<string>('put_subject_data', {request: {
            pod_address: activePod.address,
            subject_address: file.autonomiAddress,
            data: JSON.stringify(fileMetaJson)
          }});
          addToast(`Successfilly added ${file.name} to pod!`, "success")
          console.log(result)
        }        
      } catch (error) {
        console.error(error) 
      }
    }
  }

  async function uploadAllPods() {
    try {
      const result = await invoke<string>('upload_all');
      addToast(result, "success");
      console.log(result); // "Successfully uploaded all updated pods to Autonomi"
    } catch (error) {
      console.error('Upload failed:', error);
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
      // result will likely be { addresses: [ ..pod addresses.. ] }
      console.log(JSON.stringify(results));
      console.log(results);
      console.log('Pods:', results);
      return results
      // you can now use result.addresses in your UI
    } catch (e) {
      console.error('Failed to fetch pods:', e);
    }
  }

  async function refreshReference(depthValue: number) {
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
        const podInfo = await invoke('add_pod', { request: {name: newPodName} }) as PodInfo;
        podObj["address"] = podInfo.address;
        addToast('Pod created at address:'+ podInfo.address, "info")
        await invoke("write_keystore_to_file", {password: await getPassword()})
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
    if (fileToAdd && !activePod.fileObjs.some(f => f.name === fileToAdd.name)) {
      // Add only if not already present
      activePod.fileObjs = [...activePod.fileObjs, fileToAdd];
    }
    selectedFileName = ""; // Optionally reset selection
  }

  function formatFileSize(size: number): string {
    if (!size) return "0 B";
    const kb = 1024, mb = kb * 1024, gb = mb * 1024;
    if (size >= gb) return (size/gb).toFixed(2) + ' GB';
    if (size >= mb) return (size/mb).toFixed(2) + ' MB';
    if (size >= kb) return (size/kb).toFixed(2) + ' KB';
    return size + ' B';
  }

  async function loadTable() {
    // createdPods = await ps.getPodCache() as [];
    createdPods = await fetchPods();
    console.log(createdPods)
  }

  function makeDateReadable(date: string | undefined) {
    if (date) {
      return new Date(date).toLocaleString();
    } else {
      return;
    }
  }

  onMount(async () => {
    // await initPodManager();
    await loadTable();
    uploadedFiles = await ps.getUploadedFilesArray();
  })
</script>

<main>
  <!-- <div class="row">
    <h2 class="h2" style="text-align: center;">Your Pods</h2>
  </div> -->
  <Drawer>
    <div slot="main">
      <!-- Your primary page content, e.g., the "Your Pods" table and modals -->
      <!-- ... main content ... -->
      <div class="row" style="display: flex; flex-direction: row; justify-content: space-between; padding-top:4vh;">
        <h2 class="h2">Your Pods</h2>
        <div class="utility-bar" style="display: flex;">
          <button class="btn btn-neutral btn-soft" onclick={() => refreshReference(0)} disabled={$podsSyncing}>Sync Pods</button>
          <button class="btn btn-neutral btn-soft" onclick={() => uploadAllPods()}>Upload All Pods</button>
          <button class="btn btn-warning" onclick={createNewPodModal.showModal()}>Create New Pod</button>
        </div>
      </div>
      <div class="row">
        <div class="card bg-base-100 w-96 shadow-lg card-xl" style="width: auto;">
          <div class="card-body items-center text-center p-4">
            <!-- <h2 class="card-title h2">Your Pods</h2> -->
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
                      <!-- <td>{pod.name}</td> -->
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
                        <!-- <button 
                          class="btn btn-accent"
                          onclick={() => { activePod = pod; uploadPodModal.showModal(); }}>
                          u
                        </button> -->
                        <button 
                          class="btn btn-accent btn-square"
                          onclick={() => { activePod = pod; uploadSinglePod(); }}>
                          <img src="/app-icons/cloud-data-upload-icon.svg" alt="upload icon" width="24" height="24" />
                        </button>
                        <button 
                          class="btn btn-warning btn-square"
                          onclick={() => { activePod = pod; editPodModal.showModal(); }}>
                          <img src="/app-icons/pencil-icon.svg" alt="edit icon" width="19" height="19" />
                        </button>
                        <button 
                          class="btn btn-error btn-square"
                          onclick={() => { activePod = pod; deletePodModal.showModal(); }}>
                          <img src="/app-icons/trash-icon.svg" alt="trash icon" width="16" height="16" />
                        </button>
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
          <!-- if there is a button in form, it will close the modal -->
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
          <button class="btn btn-error">Delete</button>
          <button class="btn btn-soft btn-error">Cancel</button>
        </form>
      </div>
    </div>
  </dialog>
  <dialog id="editPodModal" class="modal">
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
                    <!-- operations per file -->
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
  </dialog>
  <dialog id="editFileMetadataModal" class="modal">
    <div class="modal-box w-5/12 max-w-xl">
      <h3 class="text-lg font-bold">File Metadata</h3>
      <div class="py-4" style="justify-content: center;">
        <fieldset class="fieldset">
          <legend class="fieldset-legend">Title</legend>
          <input type="text" class="input" placeholder="Media Title" />
          <legend class="fieldset-legend">Artist</legend>
          <input type="text" class="input" placeholder="Artist" />
          <legend class="fieldset-legend">Album</legend>
          <input type="text" class="input" placeholder="Album" />
          <legend class="fieldset-legend">Release Date</legend>
          <input type="text" class="input" placeholder="mm/dd/yyyy" />
        </fieldset>
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button class="btn btn-error">Delete</button>
          <button class="btn btn-soft btn-error">Cancel</button>
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
  gap: 5px; /* Space between utility items */
}
.upload-info {
  display: flex;
  flex-direction: column;
  align-items: center;
}
.user-pods-container {
  text-align: center;
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.svelte-kit:hover {
  filter: drop-shadow(0 0 2em #ff3e00);
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
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
  flex-direction: column;
  margin-left: 2%;
  margin-right: 2%;
  /* padding-top: 2vh; */
  padding-bottom: 2vh;
  /* justify-content: center; */
}


#greet-input {
  margin-right: 5px;
}

.table th:last-child,
.table td:last-child {
  min-width: 170px; /* Adjust to your desired minimum width */
}

</style>
