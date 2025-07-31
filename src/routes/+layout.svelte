<script lang="ts">
  import "../app.css";
  import Toast from '../components/toast.svelte';

  import { trace, debug, info, warn, error } from '@tauri-apps/plugin-log';

  // Optional: show Rust logs in DevTools
  // attachConsole();

  function serialize(arg: any): string {
    if (Array.isArray(arg) || (typeof arg === "object" && arg !== null)) {
      try {
        return JSON.stringify(arg);
      } catch {
        return "[Unserializable Object]";
      }
    }
    return String(arg);
  }

  function forwardConsole(fnName: string, logger: (message: string) => void): void {
    // Prevent duplicate patching (important with HMR)
    if ((console as any)[fnName]._isPatchedByApp) return;

    const original = (console as any)[fnName];
    (console as any)[fnName] = (...args: any[]) => {
      original(...args); // Continue DevTools logging
      logger(args.map(serialize).join(' ')); // Forward to Tauri
    };
    (console as any)[fnName]._isPatchedByApp = true;
  }

  // Hook up all the console methods to Tauri logger
  forwardConsole('log', trace);
  forwardConsole('debug', debug);
  forwardConsole('info', info);
  forwardConsole('warn', warn);
  forwardConsole('error', error);
</script>

<div class="bg-base-200 dark:bg-base-300">
  <slot/>
  <Toast />
</div>

<style>
  /* .layout-container {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }

  .content {
    flex: 1;
  } */
</style>