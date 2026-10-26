use std::fs;
use serde_json::json;
use ultragpt_connect_lib::{adapter, capabilities, models::{Client, ValidatedInput}, storage::{self, FileKind, Paths}};

fn fixture(client: Client) -> ValidatedInput {
    let model = if client.is_codex() { "gpt-test" } else { "claude-sonnet-test" };
    ValidatedInput {
        client,
        api_key: "sk-ultragpt-fixture-only".into(),
        root: "https://ultragpt.xyz.org.mx".into(),
        model: model.into(),
        selected_models: vec![model.into()],
        catalog: None,
        capabilities: Default::default(),
        tool_search: Default::default(),
        options: Default::default(),
    }
}

#[test]
fn ultragpt_codex_import_uses_exact_gateway_and_own_restore_folder() {
    let home = tempfile::tempdir().unwrap();
    let paths = Paths::for_home(home.path());
    let original = "# original user config\nmodel = \"old\"\n";
    storage::atomic_write(&paths.target(FileKind::CodexConfig), original.as_bytes()).unwrap();
    adapter::apply(&paths, fixture(Client::Codex)).unwrap();
    let value = fs::read_to_string(paths.target(FileKind::CodexConfig)).unwrap().parse::<toml_edit::DocumentMut>().unwrap();
    assert_eq!(value["model_provider"].as_str(), Some("ultragpt-connect"));
    assert_eq!(value["model_providers"]["ultragpt-connect"]["base_url"].as_str(), Some("https://ultragpt.xyz.org.mx/v1"));
    assert_eq!(value["model_providers"]["ultragpt-connect"]["name"].as_str(), Some("UltraGPT-Connect"));
    assert_eq!(paths.state, home.path().join(".ultragpt-connect"));
    assert!(!home.path().join(".supergpt-connect").exists());
    storage::restore(&paths, Client::Codex).unwrap();
    assert_eq!(fs::read_to_string(paths.target(FileKind::CodexConfig)).unwrap(), original);
}

#[test]
fn ultragpt_claude_import_uses_site_root_and_restores() {
    let home = tempfile::tempdir().unwrap();
    let paths = Paths::for_home(home.path());
    adapter::apply(&paths, fixture(Client::Claude)).unwrap();
    let value: serde_json::Value = serde_json::from_str(&fs::read_to_string(paths.target(FileKind::ClaudeSettings)).unwrap()).unwrap();
    assert_eq!(value["env"]["ANTHROPIC_BASE_URL"], "https://ultragpt.xyz.org.mx");
    storage::restore(&paths, Client::Claude).unwrap();
    assert!(!paths.target(FileKind::ClaudeSettings).exists());
}

#[test]
fn capability_protocol_accepts_ultragpt_and_legacy_tags_without_changing_destination() {
    for name in ["ultragpt.connect_capabilities", "supergpt.connect_capabilities"] {
        assert!(capabilities::parse(&json!({"object":name,"schema_version":1,"models":{}})).is_some());
    }
    assert!(capabilities::parse(&json!({"object":"unknown","schema_version":1,"models":{}})).is_none());
}
