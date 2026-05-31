use chrono::Local;

pub const GITHUB_ACCEPT_HEADER: &str = "application/vnd.github.v3+json";

pub fn get_current_date_in_iso_8601() -> String {
    let now = Local::now();

    now.format("%Y-%m-%d").to_string()
}
