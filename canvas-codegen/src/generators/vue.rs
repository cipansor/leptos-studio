//! VueGenerator: vue output.

use crate::config::GeneratorConfig;
use crate::domain::{AppResult, CanvasComponent, Variable, VariableType};
use crate::generator::CodeGenerator;
use crate::utils::helpers::{attribution, component_inline_css};
use crate::utils::sanitize::escape_html;

/// Vue component generator
pub struct VueGenerator;

impl CodeGenerator for VueGenerator {
    fn generate_with(
        &self,
        config: &GeneratorConfig,
        components: &[CanvasComponent],
        variables: &[Variable],
    ) -> AppResult<String> {
        let mut template = String::from("<template>\n  <div class=\"generated-layout\">\n");

        for component in components {
            Self::generate_vue(component, &mut template, 2)?;
        }

        template.push_str("  </div>\n</template>\n\n");

        // Script section
        template.push_str("<script setup lang=\"ts\">\n");
        template.push_str("import { ref } from 'vue';\n");
        template.push_str(&attribution(config, "// ", ""));

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

        template.push_str(&format!(
            "const vars = ref<Record<string, any>>({});\n",
            vars_init
        ));
        template.push_str("</script>\n\n");

        // Style section
        template.push_str("<style scoped>\n");
        template.push_str(".generated-layout {\n");
        template.push_str("  /* Add your styles here */\n");
        template.push_str("}\n");
        template.push_str("</style>\n");

        Ok(template)
    }

    fn file_extension(&self) -> &str {
        "vue"
    }
}

impl VueGenerator {
    fn generate_vue(
        component: &CanvasComponent,
        output: &mut String,
        indent_level: usize,
    ) -> AppResult<()> {
        let indent = "  ".repeat(indent_level);

        match component {
            CanvasComponent::Button(btn) => {
                let id_attr = if let Some(bind) = btn.bindings.get("id") {
                    format!(" :id=\"vars['{}']\"", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = btn.bindings.get("custom_css_classes") {
                    format!(" :class=\"vars['{}']\"", bind)
                } else {
                    String::new()
                };

                let label_expr = if let Some(bind) = btn.bindings.get("label") {
                    format!("{{{{ vars['{}'] }}}}", bind)
                } else {
                    btn.label.clone()
                };

                let disabled_attr = if let Some(bind) = btn.bindings.get("disabled") {
                    format!(":disabled=\"vars['{}']\"", bind)
                } else {
                    format!(":disabled=\"{}\"", btn.disabled)
                };

                output.push_str(&format!(
                    "{}<button{}{} {}>{}</button>\n",
                    indent, id_attr, class_attr, disabled_attr, label_expr
                ));
            }
            CanvasComponent::Text(txt) => {
                let id_attr = if let Some(bind) = txt.bindings.get("id") {
                    format!(" :id=\"vars['{}']\"", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = txt.bindings.get("custom_css_classes") {
                    format!(" :class=\"vars['{}']\"", bind)
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
                    format!("{{{{ vars['{}'] }}}}", bind)
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
                    format!(" :id=\"vars['{}']\"", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = inp.bindings.get("custom_css_classes") {
                    format!(" :class=\"vars['{}']\"", bind)
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
                    format!(":placeholder=\"vars['{}']\"", bind)
                } else {
                    format!("placeholder=\"{}\"", inp.placeholder)
                };

                let disabled_attr = if let Some(bind) = inp.bindings.get("disabled") {
                    format!(":disabled=\"vars['{}']\"", bind)
                } else {
                    format!(":disabled=\"{}\"", inp.disabled)
                };

                output.push_str(&format!(
                    "{}<input{}{} type=\"{}\" {} :required=\"{}\" {} />\n",
                    indent,
                    id_attr,
                    class_attr,
                    input_type,
                    placeholder_attr,
                    inp.required,
                    disabled_attr
                ));
            }
            CanvasComponent::Select(sel) => {
                let id_attr = if let Some(bind) = sel.bindings.get("id") {
                    format!(" :id=\"vars['{}']\"", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = sel.bindings.get("custom_css_classes") {
                    format!(" :class=\"vars['{}']\"", bind)
                } else {
                    String::new()
                };

                let disabled_attr = if let Some(bind) = sel.bindings.get("disabled") {
                    format!(":disabled=\"vars['{}']\"", bind)
                } else {
                    format!(":disabled=\"{}\"", sel.disabled)
                };

                output.push_str(&format!(
                    "{}<select{}{} {}>\n",
                    indent, id_attr, class_attr, disabled_attr
                ));
                if !sel.placeholder.is_empty() {
                    let placeholder_expr = if let Some(bind) = sel.bindings.get("placeholder") {
                        format!("{{{{ vars['{}'] }}}}", bind)
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
                        "{}  <option v-for=\"opt in vars['{}']?.split(',')\" :key=\"opt\" :value=\"opt.trim()\">{{{{ opt.trim() }}}}</option>\n",
                        indent, bind
                    ));
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
                let id_attr = if let Some(bind) = container.bindings.get("id") {
                    format!(" :id=\"vars['{}']\"", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = container.bindings.get("custom_css_classes") {
                    format!(" :class=\"vars['{}']\"", bind)
                } else {
                    String::new()
                };

                output.push_str(&format!("{}<div{}{}>\n", indent, id_attr, class_attr));
                for child in &container.children {
                    Self::generate_vue(child, output, indent_level + 1)?;
                }
                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Image(img) => {
                let id_attr = if let Some(bind) = img.bindings.get("id") {
                    format!(" :id=\"vars['{}']\"", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = img.bindings.get("custom_css_classes") {
                    format!(" :class=\"vars['{}']\"", bind)
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
                    format!(":src=\"vars['{}']\"", bind)
                } else {
                    format!("src=\"{}\"", img.src)
                };

                let alt_val = if let Some(bind) = img.bindings.get("alt") {
                    format!(":alt=\"vars['{}']\"", bind)
                } else {
                    format!("alt=\"{}\"", img.alt)
                };

                output.push_str(&format!(
                    "{}<img{}{} {} {}{}{} />\n",
                    indent, id_attr, class_attr, src_val, alt_val, width_attr, height_attr
                ));
            }
            CanvasComponent::Card(card) => {
                let shadow_class = if card.shadow { "shadow-md" } else { "" };
                let border_class = if card.border { "border" } else { "" };

                let id_attr = if let Some(bind) = card.bindings.get("id") {
                    format!(" :id=\"vars['{}']\"", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = card.bindings.get("custom_css_classes") {
                    format!(" :class=\"vars['{}']\"", bind)
                } else {
                    format!(" class=\"card {} {}\"", shadow_class, border_class)
                };

                output.push_str(&format!(
                    "{}<div{}{} style=\"padding: {}px; border-radius: {}px;\">\n",
                    indent, id_attr, class_attr, card.padding, card.border_radius
                ));
                for child in &card.children {
                    Self::generate_vue(child, output, indent_level + 1)?;
                }
                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Custom(custom) => {
                let id_attr = if let Some(bind) = custom.bindings.get("id") {
                    format!(" :id=\"vars['{}']\"", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = custom.bindings.get("custom_css_classes") {
                    format!(" :class=\"vars['{}']\"", bind)
                } else {
                    String::new()
                };

                output.push_str(&format!("{}<!-- {} -->\n", indent, custom.name));
                output.push_str(&format!(
                    "{}<div{}{} v-html=\"`{}`\"></div>\n",
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
                    "{}<label>\n{}  <input type=\"checkbox\" :checked=\"{}\" :disabled=\"{}\" />\n{}  {}\n{}</label>\n",
                    indent, indent, checkbox.checked, checkbox.disabled, indent, checkbox.label, indent
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
                        "{}  <label><input type=\"radio\" name=\"{}\" :checked=\"{}\" :disabled=\"{}\" /> {}</label>\n",
                        indent, name, checked, radio.disabled, opt
                    ));
                }
                output.push_str(&format!("{}</div>\n", indent));
            }
            CanvasComponent::Switch(switch) => {
                output.push_str(&format!(
                    "{}<label class=\"switch\">\n{}  <input type=\"checkbox\" :checked=\"{}\" :disabled=\"{}\" />\n{}  {}\n{}</label>\n",
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
                    "{}<span class=\"badge badge-{}\">{}</span>\n",
                    indent, variant, badge.text
                ));
            }
            CanvasComponent::Progress(progress) => {
                output.push_str(&format!(
                    "{}<progress :value=\"{}\" :max=\"{}\" style=\"width: 100%;\"></progress>\n",
                    indent, progress.value, progress.max
                ));
            }
            CanvasComponent::Link(link) => {
                let id_attr = if let Some(bind) = link.bindings.get("id") {
                    format!(" :id=\"vars['{}']\"", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = link.bindings.get("custom_css_classes") {
                    format!(" :class=\"vars['{}']\"", bind)
                } else {
                    String::new()
                };

                let href_attr = if let Some(bind) = link.bindings.get("href") {
                    format!(":href=\"vars['{}']\"", bind)
                } else {
                    format!("href=\"{}\"", escape_html(&link.href))
                };

                let text_expr = if let Some(bind) = link.bindings.get("text") {
                    format!("{{{{ vars['{}'] }}}}", bind)
                } else {
                    escape_html(&link.text).to_string()
                };

                // Vue takes a CSS declaration string, as the canvas does.
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
