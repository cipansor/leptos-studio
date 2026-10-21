//! SvelteGenerator: svelte output.

use crate::config::GeneratorConfig;
use crate::domain::{AppResult, CanvasComponent, Variable, VariableType};
use crate::generator::CodeGenerator;
use crate::utils::helpers::{attribution, component_inline_css};
use crate::utils::sanitize::escape_html;

/// Svelte component generator
pub struct SvelteGenerator;

impl CodeGenerator for SvelteGenerator {
    fn generate_with(
        &self,
        config: &GeneratorConfig,
        components: &[CanvasComponent],
        variables: &[Variable],
    ) -> AppResult<String> {
        let mut output = String::new();

        // Script section
        output.push_str("<script lang=\"ts\">\n");
        output.push_str(&attribution(config, "  // ", ""));

        let vars_init = if variables.is_empty() {
            "{}".to_string()
        } else {
            let entries: Vec<String> = variables
                .iter()
                .map(|v| {
                    let val = match v.data_type {
                        VariableType::String => format!("'{}'", v.default_value),
                        _ => v.default_value.clone(),
                    };
                    format!("{}: {}", v.name, val)
                })
                .collect();
            format!("{{ {} }}", entries.join(", "))
        };

        output.push_str(&format!("  let vars = {};\n", vars_init));
        output.push_str("</script>\n\n");

        // Template section
        output.push_str("<div class=\"generated-layout\">\n");

        for component in components {
            Self::generate_svelte(component, &mut output, 1)?;
        }

        output.push_str("</div>\n\n");

        // Style section
        output.push_str("<style>\n");
        output.push_str("  .generated-layout {\n");
        output.push_str("    /* Add your styles here */\n");
        output.push_str("  }\n");
        output.push_str("  \n");
        output.push_str("  .btn {\n");
        output.push_str("    padding: 8px 16px;\n");
        output.push_str("    border-radius: 6px;\n");
        output.push_str("    cursor: pointer;\n");
        output.push_str("    transition: all 0.2s;\n");
        output.push_str("  }\n");
        output.push_str("  \n");
        output.push_str("  .btn-primary {\n");
        output.push_str("    background: #3b82f6;\n");
        output.push_str("    color: white;\n");
        output.push_str("    border: none;\n");
        output.push_str("  }\n");
        output.push_str("  \n");
        output.push_str("  .btn-secondary {\n");
        output.push_str("    background: #e5e7eb;\n");
        output.push_str("    color: #374151;\n");
        output.push_str("    border: none;\n");
        output.push_str("  }\n");
        output.push_str("  \n");
        output.push_str("  input {\n");
        output.push_str("    width: 100%;\n");
        output.push_str("    padding: 8px 12px;\n");
        output.push_str("    border: 1px solid #d1d5db;\n");
        output.push_str("    border-radius: 6px;\n");
        output.push_str("  }\n");
        output.push_str("</style>\n");

        Ok(output)
    }

    fn file_extension(&self) -> &str {
        "svelte"
    }
}

impl SvelteGenerator {
    fn generate_svelte(
        component: &CanvasComponent,
        output: &mut String,
        indent_level: usize,
    ) -> AppResult<()> {
        let indent = "  ".repeat(indent_level);

        match component {
            CanvasComponent::Button(btn) => {
                let id_attr = if let Some(bind) = btn.bindings.get("id") {
                    format!(" id={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = btn.bindings.get("custom_css_classes") {
                    format!(" class={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let variant_class = match btn.variant {
                    crate::domain::ButtonVariant::Primary => "btn btn-primary",
                    crate::domain::ButtonVariant::Secondary => "btn btn-secondary",
                    crate::domain::ButtonVariant::Outline => "btn btn-outline",
                    crate::domain::ButtonVariant::Ghost => "btn btn-ghost",
                };

                let label_expr = if let Some(bind) = btn.bindings.get("label") {
                    format!("{{vars['{}']}}", bind)
                } else {
                    btn.label.clone()
                };

                let disabled_expr = if let Some(bind) = btn.bindings.get("disabled") {
                    format!("disabled={{vars['{}']}}", bind)
                } else {
                    if btn.disabled { " disabled" } else { "" }.to_string()
                };

                output.push_str(&format!(
                    "{}<button{}{} class=\"{}\" {}>{}</button>\n",
                    indent, id_attr, class_attr, variant_class, disabled_expr, label_expr
                ));
            }
            CanvasComponent::Text(txt) => {
                let id_attr = if let Some(bind) = txt.bindings.get("id") {
                    format!(" id={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = txt.bindings.get("custom_css_classes") {
                    format!(" class={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let tag = match txt.tag {
                    crate::domain::TextTag::H1 => "h1",
                    crate::domain::TextTag::H2 => "h2",
                    crate::domain::TextTag::H3 => "h3",
                    crate::domain::TextTag::P => "p",
                    crate::domain::TextTag::Span => "span",
                };

                let content_expr = if let Some(bind) = txt.bindings.get("content") {
                    format!("{{vars['{}']}}", bind)
                } else {
                    txt.content.clone()
                };

                output.push_str(&format!(
                    "{}<{} {}{}>{}</{}>\n",
                    indent, tag, id_attr, class_attr, content_expr, tag
                ));
            }
            CanvasComponent::Input(inp) => {
                let id_attr = if let Some(bind) = inp.bindings.get("id") {
                    format!(" id={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = inp.bindings.get("custom_css_classes") {
                    format!(" class={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let input_type = match inp.input_type {
                    crate::domain::InputType::Text => "text",
                    crate::domain::InputType::Password => "password",
                    crate::domain::InputType::Email => "email",
                    crate::domain::InputType::Number => "number",
                    crate::domain::InputType::Tel => "tel",
                };

                let placeholder_attr = if let Some(bind) = inp.bindings.get("placeholder") {
                    format!("placeholder={{vars['{}']}}", bind)
                } else {
                    format!("placeholder=\"{}\"", inp.placeholder)
                };

                let disabled_attr = if let Some(bind) = inp.bindings.get("disabled") {
                    format!("disabled={{vars['{}']}}", bind)
                } else {
                    if inp.disabled { " disabled" } else { "" }.to_string()
                };

                output.push_str(&format!(
                    "{}<input{}{} type=\"{}\" {} {} {} />\n",
                    indent,
                    id_attr,
                    class_attr,
                    input_type,
                    placeholder_attr,
                    if inp.required { " required" } else { "" },
                    disabled_attr
                ));
            }
            CanvasComponent::Select(sel) => {
                let id_attr = if let Some(bind) = sel.bindings.get("id") {
                    format!(" id={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = sel.bindings.get("custom_css_classes") {
                    format!(" class={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let disabled_attr = if let Some(bind) = sel.bindings.get("disabled") {
                    format!("disabled={{vars['{}']}}", bind)
                } else {
                    if sel.disabled { " disabled" } else { "" }.to_string()
                };

                output.push_str(&format!(
                    "{}<select{}{} {}>\n",
                    indent, id_attr, class_attr, disabled_attr
                ));
                if !sel.placeholder.is_empty() {
                    let placeholder_expr = if let Some(bind) = sel.bindings.get("placeholder") {
                        format!("{{vars['{}']}}", bind)
                    } else {
                        sel.placeholder.clone()
                    };
                    output.push_str(&format!(
                        "{}  <option value=\"\" disabled selected>{}</option>\n",
                        indent, placeholder_expr
                    ));
                }

                if let Some(bind) = sel.bindings.get("options") {
                    output.push_str(&format!(
                        "{}  {{#each (vars['{}']?.split(',') || []) as opt}}\n",
                        indent, bind
                    ));
                    output.push_str(&format!(
                        "{}    <option value={{opt.trim()}}>{{opt.trim()}}</option>\n",
                        indent
                    ));
                    output.push_str(&format!("{}  {{/each}}\n", indent));
                } else {
                    for option in sel.options.split(',') {
                        let opt = option.trim();
                        output.push_str(&format!(
                            "{}  <option value=\"{}\">{}</option>\n",
                            indent, opt, opt
                        ));
                    }
                }
                output.push_str(&format!("{}</select>\n", indent));
            }
            CanvasComponent::Container(container) => {
                let style = format!(
                    "display: flex; flex-direction: {}; gap: {}px; padding: {}px {}px {}px {}px;",
                    match &container.layout {
                        crate::domain::LayoutType::Flex { direction, .. } => {
                            match direction {
                                crate::domain::FlexDirection::Row => "row",
                                crate::domain::FlexDirection::Column => "column",
                            }
                        }
                        _ => "column",
                    },
                    container.gap,
                    container.padding.top,
                    container.padding.right,
                    container.padding.bottom,
                    container.padding.left
                );

                let id_attr = if let Some(bind) = container.bindings.get("id") {
                    format!(" id={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = container.bindings.get("custom_css_classes") {
                    format!(" class={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                output.push_str(&format!(
                    "{}<div{}{} style=\"{}\">\n",
                    indent, id_attr, class_attr, style
                ));

                for child in &container.children {
                    Self::generate_svelte(child, output, indent_level + 1)?;
                }

                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Image(img) => {
                let id_attr = if let Some(bind) = img.bindings.get("id") {
                    format!(" id={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = img.bindings.get("custom_css_classes") {
                    format!(" class={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let src_val = if let Some(bind) = img.bindings.get("src") {
                    format!("src={{vars['{}']}}", bind)
                } else {
                    format!("src=\"{}\"", img.src)
                };

                let alt_val = if let Some(bind) = img.bindings.get("alt") {
                    format!("alt={{vars['{}']}}", bind)
                } else {
                    format!("alt=\"{}\"", img.alt)
                };

                output.push_str(&format!(
                    "{}<img{}{} {} {} />\n",
                    indent, id_attr, class_attr, src_val, alt_val
                ));
            }
            CanvasComponent::Card(card) => {
                let style = format!(
                    "padding: {}px; border-radius: {}px; {}; {}",
                    card.padding,
                    card.border_radius,
                    if card.shadow {
                        "box-shadow: 0 4px 6px -1px rgba(0,0,0,0.1)"
                    } else {
                        ""
                    },
                    if card.border {
                        "border: 1px solid #e5e7eb"
                    } else {
                        ""
                    }
                );

                let id_attr = if let Some(bind) = card.bindings.get("id") {
                    format!(" id={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = card.bindings.get("custom_css_classes") {
                    format!(" class={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                output.push_str(&format!(
                    "{}<div{}{} style=\"{}\">\n",
                    indent, id_attr, class_attr, style
                ));

                for child in &card.children {
                    Self::generate_svelte(child, output, indent_level + 1)?;
                }

                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Custom(custom) => {
                let id_attr = if let Some(bind) = custom.bindings.get("id") {
                    format!(" id={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = custom.bindings.get("custom_css_classes") {
                    format!(" class={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                output.push_str(&format!("{}<!-- Custom: {} -->\n", indent, custom.name));
                output.push_str(&format!(
                    "{}<div{}{}>{{@html `{}`}}</div>\n",
                    indent, id_attr, class_attr, custom.template
                ));
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
                output.push_str(&format!("{}<hr style=\"{}\" />\n", indent, style));
            }
            CanvasComponent::Checkbox(checkbox) => {
                output.push_str(&format!(
                    "{}<label>\n{}  <input type=\"checkbox\"{}{} />\n{}  {}\n{}</label>\n",
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
                output.push_str(&format!("{}<div class=\"radio-group\">\n", indent));
                for opt in radio
                    .options
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                {
                    let checked = !radio.selected.is_empty() && radio.selected == opt;
                    output.push_str(&format!(
                        "{}  <label><input type=\"radio\" name=\"{}\"{}{} /> {}</label>\n",
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
                    "{}<label class=\"switch\">\n{}  <input type=\"checkbox\"{}{} />\n{}  {}\n{}</label>\n",
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
                let variant = match badge.variant {
                    crate::domain::BadgeVariant::Default => "default",
                    crate::domain::BadgeVariant::Primary => "primary",
                    crate::domain::BadgeVariant::Success => "success",
                    crate::domain::BadgeVariant::Warning => "warning",
                    crate::domain::BadgeVariant::Error => "error",
                };
                output.push_str(&format!(
                    "{}<span class=\"badge badge-{}\">{}</span>\n",
                    indent, variant, badge.text
                ));
            }
            CanvasComponent::Progress(progress) => {
                output.push_str(&format!(
                    "{}<progress value={{{}}} max={{{}}} style=\"width: 100%;\"></progress>\n",
                    indent, progress.value, progress.max
                ));
            }
            CanvasComponent::Link(link) => {
                let id_attr = if let Some(bind) = link.bindings.get("id") {
                    format!(" id={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = link.bindings.get("custom_css_classes") {
                    format!(" class={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let href_attr = if let Some(bind) = link.bindings.get("href") {
                    format!("href={{vars['{}']}}", bind)
                } else {
                    format!("href=\"{}\"", escape_html(&link.href))
                };

                let text_expr = if let Some(bind) = link.bindings.get("text") {
                    format!("{{vars['{}']}}", bind)
                } else {
                    escape_html(&link.text).to_string()
                };

                let inline_css = component_inline_css(&link.style, &link.animation);
                let style_attr = if inline_css.is_empty() {
                    String::new()
                } else {
                    format!(" style=\"{}\"", escape_html(&inline_css))
                };

                output.push_str(&format!(
                    "{}<a{}{} {}{}>{}</a>\n",
                    indent, id_attr, class_attr, href_attr, style_attr, text_expr
                ));
            }
        }

        Ok(())
    }
}
