<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<main class="wallet-container">
      <div class="row">
        <div class="card bg-base-100 w-96 shadow-lg card-xl" style="width: auto;">
          <div class="card-body items-center text-center p-4">
            <!-- <h2 class="card-title h2">Your Pods</h2> -->
            <table class="table table-zebra">
              <thead>
                <tr>
                  <th></th>
                  <th>Wallet Name</th>
                  <th>Wallet Address</th>
                  <th>Balance</th>
                  <th>Operations</th>
                </tr>
              </thead>
              <tbody>
                <tr>
                  <th>1</th>
                  <td>Main Wallet</td>
                  <td>0xAbC1234ef5678901234567890abcdef123456789</td>
                  <td>
                    <div class="block">ETH: 1.845</div>
                    <div class="block">ANT: 150</div>
                  </td>
                  <td>
                    <button class="btn btn-accent" onclick={uploadPodModal.showModal()}>u</button>
                    <button class="btn btn-error" onclick={deletePodModal.showModal()}>d</button>
                  </td>
                </tr>
                <tr>
                  <th>2</th>
                  <td>Second Wallet</td>
                  <td>0x4567ab98ef12345cd67890abC123400987654321</td>
                  <td>
                    <div class="block">ETH: 0.0734</div>
                    <div class="block">ANT: 0</div>
                  </td>
                  <td>
                    <button class="btn btn-disabled btn-accent">u</button>
                    <button class="btn btn-error">d</button>
                  </td>
                </tr>
                <tr>
                  <th>3</th>
                  <td>Trading Wallet</td>
                  <td>0x22ab3344Ef8898aCde44f3321B99887766554432</td>
                  <td>
                    <div class="block">ETH: 0.083</div>
                    <div class="block">ANT: 5420</div>
                  </td>
                  <td>
                    <button class="btn btn-accent">u</button>
                    <button class="btn btn-error">d</button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
</main>

<style>
.wallet-container {
  /* margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column; */
  padding-top: 3vh;
  justify-content: center;
  text-align: center;
  overflow-y: auto;
  /* Set a fixed max-width that accounts for scrollbar */
  /* max-width: 64rem;
  width: 100%;
  box-sizing: border-box; */
}

.input {
  width: 60%;
  max-width: 600px;
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
  overflow-y: auto;
  /* Set a fixed max-width that accounts for scrollbar */
  max-width: 64rem;
  width: 100%;
  box-sizing: border-box;
}

/* Remove the responsive breakpoints that cause the shift */
@media (width >= 48rem) {
  .container {
    max-width: 64rem; /* Keep the same max-width as the larger breakpoint */
  }
}

/* .logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
} */

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

/* button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
} */

#greet-input {
  margin-right: 5px;
}

</style>
