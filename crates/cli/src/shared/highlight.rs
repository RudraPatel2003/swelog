use std::fmt::Display;

use owo_colors::{
    OwoColorize,
    Stream,
};

pub fn highlight(text: impl Display) -> String {
    format!("{}", text.if_supports_color(Stream::Stdout, |text| text.cyan()))
}
