use miette::{
    IntoDiagnostic,
    Result,
    WrapErr,
};
use reqwest::{
    Response,
    StatusCode,
};

use crate::errors::{
    GoogleCalendarAuthorizationRejected,
    UnsuccessfulGoogleCalendarResponse,
};

pub async fn read_successful_response_body(response: Response) -> Result<String> {
    let status_code = response.status();

    let response_text = response
        .text()
        .await
        .into_diagnostic()
        .wrap_err("failed to read Google Calendar response body")?;

    if matches!(status_code, StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN) {
        let google_calendar_authorization_rejected_error =
            GoogleCalendarAuthorizationRejected { status_code };

        return Err(google_calendar_authorization_rejected_error.into());
    }

    if !status_code.is_success() {
        let unsuccessful_google_calendar_response_error =
            UnsuccessfulGoogleCalendarResponse { status_code, response_text };

        return Err(unsuccessful_google_calendar_response_error.into());
    }

    Ok(response_text)
}
