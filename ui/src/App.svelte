<!-- Window shell: drag strip for the overlay title bar, sidebar and main pane. -->
<script lang="ts">
  import Main from "./components/Main.svelte";
  import Sidebar from "./components/Sidebar.svelte";
  import { appVersion } from "./lib/api";
  import type { VersionState } from "./lib/version";

  let version = $state<VersionState>({ kind: "loading" });

  $effect(() => {
    appVersion().then(
      (v) => (version = { kind: "ready", version: v }),
      () => (version = { kind: "failed" }),
    );
  });
</script>

<div class="drag" data-tauri-drag-region></div>
<div class="shell">
  <Sidebar {version} />
  <Main />
</div>

<style>
  .drag {
    position: fixed;
    inset: 0 0 auto 0;
    height: 28px;
    z-index: 10;
  }

  .shell {
    display: grid;
    grid-template-columns: 280px 1fr;
    height: 100%;
  }
</style>
