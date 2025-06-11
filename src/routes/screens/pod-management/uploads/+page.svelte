<script lang="ts">
  import Drawer from "../../../../components/drawer.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from '@tauri-apps/plugin-dialog';
  import { addToast }  from '../../../../stores/toast';
  import ps, { type UploadedFileObj } from "../../../../stores/persistantStorage";

  let fileObjs = [];
  let workingFileObj: UploadedFileObj | undefined;

  async function copyAddress(address: string) {
    await navigator.clipboard.writeText(address);
    addToast('Copied address to clipboard!', 'success');
  }

  function handleCopyAddress(event: MouseEvent) {
    const button = event.currentTarget as HTMLButtonElement;
    console.log(button)
    const address = button.dataset.address;
    if (address) {
      copyAddress(address);
    }
  }

  let isPreviewLoading = $state(false);
  let selectedPath = $state("");
  let selectedFileName = $state("");
  let uploadCost = $state("");
  let wasUploadCanceled = $state(false)

  let showToast = $state(false);
  let toastMessage = $state("");
  let toastType = $state("info"); // can be "info", "success", "error", etc.


  async function selectFile() {
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

      workingFileObj = {
        name: selectedFileName,
        path: filePath,
        extension: selectedFileExtension,
        uploadedDate: new Date().toISOString(),
        autonomiAddress: "",
        previewCost: uploadCost,
        actualCost: "",
        fileSize: fileSize
      }
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
    return uploadCostResult;
  }

  async function uploadFile() {
    let uploadResult = "";
    let address = "";
    if (selectedPath) {
      try {
        [uploadResult, address] = await invoke('upload_data', {
          request: { file_path: selectedPath }
        });
        if (workingFileObj) {
          workingFileObj.actualCost = uploadResult;
          workingFileObj.autonomiAddress = address;
          addToast(`Uploaded ${workingFileObj.name} at address ${workingFileObj.autonomiAddress}`)
        }
        console.log(workingFileObj)
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
    workingFileObj = undefined;
  }

  // ... rest of script ...
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
            <p style="margin: 0;" id="totalUplaodedCounter">7.5 MB</p>
            <p style="margin: 0;">uploaded</p>
          </div>
          <button class="btn btn-warning" onclick={uploadNewFile.showModal()}>Upload New File</button>
        </div>
      </div>
            <div class="row">
        <div class="card bg-base-100 w-96 shadow-lg card-xl" style="width: 100%;">
          <div class="card-body items-center text-center p-4">
            <!-- <h2 class="card-title h2">Your Pods</h2> -->
            <table class="table table-zebra">
              <thead>
                <tr>
                  <th></th>
                  <th>Name</th>
                  <th>Upload Address</th>
                  <th>Size</th>
                </tr>
              </thead>
              <tbody>
                <tr>
                  <th>1</th>
                  <td>project_report.pdf</td>
                  <td>d68eae7ede9d4d4eec5e3fc0d8393e65b4fa63e649a4377118321a4fb93fd432</td>
                  <td>785 KB</td>
                </tr>
                <tr>
                  <th>2</th>
                  <td>vacation_photo.jpg</td>
                  <td>994b22801af3e033e8f422c3aa23460617ccbdcf5728cffb534ef681b803bd94</td>
                  <td>4.1 MB</td>
                </tr>
                <tr>
                  <th>3</th>
                  <td>research_data.csv</td>
                  <td>
                    <div class="tooltip tooltip-warning" data-tip="d68eae7ede9d4d4eec5e3fc0d8393e65b4fa63e649a4377118321a4fb93fd432">                      
                      <button 
                      class="address-tooltip" 
                      data-address={"d68eae7ede9d4d4eec5e3fc0d8393e65b4fa63e649a4377118321a4fb93fd432"}
                      onclick={handleCopyAddress}
                      tabindex="0"
                      style="cursor: pointer; font-style: italic; text-decoration: underline dotted;"
                      >autonomi address</button>
                    </div>
                  </td>
                  <!-- <td>
                    <div class="tooltip tooltip-open" data-tip="b6a6e5b12b497962a6b40a7a75f3167eed27218f52885787ce3c88ef4eed52ab">
                      <p>Network Address</p>
                    </div>
                  </td> -->
                  <!-- <td>b6a6e5b12b497962a6b40a7a75f3167eed27218f52885787ce3c88ef4eed52ab</td> -->
                  <td>2.6 MB</td>
                </tr>
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