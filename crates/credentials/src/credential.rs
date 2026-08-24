use clap::ValueEnum;

/// A secret swelog stores in the operating system keyring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Credential {
    Github,
    OpenAi,
    OpenRouter,
    Linear,
}

impl Credential {
    pub const ALL_CREDENTIALS: [Self; 4] =
        [Self::Github, Self::OpenAi, Self::OpenRouter, Self::Linear];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Github => "GitHub token",
            Self::OpenAi => "OpenAI API key",
            Self::OpenRouter => "OpenRouter API key",
            Self::Linear => "Linear authorization",
        }
    }

    #[must_use]
    pub const fn command_name(self) -> &'static str {
        match self {
            Self::Github => "github",
            Self::OpenAi => "open-ai",
            Self::OpenRouter => "open-router",
            Self::Linear => "linear",
        }
    }

    #[must_use]
    pub const fn environment_variable(self) -> Option<&'static str> {
        match self {
            Self::Github => Some("GITHUB_TOKEN"),
            Self::OpenAi => Some("OPENAI_API_KEY"),
            Self::OpenRouter => Some("OPENROUTER_API_KEY"),
            Self::Linear => None, // OAuth only
        }
    }

    #[must_use]
    pub(crate) const fn prompt_instructions(self) -> Option<&'static str> {
        match self {
            Self::Github => {
                Some("Create one at https://github.com/settings/tokens with the `repo` scope.")
            }
            Self::OpenAi => Some("Create one at https://platform.openai.com/api-keys."),
            Self::OpenRouter => Some("Create one at https://openrouter.ai/keys."),
            Self::Linear => None, // OAuth only
        }
    }

    pub(crate) const fn keyring_username(self) -> &'static str {
        match self {
            Self::Github => "github-token",
            Self::OpenAi => "openai-api-key",
            Self::OpenRouter => "openrouter-api-key",
            Self::Linear => "linear-oauth",
        }
    }
}
