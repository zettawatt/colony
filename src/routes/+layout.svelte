<script>
  import "../app.css";
  import Toast from '../components/toast.svelte';

  import { attachConsole, trace, debug, info, warn, error } from '@tauri-apps/plugin-log';

  // Optional: show Rust logs in DevTools
  // attachConsole();

  // This forwards browser logs to Tauri
  function forwardConsole(fnName, logger) {
    const original = console[fnName];
    console[fnName] = (...args) => {
      original(...args); // still show logs in DevTools
      logger(args.map(String).join(' ')); // send logs to Tauri
    };
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