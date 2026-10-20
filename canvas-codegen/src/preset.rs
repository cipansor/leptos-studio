//! Styling preset for the Leptos exporter.

use serde::{Deserialize, Serialize};

/// Which Leptos styling library the generated component should assume.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ExportPreset {
    /// Plain CSS classes, no external dependency.
    #[default]
    Plain,
    /// `thaw` components.
    ThawUi,
    /// `leptos-material` components.
    LeptosMaterial,
    /// `leptos-use` helpers.
    LeptosUse,
}
