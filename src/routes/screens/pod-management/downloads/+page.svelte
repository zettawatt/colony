<script lang="ts">
  import Drawer from "../../../../components/drawer.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { open } from '@tauri-apps/plugin-dialog';

  let isPreviewLoading = $state(false);
  let selectedPath = $state("");
  let selectedFileName = $state();
  let uploadCost = $state("");
  let wasUploadCanceled = $state(false)

  let showToast = $state(false);
  let toastMessage = $state("");
  let toastType = $state("info"); // can be "info", "success", "error", etc.

  function handleShowToast(msg: string, type = "info") {
    toastMessage = msg;
    toastType = type;
    showToast = true;
    // setTimeout(() => (showToast = false), 5000);
  }

  async function downloadFile() {
    const request = {
      address: "b4108849f2562b7580b48225f11eda9f35cdf44d6dedfa75ac900d3d7bfc4f4d", // hex address
      destination_path: "/Users/maxxrodriguez/Downloads/temp.txt"
    };
    try {
      const msg = await invoke<string>('download_data', { request });
      console.log(msg);
      handleShowToast(msg, "primary");
    } catch (err) {
      console.error("Download failed", err);
      handleShowToast(String(err), "error");
    }
  }

  async function selectFile() {
    const filePath = await open({ multiple: false });
    if (filePath) {
      isPreviewLoading = true;
      wasUploadCanceled = false;
      selectedPath = filePath;
      selectedFileName = filePath.split(/[/\\]/).pop() || "";
      console.log('Selected file path:', selectedPath);
      uploadCost = await uploadPreview(); // Trigger upload cost preview
      isPreviewLoading = false;
      // Optionally, send to the backend immediately
      // await invoke('process_file', { path: selectedPath });
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
    if (selectedPath) {
      try {
        uploadResult = await invoke('upload_data', {
          request: { file_path: selectedPath }
        });
        handleShowToast(uploadResult, "primary");
      } catch (e) {
        handleShowToast(String(e), "error");
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
  }

  // ... rest of script ...
</script>

<main>
  {#if showToast}
    <div class="toast toast-end toast-bottom" style="max-width: 50%;">
      <div class={"alert alert-" + toastType} onclick={() => (showToast = false)}>
        <span>{toastMessage}</span>
      </div>
    </div>
  {/if}
  <Drawer>
    <div slot="main">
      <!-- Your primary page content, e.g., the "Your Pods" table and modals -->
      <!-- ... main content ... -->
      <div class="row" style="display: flex; flex-direction: row; justify-content: space-between; align-items: center; padding-top:4vh;">
        <h2 class="h2">Downloads</h2>
        <div class="utility-bar" style="display: flex; align-items: center; gap: 1rem;">
          <div class="upload-info">
            <p style="margin: 0;" id="totalUplaodedCounter">511.8 MB</p>
            <p style="margin: 0;">downloaded</p>
          </div>
          <button class="btn btn-secondary" onclick={downloadFile()}>Download File Test</button>
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
                  <th>From Address</th>
                  <th>Size</th>
                  <th>Download Directory</th>
                </tr>
              </thead>
              <tbody>
                <tr>
                  <th>1</th>
                  <td>rust_cheat_sheet.pdf</td>
                  <td>347eec6a3f571eda37252fbe539a9be7d46b73f5dedeadeb069b601df70adb67</td>
                  <td>824 KB</td>
                  <td>/Users/maxxrodriguez/Documents/Reports/</td>
                </tr>
                <tr>
                  <th>2</th>
                  <td>travel_photos.zip</td>
                  <td>45c93a4c0323d80f05f5c5103e431af0b9e8e4c405ff1b52aa9d6b30622a8efd</td>
                  <td>120.6 MB</td>
                  <td>/Users/maxxrodriguez/Downloads/Photos/</td>
                </tr>
                <tr>
                  <th>3</th>
                  <td>music_album.flac</td>
                  <td>f456f7dbb8884409f98558d31325f0e09496c86dc650a74b0696663b222b817b</td>
                  <td>390.4 MB</td>
                  <td>/Users/maxxrodriguez/Music/</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
    <ul slot="sidebar" class="menu bg-base-100 text-base-content min-h-full w-40 p-5">
      <li><a href="/screens/pod-management/your-pods">Your Pods</a></li>
      <li><a href="/screens/pod-management/uploads">Uploads</a></li>
      <li><a href="#" class="menu-active">Downloads</a></li>
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