<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';


  let name = $state("");
  let greetMsg = $state("");
  let downloadPath = $state("");

  async function toast() {
    let permissionGranted = await isPermissionGranted();
    console.log('here', permissionGranted)
    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === 'granted';
    }
    if (permissionGranted) {
      sendNotification('Tauri is awesome!');
      sendNotification({ title: 'TAURI', body: 'Tauri is awesome!' });
    }
  }

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<main>
  <!-- <div class="row">
    <h2 class="h2" style="text-align: center;">Your Pods</h2>
  </div> -->
  <div class="drawer drawer-open" style="padding-top:1vh;">
    <input id="my-drawer-2" type="checkbox" class="drawer-toggle" />
    <div class="drawer-content flex flex-col">
      <div class="row" style="display: flex; flex-direction: row; justify-content: space-between; padding-top:4vh;">
        <h2 class="h2">Your Pods</h2>
        <button class="btn btn-warning" onclick={createNewPodModal.showModal()}>Create New Pod</button>
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
                <tr>
                  <th>1</th>
                  <td>Cy Ganderton</td>
                  <td>Quality Control Specialist</td>
                  <td>Blue</td>
                  <td>Blue</td>
                  <td>
                    <button class="btn btn-accent" onclick={uploadPodModal.showModal()}>u</button>
                    <button class="btn btn-warning">e</button>
                    <button class="btn btn-error" onclick={deletePodModal.showModal()}>d</button>
                  </td>
                </tr>
                <tr>
                  <th>2</th>
                  <td>Hart Hagerty</td>
                  <td>Desktop Support Technician</td>
                  <td>Purple</td>
                  <td>Blue</td>
                  <td>
                    <button class="btn btn-disabled btn-accent">u</button>
                    <button class="btn btn-warning">e</button>
                    <button class="btn btn-error">d</button>
                  </td>
                </tr>
                <tr>
                  <th>3</th>
                  <td>Brice Swyre</td>
                  <td>Tax Accountant</td>
                  <td>Red</td>
                  <td>Blue</td>
                  <td>
                    <button class="btn btn-accent">u</button>
                    <button class="btn btn-warning">e</button>
                    <button class="btn btn-error">d</button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
      <!-- Page content here -->
      <!-- <label for="my-drawer-2" class="btn btn-primary drawer-button lg:hidden">
        Open drawer
      </label> -->
    </div>
    <div class="drawer-side" style="height: 90vh;">
      <label for="my-drawer-2" aria-label="close sidebar" class="drawer-overlay"></label>
      <ul class="menu bg-base-100 text-base-content min-h-full w-40 p-5">
        <!-- Sidebar content here -->
        <li><a>Your Pods</a></li>
        <li><a>Uploads</a></li>
        <li><a>Downloads</a></li>
      </ul>
    </div>
  </div>
  <dialog id="createNewPodModal" class="modal">
    <div class="modal-box">
      <h3 class="text-lg font-bold">Create New Pod</h3>
      <div class="py-4">
        <input type="text" placeholder="Please enter a name for your pod..." class="input" />
      </div>
      <div class="modal-action">
        <form method="dialog">
          <!-- if there is a button in form, it will close the modal -->
          <button class="btn btn-primary">Create</button>
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
      <div class="modal-action modal-backdrop">
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
      <div class="modal-action modal-backdrop">
        <form method="dialog">
          <button class="btn btn-error">Delete</button>
          <button class="btn btn-soft btn-error">Cancel</button>
        </form>
      </div>
    </div>
  </dialog>
</main>

<style>

.user-pods-container {
  text-align: center;
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.svelte-kit:hover {
  filter: drop-shadow(0 0 2em #ff3e00);
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
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

</style>
