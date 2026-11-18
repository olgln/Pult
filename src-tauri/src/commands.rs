//! Invoke handlers. Thin: each one calls into `pult-core`.

/// The Pult version, shown in the window.
#[tauri::command]
pub fn app_version() -> String {
    pult_core::version().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_version_comes_from_core() {
        assert_eq!(app_version(), pult_core::version());
    }
}
