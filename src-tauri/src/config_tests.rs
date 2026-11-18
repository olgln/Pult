//! Guards on `tauri.conf.json` and the capabilities: no network, no updater, minimal
//! permissions.

use serde_json::Value;

fn load(relative: &str) -> Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{relative}: {e}"));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{relative}: {e}"))
}

fn config() -> Value {
    load("tauri.conf.json")
}

fn csp(name: &str) -> String {
    config()["app"]["security"][name]
        .as_str()
        .unwrap_or_else(|| panic!("app.security.{name} must be a string"))
        .to_string()
}

/// Every source expression of a CSP string, across all directives.
fn sources(policy: &str) -> Vec<String> {
    policy
        .split(';')
        .flat_map(|directive| directive.split_whitespace().skip(1))
        .map(str::to_string)
        .collect()
}

/// Sources that are allowed in the production CSP.
fn is_local_source(source: &str) -> bool {
    matches!(source, "'self'" | "'none'" | "'unsafe-inline'")
}

/// Every string anywhere in a JSON value.
fn strings(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::String(s) => out.push(s.clone()),
        Value::Array(items) => items.iter().for_each(|v| strings(v, out)),
        Value::Object(map) => map.values().for_each(|v| strings(v, out)),
        _ => {}
    }
}

#[test]
fn csp_has_no_remote_origins() {
    let policy = csp("csp");
    assert!(policy.contains("default-src 'self'"), "{policy}");
    for source in sources(&policy) {
        assert!(is_local_source(&source), "non-local CSP source: {source}");
    }
}

#[test]
fn dev_csp_allows_only_the_local_dev_server() {
    let policy = csp("devCsp");
    assert!(policy.contains("default-src 'self'"), "{policy}");
    for source in sources(&policy) {
        assert!(
            is_local_source(&source) || source == "ws://localhost:1420",
            "non-local dev CSP source: {source}"
        );
    }
}

#[test]
fn csp_helpers_classify_sources() {
    assert_eq!(
        sources("default-src 'self'; img-src 'self' https://example.com"),
        ["'self'", "'self'", "https://example.com"]
    );
    assert!(is_local_source("'self'"));
    assert!(!is_local_source("https://example.com"));
    assert!(!is_local_source("*"));
    assert!(!is_local_source("data:"));
}

#[test]
fn no_updater_is_configured() {
    let conf = config();
    assert!(
        conf["plugins"]["updater"].is_null(),
        "updater plugin configured"
    );
    assert_ne!(conf["bundle"]["createUpdaterArtifacts"], Value::Bool(true));
    let text = conf.to_string().to_lowercase();
    assert!(
        !text.contains("updater"),
        "updater mentioned in tauri.conf.json"
    );
}

/// URLs that may appear in the config: the local Vite dev server only.
fn is_allowed_url(token: &str) -> bool {
    let has_scheme = token.contains("://");
    !has_scheme || token == "http://localhost:1420" || token == "ws://localhost:1420"
}

#[test]
fn no_remote_urls_in_config() {
    let mut all = Vec::new();
    strings(&config(), &mut all);
    for token in all.iter().flat_map(|s| s.split_whitespace()) {
        assert!(is_allowed_url(token), "remote URL in config: {token}");
    }
}

#[test]
fn url_helper_rejects_remote_urls() {
    assert!(is_allowed_url("../dist"));
    assert!(is_allowed_url("http://localhost:1420"));
    assert!(!is_allowed_url("https://example.com"));
    assert!(!is_allowed_url("http://localhost:8000"));
}

#[test]
fn window_matches_the_spec() {
    let conf = config();
    assert_eq!(conf["productName"], "Pult");
    assert_eq!(conf["identifier"], "app.pult.desktop");
    let windows = conf["app"]["windows"].as_array().expect("windows");
    assert_eq!(windows.len(), 1);
    let w = &windows[0];
    assert_eq!(w["width"], 1200);
    assert_eq!(w["height"], 780);
    assert_eq!(w["titleBarStyle"], "Overlay");
    assert_eq!(w["hiddenTitle"], true);
    assert_eq!(conf["bundle"]["targets"], serde_json::json!(["app"]));
}

#[test]
fn capabilities_are_minimal() {
    let cap = load("capabilities/default.json");
    assert_eq!(cap["windows"], serde_json::json!(["main"]));
    let permissions: Vec<&str> = cap["permissions"]
        .as_array()
        .expect("permissions")
        .iter()
        .map(|p| p.as_str().expect("plain permission identifiers only"))
        .collect();
    assert_eq!(
        permissions,
        [
            "core:window:allow-start-dragging",
            "notification:default",
            "dialog:allow-open",
        ]
    );
}
