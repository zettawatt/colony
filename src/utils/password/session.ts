import { invoke } from '@tauri-apps/api/core';


export async function setPassword(pw: string) {
  return invoke('set_password', { pw });
}

export async function getPassword(): Promise<string | null> {
  return await invoke<string | null>('get_password');
}

export async function clearPassword() {
  return invoke('clear_password');
}