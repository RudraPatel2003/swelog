use std::path::PathBuf;

use super::*;

#[test]
fn format_config_groups_fields_under_section_headings() {
    let mut config = SwelogConfig::get_default_config();

    config.obsidian_vault_path = PathBuf::from("/home/user/vault");

    let output = format_config(&config);

    assert!(output.contains("Vault\n"));
    assert!(output.contains("Files\n"));
    assert!(output.contains("Logs\n"));
    assert!(output.contains("Language Model\n"));
    assert!(output.contains("Integrations\n"));

    assert!(output.contains("Obsidian vault path  /home/user/vault\n"));
    assert!(output.contains("Provider"));
    assert!(output.contains("Ollama\n"));
    assert!(output.contains("Model                llama3.2\n"));
    assert!(output.contains("Linear username      Not configured\n"));
}
