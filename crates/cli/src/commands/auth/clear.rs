use clap::Args;
use credentials::{
    Credential,
    clear_credential,
};
use miette::Result;

#[derive(Debug, Args)]
pub struct ClearArgs {
    /// Credential to remove from your operating system keyring.
    #[arg(value_name = "CREDENTIAL", required_unless_present = "all")]
    credential: Option<Credential>,

    /// Remove every credential swelog has stored.
    #[arg(long, conflicts_with = "credential")]
    all: bool,
}

impl ClearArgs {
    pub fn run(self) -> Result<()> {
        let credentials = self
            .credential
            .map_or_else(|| Credential::ALL_CREDENTIALS.to_vec(), |credential| vec![credential]);

        for credential in credentials {
            let was_stored = clear_credential(credential)?;

            if was_stored {
                println!("Removed the stored {}.", credential.label());
            } else {
                println!("No {} was stored.", credential.label());
            }
        }

        Ok(())
    }
}
