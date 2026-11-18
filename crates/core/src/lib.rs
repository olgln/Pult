//! `pult-core`: everything Pult knows about Claude Code sessions, testable without a window.
//!
//! The app and the `pult` CLI both call into this crate, so they always give the same
//! answer about a session.

pub mod model;

/// Version of Pult, shared by the app and the CLI (the workspace version).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the Pult version string, e.g. `0.1.0`.
pub fn version() -> &'static str {
    VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_the_workspace_version() {
        assert_eq!(version(), env!("CARGO_PKG_VERSION"));
        assert_eq!(version(), VERSION);
    }

    #[test]
    fn version_is_semver_like() {
        let v = version();
        let parts: Vec<&str> = v.split('.').collect();
        assert_eq!(parts.len(), 3, "expected major.minor.patch, got {v}");
        for part in parts {
            assert!(part.parse::<u64>().is_ok(), "not a number: {part}");
        }
    }
}
