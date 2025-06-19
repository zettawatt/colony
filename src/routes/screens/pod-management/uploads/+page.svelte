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

  let fileObjs: FileObj[] = [];
  let workingFileObj: FileInfo | undefined;
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
      const fileSize = await invoke('get_file_size', { path: filePath }) as number;
      isPreviewLoading = true;
      wasUploadCanceled = false;
      selectedPath = filePath;
      selectedFileName = filePath.split(/[/\\]/).pop() || "";
      let selectedFileExtension = selectedFileName.split('.').pop() || "";
      console.log('Selected file path:', selectedPath);
      uploadCost = await uploadPreview(); // Trigger upload cost preview
      isPreviewLoading = false;

      // TODO: in the future, when we let users select multiple files for upload
      const newFileObj = new FileObj(
        {
          name: selectedFileName,
          path: filePath,
          extension: selectedFileExtension,
          previewCost: uploadCost,
          fileSize: fileSize
        }
      );
      fileObjs.push(newFileObj);
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

  async function uploadFile() {
    let uploadResult = "";
    let address = "";
    if (selectedPath) {
      addToast("uploading file to the network...", "info", 7000)
      try {
        [uploadResult, address] = await invoke('upload_data', {
          request: { file_path: selectedPath }
        });
        if (fileObjs[0]) {
          fileObjs[0].setActualCost(uploadResult);
          fileObjs[0].setAutonomiAddress(address);
          await ps.addUploadedFileObj(fileObjs[0]);
          addToast(`Uploaded ${fileObjs[0].name} at address ${fileObjs[0].autonomiAddress}`, "success")
        }
        console.log(fileObjs[0].toJSON())
        loadTable();
        resetUploadState();
      } catch (e) {
        uploadResult = 'Error: ' + e;
      }
    }
    console.log('uploadResult', uploadResult)
  }

  function resetUploadState() {
    selectedPath = "";
    selectedFileName = "";
    uploadCost = "";
    wasUploadCanceled = true;
    fileObjs = [];
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

<main>
  <Drawer>
    <div slot="main">
      <!-- Your primary page content, e.g., the "Your Pods" table and modals -->
      <!-- ... main content ... -->
      <div class="row" style="display: flex; flex-direction: row; justify-content: space-between; align-items: center; padding-top:4vh;">
        <h2 class="h2">Uploads</h2>
        <div class="utility-bar" style="display: flex; align-items: center; gap: 1rem;">
          <div class="upload-info">
            <p style="margin: 0;" id="totalUploadedCounter">0.0 B</p>
            <p style="margin: 0;">uploaded</p>
          </div>
          <button class="btn btn-warning" onclick={uploadNewFile.showModal()}>Upload New File</button>
        </div>
      </div>
            <div class="row">
        <div class="card bg-base-100 w-96 shadow-lg card-xl" style="width: 100%;">
          <div class="card-body items-center text-center p-4">
            <!-- <h2 class="card-title h2">Your Pods</h2> -->
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
                        <div class="tooltip tooltip-warning" data-tip={file.autonomiAddress}>
                          <button
                            class="address-tooltip"
                            data-address={file.autonomiAddress}
                            onclick={handleCopyAddress}
                            tabindex="0"
                            style="cursor: pointer; font-style: italic; text-decoration: underline dotted;"
                          >autonomi address</button>
                        </div>
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
    <ul slot="sidebar" class="menu bg-base-100 text-base-content min-h-full w-40 p-5">
      <li><a href="/screens/pod-management/your-pods">Your Pods</a></li>
      <li><a href="#"class="menu-active">Uploads</a></li>
      <li><a href="/screens/pod-management/downloads">Downloads</a></li>
    </ul>
  </Drawer>

  <dialog id="uploadNewFile" class="modal">
    <div class="modal-box">
      <h3 class="text-lg font-bold">Select File for Upload</h3>
      <div class="py-2">
        <div class="join join-vertical lg:join-horizontal">
          <button class="btn join-item" onclick={selectFile}>Choose File</button>
          <input 
            type="text" 
            value={selectedFileName ?? "waiting for a file..."} 
            class="input" 
            disabled
            style="min-width: 100%;"
          />
        </div>
        <p id="uploadCostText" class="mt-4">{(uploadCost && !wasUploadCanceled) ? uploadCost : "Your upload cost to the Autonomi network will be shown here..."}</p>
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button class="btn btn-primary" disabled={isPreviewLoading || !selectedPath} onclick={uploadFile}>
            {#if isPreviewLoading}
              <span class="loading loading-spinner"></span> Checking price
            {:else}
              Upload to Autonomi
            {/if}
          </button>
          <button class="btn btn-soft btn-error" onclick={resetUploadState}>Cancel Upload</button>
        </form>
      </div>
    </div>
  </dialog>
</main>

<style>
  .tooltip[data-tip]::before,
  .tooltip.tooltip-open[data-tip]::before {
    max-width: 50rem !important;
    min-width: 16rem;
    white-space: pre-wrap !important;
    font-family: monospace !important;
  }
  .address-tooltip {
    transition: color 0.15s;
  }
  .address-tooltip:hover, .address-tooltip:focus {
    color: #009799;
    text-decoration-style: solid;
  }
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