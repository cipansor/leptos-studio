//! Per-call settings for a generator.

/// Settings that steer how a generator writes its output.
///
/// Deliberately small: the one thing a generator must not decide on its own is
/// whether the output advertises where it came from. Leave `attribution` as
/// `None` and the generated file belongs entirely to whoever ran the generator.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GeneratorConfig {
    /// Text to write into the output as an attribution, if any.
    ///
    /// `None` (the default) writes no attribution at all.
    pub attribution: Option<String>,
}

impl GeneratorConfig {
    /// A config that writes `text` into the output as an attribution.
    pub fn with_attribution(text: impl Into<String>) -> Self {
        Self {
            attribution: Some(text.into()),
        }
    }
}
