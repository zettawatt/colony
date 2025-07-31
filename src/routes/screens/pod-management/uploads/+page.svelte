<script lang="ts">
  import Drawer from "../../../../components/drawer.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from '@tauri-apps/plugin-dialog';
  import { addToast }  from '../../../../stores/toast';
  import ps from "../../../../stores/persistantStorage";
  import { FileObj, type FileInfo } from "../../../../classes/FileObj";
  import { onMount } from "svelte";
  import { formatFileSize, totalFileSizeCounter } from "../../../../utils/fileFormaters";
  import { handleCopyAddress } from "../../../../utils/copyAutonomiAddress";
  import AddressDisplay from "../../../../components/AddressDisplay.svelte";

  let fileObjs: FileObj[] = [];
  let stagedFileObj = $state<FileObj | null>(null);
  let workingFileObj: FileInfo | undefined;
  let uploadNewFileModal: HTMLDialogElement;
  let isPreviewLoading = $state(false);
  let selectedPath = $state("");
  let selectedFileName = $state("");
  let uploadCost = $state("");
  let wasUploadCanceled = $state(false)
  let uploadedFiles = $state<FileObj[]>([]);


  async function selectFile() {
    resetUploadState();

    const filePath = await open({ multiple: false, directory: false });
    if (filePath) {
      isPreviewLoading = true;
      const fileSize = await invoke('get_file_size', { path: filePath }) as number;
      const name = filePath.split(/[/\\]/).pop() || "";
      const ext = name.split('.').pop() || "";

      stagedFileObj = new FileObj({
        name,
        path: filePath,
        extension: ext,
        previewCost: "", // optional
        fileSize,
      });

      selectedPath = filePath;
      selectedFileName = name;
      isPreviewLoading = false;
    }
  }

  async function uploadPreview() {
    let uploadCostResult = "";
    if (selectedPath) {
      try {
        uploadCostResult = await invoke('upload_cost', {
          request: { file_path: selectedPath }
        });
      } catch (e) {
        console.log(JSON.stringify(e))
        uploadCostResult = 'Error: ' + e;
      }
    }
    console.log('uploadCostRes', uploadCostResult)
    return `Estimated upload cost ${uploadCostResult} ANT`;
  }

  async function uploadFile(fileObj: FileObj) {
    addToast(`Uploading ${fileObj.name}...`, "info", 7000);
    try {
      const result = await invoke('upload_data', {
        request: {
          id: fileObj.uuid,
          file_path: fileObj.path
        }
      }) as [any, string];
      const [uploadResult, address] = result;

      fileObj.setActualCost(uploadResult);
      fileObj.setAutonomiAddress(address);
      await ps.addUploadedFileObj(fileObj);

      addToast(`Uploaded ${fileObj.name} to ${fileObj.autonomiAddress}`, "success");
      await loadTable();
    } catch (e) {
      console.error(`Upload error for ${fileObj.name}:`, e);
      addToast(`Failed to upload ${fileObj.name}`, "error");
    }
  }


  function resetUploadState() {
    selectedPath = "";
    selectedFileName = "";
    uploadCost = "";
    wasUploadCanceled = true;
    fileObjs = [];
    stagedFileObj = null; // clear state
  }

  function updateTotalUploadedCounter() {
    let formatted = totalFileSizeCounter(uploadedFiles)
    const el = document.getElementById("totalUploadedCounter");
    if (el) el.innerText = formatted;
  }

  async function loadTable() {
    const uploadedFileObjs = await ps.getUploadedFilesArray();
    uploadedFiles = uploadedFileObjs;
    updateTotalUploadedCounter()
    console.log(uploadedFiles)
  }

  onMount(async () => {
    await loadTable();
  })

</script>

<main style="height: 100%; display: flex; flex-direction: column; overflow: hidden;">
  <Drawer>
    <div slot="main" style="height: 100%; display: flex; flex-direction: column; overflow: hidden;">
      <!-- Your primary page content, e.g., the "Your Pods" table and modals -->
      <!-- ... main content ... -->
      <div class="row" style="display: flex; flex-direction: row; justify-content: space-between; align-items: center; padding: 20px; flex-shrink: 0;">
        <h2 class="h2">Uploads</h2>
        <div class="utility-bar" style="display: flex; align-items: center; gap: 1rem;">
          <div class="upload-info">
            <p style="margin: 0;" id="totalUploadedCounter">0.0 B</p>
            <p style="margin: 0;">uploaded</p>
          </div>
          <button class="btn btn-warning" onclick={() => uploadNewFileModal.showModal()}>Upload New File</button>
        </div>
      </div>
      <div class="row" style="flex: 1; min-height: 0; overflow: hidden;">
        <div class="card bg-base-100 w-96 shadow-lg card-xl" style="width: 100%; height: 100%; display: flex; flex-direction: column;">
          <div class="card-body items-center text-center p-4" style="flex: 1; min-height: 0; overflow: hidden; display: flex; flex-direction: column;">
            <!-- <h2 class="card-title h2">Your Pods</h2> -->
            <div style="flex: 1; min-height: 0; overflow-y: auto; width: 100%;">
              <table class="table table-zebra" id="uploadsTable">
                <thead>
                  <tr>
                    <th></th>
                    <th>Name</th>
                    <th>Upload Address</th>
                    <th>Upload Date</th>
                    <th>Size</th>
                  </tr>
                </thead>
                <tbody>
                  {#if uploadedFiles.length > 0}
                    {#each uploadedFiles as file, idx}
                      <tr>
                        <th>{idx + 1}</th>
                        <td>{file.name}</td>
                        <td>
                          <AddressDisplay address={file.autonomiAddress || ''} />
                        </td>
                        <td>{file.uploadedDate}</td>
                        <td>{formatFileSize(file.fileSize)}</td>
                      </tr>
                    {/each}
                  {:else}
                    <tr>
                      <td colspan="5" style="text-align:center;">No uploads yet</td>
                    </tr>
                  {/if}
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>
    </div>
    <ul slot="sidebar" class="menu bg-base-100 text-base-content min-h-full w-40 p-5">
      <li><a href="/screens/pod-management/your-pods">Your Pods</a></li>
      <li><span class="menu-active">Uploads</span></li>
      <li><a href="/screens/pod-management/downloads">Downloads</a></li>
    </ul>
  </Drawer>

  <dialog bind:this={uploadNewFileModal} id="uploadNewFile" class="modal">
    <div class="modal-box">
      <h3 class="text-lg font-bold">Select File for Upload</h3>
      <div class="py-2">
        <div class="join">
          <button class="btn join-item" onclick={selectFile}>Choose File</button>
          <input 
            type="text" 
            value={selectedFileName ?? "waiting for a file..."} 
            class="input" 
            disabled
            style="min-width: 100%;"
          />
        </div>
        <!-- <p id="uploadCostText" class="mt-4">{(uploadCost && !wasUploadCanceled) ? uploadCost : "Your upload cost to the Autonomi network will be shown here..."}</p> -->
        {#if selectedPath}
          <p id="uploadText" class="mt-4">{`Are you sure you want to upload your selected file to the Autonomi network?`}</p>
        {/if}
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button
            class="btn btn-primary"
            disabled={isPreviewLoading || !stagedFileObj}
            onclick={() => {
              if (stagedFileObj) {
                void uploadFile(stagedFileObj); // start upload in background
                resetUploadState();   // hide modal, reset UI
                uploadNewFileModal.close();
              }
            }}
          >
            Upload to Autonomi
          </button>
          <!-- <button class="btn btn-primary" disabled={isPreviewLoading || !selectedPath} onclick={uploadFile}>
            {#if isPreviewLoading}
              <span class="loading loading-spinner"></span> Checking price
            {:else}
              Upload to Autonomi
            {/if}
          </button> -->
          <button class="btn btn-soft btn-error" onclick={resetUploadState}>Cancel Upload</button>
        </form>
      </div>
    </div>
  </dialog>
</main>

<style>

  .row {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    margin-left: 2%;
    margin-right: 2%;
    padding-bottom: 2vh;
  }
  .utility-bar {
    display: flex;
    align-items: center;
    gap: 1rem; /* Space between utility items */
  }
  .upload-info {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
</style>