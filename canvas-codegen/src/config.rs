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
    ///
    /// Every generator honours this, but the shape depends on the target: a
    /// comment in the target's syntax for the code and markup formats, an HTML
    /// comment for Markdown, and the reserved `$comment` keyword for JSON Schema.
    /// [`JsonGenerator`](crate::generators::JsonGenerator) is the one exception:
    /// JSON has no comment syntax and the payload is the caller's own data, so
    /// there is nowhere to put the text without inventing a field in their
    /// document. It therefore ignores the attribution rather than corrupting the
    /// output.
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
