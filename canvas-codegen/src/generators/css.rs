//! CssGenerator: css output.

use crate::config::GeneratorConfig;
use crate::domain::{AppResult, CanvasComponent, Variable};
use crate::generator::CodeGenerator;
use crate::utils::helpers::attribution;

/// CSS generator (extracts styles)
pub struct CssGenerator;

impl CodeGenerator for CssGenerator {
    fn generate_with(
        &self,
        config: &GeneratorConfig,
        components: &[CanvasComponent],
        _variables: &[Variable],
    ) -> AppResult<String> {
        let mut css = attribution(config, "/* ", " */") + "\n";

        // Basic button styles
        css.push_str(".btn-primary {\n");
        css.push_str("  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);\n");
        css.push_str("  color: white;\n");
        css.push_str("  border: none;\n");
        css.push_str("  padding: 8px 16px;\n");
        css.push_str("  border-radius: 6px;\n");
        css.push_str("  cursor: pointer;\n");
        css.push_str("}\n\n");

        css.push_str(".btn-secondary {\n");
        css.push_str("  background: white;\n");
        css.push_str("  color: #475569;\n");
        css.push_str("  border: 1px solid #cbd5e1;\n");
        css.push_str("  padding: 8px 16px;\n");
        css.push_str("  border-radius: 6px;\n");
        css.push_str("  cursor: pointer;\n");
        css.push_str("}\n\n");

        css.push_str(".btn-outline {\n");
        css.push_str("  background: transparent;\n");
        css.push_str("  color: #3b82f6;\n");
        css.push_str("  border: 1px solid #3b82f6;\n");
        css.push_str("  padding: 8px 16px;\n");
        css.push_str("  border-radius: 6px;\n");
        css.push_str("  cursor: pointer;\n");
        css.push_str("}\n\n");

        css.push_str(".btn-ghost {\n");
        css.push_str("  background: transparent;\n");
        css.push_str("  color: #6b7280;\n");
        css.push_str("  border: none;\n");
        css.push_str("  padding: 8px 16px;\n");
        css.push_str("  cursor: pointer;\n");
        css.push_str("}\n\n");

        // Size modifiers
        css.push_str(".btn-sm { padding: 4px 12px; font-size: 12px; }\n");
        css.push_str(".btn-md { padding: 8px 16px; font-size: 14px; }\n");
        css.push_str(".btn-lg { padding: 12px 24px; font-size: 16px; }\n\n");

        // Container styles
        Self::extract_container_styles(components, &mut css);

        Ok(css)
    }

    fn file_extension(&self) -> &str {
        "css"
    }
}

impl CssGenerator {
    fn extract_container_styles(components: &[CanvasComponent], css: &mut String) {
        for component in components {
            if let CanvasComponent::Container(container) = component {
                let id = container.id.as_string();
                let class_name = format!("container-{}", &id[..8]);

                css.push_str(&format!(".{} {{\n", class_name));

                match &container.layout {
                    crate::domain::LayoutType::Flex {
                        direction,
                        wrap,
                        align_items,
                        justify_content,
                    } => {
                        css.push_str("  display: flex;\n");
                        css.push_str(&format!(
                            "  flex-direction: {};\n",
                            match direction {
                                crate::domain::FlexDirection::Row => "row",
                                crate::domain::FlexDirection::Column => "column",
                            }
                        ));
                        if *wrap {
                            css.push_str("  flex-wrap: wrap;\n");
                        }

                        let align_css = match align_items {
                            crate::domain::FlexAlign::Start => "flex-start",
                            crate::domain::FlexAlign::Center => "center",
                            crate::domain::FlexAlign::End => "flex-end",
                            crate::domain::FlexAlign::Stretch => "stretch",
                            crate::domain::FlexAlign::Baseline => "baseline",
                        };

                        let justify_css = match justify_content {
                            crate::domain::FlexJustify::Start => "flex-start",
                            crate::domain::FlexJustify::Center => "center",
                            crate::domain::FlexJustify::End => "flex-end",
                            crate::domain::FlexJustify::Between => "space-between",
                            crate::domain::FlexJustify::Around => "space-around",
                            crate::domain::FlexJustify::Evenly => "space-evenly",
                        };

                        css.push_str(&format!("  align-items: {};\n", align_css));
                        css.push_str(&format!("  justify-content: {};\n", justify_css));
                    }
                    crate::domain::LayoutType::Grid { columns, rows } => {
                        css.push_str("  display: grid;\n");
                        css.push_str(&format!(
                            "  grid-template-columns: repeat({}, 1fr);\n",
                            columns
                        ));
                        css.push_str(&format!("  grid-template-rows: repeat({}, auto);\n", rows));
                    }
                    crate::domain::LayoutType::Stack => {
                        css.push_str("  display: flex;\n");
                        css.push_str("  flex-direction: column;\n");
                    }
                }

                css.push_str(&format!("  gap: {}px;\n", container.gap));
                css.push_str(&format!(
                    "  padding: {}px {}px {}px {}px;\n",
                    container.padding.top,
                    container.padding.right,
                    container.padding.bottom,
                    container.padding.left
                ));

                css.push_str("}\n\n");

                // Recurse into children
                Self::extract_container_styles(&container.children, css);
            } else if let CanvasComponent::Card(card) = component {
                let id = card.id.as_string();
                let class_name = format!("card-{}", &id[..8]);

                css.push_str(&format!(".{} {{\n", class_name));
                css.push_str(&format!("  padding: {}px;\n", card.padding));
                css.push_str(&format!("  border-radius: {}px;\n", card.border_radius));
                if card.shadow {
                    css.push_str("  box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1);\n");
                }
                if card.border {
                    css.push_str("  border: 1px solid #e5e7eb;\n");
                }
                css.push_str("}\n\n");

                Self::extract_container_styles(&card.children, css);
            }
        }
    }
}

// Additional Generators
// ============================================================================
