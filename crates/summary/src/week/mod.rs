use chrono::NaiveDate;

pub fn get_weekly_log_file_name(monday_date: &NaiveDate) -> String {
    let monday_date_string = monday_date.format("%m-%d-%Y").to_string();

    format!("Week of {monday_date_string}.md")
}
