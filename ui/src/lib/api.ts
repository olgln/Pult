// Calls into the Rust side. Every `invoke` goes through this module so tests can mock it.
import { invoke } from "@tauri-apps/api/core";

/** The Pult version reported by the app (from `pult-core`). */
export function appVersion(): Promise<string> {
  return invoke<string>("app_version");
}
