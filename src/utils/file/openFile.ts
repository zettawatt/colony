import { invoke } from "@tauri-apps/api/core";
import { openPath } from '@tauri-apps/plugin-opener';
import { addToast } from "../../stores/toast";

// Android detection
const isAndroid = typeof window !== 'undefined' && /Android/i.test(navigator.userAgent);

/**
 * Opens a file with the default application on the current platform
 * @param filePath - Full path to the file to open
 * @param fileName - Optional file name for toast messages
 * @returns Promise that resolves when file is opened
 */
export async function openFileWithDefaultApp(filePath: string, fileName?: string): Promise<void> {
  try {
    if (isAndroid) {
      // Use custom Android command for file opening
      await invoke('open_file_with_default_app', { filePath });
      addToast(`Opened ${fileName || 'file'}`, "info");
    } else {
      // Use Tauri openPath plugin for desktop platforms
      await openPath(filePath);
      addToast(`Opened ${fileName || 'file'}`, "info");
    }
  } catch (err) {
    console.error("Failed to open file", err);
    addToast(`Failed to open ${fileName || 'file'}: ${String(err)}`, "error");
    throw err;
  }
}


