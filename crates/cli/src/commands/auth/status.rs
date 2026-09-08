use clap::Args;
use credentials::{
    credential::Credential,
    resolution::read_credential_from_environment,
    store::read_credential,
};
use highlight::stdout::{
    highlight_dimmed,
    highlight_green,
    highlight_yellow,
};
use miette::Result;

const LABEL_WIDTH: usize = 31;

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

fn describe_credential_status(credential: Credential) -> Result<String> {
    if let Some(environment_variable) = credential.environment_variable()
        && read_credential_from_environment(credential).is_some()
    {
        return Ok(highlight_yellow(format!("set by ${environment_variable}")));
    }

    let description = if read_credential(credential)?.is_some() {
        highlight_green("stored")
    } else {
        highlight_dimmed("not stored")
    };

    Ok(description)
}
