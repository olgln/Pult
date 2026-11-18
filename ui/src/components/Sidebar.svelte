<!-- Left column: app name, the session list (placeholder for now), version footer. -->
<script lang="ts">
  import { versionLabel, type VersionState } from "../lib/version";

  let { version }: { version: VersionState } = $props();
</script>

<aside class="sidebar" aria-label="Sessions">
  <header class="brand">Pult</header>
  <p class="placeholder">Sessions will appear here.</p>
  <footer class="version" data-testid="version">{versionLabel(version)}</footer>
</aside>

<style>
  .sidebar {
    display: flex;
    flex-direction: column;
    padding: 40px 16px 12px;
    background: var(--panel);
    border-right: 1px solid var(--line);
  }

  .brand {
    font-weight: 600;
    font-size: 15px;
  }

  .placeholder {
    flex: 1;
    color: var(--muted);
  }

  .version {
    color: var(--muted);
    font-size: 11px;
  }
</style>
