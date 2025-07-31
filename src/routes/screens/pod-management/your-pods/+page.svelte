<script lang="ts">
  import Drawer from "../../../../components/drawer.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { addToast }  from '../../../../stores/toast';
  import { onMount } from "svelte";
  import { FileObj } from "../../../../classes/FileObj";
  import ps from "../../../../stores/persistantStorage";
  import { getPassword } from "../../../../utils/password/session";
  import AddressDisplay from "../../../../components/AddressDisplay.svelte";
  import { podsSyncing, allPodsUploading } from "../../../../stores/globals";
  import { v4 as uuidv4 } from 'uuid';
  import { parseSubjectData } from "../../../../utils/pod-management/parseSubjectData";
  import { templates } from "../../../../utils/pod-management/jsonLDTemplates";



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

  let selectedType = $state("Book");
  let jsonInputText = $derived(JSON.stringify((templates as any)[selectedType], null, 2));
  let isLoading = $state(false);
  let newPodName = $state("");
  let createdPods = $state<any[]>([]) as PodMetaData[];
  let activePod = $state<any>({ fileObjs: [] }); // Holds the pod for the currently active modal
  let uploadedFiles = $state<any[]>([]);
  let selectedFileName = $state(""); // <-- Track the filename selected for adding
  let userConfigPod = $state<any>();
  let podRefAddress = $state(""); // address of the pod reference user wants to add
  let autonomiFileAddress = $state(""); // address of the autonomi file user wants to add
  let editingPodItem = $state<any>();
  let deletedPodItems = $state<any[]>([]);
  let isValid = $state(false);
  let error = $state<string | null>(null);
  let parsed = $state<any>(null);

  // Advanced/Simple mode toggle
  let advancedMode = $state(false);
  let simpleTableData = $state<{predicate: string, value: string}[]>([]);
  let originalTemplate = $state<any>(null);
  let editMetadataFields = $state<Record<string, any>>({});
  let activeFileType = $state<string>("other");

  // Modal references
  let syncingInProgressModal: HTMLDialogElement;
  let uploadingInProgressModal: HTMLDialogElement;
  let addAutonomiFileModal: HTMLDialogElement;
  let editFileMetadataModal: HTMLDialogElement;
  let editPodModal: HTMLDialogElement;
  let addPodRefModal: HTMLDialogElement;
  let createNewPodModal: HTMLDialogElement;
  let deletePodModal: HTMLDialogElement;
  let syncPodsModal: HTMLDialogElement;

  // Reactive updates to sync data between modes
  let isUpdatingFromMode = false;

  // Watch for changes in simple table data and update JSON-LD
  $effect(() => {
    if (!advancedMode && !isUpdatingFromMode && simpleTableData.length > 0) {
      isUpdatingFromMode = true;
      jsonInputText = simpleTableToJsonLd(simpleTableData);
      isUpdatingFromMode = false;
    }
  });

  // Watch for changes in JSON-LD and update simple table data
  $effect(() => {
    if (advancedMode && !isUpdatingFromMode && jsonInputText) {
      try {
        isUpdatingFromMode = true;
        const newTableData = jsonLdToSimpleTable(jsonInputText);
        if (newTableData.length > 0) {
          simpleTableData = newTableData;
        }
        isUpdatingFromMode = false;
      } catch (e) {
        isUpdatingFromMode = false;
      }
    }
  });


  $effect(()=> {
    console.log('activepod', activePod)
  })

  $effect(()=> {
    // Whenever podsSyncing changes, show/hide the syncing dialog
    if ($podsSyncing) {
      syncingInProgressModal?.showModal?.();
      // Prevent closing via Escape key
      syncingInProgressModal.addEventListener('cancel', (e) => {
        e.preventDefault();
      });
    } else {
      syncingInProgressModal?.close?.();
    }
  })

  $effect(()=> {
    // Whenever allPodsUploading changes, show/hide the uploading dialog
    if ($allPodsUploading) {
      uploadingInProgressModal?.showModal?.();
      // Prevent closing via Escape key
      uploadingInProgressModal.addEventListener('cancel', (e) => {
        e.preventDefault();
      });
    } else {
      uploadingInProgressModal?.close?.();
    }
  })

  function loadTemplate(type: string): void {
    isUpdatingFromMode = true;

    selectedType = type;
    const template = { ...(templates as any)[type] }; // Create a copy to avoid modifying the original

    // Store the original template for comparison later
    originalTemplate = { ...(templates as any)[type] };

    // automatically set name and contentSize in the template
    template["schema:contentSize"] = editingPodItem.fileSize ?? "0";
    template["schema:name"] = editingPodItem.name;

    jsonInputText = JSON.stringify(template, null, 2);

    // Update simple table data
    simpleTableData = jsonLdToSimpleTable(jsonInputText);

    isValid = false;
    error = null;
    parsed = null;

    isUpdatingFromMode = false;
  }

  function validateJsonLd() {
    try {
      const obj = JSON.parse(jsonInputText);
      if (!obj["@context"] || !obj["@type"]) {
        throw new Error("Missing required @context or @type fields.");
      }
      parsed = obj;
      isValid = true;
      error = null;
    } catch (e) {
      isValid = false;
      error = (e as Error).message;
      parsed = null;
    }
  }

  // Convert JSON-LD to simple table format
  function jsonLdToSimpleTable(jsonLdText: string): any[] {
    try {
      const obj = JSON.parse(jsonLdText);
      const tableData: any[] = [];

      for (const [key, value] of Object.entries(obj)) {
        if (key === "@context") {
          // Handle @context as comma-separated list
          if (typeof value === 'object' && value !== null) {
            const contextValues = Object.entries(value).map(([k, v]) => `${k}=${v}`);
            tableData.push({
              predicate: key,
              value: contextValues.join(', ')
            });
          } else {
            tableData.push({
              predicate: key,
              value: String(value)
            });
          }
        } else {
          tableData.push({
            predicate: key,
            value: typeof value === 'object' ? JSON.stringify(value) : String(value)
          });
        }
      }

      return tableData;
    } catch (e) {
      console.error("Error converting JSON-LD to simple table:", e);
      return [];
    }
  }

  // Convert simple table format back to JSON-LD
  function simpleTableToJsonLd(tableData: any[]): string {
    const obj: any = {};

    for (const row of tableData) {
      if (row.predicate && row.predicate.trim()) {
        if (row.predicate === "@context") {
          // Handle @context conversion from comma-separated list
          if (row.value.includes('=')) {
            const contextObj: any = {};
            const pairs = row.value.split(',').map((s: string) => s.trim());
            for (const pair of pairs) {
              const equalIndex = pair.indexOf('=');
              if (equalIndex > 0) {
                const key = pair.substring(0, equalIndex).trim();
                const value = pair.substring(equalIndex + 1).trim();
                if (key && value) {
                  contextObj[key] = value;
                }
              }
            }
            obj["@context"] = contextObj;
          } else {
            obj["@context"] = row.value;
          }
        } else {
          try {
            // Try to parse as JSON first (for objects/arrays)
            obj[row.predicate] = JSON.parse(row.value);
          } catch {
            // If not valid JSON, treat as string
            obj[row.predicate] = row.value;
          }
        }
      }
    }

    return JSON.stringify(obj, null, 2);
  }

  // Toggle between advanced and simple mode
  function toggleAdvancedMode() {
    isUpdatingFromMode = true;

    if (advancedMode) {
      // Switching from advanced to simple mode
      // First validate the JSON-LD
      validateJsonLd();
      if (!isValid) {
        // Don't switch if JSON-LD is invalid
        advancedMode = true; // Keep it in advanced mode
        isUpdatingFromMode = false;
        return;
      }

      // Convert JSON-LD to simple table
      simpleTableData = jsonLdToSimpleTable(jsonInputText);
    } else {
      // Switching from simple to advanced mode
      // Convert simple table back to JSON-LD
      jsonInputText = simpleTableToJsonLd(simpleTableData);
      validateJsonLd();
    }

    isUpdatingFromMode = false;
  }

  // Add a new row to the simple table
  function addSimpleTableRow() {
    simpleTableData = [...simpleTableData, { predicate: "", value: "" }];
  }

  // Remove a row from the simple table
  function removeSimpleTableRow(index: number): void {
    simpleTableData = simpleTableData.filter((_, i) => i !== index);
  }

  async function addPodRef(podAddress: string, podRefAddress: string): Promise<any> {
    try {
      const response = await invoke('add_pod_ref', {
        request: {
          pod_address: podAddress,
          pod_ref_address: podRefAddress
        }
      });
      console.log('Pod added:', response);
      // response.address is the pod's address (from your Rust PodInfo)
      return response;
    } catch (error) {
      console.error('Error adding pod ref:', error);
      throw error;
    }
  }

  async function removePodRef(podAddress: string, podRefAddress: string): Promise<any> {
    try {
      const response = await invoke('remove_pod_ref', {
        request: {
          pod_address: podAddress,
          pod_ref_address: podRefAddress,
        }
      });
      // response is an object like: { address: 'the pod address' }
      console.log('Removed pod ref:', response);
      return response;
    } catch (error) {
      console.error('Error removing pod ref:', error);
      throw error;
    }
  }

  async function putSubjectData(podAddress: string, subjectAddress: string, data: any): Promise<any> {
    try {
      let jsonData = JSON.stringify(data);
      const result = await invoke('put_subject_data', {
        request: {
          pod_address: podAddress,
          subject_address: subjectAddress,
          data: jsonData,
        }
      });
      // `result` will be a String: "Successfully put data for subject ... in pod ..."
      console.log(result);
      return result;
    } catch (error) {
      console.error('Error putting subject data:', error);
      throw error;
    }
  }

  async function savePod() {
    console.log("savePod", activePod)

    if (activePod.fileObjs.length > 0 || deletedPodItems.length > 0) {
      for (const file of activePod.fileObjs) {
        if (file.type === 'file' && file.modified === true){
          if (("metadata" in file) && Object.keys(file.metadata).length === 0){
            file.metadata = JSON.parse(JSON.stringify(templates["Simple"]));
            if ("uploadedDate" in file && !file.isAutonomiOnly) {
              file.metadata["schema:contentSize"] = file.fileSize ?? "0";
              file.metadata["schema:name"] = file.name;
            }
          }
          if ("autonomiAddress" in file) {
            file.metadata["@id"] = `ant://${file.autonomiAddress}`;
          } else {
            addToast("File couldn't be added to pod because it's never been uploaded to the network before.", "error");
            console.error("file doesn't have an Autonomi address for some reason");
            continue;
          }
          console.log(file.metadata)
          // const metadataJson = generateFileMetaJson(file)
          // console.log({
          //   pod_address: activePod.address,
          //   subject_address: file.autonomiAddress,
          //   data: JSON.stringify(metadataJson)
          // })
          // const result = await invoke<string>('put_subject_data', {request: {
          //   pod_address: activePod.address,
          //   subject_address: file.autonomiAddress,
          //   // data: JSON.stringify({})
          //   data: JSON.stringify(metadataJson)
          // }});
          await putSubjectData(activePod.address, file.autonomiAddress, file.metadata)
          addToast(`Successfilly added ${file.name} to pod!`, "success")
        } else if (file.type === 'pod-ref'){
          await addPodRef(activePod.address, file.autonomiAddress)
          // console.log(result);
        }
      }

      // remove any deleted items
      for (const file of deletedPodItems) {
        if (file.type === 'file'){
          await putSubjectData(activePod.address, file.autonomiAddress, {})
          // addToast(`Successfilly added ${file.name} to pod!`, "success")
        } else if (file.type === 'pod-ref'){
          await removePodRef(activePod.address, file.autonomiAddress)
          // do something else to add pod reference
        }
      }
    }
  }

  async function deletePod(podName: string) {
    try {
      const response = await invoke('remove_pod', {
        request: { name: podName }
      });
      // response will be { address: podName }
      console.log('Pod removed:', response);
      return response;
    } catch (error) {
      console.error('Error removing pod:', error);
    }
  }


  function generateFileMetaJson(file: any) {
    const fileMetaJson: Record<string, any> = {
      "@context": { "schema": "http://schema.org/" },
      "@type": "",
      "@id": `ant://${file.autonomiAddress}`,
      "schema:name": file.name,
      "schema:description": "",
      "schema:contentSize": file.fileSize
    };
    switch (file.metadata.type) {
      case 'audio':
        fileMetaJson["@type"] = "schema:MusicRecording";
        fileMetaJson["schema:alternateName"] = file.metadata["Title"];
        fileMetaJson["schema:byArtist"] = file.metadata["Artist"];
        fileMetaJson["schema:inAlbum"] = file.metadata["Album"];
        fileMetaJson["schema:datePublished"] = file.metadata["Release Date"];
        fileMetaJson["schema:comment"] = file.metadata["Comment"];
        break;
      case 'video':
        fileMetaJson["@type"] = "schema:VideoObject";
        fileMetaJson["schema:alternateName"] = file.metadata["Title"];
        fileMetaJson["schema:director"] = file.metadata["Director"];
        fileMetaJson["schema:datePublished"] = file.metadata["Release Date"];
        fileMetaJson["schema:duration"] = file.metadata["Duration"];
        fileMetaJson["schema:comment"] = file.metadata["Comment"];
        break;
      case 'image':
        fileMetaJson["@type"] = "schema:ImageObject";
        fileMetaJson["schema:alternateName"] = file.metadata["Title"];
        fileMetaJson["schema:description"] = file.metadata["Description"];
        fileMetaJson["schema:dateCreated"] = file.metadata["Date Taken"];
        fileMetaJson["schema:comment"] = file.metadata["Comment"];
        break;
      case 'book':
        fileMetaJson["@type"] = "schema:Book";
        fileMetaJson["schema:alternateName"] = file.metadata["Title"];
        fileMetaJson["schema:author"] = file.metadata["Author"];
        fileMetaJson["schema:publisher"] = file.metadata["Publisher"];
        fileMetaJson["schema:datePublished"] = file.metadata["Publication Date"];
        fileMetaJson["schema:comment"] = file.metadata["Comment"];
        break;
      default:
        fileMetaJson["@type"] = "schema:CreativeWork";
        fileMetaJson["schema:alternateName"] = file.metadata["Title"];
        fileMetaJson["schema:description"] = file.metadata["Description"];
        fileMetaJson["schema:comment"] = file.metadata["Comment"];
        break;
    }
    return fileMetaJson;
  }

  async function uploadAllPods() {
    try {
      addToast("Uploading all pods to the network...", "info", 8000);
      allPodsUploading.set(true);
      const result = await invoke<string>('upload_all');
      addToast(result, "success");
      allPodsUploading.set(false);
      addToast("All pods have been uploaded!", "success");
      console.log(result); // "Successfully uploaded all updated pods to Autonomi"
    } catch (error) {
      console.error('Upload failed:', error);
      allPodsUploading.set(false);
      addToast("There was an error uploading all pods. See logs...", "error");
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
      addToast(String(err), "error");
    }
  }

  async function fetchPods() {
    try {
      const results = await invoke('list_my_pods') as any[];
      const regularPods = results.filter((pod: any) => pod.name !== "User Configuration");
      userConfigPod = results.find((pod: any) => pod.name === "User Configuration");
      // result will likely be { addresses: [ ..pod addresses.. ] }
      console.log('Pods:', results);
      console.log('user config pod', userConfigPod)
      return regularPods
    } catch (e) {
      console.error('Failed to fetch pods:', e);
    }
  }

  async function fetchPodSubjects(address: string) {
    try {
      // The name must exactly match your Rust function (snake_case)
      const result = await invoke('list_pod_subjects', { address }) as any;
      // result will be your AddressList Rust struct as a JS object
      console.log(result.addresses);
      return result.addresses;
    } catch (error) {
      // Handle error from Rust
      console.error("Error calling list_pod_subjects:", error);
      return null;
    }
  }

  async function fetchSubjectData(subjectAddress: string) {
    try {
      // Call the Tauri command "get_subject_data"
      const result = await invoke('get_subject_data', { request: { subject_address: subjectAddress } }) as any;
      const parsedResult = JSON.parse(result.data);
      // result will be your SubjectDataResult struct as a JS object, ex: { data: ... }
      return parsedResult;
    } catch (e) {
      console.error('Failed to get subject data:', e);
      return null;
    }
  }

  async function syncPods(depthValue: number) {
    try {
      podsSyncing.set(true);
      addToast("Syncing pods....", "info", 30000);
      const response = await invoke('refresh_ref', {
        request: {
          depth: String(depthValue),
        }
      });
      console.log('Success:', response);
      podsSyncing.set(false);
      addToast("Pods have been synced", "info", 30000);
      await loadTable();
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
        const pw = await getPassword();
        const podInfo = await invoke('add_pod', { request: {name: newPodName} }) as PodInfo;
        podObj["address"] = podInfo.address;
        addToast('Pod created at address:'+ podInfo.address, "info")
        await invoke("write_keystore_to_file", {password: pw})
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
    const fileToAdd = uploadedFiles.find((f: any) => f.name === selectedFileName);
    if (fileToAdd && !activePod?.fileObjs.some((f: any) => f.name === fileToAdd.name)) {
      // Add only if not already present
      activePod.fileObjs = [...activePod.fileObjs, fileToAdd];
    }
    selectedFileName = ""; // Optionally reset selection
  }

  async function deletePodHandler() {
    try {
      const pw = await getPassword();
      deletePodModal.close();
      await deletePod(activePod.name);
      await invoke("write_keystore_to_file", {password: pw})
      await loadTable();
      addToast("Pod deleted!", "success");
    } catch (error) {
      console.error(error);
      addToast("Error removing pod. Check logs...", "error");
    }
  }

  async function loadTable() {
    // createdPods = await ps.getPodCache() as [];
    createdPods = (await fetchPods()) || [];
    console.log("createdPods", createdPods)
  }

  function makeDateReadable(date: string | undefined) {
    if (date) {
      return new Date(date).toLocaleString();
    } else {
      return;
    }
  }

  function toggleSelection(list: any[], id: string) {
    console.log('here maxx');
    
    return list.map((item: any) =>
      item.uuid === id ? {...item, selected: !item.selected} : item
    );
  }

  function transferItems(from: any[], to: any[]) {
    const selectedItems = from.filter((item: any) => item.selected);

    // Filter out items whose uuid is already present in 'to'
    const toUuids = new Set(to.map((item: any) => item.uuid));
    const newItems = selectedItems.filter((item: any) => !toUuids.has(item.uuid));

    return {
      newFrom: from.map((item: any) => ({ ...item, selected: false })),
      newTo: [
        ...to,
        ...newItems.map((item: any) => ({
          ...item,
          selected: false,
          metadata: {},
          type: 'file',
          modified: true,
        }))
      ]
    };
  }

  function removeItems(from: any[]){
    const selectedItems = from.filter((item: any) => item.selected);
    deletedPodItems = deletedPodItems.concat(selectedItems);
    return from.filter((item: any) => !item.selected)
  }

  function addPodReference() {
    if (!podRefAddress) return;
    if (!activePod.fileObjs) activePod.fileObjs = [];
    activePod.fileObjs.push({
      autonomiAddress: podRefAddress,
      type: "pod-ref",
      uuid: uuidv4(),
    })
  }

  function addAutonomiFile() {
    if (!autonomiFileAddress) return;
    if (!uploadedFiles) uploadedFiles = [];

    // Create a new file object for the Autonomi file
    const newFileObj = new FileObj({
      name: `Autonomi File (${autonomiFileAddress.slice(0, 8)}...)`,
      path: "", // No local path for Autonomi files
      extension: "", // Unknown extension
      autonomiAddress: autonomiFileAddress,
      fileSize: 0, // Unknown size
      isAutonomiOnly: true, // Mark this as an Autonomi-only file so name/contentSize won't be overridden
    });

    // Add to uploaded files list so it appears in Available Files
    uploadedFiles = [...uploadedFiles, newFileObj];

    // Reset the address field
    autonomiFileAddress = "";
  }


  function saveMetaDataToItem() {
    try {
      if (editingPodItem) {
        // Convert simple table to JSON-LD if in simple mode
        if (!advancedMode) {
          jsonInputText = simpleTableToJsonLd(simpleTableData);
        }

        let metadata = JSON.parse(jsonInputText);

        // Remove unchanged template values (except for protected fields)
        if (originalTemplate) {
          const protectedFields = ["schema:name", "schema:contentSize", "@type", "schema:encodingFormat"];

          for (const [key, value] of Object.entries(originalTemplate)) {
            if (!protectedFields.includes(key) && metadata[key] === value) {
              delete metadata[key];
            }
          }
        }

        editingPodItem.metadata = metadata;
        editingPodItem.modified = true;

        if (Object.keys(editingPodItem.metadata).length === 0){
          throw Error("Metadata can't be empty!")
        }

        if ("uploadedDate" in editingPodItem && !editingPodItem.isAutonomiOnly) {
          editingPodItem.metadata["schema:contentSize"] = editingPodItem.fileSize ?? "0";
          editingPodItem.metadata["schema:name"] = editingPodItem.name;
        }
        addToast('Metadata saved!', 'success');
      }
      console.log(editingPodItem)
      editFileMetadataModal.close();
    } catch (error) {
      console.error(error);
      addToast("Could not save your metadata, ensure that it's valid JSON first.", "error");
    }
  }

  function openEditMetadata(item: any) {
    try {
      isUpdatingFromMode = true;

      // Reset mode to simple by default
      advancedMode = false;
      originalTemplate = null;

      if (Object.keys(item.metadata).length === 0) {
        loadTemplate("Book")
      } else {
        jsonInputText = JSON.stringify(item.metadata, null, 2);
        // Initialize simple table data
        simpleTableData = jsonLdToSimpleTable(jsonInputText);
      }

      isUpdatingFromMode = false;
    } catch (error) {
      console.error(error);
      addToast("Couldn't parse metadata for some reason. See logs...", "error")
      jsonInputText = JSON.stringify({}, null, 2);
      simpleTableData = [];
      isUpdatingFromMode = false;
    }
    editingPodItem = item;
    editFileMetadataModal.showModal();
  }

  async function openEditPod() {
    const subjects = await fetchPodSubjects(activePod.address)
    const tempPodItems = [];
    if (subjects.length > 0) {
      for (let subject of subjects) {
        // console.log("subject", subject)
        const data = await fetchSubjectData(subject)
        const item = parseSubjectData(data, activePod.address, subject)
        if ('type' in item && (item.type === 'pod-ref' || item.type === 'file')){
          tempPodItems.push(item);
        }
      }
    }
    console.log("tempPodItems", tempPodItems)
    activePod.fileObjs = tempPodItems;
    editPodModal.showModal();
  }

  function resetState() {
    deletedPodItems = [];
    activePod = { fileObjs: [] };
    editingPodItem = undefined;
    editMetadataFields = {};
    podRefAddress = "";
    activeFileType = "other";
    // Optionally deselect any files in uploadedFiles too:
    uploadedFiles = uploadedFiles.map(f => ({ ...f, selected: false }));
    // Close the modal if open for manual or programmatic triggers (defensive)
    editPodModal.close();
  }

  onMount(async () => {
    // await initPodManager();
    await loadTable();
    uploadedFiles = (await ps.getUploadedFilesArray()).map(file => ({
      ...file,
      disabled: false,
      selected: false
    }));
    console.log(uploadedFiles)
  });
</script>
<main style="height: 100%; display: flex; flex-direction: column; overflow: hidden;">
  <Drawer>
    <div slot="main" style="height: 100%; display: flex; flex-direction: column; overflow: hidden;">
      <div class="row" style="display: flex; flex-direction: row; justify-content: space-between; padding: 20px; flex-shrink: 0;">
        <h2 class="h2">Your Pods</h2>
        <div class="utility-bar" style="display: flex;">
          <button class="btn btn-neutral btn-soft dark:bg-primary" onclick={() => syncPodsModal.show()} disabled={$podsSyncing}>Sync Pods</button>
          <button class="btn btn-neutral" onclick={() => uploadAllPods()} disabled={$allPodsUploading}>Upload All Pods</button>
          <button class="btn btn-warning" onclick={() => createNewPodModal.showModal()}>Create New Pod</button>
        </div>
      </div>
      <div class="row" style="flex: 1; min-height: 0; overflow: hidden;">
        <div class="card bg-base-100 w-96 shadow-lg card-xl" style="width: auto; height: 100%; display: flex; flex-direction: column;">
          <div class="card-body items-center text-center p-4" style="flex: 1; min-height: 0; overflow: hidden; display: flex; flex-direction: column;">
            <div style="flex: 1; min-height: 0; overflow-y: auto; width: 100%;">
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
                        <td>{pod.name}</td>
                        <td>
                          <AddressDisplay address={pod.address} />
                        </td>
                        <td>{makeDateReadable(pod.creation)}</td>
                        <td>{makeDateReadable(pod.modified)}</td>
                        <td>
                          {#if pod.name !== "User Configuration"}
                            <!-- <button
                              class="btn btn-accent btn-square"
                              onclick={() => { activePod = pod; activePod.fileObjs = []; uploadSinglePod(); }}>
                              <img src="/app-icons/cloud-data-upload-icon.svg" alt="upload icon" width="24" height="24" />
                            </button> -->
                            <button
                              class="btn btn-warning btn-square"
                              onclick={() => { activePod = pod; activePod.fileObjs = []; openEditPod(); }}>
                              <img src="/app-icons/pencil-icon.svg" alt="edit icon" width="19" height="19" />
                            </button>
                            <button
                              class="btn btn-error btn-square"
                              onclick={() => { activePod = pod; activePod.fileObjs = []; deletePodModal.showModal(); }}>
                              <img src="/app-icons/trash-icon.svg" alt="trash icon" width="16" height="16" />
                            </button>
                          {/if}
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
    </div>
    <ul slot="sidebar" class="menu bg-base-100 text-base-content min-h-full w-40 p-5">
      <li><a href="/screens/pod-management/your-pods" class="menu-active">Your Pods</a></li>
      <li><a href="/screens/pod-management/uploads">Uploads</a></li>
      <li><a href="/screens/pod-management/downloads">Downloads</a></li>
    </ul>
  </Drawer>
  <dialog id="createNewPodModal" class="modal" bind:this={createNewPodModal}>
    <div class="modal-box">
      <h3 class="text-lg font-bold">Create New Pod</h3>
      <div class="py-4">
        <input type="text" placeholder="Please enter a name for your pod..." class="input w-full" bind:value={newPodName} />
      </div>
      <div class="modal-action">
        <form method="dialog">
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
  <dialog id="deletePodModal" class="modal" bind:this={deletePodModal}>
    <div class="modal-box w-8/12 max-w-xl">
      <h3 class="text-lg font-bold">Pod Deletion</h3>
      <div class="py-4" style="justify-content: center;">
        <p class="pb-3">Are you use you want to delete your pod?</p>
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button class="btn btn-error" onclick={()=>{deletePodHandler()}}>Delete</button>
          <button class="btn btn-soft btn-error">Cancel</button>
        </form>
      </div>
    </div>
  </dialog>
  <dialog id="editPodModal" class="modal" bind:this={editPodModal}>
    <div class="modal-box w-10/12 max-w-5xl max-h-lg">
      <h3 class="text-lg font-bold">Editing Pod: {activePod?.name}</h3>
      <div class="py-2 flex items-center justify-center gap-x-1">
        <div class="flex flex-col items-center">
          <h4 class="text-center font-semibold">Pod Items</h4>
          <ul id="podItems" class="item-container flex flex-col mb-1">
            {#each activePod?.fileObjs as item (item.uuid)}
              <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              {#if item.type === 'pod-ref'}
                <li
                  class="flex item {item.selected ? 'item-selected' : ''} {item.disabled ? 'item-disabled' : ''}"
                  onclick={() => {
                    if (!item.disabled) {
                      activePod.fileObjs = toggleSelection(activePod?.fileObjs, item.uuid);
                    }
                  }}
                >
                  <span class="truncate">{item.autonomiAddress}</span>
                  <button
                    class="edit-button btn btn-sm"
                    onclick={(e) => {
                      editingPodItem = item;
                      podRefAddress = item.autonomiAddress;
                      console.log("editingPodItem", editingPodItem)
                      e.stopPropagation();
                      editFileMetadataModal.showModal();
                    }}
                  >
                    Edit
                  </button>
                </li>
              {:else}
                <li
                  class="flex item {item.selected ? 'item-selected' : ''} {item.disabled ? 'item-disabled' : ''}"
                  onclick={() => {
                    if (!item.disabled) {
                      activePod.fileObjs = toggleSelection(activePod?.fileObjs, item.uuid);
                    }
                  }}
                >
                  <span class="truncate">{item.name}</span>
                  <button
                    class="edit-button btn btn-sm"
                    onclick={(e) => {
                      e.stopPropagation();
                      openEditMetadata(item);
                      // editingPodItem = item;
                      // console.log("editingPodItem", editingPodItem)
                      // event.stopPropagation();
                      // editFileMetadataModal.showModal();
                    }}
                  >
                    Edit
                  </button>
                </li>
              {/if}
            {/each}
          </ul>
          <div class="w-full ml-5">
            <button
              class="btn btn-primary"
              onclick={() => {
                podRefAddress = "";
                addPodRefModal.showModal()
              }}
            >
              Add Pod Ref
            </button>
          </div>
        </div>

        <div class="mx-4 flex flex-col gap-2 items-center">
          <button 
            class="btn btn-error btn-sm w-full" 
            disabled={!activePod.fileObjs?.some((f: any) => f.selected)}
            onclick={()=>{
              const result = removeItems(activePod?.fileObjs);
              activePod.fileObjs = result;
            }}  
          >
            Remove
          </button>
          <button 
            class="btn btn-primary btn-sm" 
            disabled={!uploadedFiles?.some(f => f.selected)}
            onclick={() => {
              const result = transferItems(uploadedFiles, activePod?.fileObjs);
              uploadedFiles = result.newFrom;
              activePod.fileObjs = result.newTo;
            }}
          >
            &larr;&nbsp;Transfer
          </button>
        </div>

        <div class="flex flex-col items-center">
          <h4 class="text-center font-semibold">Available Files</h4>
          <ul id="files" class="item-container flex flex-col mb-1">
            {#each uploadedFiles as item (item.uuid)}
              <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <li
                class="flex item {item.selected ? 'item-selected' : ''} {item.disabled ? 'item-disabled' : ''}"
                onclick={() => {
                  if (!item.disabled) {
                    uploadedFiles = toggleSelection(uploadedFiles, item.uuid);
                  }
                }}
              >
                <span class="truncate">{item.name}</span>
              </li>
            {/each}
          </ul>
          <div class="w-full ml-5">
            <button
              class="btn btn-primary"
              onclick={() => {
                autonomiFileAddress = "";
                addAutonomiFileModal.showModal()
              }}
            >
              Add Autonomi File
            </button>
          </div>
        </div>
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button class="btn btn-primary" onclick={() => savePod()}>Save Pod</button>
          <button class="btn btn-soft btn-error" onclick={() => resetState()}>Cancel</button>
        </form>
      </div>
    </div>
  </dialog>
  <dialog id="editFileMetadataModal" class="modal" bind:this={editFileMetadataModal}>
    <div class="modal-box w-7/12 max-w-4xl">
      <h3 class="text-lg font-bold">Editing Metadata</h3>
      <div class="py-4" style="justify-content: center;">
        {#if editingPodItem?.type === "pod-ref"}
          <fieldset class="fieldset">
            <legend class="fieldset-legend">Pod Address</legend>
            <input type="text" class="input w-full" placeholder="some address" bind:value={podRefAddress}/>
          </fieldset>
        {:else}
          <div class="flex items-center justify-between mb-4">
            <div class="flex items-center gap-4">
              <label class="font-semibold" for="template-type-select">Choose a type:</label>
              <select id="template-type-select" class="p-2 select" bind:value={selectedType} onchange={() => loadTemplate(selectedType)}>
                {#each Object.keys(templates) as type}
                  <option value={type}>{type}</option>
                {/each}
              </select>
            </div>

            <div class="flex items-center gap-2">
              <span class="text-sm">Advanced</span>
              <input
                type="checkbox"
                class="toggle toggle-primary"
                bind:checked={advancedMode}
                onchange={() => toggleAdvancedMode()}
              />
            </div>
          </div>

          {#if advancedMode}
            <!-- Advanced Mode: JSON-LD Editor -->
            <fieldset class="fieldset">
              <legend class="fieldset-legend">File Metadata (JSON-LD)</legend>
              <textarea
                class="textarea code-input"
                style="min-height: 300px; width:100%"
                placeholder=""
                bind:value={jsonInputText}

                spellcheck="false"
              >
              </textarea>
            </fieldset>
            <button class="mt-4 btn btn-primary" onclick={validateJsonLd}>
              Validate
            </button>

            {#if isValid}
              <p class="mt-4 text-green-600 font-medium">✅ Valid JSON-LD!</p>
            {:else if error}
              <p class="mt-4 text-red-600">❌ {error}</p>
              <p class="mt-4 text-red-600">❌ Check for invalid commas and that key value pairs are double quoted.</p>
            {/if}
          {:else}
            <!-- Simple Mode: Table Editor -->
            <fieldset class="fieldset">
              <legend class="fieldset-legend">File Metadata (Simple Mode)</legend>
              <!-- Fixed Header -->
              <div class="w-full">
                <table class="table w-full">
                  <thead>
                    <tr class="bg-base-200">
                      <th style="width: 200px; min-width: 200px;">Predicate</th>
                      <th style="width: calc(100% - 250px);">Value</th>
                      <th style="width: 50px; min-width: 50px;" class="text-center">Actions</th>
                    </tr>
                  </thead>
                </table>
              </div>
              <!-- Scrollable Body -->
              <div class="overflow-auto border border-base-300" style="max-height: 300px;">
                <table class="table table-zebra w-full">
                  <tbody>
                    {#each simpleTableData as row, index}
                      <tr>
                        <td style="width: 200px; min-width: 200px;">
                          <input
                            type="text"
                            class="input input-sm"
                            style="width: 190px;"
                            bind:value={row.predicate}
                            placeholder="e.g., schema:title"
                          />
                        </td>
                        <td style="width: calc(100% - 250px);">
                          <input
                            type="text"
                            class="input input-sm w-full"
                            bind:value={row.value}
                            placeholder="Enter value"
                          />
                        </td>
                        <td style="width: 50px; min-width: 50px;" class="text-center">
                          <button
                            class="btn btn-sm btn-error btn-square"
                            onclick={() => removeSimpleTableRow(index)}
                            title="Remove row"
                          >
                            <img src="/app-icons/trash-icon.svg" alt="trash icon" width="12" height="12" />
                          </button>
                        </td>
                      </tr>
                    {/each}
                  </tbody>
                </table>
              </div>
              <div class="flex gap-2 mt-4">
                <button class="btn btn-primary" onclick={addSimpleTableRow}>
                  Add Row
                </button>
              </div>
            </fieldset>
          {/if}
        {/if}
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button class="btn btn-primary" type="button" onclick={saveMetaDataToItem}>Save</button>
          <button class="btn btn-soft btn-error">Cancel</button>
        </form>
      </div>
    </div>
  </dialog>
  <dialog id="addPodRefModal" class="modal" bind:this={addPodRefModal}>
    <div class="modal-box w-5/12 max-w-xl">
      <h3 class="text-lg font-bold">Add Pod Reference</h3>
      <div class="py-4" style="justify-content: center;">
        <fieldset class="fieldset">
          <legend class="fieldset-legend">Pod Address</legend>
          <input type="text" class="input w-full" placeholder="some address" bind:value={podRefAddress}/>
        </fieldset>
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button class="btn btn-neutral" onclick={() => {addPodReference()}}>Add</button>
          <button class="btn btn-soft btn-error" onclick={()=>{podRefAddress=""}}>Cancel</button>
        </form>
      </div>
    </div>
  </dialog>
  <dialog bind:this={addAutonomiFileModal} class="modal">
    <div class="modal-box w-5/12 max-w-xl">
      <h3 class="text-lg font-bold">Add Autonomi File</h3>
      <div class="py-4" style="justify-content: center;">
        <p class="mb-4">Add a file already uploaded to Autonomi by pasting its address here:</p>
        <fieldset class="fieldset">
          <legend class="fieldset-legend">Autonomi Address</legend>
          <input type="text" class="input w-full" placeholder="Enter Autonomi address" bind:value={autonomiFileAddress}/>
        </fieldset>
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button class="btn btn-neutral" onclick={() => {addAutonomiFile()}}>Add</button>
          <button class="btn btn-soft btn-error" onclick={()=>{autonomiFileAddress=""}}>Cancel</button>
        </form>
      </div>
    </div>
  </dialog>
  <dialog id="syncPodsModal" class="modal" bind:this={syncPodsModal}>
    <div class="modal-box w-5/12 max-w-xl">
      <h3 class="text-lg font-bold">Warning</h3>
      <div class="py-4" style="justify-content: center;">
        <p>Syncing pods attempts to sync your local pods with any pods you have uploaded on the Autonomi Network. This may overwrite your local pods. If you have made any changes to your local pods that you want saved, you should upload your pods first!</p>
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button class="btn btn-neutral" onclick={() => {syncPods(0)}}>Sync Pods</button>
          <button class="btn btn-soft btn-error">Cancel</button>
        </form>
      </div>
    </div>
  </dialog>
  <dialog id="syncingInProgressModal" class="modal" bind:this={syncingInProgressModal}>
    <div class="modal-box flex flex-col items-center">
      <h3 class="text-lg font-bold mb-2">Syncing is in Progress</h3>
      <div class="my-4">
        <span class="loading loading-spinner loading-lg"></span>
      </div>
      <p class="mb-2 text-center">Pods are syncing. Please do not close or leave this page until syncing is complete.</p>
    </div>
  </dialog>
  <dialog id="uploadingInProgressModal" class="modal" bind:this={uploadingInProgressModal}>
    <div class="modal-box flex flex-col items-center">
      <h3 class="text-lg font-bold mb-2">Uploading All Pods</h3>
      <div class="my-4">
        <span class="loading loading-spinner loading-lg"></span>
      </div>
      <p class="mb-2 text-center">All pods are being uploaded. Please do not close or leave this page until uploading is complete.</p>
    </div>
  </dialog>
</main>

<style>

.utility-bar {
  display: flex;
  align-items: center;
  gap: 5px;
}

.item {
  justify-content: space-between;
  padding: 3px;
  background-color: #f2f2f2;
  border: 1px solid #d9d9d9;
  cursor: grab;
  user-select: none;
  width: 100%;
}

.item-disabled {
  color: #aaa;
  background-color: #eee;
  cursor: not-allowed;
}

.item-selected {
  background-color: #f9c7c8 !important;
  border: solid red 1px !important;
  z-index: 1 !important;
}

.item-container {
  min-width: 300px;
  max-width: 100%; /* Allow it to expand based on available space */
  min-height: 300px;
  height: 300px;
  overflow-y: auto;
  overflow-x: scroll;
  border: 2px solid #ccc;
  margin: 10px;
}

@media (min-width: 768px) {
  .item-container {
    max-width: 300px; /* Adjust as necessary for wider screens */
  }
}

@media (min-width: 1024px) {
  .item-container {
    max-width: 400px; /* Further expand on even wider screens */
  }
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

.table th:last-child,
.table td:last-child {
  min-width: 170px;
}

@media (prefers-color-scheme: dark) {

  .item {
    background-color: #333;
    border-color: #666;
    color: #ddd;
  }

  .item-container {
    border-color: #666;
  }

  .modal-box {
    background-color: #1e1e1e;
    color: #f0f0f0;
  }

  .item-selected {
    background-color: #e0282b !important;
    color: black;
    border: solid red 1px !important;
    z-index: 1 !important;
  }
}

.code-input { font-family: monospace; }

</style>
