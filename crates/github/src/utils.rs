use chrono::Local;

pub const GITHUB_ACCEPT_HEADER: &str = "application/vnd.github.v3+json";

pub const SWELOG_USER_AGENT: &str = "RudraPatel2003/swelog-cli";

pub fn get_current_date_in_iso_8601() -> String {
    let now = Local::now();

    now.format("%Y-%m-%d").to_string()
}

const GITHUB_REPOS_API_URL: &str = "https://api.github.com/repos/";

/// ```
/// use github::utils::get_repository_name_from_repository_url;
///
/// let repository_url = "https://api.github.com/repos/RudraPatel2003/swelog-cli";
///
/// let repository_name = get_repository_name_from_repository_url(repository_url);
///
/// assert_eq!(repository_name, "RudraPatel2003/swelog-cli");
/// ```
pub fn get_repository_name_from_repository_url(repository_url: &str) -> String {
    let Some(repo) = repository_url.strip_prefix(GITHUB_REPOS_API_URL) else {
        return String::from("Unknown repository");
    };

    repo.to_string()
}
