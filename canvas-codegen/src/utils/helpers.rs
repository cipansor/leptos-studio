//! Shared helpers the generators build their output with.

use crate::config::GeneratorConfig;
use crate::domain::{Animation, ComponentStyle};

/// Helper to generate animation styles
pub(crate) fn get_animation_css(animation: &Option<Animation>) -> String {
    animation
        .as_ref()
        .map(|a| a.to_css_string())
        .unwrap_or_default()
}

/// The inline CSS a component's visual settings produce: its [`ComponentStyle`]
/// plus any animation, in the same order the canvas renderer applies them.
///
/// Every visual exporter must go through here for a component's visual settings,
/// so the exported markup matches the canvas and no exporter invents a second
/// representation. Empty when the component has neither, so callers can skip the
/// `style` attribute entirely rather than emit an empty one.
pub(crate) fn component_inline_css(
    style: &ComponentStyle,
    animation: &Option<Animation>,
) -> String {
    let anim = get_animation_css(animation);
    let custom = style.to_css_string();
    match (anim.is_empty(), custom.is_empty()) {
        (true, true) => String::new(),
        (false, true) => anim,
        (true, false) => custom,
        (false, false) => format!("{anim} {custom}"),
    }
}

/// Just the animation value (`fadeIn 0.3s ease-in-out 0s 1 both`), for exporters
/// that need a bare CSS value rather than a full declaration.
pub(crate) fn animation_value(animation: &Option<Animation>) -> String {
    animation
        .as_ref()
        .filter(|a| a.animation_type != crate::domain::AnimationType::None)
        .map(|a| {
            a.to_css_string()
                .trim_start_matches("animation:")
                .trim()
                .trim_end_matches(';')
                .to_string()
        })
        .unwrap_or_default()
}

/// [`ComponentStyle`] plus animation as camelCase key/value pairs, the shape a
/// CSS-in-JS exporter (React's `style={{ … }}`) needs. This is the JS-side twin
/// of [`component_inline_css`], so both carry every visual setting and a new
/// `ComponentStyle` field added to one must be added to the other.
pub(crate) fn style_js_properties(
    style: &ComponentStyle,
    animation: &Option<Animation>,
) -> Vec<(&'static str, String)> {
    let mut props: Vec<(&'static str, String)> = Vec::new();

    if let Some(v) = &style.padding {
        props.push(("padding", v.clone()));
    }
    if let Some(v) = &style.margin {
        props.push(("margin", v.clone()));
    }
    if let Some(v) = &style.width {
        props.push(("width", v.clone()));
    }
    if let Some(v) = &style.height {
        props.push(("height", v.clone()));
    }
    if let Some(v) = &style.color {
        props.push(("color", v.clone()));
    }
    if let Some(v) = &style.background_color {
        props.push(("backgroundColor", v.clone()));
    }
    if let (Some(color), Some(width)) = (&style.border_color, style.border_width) {
        props.push(("border", format!("{width}px solid {color}")));
    }
    if let Some(radius) = style.border_radius {
        props.push(("borderRadius", format!("{radius}px")));
    }
    if let Some(size) = style.font_size {
        props.push(("fontSize", format!("{size}px")));
    }
    if let Some(v) = &style.font_weight {
        props.push(("fontWeight", v.clone()));
    }
    if let Some(v) = &style.text_align {
        props.push(("textAlign", v.clone()));
    }
    if let Some(v) = &style.display {
        props.push(("display", v.clone()));
    }
    if let Some(v) = &style.flex_direction {
        props.push(("flexDirection", v.clone()));
    }
    if let Some(v) = &style.gap {
        props.push(("gap", v.clone()));
    }

    let anim = animation_value(animation);
    if !anim.is_empty() {
        props.push(("animation", anim));
    }

    props
}

/// A single-quoted JS string literal. Backslash and quote are escaped so a style
/// value or label cannot terminate the literal and corrupt the generated module.
pub(crate) fn js_string_literal(value: &str) -> String {
    let escaped = value.replace('\\', "\\\\").replace('\'', "\\'");
    format!("'{escaped}'")
}

/// Escape `value` for inclusion *inside* a Rust string literal (the surrounding
/// quotes are added by the caller).
///
/// This is deliberately not [`escape_html`]. A Leptos `view!` macro compiles the
/// string literal straight into Rust data and sets it on the DOM, so HTML
/// entities are **not** decoded: escaping `&` to `&amp;` would change the
/// component's runtime value, turning an href of `?a=1&b=2` into
/// `?a=1&amp;b=2` on the rendered page. What actually has to be escaped is
/// whatever would end the literal or be reinterpreted by the Rust lexer:
/// backslash, the double quote, and the control characters that have Rust escape
/// sequences. The `view!` tokenizer sees the whole quoted literal as one token,
/// so braces and angle brackets inside it need no special handling.
///
/// HTML escaping stays the right tool for the templates that are themselves HTML
/// (HTML, Vue, Svelte, Tailwind) and for Markdown, where entities *are* decoded.
pub(crate) fn rust_string_literal(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len() + 8);
    for c in value.chars() {
        match c {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            // Any other control character must not appear raw in the literal.
            c if c.is_control() => escaped.push_str(&format!("\\u{{{:x}}}", c as u32)),
            c => escaped.push(c),
        }
    }
    escaped
}

/// Base component CSS embedded into Plain exports so the result is self-contained.
/// Must not contain double quotes (embedded as a string literal in generated code).
pub(crate) const EXPORT_CSS: &str = r#"
button { display: inline-flex; align-items: center; justify-content: center; padding: 0.5rem 1rem; border: none; border-radius: 0.375rem; font-weight: 500; cursor: pointer; }
button:disabled { opacity: 0.5; cursor: not-allowed; }
.btn-primary { background-color: #007bff; color: white; }
.btn-primary:hover { background-color: #0056b3; }
.btn-secondary { background-color: #6c757d; color: white; }
.btn-secondary:hover { background-color: #545b62; }
.btn-outline { background-color: transparent; border: 1px solid #6c757d; color: #6c757d; }
.btn-outline:hover { background-color: #6c757d; color: white; }
.btn-ghost { background-color: transparent; color: #007bff; }
.btn-ghost:hover { background-color: rgba(0, 123, 255, 0.1); }
.btn-sm { padding: 0.25rem 0.5rem; font-size: 0.875rem; }
.btn-md { padding: 0.5rem 1rem; font-size: 1rem; }
.btn-lg { padding: 0.75rem 1.5rem; font-size: 1.125rem; }

.text-heading1 { font-size: 2rem; font-weight: 700; line-height: 1.2; }
.text-heading2 { font-size: 1.5rem; font-weight: 600; line-height: 1.3; }
.text-heading3 { font-size: 1.25rem; font-weight: 600; line-height: 1.4; }
.text-body { font-size: 1rem; font-weight: 400; line-height: 1.5; }
.text-caption { font-size: 0.875rem; font-weight: 400; line-height: 1.4; }

.container.flex-row { display: flex; flex-direction: row; }
.container.flex-col { display: flex; flex-direction: column; }
.container.flex-wrap { flex-wrap: wrap; }
.container.grid { display: grid; }
.container.stack { position: relative; }
.container.stack > * { position: absolute; width: 100%; height: 100%; }

.card-bordered { border: 1px solid #e5e7eb; }

.badge { display: inline-block; padding: 0.2em 0.6em; border-radius: 9999px; font-size: 0.875rem; font-weight: 600; }
.badge-default { background-color: #6c757d; color: white; }
.badge-primary { background-color: #007bff; color: white; }
.badge-success { background-color: #28a745; color: white; }
.badge-warning { background-color: #ffc107; color: #212529; }
.badge-error { background-color: #dc3545; color: white; }

progress { accent-color: #007bff; }
.checkbox input, .switch input { margin-right: 0.5rem; }
"#;

/// Format `config`'s attribution as a comment, for generators that write one.
///
/// `prefix` and `suffix` are the target language's comment delimiters, so each
/// generator keeps its own syntax while sharing one switch. Empty string when no
/// attribution was configured, which is the default.
///
/// When the delimiters are a comment whose body cannot contain a particular
/// sequence, that sequence is broken up first. An HTML comment ends at the first
/// `--` and a CSS comment at the first `*/`, so an attribution containing `-->`
/// or `*/` would close the comment early and the rest of the text would become
/// live markup or live CSS. Callers pass whatever text they like, so the escaping
/// has to happen here rather than being left to each generator.
pub(crate) fn attribution(config: &GeneratorConfig, prefix: &str, suffix: &str) -> String {
    match &config.attribution {
        Some(text) => {
            let body = if prefix.contains("<!--") {
                text.replace("--", "- -")
            } else if prefix.contains("/*") {
                text.replace("*/", "* /")
            } else {
                text.clone()
            };
            format!("{prefix}{body}{suffix}\n")
        }
        None => String::new(),
    }
}
