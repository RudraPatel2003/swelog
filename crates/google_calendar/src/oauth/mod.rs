mod application;
mod credentials;
mod flow;
mod redirect;
mod token;

use miette::Result;

use crate::oauth::{
    application::get_compiled_in_oauth_application,
    credentials::{
        clear_google_credentials,
        get_current_epoch_seconds,
        read_google_credentials,
        write_google_credentials,
    },
    flow::authorize_in_browser,
    token::{
        refresh_access_token,
        structs::RefreshOutcome,
    },
};

pub async fn get_access_token_authorizing_if_needed() -> Result<String> {
    let application = get_compiled_in_oauth_application()?;

    let Some(google_credentials) = read_google_credentials()? else {
        let authorized_credentials = authorize_in_browser(&application).await?;

        write_google_credentials(&authorized_credentials)?;

        return Ok(authorized_credentials.access_token);
    };

    if !google_credentials.is_expiring(get_current_epoch_seconds()) {
        return Ok(google_credentials.access_token);
    }

    let refreshed_credentials =
        match refresh_access_token(&application, &google_credentials.refresh_token).await? {
            RefreshOutcome::Refreshed(refreshed_credentials) => refreshed_credentials,

            RefreshOutcome::ReauthorizationRequired => authorize_in_browser(&application).await?,
        };

    write_google_credentials(&refreshed_credentials)?;

    Ok(refreshed_credentials.access_token)
}

pub fn clear_google_calendar_authorization() -> Result<()> {
    clear_google_credentials()
}
