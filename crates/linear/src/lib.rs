mod client;
mod errors;
mod oauth;
mod response;

use serde::Deserialize;

pub use crate::{
    client::get_active_assigned_issues,
    oauth::clear_linear_authorization,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LinearIssue {
    #[serde(rename = "id")]
    pub identifier: String,

    pub title: String,

    pub url: String,

    #[serde(rename = "status")]
    pub status_name: String,

    pub status_type: LinearStatusType,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LinearStatusType {
    Backlog,
    Unstarted,
    Started,
    Completed,

    #[serde(alias = "cancelled")]
    Canceled,

    #[serde(other)]
    Other,
}

impl LinearStatusType {
    #[must_use]
    pub const fn is_active(self) -> bool {
        !matches!(self, Self::Completed | Self::Canceled)
    }
}
