<script lang="ts">
  import Drawer from "../../../../components/drawer.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { addToast } from "../../../../stores/toast";
  import { FileObj, type FileInfo } from "../../../../classes/FileObj";
  import ps from "../../../../stores/persistantStorage";
  import { formatFileSize, totalFileSizeCounter } from "../../../../utils/fileFormaters";
  import { handleCopyAddress } from "../../../../utils/copyAutonomiAddress";
  import { onMount } from "svelte";
  import AddressDisplay from "../../../../components/AddressDisplay.svelte";
  import { v4 as uuidv4 } from 'uuid';
  import { openPath } from '@tauri-apps/plugin-opener';
  import { isMobile } from '../../../../utils/responsive.js';

  // $: downloads = Object.values($downloadManager);

  let fileObjs: FileObj[] = [];
  let downloadedFiles = $state<FileObj[]>([]);

  async function downloadFile(dummyAddress: string) {
    const downloadDir = await ps.getDownloadDir();
    const dummyFilename = dummyAddress+".txt";
    // const dummyAddress = "51839d5f9fbf79d1b9c267508613f2c69299ad6ce93213756867c776d5f8c625";
    const request = {
      id: uuidv4(),
      address: dummyAddress,
      destination_path: `${downloadDir}/${dummyFilename}`,
      size: 64
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
    const el = document.getElementById("totalDownloadedCounter");
    if (el) el.innerText = formatted;
  }

  async function loadTable() {
    downloadedFiles = await ps.getDownloadedFilesArray();
    updateTotalDownloadedCounter()
  }

  async function openFile(file: FileObj) {
    try {
      // Construct the full file path
      const fullPath = `${file.downloadPath}/${file.name}`;
      await openPath(fullPath);
      addToast(`Opened ${file.name}`, "info");
    } catch (err) {
      console.error("Failed to open file", err);
      addToast(`Failed to open ${file.name}: ${String(err)}`, "error");
    }
  }

  onMount(async () => {
    await loadTable()
  })

</script>

<main style="height: 100%; display: flex; flex-direction: column; overflow: hidden;">
  <Drawer>
    <div slot="main" style="height: 100%; display: flex; flex-direction: column; overflow: hidden;">
      <div class="row" style="display: flex; flex-direction: row; justify-content: space-between; align-items: center; padding: 20px; flex-shrink: 0;">
        <h2 class="h2">Downloads</h2>
        <div class="utility-bar" style="display: flex; align-items: center; gap: 1rem;">
          <div class="download-info">
            <p style="margin: 0;" id="totalDownloadedCounter">0.0 B</p>
            <p style="margin: 0;">downloaded</p>
          </div>
        </div>
      </div>
      <div class="row" style="flex: 1; min-height: 0; overflow: hidden;">
        <div class="card bg-base-100 w-96 shadow-lg card-xl" style="width: 100%; height: 100%; display: flex; flex-direction: column;">
          <div class="card-body items-center text-center p-4" style="flex: 1; min-height: 0; overflow: hidden; display: flex; flex-direction: column;">
            <div style="flex: 1; min-height: 0; overflow-y: auto; width: 100%;">
              <table class="table table-zebra" id="downloadsTable">
                <thead>
                  <tr>
                    {#if !$isMobile}
                      <th></th>
                    {/if}
                    <th>Name</th>
                    <th>From Address</th>
                    <th>Size</th>
                    {#if !$isMobile}
                      <th>Downloaded Date</th>
                      <th>Download Directory</th>
                    {/if}
                  </tr>
                </thead>
                <tbody>
                  {#if downloadedFiles.length > 0}
                    {#each downloadedFiles as file, idx}
                      <tr>
                        {#if !$isMobile}
                          <th>{idx + 1}</th>
                        {/if}
                        <td>
                          <button
                            class="file-name-button"
                            onclick={() => openFile(file)}
                            tabindex="0"
                            title="Click to open file"
                          >
                            {file.name}
                          </button>
                        </td>
                        <td>
                          <AddressDisplay address={file.autonomiAddress || ''} />
                        </td>
                        <td>{formatFileSize(file.fileSize)}</td>
                        {#if !$isMobile}
                          <td>{file.downloadedDate}</td>
                          <td>{file.downloadPath}</td>
                        {/if}
                      </tr>
                    {/each}
                  {:else}
                    <tr>
                      <td colspan={$isMobile ? 3 : 6} style="text-align:center;">No downloads yet</td>
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
      <li><a href="/screens/pod-management/uploads">Uploads</a></li>
      <li><a href="/screens/pod-management/downloads" class="menu-active">Downloads</a></li>
    </ul>
  </Drawer>
</main>

<style>

  .file-name-button {
    background: none;
    border: none;
    padding: 0;
    font: inherit;
    cursor: pointer;
    color: #009799;
    text-decoration: underline dotted;
    transition: all 0.15s;
    text-align: left;
    width: 100%;
  }
  .file-name-button:hover, .file-name-button:focus {
    color: #007577;
    text-decoration-style: solid;
    background-color: rgba(0, 151, 153, 0.1);
    border-radius: 4px;
    padding: 2px 4px;
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