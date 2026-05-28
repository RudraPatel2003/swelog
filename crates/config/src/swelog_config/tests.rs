use super::*;

#[test]
fn default_config_uses_ollama_and_includes_open_ai_model() {
    let config = SwelogConfig::get_default_config();

    assert_eq!(config.language_model_provider, LanguageModelProvider::Ollama);
    assert_eq!(config.ollama_model, DEFAULT_OLLAMA_MODEL);
    assert_eq!(config.open_ai_model, DEFAULT_OPEN_AI_MODEL);
}

#[test]
fn config_parses_open_ai_provider() {
    let config_json = r#"
        {
          "obsidianVaultPath": "/tmp/vault",
          "swelogFolderName": "swelog",
          "workFileName": "WORK.md",
          "contextFileName": "CONTEXT.md",
          "dailyLogFolderName": "Daily",
          "weeklyLogFolderName": "Weekly",
          "llm": "openAi",
          "ollamaModel": "llama3.2",
          "openAiModel": "gpt-5.4-mini"
        }
    "#;

    let config: SwelogConfig =
        serde_json::from_str(config_json).expect("config should parse as valid JSON");

    assert_eq!(config.language_model_provider, LanguageModelProvider::OpenAi);
    assert_eq!(config.open_ai_model, DEFAULT_OPEN_AI_MODEL);
}

#[test]
fn config_without_open_ai_model_uses_default() {
    let config_json = r#"
        {
          "obsidianVaultPath": "/tmp/vault",
          "swelogFolderName": "swelog",
          "workFileName": "WORK.md",
          "contextFileName": "CONTEXT.md",
          "dailyLogFolderName": "Daily",
          "weeklyLogFolderName": "Weekly",
          "llm": "ollama",
          "ollamaModel": "llama3.2"
        }
    "#;

    let config: SwelogConfig =
        serde_json::from_str(config_json).expect("config should parse as valid JSON");

    assert_eq!(config.open_ai_model, DEFAULT_OPEN_AI_MODEL);
}
