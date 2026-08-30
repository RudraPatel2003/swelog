use crate::swelog_config::{
    LanguageModelProvider,
    SwelogConfig,
};

const CONFIG_WITHOUT_OPTIONAL_FIELDS: &str = r#"{
  "obsidianVaultPath": "/home/user/vault",
  "swelogFolderName": "swelog",
  "workFileName": "WORK.md",
  "dailyLogFolderName": "Daily",
  "weeklyLogFolderName": "Weekly"
}"#;

#[test]
fn swelog_config_deserializes_without_summarization_fields() {
    let config: SwelogConfig = serde_json::from_str(CONFIG_WITHOUT_OPTIONAL_FIELDS)
        .expect("config without summarization fields should parse");

    assert_eq!(config.language_model_provider, None);
    assert_eq!(config.language_model, None);
    assert_eq!(config.linear_username, None);
}

#[test]
fn swelog_config_deserializes_summarization_fields_when_present() {
    let config_file_contents = r#"{
  "obsidianVaultPath": "/home/user/vault",
  "swelogFolderName": "swelog",
  "workFileName": "WORK.md",
  "dailyLogFolderName": "Daily",
  "weeklyLogFolderName": "Weekly",
  "llm": "openRouter",
  "llmModel": "anthropic/claude-sonnet-4.5"
}"#;

    let config: SwelogConfig =
        serde_json::from_str(config_file_contents).expect("config should parse");

    assert_eq!(config.language_model_provider, Some(LanguageModelProvider::OpenRouter));
    assert_eq!(config.language_model.as_deref(), Some("anthropic/claude-sonnet-4.5"));
}

#[test]
fn swelog_config_serializes_without_absent_optional_fields() {
    let config = SwelogConfig::get_default_config();

    let serialized_config =
        serde_json::to_string(&config).expect("default config should serialize");

    assert!(!serialized_config.contains("llm"));
    assert!(!serialized_config.contains("linearUsername"));
}
