//! JsonGenerator: json output.

use crate::config::GeneratorConfig;
use crate::domain::{AppError, AppResult, CanvasComponent, Variable};
use crate::generator::CodeGenerator;

/// JSON code generator
pub struct JsonGenerator;

impl CodeGenerator for JsonGenerator {
    fn generate_with(
        &self,
        _config: &GeneratorConfig,
        components: &[CanvasComponent],
        _variables: &[Variable],
    ) -> AppResult<String> {
        serde_json::to_string_pretty(components)
            .map_err(|e| AppError::Export(format!("Failed to serialize to JSON: {}", e)))
    }

    fn file_extension(&self) -> &str {
        "json"
    }
}
