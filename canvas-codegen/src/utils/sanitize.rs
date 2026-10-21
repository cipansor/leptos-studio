//! HTML Sanitization utilities for XSS protection
//!
//! This module provides functions to sanitize user-provided HTML content
//! to prevent Cross-Site Scripting (XSS) attacks.

use regex::Regex;
use std::borrow::Cow;

/// List of allowed HTML tags for custom components
#[allow(dead_code)]
const ALLOWED_TAGS: &[&str] = &[
    "div",
    "span",
    "p",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "a",
    "img",
    "ul",
    "ol",
    "li",
    "br",
    "hr",
    "strong",
    "em",
    "b",
    "i",
    "u",
    "code",
    "pre",
    "blockquote",
    "table",
    "thead",
    "tbody",
    "tr",
    "th",
    "td",
    "form",
    "input",
    "button",
    "label",
    "select",
    "option",
    "textarea",
    "header",
    "footer",
    "nav",
    "main",
    "section",
    "article",
    "aside",
    "figure",
    "figcaption",
    "svg",
    "path",
    "circle",
    "rect",
    "line",
    "polyline",
    "polygon",
    "text",
    "g",
];

/// List of allowed attributes (safe subset)
#[allow(dead_code)]
const ALLOWED_ATTRS: &[&str] = &[
    "id",
    "class",
    "style",
    "title",
    "alt",
    "src",
    "href",
    "width",
    "height",
    "type",
    "name",
    "value",
    "placeholder",
    "disabled",
    "readonly",
    "required",
    "checked",
    "selected",
    "for",
    "role",
    "aria-label",
    "aria-hidden",
    "aria-describedby",
    "data-*",
    "tabindex",
    "viewBox",
    "fill",
    "stroke",
    "stroke-width",
    "d",
    "cx",
    "cy",
    "r",
    "x",
    "y",
    "x1",
    "y1",
    "x2",
    "y2",
    "points",
    "transform",
];

/// Dangerous patterns that should be removed
const DANGEROUS_PATTERNS: &[&str] = &[
    r"(?i)javascript:",
    r"(?i)vbscript:",
    r"(?i)data:",
    r"(?i)on\w+\s*=",
    r"(?i)<script",
    r"(?i)</script>",
    r"(?i)<iframe",
    r"(?i)</iframe>",
    r"(?i)<object",
    r"(?i)</object>",
    r"(?i)<embed",
    r"(?i)</embed>",
    r"(?i)<link",
    r"(?i)<meta",
    r"(?i)<base",
    r"(?i)expression\s*\(",
];

/// Configuration for HTML sanitization
#[derive(Debug, Clone)]
pub struct SanitizeConfig {
    /// Allow all tags (not recommended for untrusted input)
    pub allow_all_tags: bool,
    /// Custom list of additional allowed tags
    pub extra_allowed_tags: Vec<String>,
    /// Strip all HTML tags (text only output)
    pub strip_all_tags: bool,
    /// Maximum template length
    pub max_length: usize,
}

impl Default for SanitizeConfig {
    fn default() -> Self {
        Self {
            allow_all_tags: false,
            extra_allowed_tags: Vec::new(),
            strip_all_tags: false,
            max_length: 50_000,
        }
    }
}

/// Result of sanitization
#[derive(Debug, Clone)]
pub struct SanitizeResult {
    /// The sanitized HTML
    pub html: String,
    /// Whether any modifications were made
    pub was_modified: bool,
    /// List of removed dangerous patterns
    pub removed_patterns: Vec<String>,
    /// Whether the input was truncated
    pub was_truncated: bool,
}

/// Sanitize HTML content to prevent XSS attacks
pub fn sanitize_html(html: &str, config: &SanitizeConfig) -> SanitizeResult {
    let mut result = html.to_string();
    let mut was_modified = false;
    let mut removed_patterns = Vec::new();
    let mut was_truncated = false;

    // Check length first
    if result.len() > config.max_length {
        result.truncate(config.max_length);
        was_truncated = true;
        was_modified = true;
    }

    // Strip all tags if requested
    if config.strip_all_tags {
        let tag_regex = Regex::new(r"<[^>]+>").unwrap();
        let stripped = tag_regex.replace_all(&result, "");
        if stripped != result {
            was_modified = true;
        }
        return SanitizeResult {
            html: stripped.into_owned(),
            was_modified,
            removed_patterns,
            was_truncated,
        };
    }

    // Remove dangerous patterns
    for pattern in DANGEROUS_PATTERNS {
        if let Ok(regex) = Regex::new(pattern)
            && regex.is_match(&result)
        {
            removed_patterns.push(pattern.to_string());
            result = regex.replace_all(&result, "").into_owned();
            was_modified = true;
        }
    }

    // Remove event handlers more specifically
    if let Ok(event_handler_regex) = Regex::new(r#"(?i)\s+on\w+\s*=\s*"[^"]*""#)
        && event_handler_regex.is_match(&result)
    {
        result = event_handler_regex.replace_all(&result, "").into_owned();
        was_modified = true;
        let event_handlers_str = "event handlers".to_string();
        if !removed_patterns.contains(&event_handlers_str) {
            removed_patterns.push(event_handlers_str);
        }
    }

    // Also check single-quoted event handlers
    if let Ok(event_handler_regex_sq) = Regex::new(r"(?i)\s+on\w+\s*=\s*'[^']*'")
        && event_handler_regex_sq.is_match(&result)
    {
        result = event_handler_regex_sq.replace_all(&result, "").into_owned();
        was_modified = true;
    }

    // Remove javascript: and other dangerous protocols in href/src
    if let Ok(protocol_regex) =
        Regex::new(r#"(?i)(href|src)\s*=\s*"(javascript|vbscript|data):[^"]*""#)
        && protocol_regex.is_match(&result)
    {
        result = protocol_regex.replace_all(&result, r#"$1="""#).into_owned();
        was_modified = true;
    }

    // Also check single-quoted protocols
    if let Ok(protocol_regex_sq) =
        Regex::new(r"(?i)(href|src)\s*=\s*'(javascript|vbscript|data):[^']*'")
        && protocol_regex_sq.is_match(&result)
    {
        result = protocol_regex_sq.replace_all(&result, "$1=''").into_owned();
        was_modified = true;
    }

    SanitizeResult {
        html: result,
        was_modified,
        removed_patterns,
        was_truncated,
    }
}

/// Quick sanitization for simple cases (removes all dangerous patterns)
#[allow(dead_code)]
pub fn sanitize_html_quick(html: &str) -> String {
    sanitize_html(html, &SanitizeConfig::default()).html
}

/// Check if HTML contains potentially dangerous content
pub fn is_html_safe(html: &str) -> bool {
    for pattern in DANGEROUS_PATTERNS {
        if let Ok(regex) = Regex::new(pattern)
            && regex.is_match(html)
        {
            return false;
        }
    }
    true
}

/// Escape special HTML characters
pub fn escape_html(text: &str) -> Cow<'_, str> {
    let needs_escaping = text
        .chars()
        .any(|c| c == '<' || c == '>' || c == '&' || c == '"' || c == '\'');

    if !needs_escaping {
        return Cow::Borrowed(text);
    }

    let mut result = String::with_capacity(text.len() + 16);
    for c in text.chars() {
        match c {
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '&' => result.push_str("&amp;"),
            '"' => result.push_str("&quot;"),
            '\'' => result.push_str("&#x27;"),
            _ => result.push(c),
        }
    }
    Cow::Owned(result)
}

/// Unescape HTML entities
#[allow(dead_code)]
pub fn unescape_html(text: &str) -> String {
    text.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#x27;", "'")
        .replace("&#39;", "'")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_removes_script_tags() {
        let html = r#"<div>Hello</div><script>alert('xss')</script>"#;
        let result = sanitize_html(html, &SanitizeConfig::default());
        assert!(!result.html.contains("<script"));
        assert!(result.was_modified);
    }

    #[test]
    fn test_sanitize_removes_onclick() {
        let html = r#"<button onclick="alert('xss')">Click</button>"#;
        let result = sanitize_html(html, &SanitizeConfig::default());
        assert!(!result.html.contains("onclick"));
        assert!(result.was_modified);
    }

    #[test]
    fn test_sanitize_removes_javascript_href() {
        let html = r#"<a href="javascript:alert('xss')">Link</a>"#;
        let result = sanitize_html(html, &SanitizeConfig::default());
        assert!(!result.html.contains("javascript:"));
        assert!(result.was_modified);
    }

    #[test]
    fn test_sanitize_removes_onload() {
        let html = r#"<img src="x" onerror="alert('xss')">"#;
        let result = sanitize_html(html, &SanitizeConfig::default());
        assert!(!result.html.contains("onerror"));
        assert!(result.was_modified);
    }

    #[test]
    fn test_sanitize_removes_iframe() {
        let html = r#"<iframe src="evil.com"></iframe>"#;
        let result = sanitize_html(html, &SanitizeConfig::default());
        assert!(!result.html.contains("<iframe"));
        assert!(result.was_modified);
    }

    #[test]
    fn test_sanitize_preserves_safe_html() {
        let html = r#"<div class="container"><p>Hello World</p></div>"#;
        let result = sanitize_html(html, &SanitizeConfig::default());
        assert_eq!(result.html, html);
        assert!(!result.was_modified);
    }

    #[test]
    fn test_is_html_safe() {
        assert!(is_html_safe("<div>Hello</div>"));
        assert!(!is_html_safe("<script>alert('xss')</script>"));
        assert!(!is_html_safe(r#"<a href="javascript:void(0)">x</a>"#));
        assert!(!is_html_safe(r#"<div onclick="evil()">x</div>"#));
    }

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("<div>").as_ref(), "&lt;div&gt;");
        assert_eq!(escape_html("Hello & World").as_ref(), "Hello &amp; World");
        assert_eq!(escape_html("\"quoted\"").as_ref(), "&quot;quoted&quot;");
        assert_eq!(escape_html("normal text").as_ref(), "normal text");
    }

    #[test]
    fn test_unescape_html() {
        assert_eq!(unescape_html("&lt;div&gt;"), "<div>");
        assert_eq!(unescape_html("Hello &amp; World"), "Hello & World");
    }

    #[test]
    fn test_sanitize_truncates_long_content() {
        let long_html = "x".repeat(100_000);
        let config = SanitizeConfig {
            max_length: 1000,
            ..Default::default()
        };
        let result = sanitize_html(&long_html, &config);
        assert!(result.was_truncated);
        assert_eq!(result.html.len(), 1000);
    }

    #[test]
    fn test_strip_all_tags() {
        let html = "<div><p>Hello</p><span>World</span></div>";
        let config = SanitizeConfig {
            strip_all_tags: true,
            ..Default::default()
        };
        let result = sanitize_html(html, &config);
        assert_eq!(result.html, "HelloWorld");
    }

    #[test]
    fn test_sanitize_case_insensitive() {
        let html = r#"<SCRIPT>alert('xss')</SCRIPT>"#;
        let result = sanitize_html(html, &SanitizeConfig::default());
        assert!(!result.html.to_lowercase().contains("<script"));
    }

    #[test]
    fn test_sanitize_onmouseover() {
        let html = r#"<div onmouseover="evil()">hover me</div>"#;
        let result = sanitize_html(html, &SanitizeConfig::default());
        assert!(!result.html.contains("onmouseover"));
    }
}
