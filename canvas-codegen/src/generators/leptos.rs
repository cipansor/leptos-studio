//! LeptosGenerator: leptos output.

use crate::config::GeneratorConfig;
use crate::domain::{AppResult, CanvasComponent, Variable, VariableType};
use crate::generator::CodeGenerator;
use crate::preset::ExportPreset;
use crate::utils::helpers::{
    EXPORT_CSS, attribution, component_inline_css, get_animation_css, rust_string_literal,
};

/// Leptos code generator
pub struct LeptosGenerator {
    preset: ExportPreset,
}

/// The name this generator had before the crate was extracted.
///
/// A delegating struct rather than a `type` alias: the old name had an inherent
/// `new(preset)` constructor and was used as a value, which a type alias cannot
/// express. Prefer [`LeptosGenerator`] in new code.
pub struct LeptosCodeGenerator {
    inner: LeptosGenerator,
}

impl LeptosCodeGenerator {
    pub fn new(preset: ExportPreset) -> Self {
        Self {
            inner: LeptosGenerator::new(preset),
        }
    }
}

impl CodeGenerator for LeptosCodeGenerator {
    fn generate_with(
        &self,
        config: &GeneratorConfig,
        components: &[CanvasComponent],
        variables: &[Variable],
    ) -> AppResult<String> {
        self.inner.generate_with(config, components, variables)
    }

    fn file_extension(&self) -> &str {
        self.inner.file_extension()
    }
}

impl LeptosGenerator {
    pub fn new(preset: ExportPreset) -> Self {
        Self { preset }
    }

    fn generate_imports(&self) -> String {
        match self.preset {
            ExportPreset::Plain => "use leptos::prelude::*;\n".to_string(),
            ExportPreset::ThawUi => "use leptos::prelude::*;\nuse thaw::*;\n".to_string(),
            ExportPreset::LeptosMaterial => {
                "use leptos::prelude::*;\nuse leptos_material::*;\n".to_string()
            }
            ExportPreset::LeptosUse => "use leptos::prelude::*;\nuse leptos_use::*;\n".to_string(),
        }
    }

    fn generate_component(
        &self,
        component: &CanvasComponent,
        output: &mut String,
        indent_level: usize,
        signals: &mut Vec<(String, String)>,
    ) -> AppResult<()> {
        let indent = "    ".repeat(indent_level);

        match component {
            CanvasComponent::Button(btn) => {
                let id_attr = if let Some(bind) = btn.bindings.get("id") {
                    format!(" id=move || {}.get()", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = btn.bindings.get("custom_css_classes") {
                    format!(" class=move || format!(\"{{}}\", {}.get())", bind)
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

                let anim_style = get_animation_css(&btn.animation);
                let style_attr = if !anim_style.is_empty() {
                    format!(" style=\"{}\"", anim_style)
                } else {
                    String::new()
                };

                let click_handler = if let Some(handler) = &btn.on_click {
                    format!("on:click={}", handler)
                } else {
                    "on:click=move |_| { web_sys::console::log_1(&\"Clicked\".into()); }"
                        .to_string()
                };

                let label_expr = if let Some(bind) = btn.bindings.get("label") {
                    format!("move || {}.get()", bind)
                } else {
                    format!("\"{}\"", rust_string_literal(&btn.label))
                };

                let disabled_attr = if let Some(bind) = btn.bindings.get("disabled") {
                    format!("prop:disabled=move || {}.get()", bind)
                } else {
                    format!("disabled={}", btn.disabled)
                };

                // Add interactive scaffolding
                output.push_str(&format!(
                    "{}        <button{}{} class=\"{} {}\" {} {}{}>{}</button>\n",
                    indent,
                    id_attr,
                    class_attr,
                    variant_class,
                    size_class,
                    disabled_attr,
                    click_handler,
                    style_attr,
                    label_expr
                ));
            }
            CanvasComponent::Text(txt) => {
                let id_attr = if let Some(bind) = txt.bindings.get("id") {
                    format!(" id=move || {}.get()", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = txt.bindings.get("custom_css_classes") {
                    format!(
                        " class=move || format!(\"text-{{}} {{}}\", \"{}\", {}.get())",
                        match txt.style {
                            crate::domain::TextStyle::Heading1 => "heading1",
                            crate::domain::TextStyle::Heading2 => "heading2",
                            crate::domain::TextStyle::Heading3 => "heading3",
                            crate::domain::TextStyle::Body => "body",
                            crate::domain::TextStyle::Caption => "caption",
                        },
                        bind
                    )
                } else {
                    format!(
                        " class=\"text-{}\"",
                        match txt.style {
                            crate::domain::TextStyle::Heading1 => "heading1",
                            crate::domain::TextStyle::Heading2 => "heading2",
                            crate::domain::TextStyle::Heading3 => "heading3",
                            crate::domain::TextStyle::Body => "body",
                            crate::domain::TextStyle::Caption => "caption",
                        }
                    )
                };

                let tag = match txt.tag {
                    crate::domain::TextTag::H1 => "h1",
                    crate::domain::TextTag::H2 => "h2",
                    crate::domain::TextTag::H3 => "h3",
                    crate::domain::TextTag::P => "p",
                    crate::domain::TextTag::Span => "span",
                };

                let anim_style = get_animation_css(&txt.animation);
                let style_attr = if !anim_style.is_empty() {
                    format!(" style=\"{}\"", anim_style)
                } else {
                    String::new()
                };

                let content_expr = if let Some(bind) = txt.bindings.get("content") {
                    format!("move || {}.get()", bind)
                } else {
                    format!("\"{}\"", rust_string_literal(&txt.content))
                };

                output.push_str(&format!(
                    "{}        <{}{}{}{}>{}</{}>\n",
                    indent, tag, id_attr, class_attr, style_attr, content_expr, tag
                ));
            }
            CanvasComponent::Input(inp) => {
                let id_attr = if let Some(bind) = inp.bindings.get("id") {
                    format!(" id=move || {}.get()", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = inp.bindings.get("custom_css_classes") {
                    format!(" class=move || format!(\"{{}}\", {}.get())", bind)
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

                let anim_style = get_animation_css(&inp.animation);
                let style_attr = if !anim_style.is_empty() {
                    format!(" style=\"{}\"", anim_style)
                } else {
                    String::new()
                };

                let input_handler = if let Some(handler) = &inp.on_input {
                    format!("on:input={}", handler)
                } else {
                    // Track signal for this input
                    let signal_name = format!("input_{}", signals.len());
                    signals.push((signal_name.clone(), "String::new()".to_string()));
                    format!(
                        "prop:value={} on:input=move |ev| set_{}(event_target_value(&ev))",
                        signal_name, signal_name
                    )
                };

                let change_handler = if let Some(handler) = &inp.on_change {
                    format!(" on:change={}", handler)
                } else {
                    String::new()
                };

                let placeholder_attr = if let Some(bind) = inp.bindings.get("placeholder") {
                    format!("prop:placeholder=move || {}.get()", bind)
                } else {
                    format!("placeholder=\"{}\"", rust_string_literal(&inp.placeholder))
                };

                let disabled_attr = if let Some(bind) = inp.bindings.get("disabled") {
                    format!("prop:disabled=move || {}.get()", bind)
                } else {
                    format!("disabled={}", inp.disabled)
                };

                // Add binding
                output.push_str(&format!(
                    "{}        <input{}{} type=\"{}\" {} {} {} required={} {}{} />\n",
                    indent,
                    id_attr,
                    class_attr,
                    input_type,
                    placeholder_attr,
                    input_handler,
                    change_handler,
                    inp.required,
                    disabled_attr,
                    style_attr
                ));
            }
            CanvasComponent::Select(sel) => {
                let id_attr = if let Some(bind) = sel.bindings.get("id") {
                    format!(" id=move || {}.get()", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = sel.bindings.get("custom_css_classes") {
                    format!(" class=move || format!(\"{{}}\", {}.get())", bind)
                } else {
                    String::new()
                };

                let anim_style = get_animation_css(&sel.animation);
                let style_attr = if !anim_style.is_empty() {
                    format!(" style=\"{}\"", anim_style)
                } else {
                    String::new()
                };

                let change_handler = if let Some(handler) = &sel.on_change {
                    format!(" on:change={}", handler)
                } else {
                    String::new()
                };

                let options_expr = if let Some(bind) = sel.bindings.get("options") {
                    format!(
                        "move || {}.get().split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>().into_iter().map(|opt| view! {{ <option value=opt.clone()>{{opt}}</option> }}).collect_view()",
                        bind
                    )
                } else {
                    let opts = sel
                        .options
                        .split(',')
                        .map(|o| {
                            format!(
                                "view! {{ <option value=\"{}\">\"{}\"</option> }}",
                                rust_string_literal(o.trim()),
                                rust_string_literal(o.trim())
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("vec![{}].collect_view()", opts)
                };

                let disabled_attr = if let Some(bind) = sel.bindings.get("disabled") {
                    format!("prop:disabled=move || {}.get()", bind)
                } else {
                    format!("disabled={}", sel.disabled)
                };

                let placeholder_attr = if let Some(bind) = sel.bindings.get("placeholder") {
                    format!("prop:placeholder=move || {}.get()", bind)
                } else {
                    String::new()
                };

                output.push_str(&format!(
                    "{}        <select{}{} {} {} {}{}>\n",
                    indent,
                    id_attr,
                    class_attr,
                    disabled_attr,
                    placeholder_attr,
                    change_handler,
                    style_attr
                ));

                if !sel.placeholder.is_empty() {
                    output.push_str(&format!(
                        "{}            <option value=\"\" disabled selected>\"{}\"</option>\n",
                        indent,
                        rust_string_literal(&sel.placeholder)
                    ));
                }

                output.push_str(&format!("{}            {{{}}}\n", indent, options_expr));

                output.push_str(&format!("{}        </select>\n", indent));
            }
            CanvasComponent::Container(container) => {
                let (layout_class, align_style) = match &container.layout {
                    crate::domain::LayoutType::Flex {
                        direction,
                        wrap,
                        align_items,
                        justify_content,
                    } => {
                        let dir = match direction {
                            crate::domain::FlexDirection::Row => "flex-row",
                            crate::domain::FlexDirection::Column => "flex-col",
                        };
                        let mut classes = dir.to_string();
                        if *wrap {
                            classes.push_str(" flex-wrap");
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

                        (
                            classes,
                            format!(
                                "align-items: {}; justify-content: {};",
                                align_css, justify_css
                            ),
                        )
                    }
                    crate::domain::LayoutType::Grid { columns, rows } => (
                        format!("grid grid-cols-{} grid-rows-{}", columns, rows),
                        String::new(),
                    ),
                    crate::domain::LayoutType::Stack => ("stack".to_string(), String::new()),
                };

                let anim_style = get_animation_css(&container.animation);

                let click_handler = if let Some(handler) = &container.on_click {
                    format!(" on:click={}", handler)
                } else {
                    String::new()
                };

                let id_attr = if let Some(bind) = container.bindings.get("id") {
                    format!(" id=move || {}.get()", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = container.bindings.get("custom_css_classes") {
                    format!(
                        " class=move || format!(\"container {} {{}}\", {}.get())",
                        layout_class, bind
                    )
                } else {
                    format!(" class=\"container {}\"", layout_class)
                };

                output.push_str(&format!(
                    "{}        <div{}{} style=\"gap: {}px; padding: {}px {}px {}px {}px; {} {}\"{}>\n",
                    indent,
                    id_attr,
                    class_attr,
                    container.gap,
                    container.padding.top,
                    container.padding.right,
                    container.padding.bottom,
                    container.padding.left,
                    align_style,
                    anim_style,
                    click_handler
                ));

                // Recursively generate children
                for child in &container.children {
                    self.generate_component(child, output, indent_level + 1, signals)?;
                }

                output.push_str(&format!("{}        </div>\n", indent));
            }
            CanvasComponent::Image(img) => {
                let id_attr = if let Some(bind) = img.bindings.get("id") {
                    format!(" id=move || {}.get()", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = img.bindings.get("custom_css_classes") {
                    format!(" class=move || format!(\"{{}}\", {}.get())", bind)
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

                let anim_style = get_animation_css(&img.animation);
                let style_attr = if !anim_style.is_empty() {
                    format!(" style=\"{}\"", anim_style)
                } else {
                    String::new()
                };

                let click_handler = if let Some(handler) = &img.on_click {
                    format!(" on:click={}", handler)
                } else {
                    String::new()
                };

                let src_attr = if let Some(bind) = img.bindings.get("src") {
                    format!("prop:src=move || {}.get()", bind)
                } else {
                    format!("src=\"{}\"", rust_string_literal(&img.src))
                };

                let alt_attr = if let Some(bind) = img.bindings.get("alt") {
                    format!("prop:alt=move || {}.get()", bind)
                } else {
                    format!("alt=\"{}\"", rust_string_literal(&img.alt))
                };

                output.push_str(&format!(
                    "{}        <img{}{} {} {} {}{}{}{} />\n",
                    indent,
                    id_attr,
                    class_attr,
                    src_attr,
                    alt_attr,
                    width_attr,
                    height_attr,
                    style_attr,
                    click_handler
                ));
            }
            CanvasComponent::Card(card) => {
                let shadow_style = if card.shadow {
                    "box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1);"
                } else {
                    ""
                };
                let border_class = if card.border {
                    "border border-gray-200"
                } else {
                    ""
                };
                let anim_style = get_animation_css(&card.animation);

                let click_handler = if let Some(handler) = &card.on_click {
                    format!(" on:click={}", handler)
                } else {
                    String::new()
                };

                let id_attr = if let Some(bind) = card.bindings.get("id") {
                    format!(" id=move || {}.get()", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = card.bindings.get("custom_css_classes") {
                    format!(
                        " class=move || format!(\"card {} {{}}\", {}.get())",
                        border_class, bind
                    )
                } else {
                    format!(" class=\"card {}\"", border_class)
                };

                output.push_str(&format!(
                    "{}        <div{}{} style=\"padding: {}px; border-radius: {}px; {} {}\"{}>\n",
                    indent,
                    id_attr,
                    class_attr,
                    card.padding,
                    card.border_radius,
                    shadow_style,
                    anim_style,
                    click_handler
                ));
                for child in &card.children {
                    self.generate_component(child, output, indent_level + 1, signals)?;
                }
                output.push_str(&format!("{}        </div>\n", indent));
            }
            CanvasComponent::Custom(custom) => {
                let id_attr = if let Some(bind) = custom.bindings.get("id") {
                    format!(" id=move || {}.get()", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = custom.bindings.get("custom_css_classes") {
                    format!(" class=move || format!(\"{{}}\", {}.get())", bind)
                } else {
                    String::new()
                };

                output.push_str(&format!(
                    "{}        // Custom component: {}\n",
                    indent, custom.name
                ));
                output.push_str(&format!(
                    "{}        <div{}{}>{}        </div>\n",
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
                output.push_str(&format!(
                    "{}        <hr role=\"separator\" style=\"{}\" />\n",
                    indent, style
                ));
            }
            CanvasComponent::Checkbox(checkbox) => {
                let change_handler = if let Some(handler) = &checkbox.on_change {
                    format!("on:change={}", handler)
                } else {
                    let signal_name = format!("checkbox_{}", signals.len());
                    signals.push((signal_name.clone(), checkbox.checked.to_string()));
                    format!(
                        "prop:checked={} on:change=move |ev| set_{}(event_target_checked(&ev))",
                        signal_name, signal_name
                    )
                };
                output.push_str(&format!(
                    "{}        <label class=\"checkbox\">\n{}            <input type=\"checkbox\" {} disabled={} {}\n{}            {}\n{}        </label>\n",
                    indent,
                    indent,
                    change_handler,
                    checkbox.disabled,
                    get_animation_css(&checkbox.animation),
                    indent,
                    checkbox.label,
                    indent
                ));
            }
            CanvasComponent::RadioGroup(radio) => {
                let group = format!("radio-group-{}", radio.id);
                let signal_name = format!("radio_{}", signals.len());
                let on_change_attr = if let Some(handler) = &radio.on_change {
                    format!(" on:change={}", handler)
                } else {
                    format!(
                        " on:change=move |ev| set_{}(event_target_value(&ev))",
                        signal_name
                    )
                };

                signals.push((
                    signal_name.clone(),
                    format!("\"{}\".to_string()", rust_string_literal(&radio.selected)),
                ));

                output.push_str(&format!(
                    "{}        <div class=\"radio-group\" role=\"radiogroup\">\n",
                    indent
                ));
                for opt in radio
                    .options
                    .split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                {
                    output.push_str(&format!(
                        "{}            <label><input type=\"radio\" name=\"{}\" value=\"{opt}\" prop:checked=move || {}.get() == \"{opt}\" disabled={}{} /> {opt}</label>\n",
                        indent,
                        group,
                        signal_name,
                        radio.disabled,
                        on_change_attr
                    ));
                }
                output.push_str(&format!("{}        </div>\n", indent));
            }
            CanvasComponent::Switch(switch) => {
                let change_handler = if let Some(handler) = &switch.on_change {
                    format!("on:change={}", handler)
                } else {
                    let signal_name = format!("switch_{}", signals.len());
                    signals.push((signal_name.clone(), switch.checked.to_string()));
                    format!(
                        "prop:checked={} on:change=move |ev| set_{}(event_target_checked(&ev))",
                        signal_name, signal_name
                    )
                };
                output.push_str(&format!(
                    "{}        <label class=\"switch\">\n{}            <input type=\"checkbox\" {} disabled={} />\n{}            {}\n{}        </label>\n",
                    indent,
                    indent,
                    change_handler,
                    switch.disabled,
                    indent,
                    switch.label,
                    indent
                ));
            }
            CanvasComponent::Badge(badge) => {
                let variant = match badge.variant {
                    crate::domain::BadgeVariant::Default => "badge-default",
                    crate::domain::BadgeVariant::Primary => "badge-primary",
                    crate::domain::BadgeVariant::Success => "badge-success",
                    crate::domain::BadgeVariant::Warning => "badge-warning",
                    crate::domain::BadgeVariant::Error => "badge-error",
                };
                output.push_str(&format!(
                    "{}        <span class=\"badge {}\">{}</span>\n",
                    indent, variant, badge.text
                ));
            }
            CanvasComponent::Progress(progress) => {
                output.push_str(&format!(
                    "{}        <progress value=\"{}\" max=\"{}\" style=\"width: 100%;\"></progress>\n",
                    indent, progress.value, progress.max
                ));
                if progress.show_label {
                    let percent = if progress.max > 0.0 {
                        (progress.value / progress.max * 100.0).clamp(0.0, 100.0)
                    } else {
                        0.0
                    };
                    output.push_str(&format!("{}        <span>{:.0}%</span>\n", indent, percent));
                }
            }
            CanvasComponent::Link(link) => {
                // The values go into Rust string literals inside a `view!` macro,
                // so they are escaped for a Rust literal, *not* as HTML. Escaping
                // them as HTML would bake entities into the runtime value
                // (`?a=1&b=2` becoming `?a=1&amp;b=2`).
                let href_value = rust_string_literal(&link.href);
                let text_value = rust_string_literal(&link.text);

                let href_attr = if let Some(bind) = link.bindings.get("href") {
                    format!("href=move || {}.get()", bind)
                } else {
                    format!("href=\"{}\"", href_value)
                };

                let id_attr = if let Some(bind) = link.bindings.get("id") {
                    format!(" id=move || {}.get()", bind)
                } else {
                    String::new()
                };

                let class_attr = if let Some(bind) = link.bindings.get("custom_css_classes") {
                    format!(" class=move || format!(\"{{}}\", {}.get())", bind)
                } else {
                    String::new()
                };

                let text_expr = if let Some(bind) = link.bindings.get("text") {
                    format!("move || {}.get()", bind)
                } else {
                    format!("\"{}\"", text_value)
                };

                // The canvas applies the link's style and animation; the export
                // must carry the same inline CSS or a styled link regresses. The
                // CSS goes into a Rust string literal too, so it is escaped as
                // one — HTML escaping here would corrupt any value containing
                // `&`.
                let inline_css = component_inline_css(&link.style, &link.animation);
                let style_attr = if inline_css.is_empty() {
                    String::new()
                } else {
                    format!(" style=\"{}\"", rust_string_literal(&inline_css))
                };

                output.push_str(&format!(
                    "{}        <a {}{}{}{}>{}</a>\n",
                    indent, href_attr, id_attr, class_attr, style_attr, text_expr
                ));
            }
        }

        Ok(())
    }
}

impl CodeGenerator for LeptosGenerator {
    fn generate_with(
        &self,
        config: &GeneratorConfig,
        components: &[CanvasComponent],
        variables: &[Variable],
    ) -> AppResult<String> {
        let mut output = attribution(config, "// ", "");

        let mut signals: Vec<(String, String)> = Vec::new();

        // Add imports
        output.push_str(&self.generate_imports());
        output.push('\n');

        // Generate component function
        output.push_str("#[component]\n");
        output.push_str("pub fn App() -> impl IntoView {\n");

        // We need to generate the body first to populate signals, then inject them
        // This requires buffering the view body
        let mut view_body = String::new();
        view_body.push_str("    view! {\n");
        // For the Plain preset, embed base component CSS so the export is self-contained
        if self.preset == ExportPreset::Plain {
            view_body.push_str(&format!("        <style>\"{}\"</style>\n", EXPORT_CSS));
        }
        for component in components {
            self.generate_component(component, &mut view_body, 0, &mut signals)?;
        }
        view_body.push_str("    }\n");

        // Now inject signals
        // First global variables
        if !variables.is_empty() {
            output.push_str("    // Global signals\n");
            for var in variables {
                let default_val = match var.data_type {
                    VariableType::String => format!("\"{}\".to_string()", var.default_value),
                    VariableType::Number => var.default_value.clone(),
                    VariableType::Boolean => var.default_value.clone(),
                };
                output.push_str(&format!(
                    "    let ({}, set_{}) = signal({});\n",
                    var.name, var.name, default_val
                ));
            }
            output.push('\n');
        }

        // Then local required signals
        if !signals.is_empty() {
            output.push_str("    // Local signals\n");
            for (name, default) in signals.iter() {
                output.push_str(&format!(
                    "    let ({}, set_{}) = signal({});\n",
                    name, name, default
                ));
            }
            output.push('\n');
        }

        output.push_str(&view_body);
        output.push_str("}\n");

        Ok(output)
    }

    fn file_extension(&self) -> &str {
        "rs"
    }
}
