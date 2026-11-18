// The version label shown in the sidebar footer, for each loading state.

export type VersionState =
  | { kind: "loading" }
  | { kind: "ready"; version: string }
  | { kind: "failed" };

export function versionLabel(state: VersionState): string {
  switch (state.kind) {
    case "loading":
      return "…";
    case "ready":
      return `v${state.version}`;
    case "failed":
      return "Version unavailable";
  }
}
