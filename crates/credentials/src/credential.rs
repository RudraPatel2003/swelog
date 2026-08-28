use clap::ValueEnum;

/// A secret swelog stores in the operating system keyring.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Credential {
    Github,
    OpenAi,
    OpenRouter,
    Anthropic,
    Linear,
    GoogleCalendar,
}

impl Credential {
    pub const ALL_CREDENTIALS: [Self; 6] = [
        Self::Github,
        Self::OpenAi,
        Self::OpenRouter,
        Self::Anthropic,
        Self::Linear,
        Self::GoogleCalendar,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Github => "GitHub token",
            Self::OpenAi => "OpenAI API key",
            Self::OpenRouter => "OpenRouter API key",
            Self::Anthropic => "Anthropic API key",
            Self::Linear => "Linear authorization",
            Self::GoogleCalendar => "Google Calendar authorization",
        }
    }

    #[must_use]
    pub const fn command_name(self) -> &'static str {
        match self {
            Self::Github => "github",
            Self::OpenAi => "open-ai",
            Self::OpenRouter => "open-router",
            Self::Anthropic => "anthropic",
            Self::Linear => "linear",
            Self::GoogleCalendar => "google-calendar",
        }
    }

    #[must_use]
    pub const fn environment_variable(self) -> Option<&'static str> {
        match self {
            Self::Github => Some("GITHUB_TOKEN"),
            Self::OpenAi => Some("OPENAI_API_KEY"),
            Self::OpenRouter => Some("OPENROUTER_API_KEY"),
            Self::Anthropic => Some("ANTHROPIC_API_KEY"),
            Self::Linear | Self::GoogleCalendar => None, // OAuth only
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
            Self::Anthropic => Some("Create one at https://console.anthropic.com/settings/keys."),
            Self::Linear | Self::GoogleCalendar => None, // OAuth only
        }
    }

    pub(crate) const fn keyring_username(self) -> &'static str {
        match self {
            Self::Github => "github-token",
            Self::OpenAi => "openai-api-key",
            Self::OpenRouter => "openrouter-api-key",
            Self::Anthropic => "anthropic-api-key",
            Self::Linear => "linear-oauth",
            Self::GoogleCalendar => "google-calendar-oauth",
        }
    }
}
