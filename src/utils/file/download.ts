import { invoke } from "@tauri-apps/api/core";
import { addToast } from "../../stores/toast";
import { FileObj } from "../../classes/FileObj";
import ps from "../../stores/persistantStorage";
import { v4 as uuidv4 } from 'uuid';

type DownloadRequest = {
  name: string,
  address: string,
  bytes?: number
}

export async function downloadFile(downloadReq: DownloadRequest, type: "file" | "directory") {
  addToast(`Added file to downloads`, "info")
  console.log('type', type)
  const downloadDir = await ps.getDownloadDir();
  const request = {
    id: uuidv4(),
    address: downloadReq.address,
    destination_path: `${downloadDir}/${downloadReq.name}`,
    size: downloadReq.bytes ?? 0
  };
  try {
    let msg;
    if (type === 'directory') {
      msg = await invoke<string>('download_directory', { request });
    } else {
      msg = await invoke<string>('download_data', { request });
    }
    console.log(msg);
    addToast(msg, "info");
    const newFileObj = new FileObj(
      {
        name: downloadReq.name,
        path: downloadDir,
        autonomiAddress: downloadReq.address,
        extension: downloadReq.name.split('.').pop() || "",
        fileSize: downloadReq.bytes ?? 0,
        downloadPath: downloadDir
      }
    );
    console.log("downloaded file", newFileObj)
    await ps.addDownloadedFile(newFileObj);
  } catch (err) {
    console.error("Download failed", err);
    addToast("Failed to download files, see logs", "error");
  }
}