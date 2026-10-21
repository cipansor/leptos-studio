//! JsonGenerator: json output.

use crate::config::GeneratorConfig;
use crate::domain::{AppError, AppResult, CanvasComponent, Variable};
use crate::generator::CodeGenerator;

/// JSON code generator
pub struct JsonGenerator;

/// The name this generator had before the crate was extracted.
///
/// A delegating struct rather than a `type` alias: the old name was a unit
/// struct, so callers wrote `JsonCodeGenerator.generate(...)`. A type alias
/// would exist only in the type namespace and reject that. Prefer
/// [`JsonGenerator`] in new code.
pub struct JsonCodeGenerator;

impl CodeGenerator for JsonCodeGenerator {
    fn generate_with(
        &self,
        config: &GeneratorConfig,
        components: &[CanvasComponent],
        variables: &[Variable],
    ) -> AppResult<String> {
        JsonGenerator.generate_with(config, components, variables)
    }

    fn file_extension(&self) -> &str {
        JsonGenerator.file_extension()
    }
}

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
