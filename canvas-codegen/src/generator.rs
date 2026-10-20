//! The trait every output target implements.

use crate::config::GeneratorConfig;
use crate::domain::{AppResult, CanvasComponent, Variable};

/// Emits source for one target framework or format.
///
/// A generator is a unit struct or a small config holder; the trait is what
/// lets a caller pick a target at runtime, e.g. to drive a `--target` flag.
pub trait CodeGenerator {
    /// Emit source for `components`, with no attribution.
    ///
    /// Equivalent to [`generate_with`](CodeGenerator::generate_with) with a
    /// default [`GeneratorConfig`], which writes no product name into the output.
    fn generate(
        &self,
        components: &[CanvasComponent],
        variables: &[Variable],
    ) -> AppResult<String> {
        self.generate_with(&GeneratorConfig::default(), components, variables)
    }

    /// Emit source for `components`, honouring `config`.
    fn generate_with(
        &self,
        config: &GeneratorConfig,
        components: &[CanvasComponent],
        variables: &[Variable],
    ) -> AppResult<String>;

    /// Extension of the file this generator's output belongs in, without a dot.
    fn file_extension(&self) -> &str;
}
