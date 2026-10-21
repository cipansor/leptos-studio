//! TailwindHtmlGenerator: tailwind output.

use crate::config::GeneratorConfig;
use crate::domain::{AppResult, CanvasComponent, Variable};
use crate::generator::CodeGenerator;
use crate::utils::helpers::{attribution, component_inline_css};
use crate::utils::sanitize::escape_html;

/// HTML with Tailwind CSS classes generator
pub struct TailwindHtmlGenerator;

impl CodeGenerator for TailwindHtmlGenerator {
    fn generate_with(
        &self,
        config: &GeneratorConfig,
        components: &[CanvasComponent],
        _variables: &[Variable],
    ) -> AppResult<String> {
        // The attribution goes after the doctype rather than before it: a comment
        // ahead of `<!DOCTYPE html>` puts some browsers into quirks mode.
        let mut output = String::from("<!DOCTYPE html>\n");
        output.push_str(&attribution(config, "<!-- ", " with Tailwind CSS -->"));
        output.push_str("<html lang=\"en\">\n");
        output.push_str("<head>\n");
        output.push_str("  <meta charset=\"UTF-8\">\n");
        output.push_str(
            "  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        output.push_str("  <script src=\"https://cdn.tailwindcss.com\"></script>\n");
        output.push_str("  <title>Generated Layout</title>\n");
        output.push_str("</head>\n");
        output.push_str("<body class=\"min-h-screen bg-gray-50\">\n");
        output.push_str("  <main class=\"container mx-auto p-4\">\n");

        for component in components {
            Self::generate_tailwind(component, &mut output, 2)?;
        }

        output.push_str("  </main>\n");
        output.push_str("</body>\n");
        output.push_str("</html>\n");

        Ok(output)
    }

    fn file_extension(&self) -> &str {
        "html"
    }
}

impl TailwindHtmlGenerator {
    fn generate_tailwind(
        component: &CanvasComponent,
        output: &mut String,
        indent_level: usize,
    ) -> AppResult<()> {
        let indent = "  ".repeat(indent_level);

        match component {
            CanvasComponent::Button(btn) => {
                let variant_classes = match btn.variant {
                    crate::domain::ButtonVariant::Primary => {
                        "bg-blue-600 hover:bg-blue-700 text-white"
                    }
                    crate::domain::ButtonVariant::Secondary => {
                        "bg-gray-200 hover:bg-gray-300 text-gray-800"
                    }
                    crate::domain::ButtonVariant::Outline => {
                        "bg-transparent border-2 border-blue-600 text-blue-600 hover:bg-blue-50"
                    }
                    crate::domain::ButtonVariant::Ghost => {
                        "bg-transparent text-gray-600 hover:bg-gray-100"
                    }
                };

                let size_classes = match btn.size {
                    crate::domain::ButtonSize::Small => "px-3 py-1 text-sm",
                    crate::domain::ButtonSize::Medium => "px-4 py-2 text-base",
                    crate::domain::ButtonSize::Large => "px-6 py-3 text-lg",
                };

                let disabled_classes = if btn.disabled {
                    "opacity-50 cursor-not-allowed"
                } else {
                    "cursor-pointer"
                };

                output.push_str(&format!(
                    "{}<button class=\"rounded-md font-medium transition-colors {} {} {}\"{}>{}</button>\n",
                    indent,
                    variant_classes,
                    size_classes,
                    disabled_classes,
                    if btn.disabled { " disabled" } else { "" },
                    btn.label
                ));
            }
            CanvasComponent::Text(txt) => {
                let (tag, classes) = match txt.tag {
                    crate::domain::TextTag::H1 => ("h1", "text-4xl font-bold text-gray-900"),
                    crate::domain::TextTag::H2 => ("h2", "text-3xl font-semibold text-gray-800"),
                    crate::domain::TextTag::H3 => ("h3", "text-2xl font-medium text-gray-700"),
                    crate::domain::TextTag::P => ("p", "text-base text-gray-600"),
                    crate::domain::TextTag::Span => ("span", "text-base text-gray-600"),
                };

                output.push_str(&format!(
                    "{}<{} class=\"{}\">{}</{}>\n",
                    indent, tag, classes, txt.content, tag
                ));
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
                    "{}<input type=\"{}\" placeholder=\"{}\" class=\"w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-transparent\"{}{}>\n",
                    indent,
                    input_type,
                    inp.placeholder,
                    if inp.required { " required" } else { "" },
                    if inp.disabled { " disabled" } else { "" }
                ));
            }
            CanvasComponent::Select(sel) => {
                output.push_str(&format!("{}<select class=\"block w-full px-4 py-2 border border-gray-300 rounded-md shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500\" {}>\n", indent, if sel.disabled { "disabled" } else { "" }));
                if !sel.placeholder.is_empty() {
                    output.push_str(&format!(
                        "{}  <option value=\"\" disabled selected>{}</option>\n",
                        indent, sel.placeholder
                    ));
                }
                for option in sel.options.split(',') {
                    let opt = option.trim();
                    output.push_str(&format!(
                        "{}  <option value=\"{}\">{}</option>\n",
                        indent, opt, opt
                    ));
                }
                output.push_str(&format!("{}</select>\n", indent));
            }
            CanvasComponent::Container(container) => {
                let layout_classes = match &container.layout {
                    crate::domain::LayoutType::Flex {
                        direction,
                        wrap,
                        align_items,
                        justify_content,
                    } => {
                        let dir = match direction {
                            crate::domain::FlexDirection::Row => "flex-row",
                            crate::domain::FlexDirection::Column => "flex-col",
                        };
                        let wrap_cls = if *wrap { "flex-wrap" } else { "" };

                        let align_cls = match align_items {
                            crate::domain::FlexAlign::Start => "items-start",
                            crate::domain::FlexAlign::Center => "items-center",
                            crate::domain::FlexAlign::End => "items-end",
                            crate::domain::FlexAlign::Stretch => "items-stretch",
                            crate::domain::FlexAlign::Baseline => "items-baseline",
                        };

                        let justify_cls = match justify_content {
                            crate::domain::FlexJustify::Start => "justify-start",
                            crate::domain::FlexJustify::Center => "justify-center",
                            crate::domain::FlexJustify::End => "justify-end",
                            crate::domain::FlexJustify::Between => "justify-between",
                            crate::domain::FlexJustify::Around => "justify-around",
                            crate::domain::FlexJustify::Evenly => "justify-evenly",
                        };

                        format!("flex {} {} {} {}", dir, wrap_cls, align_cls, justify_cls)
                    }
                    crate::domain::LayoutType::Grid { columns, .. } => {
                        format!("grid grid-cols-{}", columns.min(&12))
                    }
                    crate::domain::LayoutType::Stack => "flex flex-col".to_string(),
                };

                let gap_class = format!("gap-{}", (container.gap / 4).clamp(1, 16));
                let padding_class = format!(
                    "pt-{} pr-{} pb-{} pl-{}",
                    (container.padding.top / 4).min(16),
                    (container.padding.right / 4).min(16),
                    (container.padding.bottom / 4).min(16),
                    (container.padding.left / 4).min(16)
                );

                output.push_str(&format!(
                    "{}<div class=\"{} {} {}\">\n",
                    indent, layout_classes, gap_class, padding_class
                ));

                for child in &container.children {
                    Self::generate_tailwind(child, output, indent_level + 1)?;
                }

                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Image(img) => {
                output.push_str(&format!(
                    "{}<img src=\"{}\" alt=\"{}\" class=\"max-w-full h-auto\" />\n",
                    indent, img.src, img.alt
                ));
            }
            CanvasComponent::Card(card) => {
                let shadow_class = if card.shadow { "shadow-md" } else { "" };
                let border_class = if card.border {
                    "border border-gray-200"
                } else {
                    ""
                };
                let padding_class = format!("p-{}", (card.padding / 4).max(1));
                let rounded_class = match card.border_radius {
                    0 => "rounded-none",
                    1..=4 => "rounded-sm",
                    5..=8 => "rounded",
                    9..=12 => "rounded-md",
                    _ => "rounded-lg",
                };

                output.push_str(&format!(
                    "{}<div class=\"bg-white {} {} {} {}\">\n",
                    indent, shadow_class, border_class, padding_class, rounded_class
                ));

                for child in &card.children {
                    Self::generate_tailwind(child, output, indent_level + 1)?;
                }

                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Custom(custom) => {
                output.push_str(&format!("{}<!-- Custom: {} -->\n", indent, custom.name));
                output.push_str(&format!("{}<div class=\"custom-component\">\n", indent));
                output.push_str(&format!("{}  {}\n", indent, custom.template));
                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Divider(divider) => {
                let class = match divider.orientation {
                    crate::domain::DividerOrientation::Horizontal => {
                        "border-t border-gray-200 my-2"
                    }
                    crate::domain::DividerOrientation::Vertical => {
                        "border-l border-gray-200 mx-2 self-stretch"
                    }
                };
                output.push_str(&format!("{}<hr class=\"{}\" />\n", indent, class));
            }
            CanvasComponent::Checkbox(checkbox) => {
                output.push_str(&format!(
                    "{}<label class=\"flex items-center gap-2\">\n{}  <input type=\"checkbox\" class=\"rounded border-gray-300\"{}{} />\n{}  <span>{}</span>\n{}</label>\n",
                    indent,
                    indent,
                    if checkbox.checked { " checked" } else { "" },
                    if checkbox.disabled { " disabled" } else { "" },
                    indent,
                    checkbox.label,
                    indent
                ));
            }
            CanvasComponent::RadioGroup(radio) => {
                let name = format!("radio-{}", radio.id);
                output.push_str(&format!("{}<div class=\"flex flex-col gap-1\">\n", indent));
                for opt in radio
                    .options
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                {
                    let checked = !radio.selected.is_empty() && radio.selected == opt;
                    output.push_str(&format!(
                        "{}  <label class=\"flex items-center gap-2\"><input type=\"radio\" name=\"{}\"{}{} /> {}</label>\n",
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
                    "{}<label class=\"flex items-center gap-2\">\n{}  <input type=\"checkbox\" class=\"toggle\"{}{} />\n{}  <span>{}</span>\n{}</label>\n",
                    indent,
                    indent,
                    if switch.checked { " checked" } else { "" },
                    if switch.disabled { " disabled" } else { "" },
                    indent,
                    switch.label,
                    indent
                ));
            }
            CanvasComponent::Badge(badge) => {
                let color = match badge.variant {
                    crate::domain::BadgeVariant::Default => "bg-gray-100 text-gray-800",
                    crate::domain::BadgeVariant::Primary => "bg-blue-100 text-blue-800",
                    crate::domain::BadgeVariant::Success => "bg-green-100 text-green-800",
                    crate::domain::BadgeVariant::Warning => "bg-amber-100 text-amber-800",
                    crate::domain::BadgeVariant::Error => "bg-red-100 text-red-800",
                };
                output.push_str(&format!(
                    "{}<span class=\"inline-block px-2 py-0.5 rounded-full text-xs font-medium {}\">{}</span>\n",
                    indent, color, badge.text
                ));
            }
            CanvasComponent::Progress(progress) => {
                let percent = if progress.max > 0.0 {
                    (progress.value / progress.max * 100.0).clamp(0.0, 100.0)
                } else {
                    0.0
                };
                output.push_str(&format!(
                    "{}<div class=\"w-full bg-gray-200 rounded-full h-2\">\n{}  <div class=\"bg-blue-500 h-2 rounded-full\" style=\"width: {:.0}%\"></div>\n{}</div>\n",
                    indent, indent, percent, indent
                ));
            }
            CanvasComponent::Link(link) => {
                // The canvas link styling is the visual contract; class-based
                // Tailwind output keeps that styling as a class alongside the
                // link colour, plus any per-component overrides inline.
                let inline_css = component_inline_css(&link.style, &link.animation);
                let style_attr = if inline_css.is_empty() {
                    String::new()
                } else {
                    format!(" style=\"{}\"", escape_html(&inline_css))
                };
                output.push_str(&format!(
                    "{}<a href=\"{}\" class=\"text-blue-600 underline hover:text-blue-800\"{}>{}</a>\n",
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
