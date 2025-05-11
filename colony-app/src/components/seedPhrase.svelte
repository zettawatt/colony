<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  export async function generate(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    let seed = await invoke("get_new_seed_phrase");
    console.log(seed)
  }
</script>

<div class="overflow-x-auto rounded-box border border-base-content/5 bg-base-100">
  <table class="table">
    <!-- head -->
    <tbody>
      <!-- row 1 -->
      <tr>
        <td><input type="text" class="input input-primary" value="temp1" id="1" /></td>
        <td><input type="text" class="input input-primary" value="temp2" id="2" /></td>
        <td><input type="text" class="input input-primary" value="temp3" id="3" /></td>
        <td><input type="text" class="input input-primary" value="temp4" id="4" /></td>
      </tr>
      <!-- row 2 -->
      <tr>
        <td><input type="text" class="input input-primary" value="temp5" id="5" /></td>
        <td><input type="text" class="input input-primary" value="temp6" id="6" /></td>
        <td><input type="text" class="input input-primary" value="temp7" id="7" /></td>
        <td><input type="text" class="input input-primary" value="temp8" id="8" /></td>
      </tr>
      <!-- row 3 -->
      <tr>
        <td><input type="text" class="input input-primary" value="temp9" id="9" /></td>
        <td><input type="text" class="input input-primary" value="temp10" id="10" /></td>
        <td><input type="text" class="input input-primary" value="temp11" id="11" /></td>
        <td><input type="text" class="input input-primary" value="temp12" id="12" /></td>
      </tr>
    </tbody>
  </table>
</div>