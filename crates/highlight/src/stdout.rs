use std::fmt::Display;

use owo_colors::Stream;

use crate::style::{
    cyan,
    dimmed,
    green,
    highlight_with_style,
    yellow,
};

pub fn highlight_cyan(text: impl Display) -> String {
    highlight_with_style(text, Stream::Stdout, cyan())
}

pub fn highlight_yellow(text: impl Display) -> String {
    highlight_with_style(text, Stream::Stdout, yellow())
}

pub fn highlight_green(text: impl Display) -> String {
    highlight_with_style(text, Stream::Stdout, green())
}

pub fn highlight_dimmed(text: impl Display) -> String {
    highlight_with_style(text, Stream::Stdout, dimmed())
}
