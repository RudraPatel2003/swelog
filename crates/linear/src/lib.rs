mod client;
mod errors;
mod oauth;
mod response;

pub use client::get_active_assigned_issues;
pub use oauth::clear_linear_authorization;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinearIssue {
    pub identifier: String,
    pub title: String,
    pub url: String,
    pub status_name: String,
    pub status_type: LinearStatusType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinearStatusType {
    Started,
    Unstarted,
    Backlog,
    Completed,
    Canceled,
    Other(String),
}

impl LinearStatusType {
    #[must_use]
    pub const fn is_active(&self) -> bool {
        !matches!(self, Self::Completed | Self::Canceled)
    }
}
