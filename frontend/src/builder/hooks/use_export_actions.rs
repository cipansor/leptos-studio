use crate::services::{CodeGenerator, HtmlGenerator, LeptosGenerator, MarkdownGenerator};
use crate::services::{
    JsonSchemaGenerator, ReactGenerator, SvelteGenerator, TailwindHtmlGenerator,
    TypeScriptGenerator,
};
use crate::state::app_state::AppState;
use leptos::prelude::*;

pub fn use_export_actions(
    show_export: WriteSignal<bool>,
    export_code: WriteSignal<String>,
    export_template: ReadSignal<String>,
) -> impl Fn(web_sys::MouseEvent) {
    let app_state = AppState::expect_context();

    move |_| {
        let comps = app_state.canvas.components.get();
        let variables = app_state.variables.get();

        let code = match export_template.get().as_str() {
            "leptos" => {
                let generator = LeptosGenerator::new(crate::state::ExportPreset::Plain);
                generator
                    .generate(&comps, &variables)
                    .unwrap_or_else(|e| e.user_message())
            }
            "html" => {
                let generator = HtmlGenerator;
                generator
                    .generate(&comps, &variables)
                    .unwrap_or_else(|e| e.user_message())
            }
            "markdown" => {
                let generator = MarkdownGenerator;
                generator
                    .generate(&comps, &variables)
                    .unwrap_or_else(|e| e.user_message())
            }
            "json" => serde_json::to_string_pretty(&comps)
                .unwrap_or_else(|e| format!("Error serializing JSON: {}", e)),
            "jsonschema" => {
                let generator = JsonSchemaGenerator;
                generator
                    .generate(&comps, &variables)
                    .unwrap_or_else(|e| e.user_message())
            }
            "typescript" => {
                let generator = TypeScriptGenerator;
                generator
                    .generate(&comps, &variables)
                    .unwrap_or_else(|e| e.user_message())
            }
            "react" => {
                let generator = ReactGenerator;
                generator
                    .generate(&comps, &variables)
                    .unwrap_or_else(|e| e.user_message())
            }
            "tailwind" => {
                let generator = TailwindHtmlGenerator;
                generator
                    .generate(&comps, &variables)
                    .unwrap_or_else(|e| e.user_message())
            }
            "svelte" => {
                let generator = SvelteGenerator;
                generator
                    .generate(&comps, &variables)
                    .unwrap_or_else(|e| e.user_message())
            }
            _ => "Unknown template".to_string(),
        };

        export_code.set(code);
        show_export.set(true);
    }
}
