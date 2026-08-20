use clap::Args;
use credentials::{
    Credential,
    read_credential,
    read_credential_from_environment,
};
use miette::Result;
use owo_colors::OwoColorize;

const LABEL_WIDTH: usize = 22;

#[derive(Debug, Args)]
pub struct StatusArgs {}

impl StatusArgs {
    pub fn run(self) -> Result<()> {
        let _ = self;

        println!("Credentials stored in your operating system keyring:");
        println!();

        for credential in Credential::ALL_CREDENTIALS {
            println!(
                "{:LABEL_WIDTH$}{}",
                credential.label(),
                describe_credential_status(credential)?
            );
        }

        println!();
        println!("Run `swelog auth clear <credential>` to remove a stored credential.");

        Ok(())
    }
}

/// Describes where a credential resolves from, without ever printing its value.
fn describe_credential_status(credential: Credential) -> Result<String> {
    if let Some(environment_variable) = credential.environment_variable()
        && read_credential_from_environment(credential).is_some()
    {
        return Ok(format!("{}", format!("set by ${environment_variable}").yellow()));
    }

    let description = if read_credential(credential)?.is_some() {
        format!("{}", "stored".green())
    } else {
        format!("{}", "not stored".dimmed())
    };

    Ok(description)
}
