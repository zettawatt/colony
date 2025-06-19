import { addToast } from "../stores/toast";

export async function copyAddress(address: string) {
  await navigator.clipboard.writeText(address);
  addToast('Copied address to clipboard!', 'success');
}

export function handleCopyAddress(event: MouseEvent) {
  const button = event.currentTarget as HTMLButtonElement;
  const address = button.dataset.address;
  if (address) {
    copyAddress(address);
  }
}