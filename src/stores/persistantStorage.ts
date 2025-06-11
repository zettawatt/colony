import { LazyStore } from "@tauri-apps/plugin-store";
import { appDataDir } from '@tauri-apps/api/path';

const STORE_NAME = "colony-app-state.json"
const STORE_VERSION = 1;
let store: LazyStore | null = null;

export async function initStore() {
    const dir = await appDataDir();
    store = new LazyStore(`${dir}/${STORE_NAME}`, {autoSave: true});
    await store.set('__version', STORE_VERSION);
    return store;
}

export async function getStore() {
  if (!store) {
    const dir = await appDataDir();
    store = new LazyStore(`${dir}/${STORE_NAME}`, {autoSave: true});
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
    store.set("hasUserCompletedIntro", has);
}

export async function getUserCompletedIntro(): Promise<Boolean | undefined> {
    const store = await getStore();
    return store.get("hasUserCompletedIntro");
}

export async function addUploadedFile() {

}

export async function addDownloadedFile() {
  
}

export const ps = {
  initStore,
  getStore,
  eraseStore,
  setUserCompletedIntro,
  getUserCompletedIntro,
  addUploadedFile,
  addDownloadedFile,
}

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