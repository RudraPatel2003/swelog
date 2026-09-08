use std::fmt::Display;

use owo_colors::{
    OwoColorize,
    Stream,
    Style,
};

pub fn highlight_with_style(text: impl Display, stream: Stream, style: Style) -> String {
    format!("{}", text.if_supports_color(stream, |text| text.style(style)))
}

pub const fn cyan() -> Style {
    Style::new().cyan()
}

pub const fn yellow() -> Style {
    Style::new().yellow()
}

pub const fn green() -> Style {
    Style::new().green()
}

pub const fn dimmed() -> Style {
    Style::new().dimmed()
}
