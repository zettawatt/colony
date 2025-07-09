import { LazyStore } from "@tauri-apps/plugin-store";
import { appDataDir, downloadDir } from '@tauri-apps/api/path';
import { FileObj } from "../classes/FileObj";

const STORE_NAME = "colony-app-state.json";
const STORE_VERSION = 1;
let store: LazyStore | null = null;

type UserConfig = {
  "downloadsDirectory": string,
  "theme": 'auto' | 'light' | 'dark'
}


export async function initStore() {
  try {
    const dir = await appDataDir();
    let defaultDownloadDir: string;

    try {
      defaultDownloadDir = await downloadDir();
      if (!defaultDownloadDir) {
        defaultDownloadDir = dir;
      }
    } catch (error) {
      console.warn("Failed to get download directory, using app data directory as fallback:", error);
      defaultDownloadDir = dir;
    }

    store = new LazyStore(`${dir}/${STORE_NAME}`, { autoSave: true });
    await store.set('__version', STORE_VERSION);
    await store.set('uploadedFiles', {});
    await store.set('downloadedFiles', {});
    await store.set('podCache', []);
    await store.set('hasUserCompletedIntro', false);
    await store.set('userConfig', {
      "downloadsDirectory": defaultDownloadDir,
      "theme": "auto" // undefined is default for automatic switching
    });
    await store.set('primaryWallet', "");
    return store;
  } catch (err) {
    console.error("Error initializing persistent store:", err);
    throw err;
  }
}

export async function getStore() {
  if (!store) {
    const dir = await appDataDir();
    store = new LazyStore(`${dir}/${STORE_NAME}`, { autoSave: true });
  }
  return store;
}

export async function eraseStore() {
  const store = await getStore();
  await store.clear();
  await store.save();
}

export async function setUserCompletedIntro(has: boolean) {
  const store = await getStore();
  await store.set("hasUserCompletedIntro", has);
}

export async function getUserCompletedIntro(): Promise<boolean | undefined> {
  const store = await getStore();
  return store.get("hasUserCompletedIntro");
}

export async function addUploadedFileObj(fileObj: FileObj) {
  const store = await getStore();
  const uploadedFiles = await getUploadedFiles();
  if(uploadedFiles && fileObj.autonomiAddress) {
    uploadedFiles[fileObj.autonomiAddress] = fileObj.toJSON()
    await store.set('uploadedFiles', uploadedFiles);
  }
}

export async function getUploadedFileObj(uuid: string) {
  const store = await getStore();
  return store.get('uploadedFiles.' + uuid);
}

export async function getUploadedFiles(): Promise<Record<string, unknown> | undefined> {
  const store = await getStore();
  return (await store.get('uploadedFiles'));
}

export async function getUploadedFilesArray(): Promise<FileObj[]> {
  const store = await getStore();
  const fileObj = await store.get("uploadedFiles");
  const fileObjArray = fileObj ? Object.values(fileObj) : [];
  // Sort by uploadedDate ascending
  return fileObjArray.sort((a: any, b: any) => {
    const dateA = new Date(a.uploadedDate).getTime();
    const dateB = new Date(b.uploadedDate).getTime();
    return dateB - dateA;
  });
}

export async function addDownloadedFile(fileObj: FileObj) {
  const store = await getStore();
  const downloadedFiles = await getDownloadedFiles();
  console.log("downloadedFiles - ps", downloadedFiles)
  if(downloadedFiles && fileObj.autonomiAddress) {
    downloadedFiles[fileObj.autonomiAddress] = fileObj.toJSON()
    await store.set('downloadedFiles', downloadedFiles);
  }
}

export async function getDownloadedFileObj(uuid: string) {
  const store = await getStore();
  return store.get('downloadFiles.' + uuid);
}

export async function getDownloadedFiles(): Promise<Record<string, unknown> | undefined>  {
  const store = await getStore();
  return (await store.get('downloadedFiles')) as Record<string, FileObj>;
}

export async function getDownloadedFilesArray(): Promise<FileObj[]> {
  const store = await getStore();
  const fileObj = await store.get("downloadedFiles");
  const fileObjArray = fileObj ? Object.values(fileObj) : [];
  // Sort by ascending
  return fileObjArray.sort((a: any, b: any) => {
    const dateA = new Date(a.downloadedDate).getTime();
    const dateB = new Date(b.downloadedDate).getTime();
    return dateB - dateA;
  });
}

export async function getPodCache() {
  const store = await getStore();
  return await store.get('podCache');
}

export async function addPodObj(pod: any) {
  const store = await getStore();
  const podCache = await getPodCache();
  if(podCache && pod.address) {
    podCache.push(pod)
    await store.set('podCache', podCache);
  }
}

export async function getDownloadDir(): Promise<string> {
  const store = await getStore();
  const config = await store.get("userConfig") as { downloadsDirectory: string };
  return config.downloadsDirectory;
} 

export async function setDownloadDir(path: string): Promise<string> {
  const store = await getStore();
  const config = await store.get("userConfig") as UserConfig;
  config.downloadsDirectory = path;
  await store.set("userConfig", config);
  return config.downloadsDirectory;
} 

export async function getPrimaryWallet(): Promise<string> {
  const store = await getStore();
  const walletName = await store.get("primaryWallet") as string;
  return walletName;
} 

export async function setPrimaryWallet(walletName: string): Promise<string> {
  const store = await getStore();
  await store.set("primaryWallet", walletName);
  return walletName;
} 

export async function setTheme(t: 'auto' | 'light' | 'dark') {
  const store = await getStore();
  const config = await store.get("userConfig") as UserConfig;
  config.theme = t;
  await store.set("userConfig", config);
  return config.theme;
}

export async function getTheme(): Promise<string> {
  const store = await getStore();
  const config = await store.get("userConfig") as UserConfig;
  if ("theme" in config) {
    return config.theme;
  } else {
    return "auto";
  }
}


const ps = {
  getPodCache,
  addPodObj,
  initStore,
  getStore,
  eraseStore,
  setUserCompletedIntro,
  getUserCompletedIntro,
  addUploadedFileObj,
  getUploadedFileObj,
  getUploadedFiles,
  getUploadedFilesArray,
  addDownloadedFile,
  getDownloadedFileObj,
  getDownloadedFiles,
  getDownloadDir,
  getDownloadedFilesArray,
  setDownloadDir,
  getPrimaryWallet,
  setPrimaryWallet,
  getTheme,
  setTheme
};

export default ps;

// this is for the future at some point
// export async function initStoreWithVersionCheck() {
//   const store = await getStore();
//   const version = await store.get<number>('__version');

//   if (version !== STORE_VERSION) {
//     console.log('Migrating store...');
//     await store.clear();
//     await store.set('__version', STORE_VERSION);
//     await store.save();
//   }
// }

// async function migrateStore(oldVersion: number, store: LazyStore) {
//   if (oldVersion < 1) {
//     // No-op
//   }

//   if (oldVersion < 2) {
//     // Rename a key
//     const oldValue = await store.get('userTheme');
//     if (oldValue) {
//       await store.set('theme', oldValue);
//       await store.delete('userTheme');
//     }
//   }

//   await store.set('__version', STORE_VERSION);
//   await store.save();
// }
