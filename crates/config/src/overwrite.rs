/// Whether an existing file should be replaced or left alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overwrite {
    Yes,
    No,
}

impl Overwrite {
    /// Converts the `--force` flag clap parsed into the choice it stands for.
    #[must_use]
    pub const fn from_force_flag(force: bool) -> Self {
        if force { Self::Yes } else { Self::No }
    }
}
