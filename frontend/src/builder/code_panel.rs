use crate::services::{CodeGenerator, HtmlGenerator, LeptosGenerator, MarkdownGenerator};
use crate::services::{
    CssGenerator, JsonSchemaGenerator, ReactGenerator, SvelteGenerator, TailwindHtmlGenerator,
    TypeScriptGenerator, VueGenerator,
};
use crate::state::app_state::{AppState, Notification};
use crate::utils::copy_to_clipboard;
use crate::utils::file::download_file;
use crate::utils::syntax_highlight::highlight_code;
use leptos::prelude::*;

#[component]
pub fn CodePanel() -> impl IntoView {
    let app_state = AppState::expect_context();
    let format = RwSignal::new("leptos".to_string());

    // Memoize code generation to avoid constant re-rendering
    let code = Memo::new(move |_| {
        let comps = app_state.canvas.components.get();
        let variables = app_state.variables.get();
        let selected_format = format.get();

        match selected_format.as_str() {
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
            "vue" => {
                let generator = VueGenerator;
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
            "css" => {
                let generator = CssGenerator;
                generator
                    .generate(&comps, &variables)
                    .unwrap_or_else(|e| e.user_message())
            }
            _ => "Unknown format".to_string(),
        }
    });

    let copy_handler = move |_| {
        let code_text = code.get();
        wasm_bindgen_futures::spawn_local(async move {
            match copy_to_clipboard(&code_text).await {
                Ok(()) => {
                    app_state.ui.notification.set(Some(Notification::success(
                        "📋 Code copied to clipboard!".to_string(),
                    )));
                }
                Err(e) => {
                    app_state
                        .ui
                        .notification
                        .set(Some(Notification::error(format!(
                            "❌ {}",
                            e.user_message()
                        ))));
                }
            }
        });
    };

    let download_handler = move |_| {
        let code_text = code.get();
        let selected_format = format.get();

        let (ext, mime) = match selected_format.as_str() {
            "leptos" => ("rs", "text/plain"),
            "html" | "tailwind" => ("html", "text/html"),
            "markdown" => ("md", "text/markdown"),
            "json" | "jsonschema" => ("json", "application/json"),
            "typescript" => ("ts", "application/typescript"),
            "react" => ("tsx", "text/plain"),
            "vue" => ("vue", "text/plain"),
            "svelte" => ("svelte", "text/plain"),
            "css" => ("css", "text/css"),
            _ => ("txt", "text/plain"),
        };

        let filename = format!("leptos-layout.{}", ext);

        if let Err(e) = download_file(&code_text, &filename, mime) {
            app_state
                .ui
                .notification
                .set(Some(Notification::error(format!(
                    "❌ Download failed: {}",
                    e.user_message()
                ))));
        } else {
            app_state
                .ui
                .notification
                .set(Some(Notification::success(format!(
                    "⬇️ Downloaded {}",
                    filename
                ))));
        }
    };

    view! {
        <div class="code-panel">
            <div class="code-panel-header">
                <select
                    prop:value=format
                    on:input=move |ev| format.set(event_target_value(&ev))
                    class="code-format-select"
                    aria-label="Select export format"
                >
                    <optgroup label="Framework Code">
                        <option value="leptos">{"Leptos"}</option>
                        <option value="react">{"React"}</option>
                        <option value="vue">{"Vue"}</option>
                        <option value="svelte">{"Svelte"}</option>
                    </optgroup>
                    <optgroup label="Web Output">
                        <option value="html">{"HTML"}</option>
                        <option value="tailwind">{"Tailwind HTML"}</option>
                        <option value="css">{"CSS"}</option>
                    </optgroup>
                    <optgroup label="Data">
                        <option value="json">{"JSON"}</option>
                        <option value="typescript">{"Types"}</option>
                        <option value="jsonschema">{"JSON Schema"}</option>
                    </optgroup>
                    <optgroup label="Documentation">
                        <option value="markdown">{"Markdown"}</option>
                    </optgroup>
                </select>
                <div class="code-actions" style="display: flex; gap: 8px;">
                    <button on:click=copy_handler class="btn btn-sm btn-secondary" title="Copy to Clipboard">
                        "📋 Copy"
                    </button>
                    <button on:click=download_handler class="btn btn-sm btn-secondary" title="Download File">
                        "⬇️ Download"
                    </button>
                </div>
            </div>
            <div class="code-preview-container">
                <pre class="code-preview">
                    <code>
                        {move || {
                            let text = code.get();
                            // Collect into a Vec<String> first to own the data
                            let lines: Vec<String> = text.lines().map(|s| s.to_string()).collect();
                            lines.into_iter().map(|line_string| {
                                let highlighted = highlight_code(line_string);
                                view! { <span class="line">{highlighted}</span> }
                            }).collect_view()
                        }}
                    </code>
                </pre>
            </div>
        </div>
    }
}
