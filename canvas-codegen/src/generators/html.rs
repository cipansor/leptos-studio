//! HtmlGenerator: html output.

use crate::config::GeneratorConfig;
use crate::domain::{AppResult, CanvasComponent, Variable};
use crate::generator::CodeGenerator;
use crate::utils::helpers::{attribution, component_inline_css};
use crate::utils::sanitize::escape_html;

/// HTML code generator
pub struct HtmlGenerator;

/// The name this generator had before the crate was extracted.
///
/// A delegating struct rather than a `type` alias: the old name was a unit
/// struct, so callers wrote `HtmlCodeGenerator.generate(...)`. A type alias
/// would exist only in the type namespace and reject that. Prefer
/// [`HtmlGenerator`] in new code.
pub struct HtmlCodeGenerator;

impl CodeGenerator for HtmlCodeGenerator {
    fn generate_with(
        &self,
        config: &GeneratorConfig,
        components: &[CanvasComponent],
        variables: &[Variable],
    ) -> AppResult<String> {
        HtmlGenerator.generate_with(config, components, variables)
    }

    fn file_extension(&self) -> &str {
        HtmlGenerator.file_extension()
    }
}

impl CodeGenerator for HtmlGenerator {
    fn generate_with(
        &self,
        config: &GeneratorConfig,
        components: &[CanvasComponent],
        _variables: &[Variable],
    ) -> AppResult<String> {
        // The attribution goes after the doctype rather than before it: a comment
        // ahead of `<!DOCTYPE html>` puts some browsers into quirks mode.
        let mut output = String::from("<!DOCTYPE html>\n");
        output.push_str(&attribution(config, "<!-- ", " -->"));
        output.push_str(
            "<html>\n<head>\n    <meta charset=\"UTF-8\">\n    <title>Generated Layout</title>\n</head>\n<body>\n",
        );

        for component in components {
            self.generate_html(component, &mut output, 1)?;
        }

        output.push_str("</body>\n</html>");
        Ok(output)
    }

    fn file_extension(&self) -> &str {
        "html"
    }
}

impl HtmlGenerator {
    #[allow(clippy::only_used_in_recursion)]
    fn generate_html(
        &self,
        component: &CanvasComponent,
        output: &mut String,
        indent_level: usize,
    ) -> AppResult<()> {
        let indent = "    ".repeat(indent_level);

        match component {
            CanvasComponent::Button(btn) => {
                output.push_str(&format!(
                    "{}<button{}>{}</button>\n",
                    indent,
                    if btn.disabled { " disabled" } else { "" },
                    btn.label
                ));
            }
            CanvasComponent::Text(txt) => {
                let tag = match txt.tag {
                    crate::domain::TextTag::H1 => "h1",
                    crate::domain::TextTag::H2 => "h2",
                    crate::domain::TextTag::H3 => "h3",
                    crate::domain::TextTag::P => "p",
                    crate::domain::TextTag::Span => "span",
                };
                output.push_str(&format!("{}<{}>{}</{}>\n", indent, tag, txt.content, tag));
            }
            CanvasComponent::Input(inp) => {
                let input_type = match inp.input_type {
                    crate::domain::InputType::Text => "text",
                    crate::domain::InputType::Password => "password",
                    crate::domain::InputType::Email => "email",
                    crate::domain::InputType::Number => "number",
                    crate::domain::InputType::Tel => "tel",
                };
                output.push_str(&format!(
                    "{}<input type=\"{}\" placeholder=\"{}\"{}{}>\n",
                    indent,
                    input_type,
                    inp.placeholder,
                    if inp.required { " required" } else { "" },
                    if inp.disabled { " disabled" } else { "" }
                ));
            }
            CanvasComponent::Select(sel) => {
                output.push_str(&format!(
                    "{}<select{}>\n",
                    indent,
                    if sel.disabled { " disabled" } else { "" }
                ));
                if !sel.placeholder.is_empty() {
                    output.push_str(&format!(
                        "{}    <option value=\"\" disabled selected>{}</option>\n",
                        indent, sel.placeholder
                    ));
                }
                for option in sel.options.split(',') {
                    let opt = option.trim();
                    output.push_str(&format!(
                        "{}    <option value=\"{}\">{}</option>\n",
                        indent, opt, opt
                    ));
                }
                output.push_str(&format!("{}</select>\n", indent));
            }
            CanvasComponent::Container(container) => {
                output.push_str(&format!("{}<div>\n", indent));
                for child in &container.children {
                    self.generate_html(child, output, indent_level + 1)?;
                }
                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Image(img) => {
                let width_attr = img
                    .width
                    .as_ref()
                    .map_or(String::new(), |w| format!(" width=\"{}\"", w));
                let height_attr = img
                    .height
                    .as_ref()
                    .map_or(String::new(), |h| format!(" height=\"{}\"", h));
                output.push_str(&format!(
                    "{}<img src=\"{}\" alt=\"{}\"{}{} />\n",
                    indent, img.src, img.alt, width_attr, height_attr
                ));
            }
            CanvasComponent::Card(card) => {
                let shadow_style = if card.shadow {
                    "box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1);"
                } else {
                    ""
                };
                let border_style = if card.border {
                    "border: 1px solid #e5e7eb;"
                } else {
                    ""
                };
                output.push_str(&format!(
                    "{}<div style=\"padding: {}px; border-radius: {}px; {} {}\">\n",
                    indent, card.padding, card.border_radius, shadow_style, border_style
                ));
                for child in &card.children {
                    self.generate_html(child, output, indent_level + 1)?;
                }
                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Custom(custom) => {
                output.push_str(&format!("{}<!-- {} -->\n", indent, custom.name));
                output.push_str(&format!("{}{}\n", indent, custom.template));
            }
            CanvasComponent::Divider(divider) => {
                let style = match divider.orientation {
                    crate::domain::DividerOrientation::Horizontal => format!(
                        "border: none; border-top: {}px solid #e5e7eb; margin: 8px 0;",
                        divider.thickness
                    ),
                    crate::domain::DividerOrientation::Vertical => format!(
                        "border: none; border-left: {}px solid #e5e7eb; margin: 0 8px; align-self: stretch;",
                        divider.thickness
                    ),
                };
                output.push_str(&format!("{}<hr style=\"{}\">\n", indent, style));
            }
            CanvasComponent::Checkbox(checkbox) => {
                output.push_str(&format!(
                    "{}<label><input type=\"checkbox\"{}{}>{}</label>\n",
                    indent,
                    if checkbox.checked { " checked" } else { "" },
                    if checkbox.disabled { " disabled" } else { "" },
                    checkbox.label
                ));
            }
            CanvasComponent::RadioGroup(radio) => {
                let name = format!("radio-group-{}", radio.id);
                output.push_str(&format!("{}<div class=\"radio-group\">\n", indent));
                for opt in radio
                    .options
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                {
                    let checked = !radio.selected.is_empty() && radio.selected == opt;
                    output.push_str(&format!(
                        "{}    <label><input type=\"radio\" name=\"{}\"{}{}>{}</label>\n",
                        indent,
                        name,
                        if checked { " checked" } else { "" },
                        if radio.disabled { " disabled" } else { "" },
                        opt
                    ));
                }
                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Switch(switch) => {
                output.push_str(&format!(
                    "{}<label class=\"switch\"><input type=\"checkbox\"{}{}>{}</label>\n",
                    indent,
                    if switch.checked { " checked" } else { "" },
                    if switch.disabled { " disabled" } else { "" },
                    switch.label
                ));
            }
            CanvasComponent::Badge(badge) => {
                output.push_str(&format!(
                    "{}<span class=\"badge badge-{}\">{}</span>\n",
                    indent,
                    match badge.variant {
                        crate::domain::BadgeVariant::Default => "default",
                        crate::domain::BadgeVariant::Primary => "primary",
                        crate::domain::BadgeVariant::Success => "success",
                        crate::domain::BadgeVariant::Warning => "warning",
                        crate::domain::BadgeVariant::Error => "error",
                    },
                    badge.text
                ));
            }
            CanvasComponent::Progress(progress) => {
                output.push_str(&format!(
                    "{}<progress value=\"{}\" max=\"{}\" style=\"width: 100%;\"></progress>\n",
                    indent, progress.value, progress.max
                ));
            }
            CanvasComponent::Link(link) => {
                let inline_css = component_inline_css(&link.style, &link.animation);
                let style_attr = if inline_css.is_empty() {
                    String::new()
                } else {
                    format!(" style=\"{}\"", escape_html(&inline_css))
                };
                output.push_str(&format!(
                    "{}<a href=\"{}\"{}>{}</a>\n",
                    indent,
                    escape_html(&link.href),
                    style_attr,
                    escape_html(&link.text)
                ));
            }
        }

        Ok(())
    }
}
