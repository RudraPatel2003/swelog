use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum DateParseError {
    #[error("invalid date `{date}`; expected MM-DD-YYYY")]
    #[diagnostic(
        code(swelog::dates::invalid_date),
        help("write the date as MM-DD-YYYY, such as 08-17-2026")
    )]
    InvalidDate { date: String },

    #[error("`{date}` is not a Monday; a week must start on a Monday")]
    #[diagnostic(
        code(swelog::dates::date_is_not_a_monday),
        help("use the Monday that begins the week you want, such as 08-17-2026")
    )]
    DateIsNotAMonday { date: String },
}
