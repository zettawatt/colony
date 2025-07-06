import { invoke } from "@tauri-apps/api/core";
import { addToast } from "../../stores/toast";
import { FileObj, type FileInfo } from "../../classes/FileObj";
import ps from "../../stores/persistantStorage";
import { v4 as uuidv4 } from 'uuid';

type DownloadRequest = {
  name: string,
  address: string,
  bytes: number
}

export async function downloadFile(downloadReq: DownloadRequest) {
  addToast(`Added file to downloads`, "info")
  const downloadDir = await ps.getDownloadDir();
  const request = {
    id: uuidv4(),
    address: downloadReq.address,
    destination_path: `${downloadDir}/${downloadReq.name}`,
    size: downloadReq.bytes
  };
  try {
    const msg = await invoke<string>('download_data', { request });
    console.log(msg);
    addToast(msg, "info");
    const newFileObj = new FileObj(
      {
        name: downloadReq.name,
        path: downloadDir,
        autonomiAddress: downloadReq.address,
        extension: downloadReq.name.split('.').pop() || "",
        fileSize: downloadReq.bytes,
        downloadPath: downloadDir
      }
    );
    console.log("downloaded file", newFileObj)
    await ps.addDownloadedFile(newFileObj);
  } catch (err) {
    console.error("Download failed", err);
    addToast(String(err), "error");
  }
}