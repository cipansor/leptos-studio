//! ReactGenerator: react output.

use crate::config::GeneratorConfig;
use crate::domain::{AppResult, CanvasComponent, Variable, VariableType};
use crate::generator::CodeGenerator;
use crate::utils::helpers::{js_string_literal, style_js_properties};
use crate::utils::sanitize::escape_html;

/// React component generator
pub struct ReactGenerator;

impl CodeGenerator for ReactGenerator {
    fn generate_with(
        &self,
        _config: &GeneratorConfig,
        components: &[CanvasComponent],
        variables: &[Variable],
    ) -> AppResult<String> {
        let mut output = String::new();

        // Imports
        output.push_str("import React from 'react';\n\n");

        // Generate component
        output.push_str("export function GeneratedLayout() {\n");
        output.push_str("  // Signals / State\n");

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

        output.push_str(&format!(
            "  const [vars, setVars] = React.useState({});\n\n",
            vars_init
        ));
        output.push_str("  return (\n");
        output.push_str("    <>\n");

        for component in components {
            Self::generate_react(component, &mut output, 3)?;
        }

        output.push_str("    </>\n");
        output.push_str("  );\n");
        output.push_str("}\n\n");

        output.push_str("export default GeneratedLayout;\n");

        Ok(output)
    }

    fn file_extension(&self) -> &str {
        "tsx"
    }
}

impl ReactGenerator {
    fn generate_react(
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
                    format!(" className={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let variant_class = match btn.variant {
                    crate::domain::ButtonVariant::Primary => "btn-primary",
                    crate::domain::ButtonVariant::Secondary => "btn-secondary",
                    crate::domain::ButtonVariant::Outline => "btn-outline",
                    crate::domain::ButtonVariant::Ghost => "btn-ghost",
                };

                let size_class = match btn.size {
                    crate::domain::ButtonSize::Small => "btn-sm",
                    crate::domain::ButtonSize::Medium => "btn-md",
                    crate::domain::ButtonSize::Large => "btn-lg",
                };

                let label_expr = if let Some(bind) = btn.bindings.get("label") {
                    format!("{{vars['{}']}}", bind)
                } else {
                    btn.label.clone()
                };

                let disabled_expr = if let Some(bind) = btn.bindings.get("disabled") {
                    format!("{{vars['{}']}}", bind)
                } else {
                    btn.disabled.to_string()
                };

                output.push_str(&format!(
                    "{}<button{} {} className=\"{} {}\" disabled={{{}}}>{}</button>\n",
                    indent,
                    id_attr,
                    class_attr,
                    variant_class,
                    size_class,
                    disabled_expr,
                    label_expr
                ));
            }
            CanvasComponent::Text(txt) => {
                let id_attr = if let Some(bind) = txt.bindings.get("id") {
                    format!(" id={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = txt.bindings.get("custom_css_classes") {
                    format!(" className={{vars['{}']}}", bind)
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
                    format!(" className={{vars['{}']}}", bind)
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

                let placeholder_expr = if let Some(bind) = inp.bindings.get("placeholder") {
                    format!("{{vars['{}']}}", bind)
                } else {
                    format!("\"{}\"", inp.placeholder)
                };

                let disabled_expr = if let Some(bind) = inp.bindings.get("disabled") {
                    format!("{{vars['{}']}}", bind)
                } else {
                    inp.disabled.to_string()
                };

                output.push_str(&format!(
                    "{}<input{} {} type=\"{}\" placeholder={} required={{{}}} disabled={{{}}} />\n",
                    indent,
                    id_attr,
                    class_attr,
                    input_type,
                    placeholder_expr,
                    inp.required,
                    disabled_expr
                ));
            }
            CanvasComponent::Select(sel) => {
                let id_attr = if let Some(bind) = sel.bindings.get("id") {
                    format!(" id={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = sel.bindings.get("custom_css_classes") {
                    format!(" className={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let disabled_expr = if let Some(bind) = sel.bindings.get("disabled") {
                    format!("{{vars['{}']}}", bind)
                } else {
                    sel.disabled.to_string()
                };

                output.push_str(&format!(
                    "{}<select{} {} disabled={{{}}}>\n",
                    indent, id_attr, class_attr, disabled_expr
                ));
                if !sel.placeholder.is_empty() {
                    let placeholder_expr = if let Some(bind) = sel.bindings.get("placeholder") {
                        format!("{{vars['{}']}}", bind)
                    } else {
                        format!("\" {} \"", sel.placeholder)
                    };
                    output.push_str(&format!(
                        "{}  <option value=\"\" disabled selected>{{{}}}</option>\n",
                        indent, placeholder_expr
                    ));
                }

                if let Some(bind) = sel.bindings.get("options") {
                    output.push_str(&format!(
                        "{}  {{vars['{}']?.split(',').map(opt => <option key={{opt}} value={{opt.trim()}}>{{opt.trim()}}</option>)}}\n",
                        indent, bind
                    ));
                } else {
                    for option in sel.options.split(',') {
                        let opt = option.trim();
                        output.push_str(&format!(
                            "{}  <option value=\"{}\">{{\" {} \"}}</option>\n",
                            indent, opt, opt
                        ));
                    }
                }
                output.push_str(&format!("{}</select>\n", indent));
            }
            CanvasComponent::Container(container) => {
                let layout_style = match &container.layout {
                    crate::domain::LayoutType::Flex {
                        direction,
                        align_items,
                        justify_content,
                        ..
                    } => {
                        let dir = match direction {
                            crate::domain::FlexDirection::Row => "row",
                            crate::domain::FlexDirection::Column => "column",
                        };
                        let align = match align_items {
                            crate::domain::FlexAlign::Start => "flex-start",
                            crate::domain::FlexAlign::Center => "center",
                            crate::domain::FlexAlign::End => "flex-end",
                            crate::domain::FlexAlign::Stretch => "stretch",
                            crate::domain::FlexAlign::Baseline => "baseline",
                        };
                        let justify = match justify_content {
                            crate::domain::FlexJustify::Start => "flex-start",
                            crate::domain::FlexJustify::Center => "center",
                            crate::domain::FlexJustify::End => "flex-end",
                            crate::domain::FlexJustify::Between => "space-between",
                            crate::domain::FlexJustify::Around => "space-around",
                            crate::domain::FlexJustify::Evenly => "space-evenly",
                        };
                        format!(
                            "display: 'flex', flexDirection: '{}', alignItems: '{}', justifyContent: '{}'",
                            dir, align, justify
                        )
                    }
                    crate::domain::LayoutType::Grid { columns, rows } => {
                        format!(
                            "display: 'grid', gridTemplateColumns: 'repeat({}, 1fr)', gridTemplateRows: 'repeat({}, auto)'",
                            columns, rows
                        )
                    }
                    crate::domain::LayoutType::Stack => {
                        "display: 'flex', flexDirection: 'column'".to_string()
                    }
                };

                let style = format!(
                    "{{ {}, gap: '{}px', padding: '{}px {}px {}px {}px' }}",
                    layout_style,
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
                    format!(" className={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                output.push_str(&format!(
                    "{}<div{} {} style={{{}}}>\n",
                    indent, id_attr, class_attr, style
                ));

                for child in &container.children {
                    Self::generate_react(child, output, indent_level + 1)?;
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
                    format!(" className={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let width_attr = img
                    .width
                    .as_ref()
                    .map_or(String::new(), |w| format!(" width=\"{}\"", w));
                let height_attr = img
                    .height
                    .as_ref()
                    .map_or(String::new(), |h| format!(" height=\"{}\"", h));
                let src_val = if let Some(bind) = img.bindings.get("src") {
                    format!("{{vars['{}']}}", bind)
                } else {
                    format!("\"{}\"", img.src)
                };

                let alt_val = if let Some(bind) = img.bindings.get("alt") {
                    format!("{{vars['{}']}}", bind)
                } else {
                    format!("\"{}\"", img.alt)
                };

                output.push_str(&format!(
                    "{}<img{} {} src={} alt={}{}{} />\n",
                    indent, id_attr, class_attr, src_val, alt_val, width_attr, height_attr
                ));
            }
            CanvasComponent::Card(card) => {
                let mut style_parts = vec![
                    format!("padding: '{}px'", card.padding),
                    format!("borderRadius: '{}px'", card.border_radius),
                ];
                if card.shadow {
                    style_parts.push("boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.1)'".to_string());
                }
                if card.border {
                    style_parts.push("border: '1px solid #e5e7eb'".to_string());
                }
                let style_str = style_parts.join(", ");

                let id_attr = if let Some(bind) = card.bindings.get("id") {
                    format!(" id={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = card.bindings.get("custom_css_classes") {
                    format!(" className={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                output.push_str(&format!(
                    "{}<div{} {} style={{{{ {} }}}}>\n",
                    indent, id_attr, class_attr, style_str
                ));
                for child in &card.children {
                    Self::generate_react(child, output, indent_level + 1)?;
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
                    format!(" className={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                output.push_str(&format!("{}<!-- Custom: {} -->\n", indent, custom.name));
                output.push_str(&format!(
                    "{}<div{}{} dangerouslySetInnerHTML={{{{ __html: `{}` }}}} />\n",
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
                output.push_str(&format!("{}<hr style={{{{ `{}` }}}} />\n", indent, style));
            }
            CanvasComponent::Checkbox(checkbox) => {
                output.push_str(&format!(
                    "{}<label className=\"checkbox\">\n{}  <input type=\"checkbox\" defaultChecked={{{}}} disabled={{{}}} />\n{}  {}\n{}</label>\n",
                    indent, indent, checkbox.checked, checkbox.disabled, indent, checkbox.label, indent
                ));
            }
            CanvasComponent::RadioGroup(radio) => {
                let name = format!("radio-{}", radio.id);
                output.push_str(&format!("{}<div className=\"radio-group\">\n", indent));
                for opt in radio
                    .options
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                {
                    let checked = !radio.selected.is_empty() && radio.selected == opt;
                    output.push_str(&format!(
                        "{}  <label><input type=\"radio\" name=\"{}\" defaultChecked={{{}}} disabled={{{}}} /> {}</label>\n",
                        indent, name, checked, radio.disabled, opt
                    ));
                }
                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Switch(switch) => {
                output.push_str(&format!(
                    "{}<label className=\"switch\">\n{}  <input type=\"checkbox\" defaultChecked={{{}}} disabled={{{}}} />\n{}  {}\n{}</label>\n",
                    indent, indent, switch.checked, switch.disabled, indent, switch.label, indent
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
                    "{}<span className=\"badge badge-{}\">{}</span>\n",
                    indent, variant, badge.text
                ));
            }
            CanvasComponent::Progress(progress) => {
                output.push_str(&format!(
                    "{}<progress value={{{}}} max={{{}}} style={{{{ width: \"100%\" }}}} />\n",
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
                    format!(" className={{vars['{}']}}", bind)
                } else {
                    String::new()
                };

                let href_expr = if let Some(bind) = link.bindings.get("href") {
                    format!("{{vars['{}']}}", bind)
                } else {
                    format!("\"{}\"", escape_html(&link.href))
                };

                let text_expr = if let Some(bind) = link.bindings.get("text") {
                    format!("{{vars['{}']}}", bind)
                } else {
                    escape_html(&link.text).to_string()
                };

                // React takes styles as an object, so the same settings the
                // canvas applies are emitted as camelCase properties here.
                let style_props = style_js_properties(&link.style, &link.animation);
                let style_attr = if style_props.is_empty() {
                    String::new()
                } else {
                    let pairs: Vec<String> = style_props
                        .iter()
                        .map(|(key, value)| format!("{key}: {}", js_string_literal(value)))
                        .collect();
                    format!(" style={{{{ {} }}}}", pairs.join(", "))
                };

                output.push_str(&format!(
                    "{}<a{} href={}{}{}>{}</a>\n",
                    indent, id_attr, href_expr, class_attr, style_attr, text_expr
                ));
            }
        }

        Ok(())
    }
}
