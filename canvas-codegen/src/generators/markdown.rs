//! MarkdownGenerator: markdown output.

use crate::config::GeneratorConfig;
use crate::domain::{AppResult, CanvasComponent, Variable};
use crate::generator::CodeGenerator;
use crate::utils::sanitize::escape_html;

/// Markdown code generator (for documentation)
pub struct MarkdownGenerator;

impl CodeGenerator for MarkdownGenerator {
    fn generate_with(
        &self,
        _config: &GeneratorConfig,
        components: &[CanvasComponent],
        _variables: &[Variable],
    ) -> AppResult<String> {
        let mut output = String::from("# Generated Layout Documentation\n\n");

        for (i, component) in components.iter().enumerate() {
            output.push_str(&format!("## Component {}\n\n", i + 1));
            self.generate_markdown(component, &mut output, 0)?;
            output.push('\n');
        }

        Ok(output)
    }

    fn file_extension(&self) -> &str {
        "md"
    }
}

impl MarkdownGenerator {
    #[allow(clippy::only_used_in_recursion)]
    fn generate_markdown(
        &self,
        component: &CanvasComponent,
        output: &mut String,
        depth: usize,
    ) -> AppResult<()> {
        let indent = "  ".repeat(depth);

        match component {
            CanvasComponent::Button(btn) => {
                output.push_str(&format!("{}- **Button**: {}\n", indent, btn.label));
                output.push_str(&format!("{}  - Variant: {:?}\n", indent, btn.variant));
                output.push_str(&format!("{}  - Size: {:?}\n", indent, btn.size));
                output.push_str(&format!("{}  - Disabled: {}\n", indent, btn.disabled));
            }
            CanvasComponent::Text(txt) => {
                output.push_str(&format!("{}- **Text**: {}\n", indent, txt.content));
                output.push_str(&format!("{}  - Style: {:?}\n", indent, txt.style));
                output.push_str(&format!("{}  - Tag: {:?}\n", indent, txt.tag));
            }
            CanvasComponent::Input(inp) => {
                output.push_str(&format!("{}- **Input**\n", indent));
                output.push_str(&format!("{}  - Type: {:?}\n", indent, inp.input_type));
                output.push_str(&format!("{}  - Placeholder: {}\n", indent, inp.placeholder));
                output.push_str(&format!("{}  - Required: {}\n", indent, inp.required));
            }
            CanvasComponent::Select(sel) => {
                output.push_str(&format!("{}- **Select**\n", indent));
                output.push_str(&format!("{}  - Placeholder: {}\n", indent, sel.placeholder));
                output.push_str(&format!("{}  - Options: {}\n", indent, sel.options));
                output.push_str(&format!("{}  - Disabled: {}\n", indent, sel.disabled));
            }
            CanvasComponent::Container(container) => {
                output.push_str(&format!("{}- **Container**\n", indent));
                output.push_str(&format!("{}  - Layout: {:?}\n", indent, container.layout));
                output.push_str(&format!(
                    "{}  - Children: {}\n",
                    indent,
                    container.children.len()
                ));
                for child in &container.children {
                    self.generate_markdown(child, output, depth + 1)?;
                }
            }
            CanvasComponent::Image(img) => {
                output.push_str(&format!("{}- **Image**\n", indent));
                output.push_str(&format!("{}  - Src: {}\n", indent, img.src));
                output.push_str(&format!("{}  - Alt: {}\n", indent, img.alt));
            }
            CanvasComponent::Card(card) => {
                output.push_str(&format!("{}- **Card**\n", indent));
                output.push_str(&format!("{}  - Padding: {}\n", indent, card.padding));
                for child in &card.children {
                    self.generate_markdown(child, output, depth + 1)?;
                }
            }
            CanvasComponent::Custom(custom) => {
                output.push_str(&format!(
                    "{}- **Custom Component**: {}\n",
                    indent, custom.name
                ));
                output.push_str(&format!("{}  - Template: {}\n", indent, custom.template));
            }
            CanvasComponent::Divider(divider) => {
                output.push_str(&format!(
                    "{}- **Divider** ({})\n",
                    indent,
                    match divider.orientation {
                        crate::domain::DividerOrientation::Horizontal => "horizontal",
                        crate::domain::DividerOrientation::Vertical => "vertical",
                    }
                ));
            }
            CanvasComponent::Checkbox(checkbox) => {
                output.push_str(&format!("{}- **Checkbox**: {}\n", indent, checkbox.label));
                output.push_str(&format!("{}  - Checked: {}\n", indent, checkbox.checked));
                output.push_str(&format!("{}  - Disabled: {}\n", indent, checkbox.disabled));
            }
            CanvasComponent::RadioGroup(radio) => {
                output.push_str(&format!("{}- **Radio Group**\n", indent));
                output.push_str(&format!("{}  - Options: {}\n", indent, radio.options));
                output.push_str(&format!("{}  - Selected: {}\n", indent, radio.selected));
            }
            CanvasComponent::Switch(switch) => {
                output.push_str(&format!("{}- **Switch**: {}\n", indent, switch.label));
                output.push_str(&format!("{}  - On: {}\n", indent, switch.checked));
            }
            CanvasComponent::Badge(badge) => {
                output.push_str(&format!("{}- **Badge**: {}\n", indent, badge.text));
                output.push_str(&format!("{}  - Variant: {:?}\n", indent, badge.variant));
            }
            CanvasComponent::Progress(progress) => {
                output.push_str(&format!("{}- **Progress**\n", indent));
                output.push_str(&format!(
                    "{}  - Value: {} / {}\n",
                    indent, progress.value, progress.max
                ));
            }
            CanvasComponent::Link(link) => {
                // Markdown cannot express style or animation, so those are lost
                // by design; the URL and text are kept as a real Markdown link
                // rather than a label so the semantics survive.
                output.push_str(&format!(
                    "{}- [{}]({})\n",
                    indent,
                    escape_html(&link.text),
                    link.href.replace('(', "%28").replace(')', "%29")
                ));
            }
        }

        Ok(())
    }
}
