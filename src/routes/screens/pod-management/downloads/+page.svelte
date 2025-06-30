<script lang="ts">
  import Drawer from "../../../../components/drawer.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { addToast } from "../../../../stores/toast";
  import { FileObj, type FileInfo } from "../../../../classes/FileObj";
  import ps from "../../../../stores/persistantStorage";
  import { formatFileSize, totalFileSizeCounter } from "../../../../utils/fileFormaters";
  import { handleCopyAddress } from "../../../../utils/copyAutonomiAddress";
  import { onMount } from "svelte";
  import { downloadManager } from "../../../../stores/downloadManager";

  // $: downloads = Object.values($downloadManager);

  let fileObjs: FileObj[] = [];
  let downloadedFiles = $state<FileObj[]>([]);

  async function downloadFile() {
    const downloadDir = await ps.getDownloadDir();
    const dummyFilename = "download_test_pdf_10mb.txt";
    const dummyAddress = "51839d5f9fbf79d1b9c267508613f2c69299ad6ce93213756867c776d5f8c625";
    const request = {
      address: dummyAddress,
      destination_path: `${downloadDir}/${dummyFilename}`
    };
    try {
      const msg = await invoke<string>('download_data', { request });
      console.log(msg);
      addToast(msg, "info");
      const newFileObj = new FileObj(
        {
          name: dummyFilename,
          path: downloadDir,
          autonomiAddress: dummyAddress,
          extension: ".txt",
          fileSize: 64,
          downloadPath: downloadDir
        }
      );
      console.log("downloaded file", newFileObj)
      await ps.addDownloadedFile(newFileObj);
      await loadTable()
    } catch (err) {
      console.error("Download failed", err);
      addToast(String(err), "error");
    }
  }

  function updateTotalDownloadedCounter() {
    let formatted = totalFileSizeCounter(downloadedFiles)
    const el = document.getElementById("totalUploadedCounter");
    if (el) el.innerText = formatted;
  }

  async function loadTable() {
    downloadedFiles = await ps.getDownloadedFilesArray();
    updateTotalDownloadedCounter()
    console.log(downloadedFiles)
  }

  function resetDownloadState() {
    selectedPath = "";
    selectedFileName = "";
    downloadCost = "";
    wasDownloadCanceled = true;
  }

  onMount(async () => {
    await loadTable()
  })

</script>

<main>
  <Drawer>
    <div slot="main">
      <!-- Your primary page content, e.g., the "Your Pods" table and modals -->
      <!-- ... main content ... -->
      <div class="row" style="display: flex; flex-direction: row; justify-content: space-between; align-items: center; padding-top:4vh;">
        <h2 class="h2">Downloads</h2>
        <div class="utility-bar" style="display: flex; align-items: center; gap: 1rem;">
          <div class="download-info">
            <p style="margin: 0;" id="totalDownloadedCounter">0.0 B</p>
            <p style="margin: 0;">downloaded</p>
          </div>
          <button class="btn btn-secondary" onclick={downloadFile()}>Download File Test</button>
        </div>
      </div>
            <div class="row">
        <div class="card bg-base-100 w-96 shadow-lg card-xl" style="width: 100%;">
          <div class="card-body items-center text-center p-4">
            <!-- <h2 class="card-title h2">Your Pods</h2> -->
            <table class="table table-zebra" id="downloadsTable">
              <thead>
                <tr>
                  <th></th>
                  <th>Name</th>
                  <th>From Address</th>
                  <th>Size</th>
                  <th>Downloaded Date</th>
                  <th>Download Directory</th>
                </tr>
              </thead>
              <tbody>
                {#if downloadedFiles.length > 0}
                  {#each downloadedFiles as file, idx}
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
                      <td>{formatFileSize(file.fileSize)}</td>
                      <td>{file.downloadedDate}</td>
                      <td>{file.downloadPath}</td>
                    </tr>
                  {/each}
                {:else}
                  <tr>
                    <td colspan="12" style="text-align:center;">No downloads yet</td>
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
      <li><a href="/screens/pod-management/uploads">Uploads</a></li>
      <li><a href="#" class="menu-active">Downloads</a></li>
    </ul>
  </Drawer>
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
  .download-info {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
</style>