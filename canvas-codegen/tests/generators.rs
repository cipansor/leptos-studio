//! The generator suite, exercising the crate the way a consumer would.

use canvas_codegen::CodeGenerator;
use canvas_codegen::domain::*;
use canvas_codegen::generators::*;
use canvas_codegen::preset::ExportPreset;

#[test]
fn test_leptos_generator() {
    let generator = LeptosGenerator::new(ExportPreset::Plain);
    let button = CanvasComponent::Button(ButtonComponent::new("Click me".to_string()));
    let code = generator.generate(&[button], &[]).unwrap();

    assert!(code.contains("use leptos::prelude::*;"));
    assert!(code.contains("#[component]"));
    assert!(code.contains("pub fn App()"));
    assert!(code.contains("Click me"));
    assert!(code.contains("on:click=")); // Check for new interactive code
}

#[test]
fn test_leptos_input_generator() {
    let generator = LeptosGenerator::new(ExportPreset::Plain);
    let variables = vec![];
    let input = CanvasComponent::Input(InputComponent {
        id: Default::default(),
        input_type: InputType::Text,
        placeholder: "Type here".to_string(),
        required: false,
        disabled: false,
        on_input: None,
        on_change: None,
        animation: None,
        bindings: Default::default(),
        style: Default::default(),
    });
    let code = generator.generate(&[input], &variables).unwrap();

    // Should generate a signal
    assert!(code.contains("let (input_0, set_input_0) = signal(String::new());"));
    // Should use the signal
    assert!(code.contains("prop:value=input_0"));
    assert!(code.contains("on:input=move |ev| set_input_0(event_target_value(&ev))"));
}

#[test]
fn test_html_generator() {
    let generator = HtmlGenerator;
    let text = CanvasComponent::Text(TextComponent::new("Hello World".to_string()));
    let code = generator.generate(&[text], &[]).unwrap();

    assert!(code.contains("<!DOCTYPE html>"));
    assert!(code.contains("<body>"));
    assert!(code.contains("Hello World"));
}

#[test]
fn test_json_generator() {
    let generator = JsonGenerator;
    let button = CanvasComponent::Button(ButtonComponent::new("Test".to_string()));
    let code = generator.generate(&[button], &[]).unwrap();

    assert!(code.contains("Button"));
    assert!(code.contains("Test"));
}

#[test]
fn test_markdown_generator() {
    let generator = MarkdownGenerator;
    let button = CanvasComponent::Button(ButtonComponent::new("Test".to_string()));
    let code = generator.generate(&[button], &[]).unwrap();

    assert!(code.contains("# Generated Layout Documentation"));
    assert!(code.contains("**Button**"));
    assert!(code.contains("Test"));
}

#[test]
fn test_leptos_select_generator() {
    use canvas_codegen::domain::SelectComponent;
    let generator = LeptosGenerator::new(ExportPreset::Plain);
    let variables = vec![];
    let select = CanvasComponent::Select(SelectComponent {
        id: Default::default(),
        options: "A, B, C".to_string(),
        placeholder: "Choose".to_string(),
        disabled: false,
        on_change: None,
        animation: None,
        bindings: Default::default(),
        style: Default::default(),
    });
    let code = generator.generate(&[select], &variables).unwrap();

    assert!(code.contains("<select disabled=false  >"));
    assert!(code.contains("<option value=\"\" disabled selected>\"Choose\"</option>"));
    assert!(code.contains("<option value=\"A\">\"A\"</option>"));
    assert!(code.contains("<option value=\"B\">\"B\"</option>"));
}

#[test]
fn test_html_select_generator() {
    use canvas_codegen::domain::SelectComponent;
    let generator = HtmlGenerator;
    let variables = vec![];
    let select = CanvasComponent::Select(SelectComponent {
        id: Default::default(),
        options: "X, Y".to_string(),
        placeholder: "Pick".to_string(),
        disabled: true,
        on_change: None,
        animation: None,
        bindings: Default::default(),
        style: Default::default(),
    });
    let code = generator.generate(&[select], &variables).unwrap();

    assert!(code.contains("<select disabled>"));
    assert!(code.contains("<option value=\"\" disabled selected>Pick</option>"));
    assert!(code.contains("<option value=\"X\">X</option>"));
}

#[test]
fn test_markdown_select_generator() {
    use canvas_codegen::domain::SelectComponent;
    let generator = MarkdownGenerator;
    let variables = vec![];
    let select = CanvasComponent::Select(SelectComponent {
        id: Default::default(),
        options: "One, Two".to_string(),
        placeholder: "Select One".to_string(),
        disabled: false,
        on_change: None,
        animation: None,
        bindings: Default::default(),
        style: Default::default(),
    });
    let code = generator.generate(&[select], &variables).unwrap();

    assert!(code.contains("- **Select**"));
    assert!(code.contains("- Placeholder: Select One"));
    assert!(code.contains("- Options: One, Two"));
}

#[test]
fn test_leptos_new_components_generator() {
    use canvas_codegen::domain::{
        BadgeComponent, CheckboxComponent, DividerComponent, ProgressComponent,
        RadioGroupComponent, SwitchComponent,
    };
    let components = vec![
        CanvasComponent::Divider(DividerComponent::new()),
        CanvasComponent::Checkbox(CheckboxComponent::new("Accept".to_string())),
        CanvasComponent::RadioGroup(RadioGroupComponent {
            options: "Alpha, Beta".to_string(),
            ..RadioGroupComponent::new()
        }),
        CanvasComponent::Switch(SwitchComponent::new("Enabled".to_string())),
        CanvasComponent::Badge(BadgeComponent::new("New".to_string())),
        CanvasComponent::Progress(ProgressComponent::new()),
    ];

    let generator = LeptosGenerator::new(ExportPreset::Plain);
    let code = generator.generate(&components, &[]).unwrap();

    assert!(code.contains("use leptos::prelude::*;"));
    assert!(code.contains("<hr role=\"separator\""));
    assert!(code.contains("Accept"));
    assert!(code.contains("type=\"radio\""));
    assert!(code.contains("Alpha"));
    assert!(code.contains("Enabled"));
    assert!(code.contains("badge badge-default"));
    assert!(code.contains("<progress value=\"50\" max=\"100\""));
    // Check that interactive signals were created
    assert!(code.contains("let (checkbox_"));
    assert!(code.contains("let (radio_"));
    assert!(code.contains("let (switch_"));
    // Plain preset embeds base CSS
    assert!(code.contains("<style>"));
}

#[test]
fn test_html_new_components_generator() {
    use canvas_codegen::domain::{BadgeComponent, DividerComponent, ProgressComponent};
    let components = vec![
        CanvasComponent::Divider(DividerComponent::new()),
        CanvasComponent::Badge(BadgeComponent::new("Tag".to_string())),
        CanvasComponent::Progress(ProgressComponent::new()),
    ];

    let generator = HtmlGenerator;
    let code = generator.generate(&components, &[]).unwrap();

    assert!(code.contains("<hr"));
    assert!(code.contains("badge-default"));
    assert!(code.contains("<progress"));
}

// ---------------------------------------------------------------------
// Link export: a styled/animated Link must keep its visual settings in
// every visual format, not collapse to a bare anchor.
// ---------------------------------------------------------------------

/// A link with a non-default URL, non-default text, a clearly visible style
/// and an animation — the shape every assertion below checks for.
fn styled_link() -> CanvasComponent {
    use canvas_codegen::domain::{Animation, AnimationType, ComponentStyle, LinkComponent};
    CanvasComponent::Link(LinkComponent {
        id: Default::default(),
        text: "Read the docs".to_string(),
        href: "https://example.com/guide?a=1&b=2".to_string(),
        style: ComponentStyle {
            color: Some("#2563eb".to_string()),
            font_size: Some(18),
            font_weight: Some("bold".to_string()),
            padding: Some("4px 8px".to_string()),
            background_color: Some("#f8fafc".to_string()),
            ..Default::default()
        },
        animation: Some(Animation {
            animation_type: AnimationType::FadeIn,
            duration: 0.5,
            delay: 0.0,
            infinite: false,
        }),
        bindings: Default::default(),
    })
}

#[test]
fn test_html_link_export_preserves_style_and_animation() {
    let html = HtmlGenerator.generate(&[styled_link()], &[]).unwrap();

    assert!(
        html.contains("href=\"https://example.com/guide?a=1&amp;b=2\""),
        "the href must be preserved and attribute-escaped, got:\n{html}"
    );
    assert!(html.contains("Read the docs"), "the text must be preserved");
    assert!(
        html.contains("color: #2563eb") && html.contains("font-size: 18px"),
        "the ComponentStyle must be exported inline, got:\n{html}"
    );
    assert!(
        html.contains("animation: fadeIn 0.5s"),
        "the animation must be exported inline, got:\n{html}"
    );
}

#[test]
fn test_leptos_link_export_preserves_style_and_animation() {
    let code = LeptosGenerator::new(ExportPreset::Plain)
        .generate(&[styled_link()], &[])
        .unwrap();

    assert!(
        code.contains("href=\"https://example.com/guide?a=1&b=2\""),
        "the Leptos href must keep its data (not HTML entities), got:\n{code}"
    );
    assert!(code.contains("Read the docs"));
    assert!(
        code.contains("color: #2563eb") && code.contains("animation: fadeIn 0.5s"),
        "the Leptos anchor must carry the same inline CSS as the canvas, got:\n{code}"
    );
}

#[test]
fn test_leptos_link_href_binding_is_exported() {
    let mut link = match styled_link() {
        CanvasComponent::Link(l) => l,
        _ => unreachable!(),
    };
    link.bindings
        .insert("href".to_string(), "next_url".to_string());

    let code = LeptosGenerator::new(ExportPreset::Plain)
        .generate(&[CanvasComponent::Link(link)], &[])
        .unwrap();

    assert!(
        code.contains("next_url.get()"),
        "an href binding must survive into the Leptos export, got:\n{code}"
    );
}

/// A `<script>` in the text must not be able to inject markup.
#[test]
fn test_link_export_escapes_text() {
    let mut link = match styled_link() {
        CanvasComponent::Link(l) => l,
        _ => unreachable!(),
    };
    link.text = "<script>alert('x')</script>".to_string();

    let html = HtmlGenerator
        .generate(&[CanvasComponent::Link(link)], &[])
        .unwrap();
    assert!(
        !html.contains("<script>"),
        "link text must be escaped, got:\n{html}"
    );
    assert!(html.contains("&lt;script&gt;"));
}

#[test]
fn test_markdown_link_is_semantic() {
    let md = MarkdownGenerator.generate(&[styled_link()], &[]).unwrap();

    // Markdown cannot carry style or animation; it must at least keep a real
    // link rather than degrade to a bare label.
    assert!(
        md.contains("[Read the docs](https://example.com/guide?a=1&b=2)"),
        "Markdown must emit a semantic link, got:\n{md}"
    );
}

/// Regression: the Leptos generator escaped the link's href and text as
/// HTML, so the entities ended up baked into the generated Rust string
/// literal. Leptos sets that literal on the DOM verbatim — entities are not
/// decoded — so `?a=1&b=2` rendered as `?a=1&amp;b=2` and a label containing
/// `&` gained a literal `&amp;`. The literal must instead be escaped for
/// Rust, so the data survives unchanged.
#[test]
fn test_leptos_link_does_not_html_escape_data() {
    let mut link = match styled_link() {
        CanvasComponent::Link(l) => l,
        _ => unreachable!(),
    };
    link.href = "https://example.com/guide?a=1&b=2&c=<3>".to_string();
    link.text = "A & B".to_string();

    let code = LeptosGenerator::new(ExportPreset::Plain)
        .generate(&[CanvasComponent::Link(link)], &[])
        .unwrap();

    assert!(
        code.contains("href=\"https://example.com/guide?a=1&b=2&c=<3>\""),
        "the href must reach the Leptos literal unchanged, got:\n{code}"
    );
    assert!(
        code.contains("\"A & B\""),
        "the link text must reach the Leptos literal unchanged, got:\n{code}"
    );
    assert!(
        !code.contains("&amp;") && !code.contains("&lt;"),
        "data must not be turned into HTML entities in a Leptos literal, got:\n{code}"
    );
}

/// The Rust literal escaper must neutralise everything that could end the
/// literal or be reinterpreted by the lexer: backslash, double quote and
/// newline. The generated module has to stay parseable while the runtime
/// value stays identical.
#[test]
fn test_leptos_link_literal_escapes_for_rust() {
    let mut link = match styled_link() {
        CanvasComponent::Link(l) => l,
        _ => unreachable!(),
    };
    link.text = "say \"hi\"\\ then\nnew line".to_string();

    let code = LeptosGenerator::new(ExportPreset::Plain)
        .generate(&[CanvasComponent::Link(link)], &[])
        .unwrap();

    assert!(
        code.contains(r#""say \"hi\"\\ then\nnew line""#),
        "quotes, backslashes and newlines must be escaped for a Rust literal, got:\n{code}"
    );
    assert!(
        !code.contains("\"say \"hi\"\\ then"),
        "an unescaped quote must not terminate the literal, got:\n{code}"
    );
}

/// HTML output is markup, so the data *must* be entity-escaped there — the
/// opposite of the Leptos case. This pins that the two contexts stay
/// separate rather than one being "fixed" into the other.
#[test]
fn test_html_link_still_uses_entities() {
    let mut link = match styled_link() {
        CanvasComponent::Link(l) => l,
        _ => unreachable!(),
    };
    link.href = "https://example.com/guide?a=1&b=2".to_string();
    link.text = "<script>alert('x') & more</script>".to_string();

    let html = HtmlGenerator
        .generate(&[CanvasComponent::Link(link)], &[])
        .unwrap();

    assert!(
        html.contains("&amp;"),
        "an HTML attribute must escape `&` as an entity, got:\n{html}"
    );
    assert!(
        !html.contains("<script>"),
        "an HTML text node must escape markup, got:\n{html}"
    );
    assert!(html.contains("&lt;script&gt;"));
}

/// JSON is a lossless serialisation, so the link's style and animation must
/// round-trip through it as structured data.
#[test]
fn test_json_link_export_keeps_style_and_animation() {
    let json = JsonGenerator.generate(&[styled_link()], &[]).unwrap();

    assert!(json.contains("\"href\": \"https://example.com/guide?a=1&b=2\""));
    assert!(json.contains("\"text\": \"Read the docs\""));
    assert!(json.contains("\"color\": \"#2563eb\""));
    assert!(json.contains("\"animation_type\": \"FadeIn\""));
}

/// Every applied `ComponentStyle` field must reach both the canvas and every
/// visual exporter. The canvas renders `StyleTag`/`to_css_string` directly, so
/// this pins the exporter side to the same set: a field added to
/// `to_css_string` without a matching exporter entry (or vice versa) fails
/// here. The reserved `custom_css` field is deliberately excluded — it is
/// applied by neither (see `custom_css_is_not_a_visual_setting`).
#[test]
fn test_every_applied_style_field_reaches_the_canvas_and_exporters() {
    use canvas_codegen::domain::{Animation, AnimationType, ComponentStyle, LinkComponent};
    use canvas_codegen::generators::{
        ReactGenerator, SvelteGenerator, TailwindHtmlGenerator, VueGenerator,
    };

    let style = ComponentStyle {
        padding: Some("11px".to_string()),
        margin: Some("12px".to_string()),
        width: Some("13px".to_string()),
        height: Some("14px".to_string()),
        color: Some("rgb(21, 22, 23)".to_string()),
        background_color: Some("rgb(31, 32, 33)".to_string()),
        border_color: Some("rgb(41, 42, 43)".to_string()),
        border_width: Some(3),
        border_radius: Some(17),
        font_size: Some(18),
        font_weight: Some("600".to_string()),
        text_align: Some("center".to_string()),
        display: Some("flex".to_string()),
        flex_direction: Some("column".to_string()),
        gap: Some("19px".to_string()),
        custom_css: None,
    };

    // The declarations the canvas applies. Each must survive into every
    // visual exporter.
    let canvas_css = style.to_css_string();
    let expected = [
        "padding: 11px;",
        "margin: 12px;",
        "width: 13px;",
        "height: 14px;",
        "color: rgb(21, 22, 23);",
        "background-color: rgb(31, 32, 33);",
        "border: 3px solid rgb(41, 42, 43);",
        "border-radius: 17px;",
        "font-size: 18px;",
        "font-weight: 600;",
        "text-align: center;",
        "display: flex;",
        "flex-direction: column;",
        "gap: 19px;",
    ];
    for declaration in expected {
        assert!(
            canvas_css.contains(declaration),
            "the canvas must apply `{declaration}`; to_css_string returned:\n{canvas_css}"
        );
    }

    let link = CanvasComponent::Link(LinkComponent {
        id: Default::default(),
        text: "Styled".to_string(),
        href: "https://example.com".to_string(),
        style,
        animation: Some(Animation {
            animation_type: AnimationType::FadeIn,
            duration: 0.5,
            delay: 0.0,
            infinite: false,
        }),
        bindings: Default::default(),
    });

    let one = std::slice::from_ref(&link);
    let visual_outputs: Vec<(&str, String)> = vec![
        (
            "Leptos",
            LeptosGenerator::new(ExportPreset::Plain)
                .generate(one, &[])
                .unwrap(),
        ),
        ("HTML", HtmlGenerator.generate(one, &[]).unwrap()),
        ("React", ReactGenerator.generate(one, &[]).unwrap()),
        ("Vue", VueGenerator.generate(one, &[]).unwrap()),
        ("Svelte", SvelteGenerator.generate(one, &[]).unwrap()),
        (
            "Tailwind HTML",
            TailwindHtmlGenerator.generate(one, &[]).unwrap(),
        ),
    ];

    for (name, output) in &visual_outputs {
        assert!(
            output.contains("https://example.com"),
            "{name} must keep the href, got:\n{output}"
        );
        assert!(
            output.contains("fadeIn") && output.contains("0.5s"),
            "{name} must keep the animation, got:\n{output}"
        );
        // Each canvas declaration's value must be present, in the framework's
        // own spelling (`backgroundColor` vs `background-color`).
        for value in [
            "11px",
            "12px",
            "13px",
            "14px",
            "rgb(21, 22, 23)",
            "rgb(31, 32, 33)",
            "rgb(41, 42, 43)",
            "17px",
            "18px",
            "600",
            "center",
            "flex",
            "column",
            "19px",
        ] {
            assert!(
                output.contains(value),
                "{name} is missing the style value `{value}` present on the canvas, got:\n{output}"
            );
        }
    }
}

#[test]
fn test_json_schema_generator() {
    let generator = JsonSchemaGenerator;
    let button = CanvasComponent::Button(ButtonComponent::new("Test".to_string()));
    let schema = generator.generate(&[button], &[]).unwrap();

    assert!(schema.contains("$schema"));
    assert!(schema.contains("definitions"));
    assert!(schema.contains("ButtonComponent"));
}

#[test]
fn test_typescript_generator() {
    let generator = TypeScriptGenerator;
    let types = generator.generate(&[], &[]).unwrap();

    assert!(types.contains("ButtonVariant"));
    assert!(types.contains("CanvasComponent"));
    assert!(types.contains("export interface"));
}

#[test]
fn test_react_generator() {
    let generator = ReactGenerator;
    let text = CanvasComponent::Text(TextComponent::new("Hello".to_string()));
    let code = generator.generate(&[text], &[]).unwrap();

    assert!(code.contains("import React"));
    assert!(code.contains("Hello"));
    assert!(code.contains("export function"));
}

#[test]
fn test_vue_generator() {
    let generator = VueGenerator;
    let text = CanvasComponent::Text(TextComponent::new("Hello".to_string()));
    let code = generator.generate(&[text], &[]).unwrap();

    assert!(code.contains("<template>"));
    assert!(code.contains("<script setup"));
    assert!(code.contains("Hello"));
}

#[test]
fn test_css_generator() {
    let generator = CssGenerator;
    let button = CanvasComponent::Button(ButtonComponent::new("Test".to_string()));
    let css = generator.generate(&[button], &[]).unwrap();

    assert!(css.contains(".btn-primary"));
    assert!(css.contains(".btn-secondary"));
}

#[test]
fn test_tailwind_generator() {
    let generator = TailwindHtmlGenerator;
    let button = CanvasComponent::Button(ButtonComponent::new("Test".to_string()));
    let html = generator.generate(&[button], &[]).unwrap();

    assert!(html.contains("class="));
    assert!(html.contains("Test"));
}

#[test]
fn test_svelte_generator() {
    let generator = SvelteGenerator;
    let text = CanvasComponent::Text(TextComponent::new("Hello".to_string()));
    let code = generator.generate(&[text], &[]).unwrap();

    assert!(code.contains("<script"));
    assert!(code.contains("Hello"));
}

// ---------------------------------------------------------------------
// Link export: every visual format must keep the link's style and
// animation instead of collapsing to a bare anchor.
// ---------------------------------------------------------------------

fn link_text() -> String {
    match styled_link() {
        CanvasComponent::Link(l) => l.text,
        _ => unreachable!(),
    }
}

#[test]
fn test_react_link_export_preserves_style_and_animation() {
    let code = ReactGenerator.generate(&[styled_link()], &[]).unwrap();

    assert!(
        code.contains("href=\"https://example.com/guide?a=1&amp;b=2\""),
        "the href must be preserved, got:\n{code}"
    );
    assert!(code.contains(&link_text()));
    // React takes styles as an object with camelCase keys.
    assert!(
        code.contains("style={{"),
        "React must emit a style object, got:\n{code}"
    );
    assert!(
        code.contains("color: '#2563eb'") && code.contains("fontSize: '18px'"),
        "the ComponentStyle must be exported as JS properties, got:\n{code}"
    );
    assert!(
        code.contains("animation: 'fadeIn 0.5s ease-in-out 0s 1 both'"),
        "the animation must be exported as a JS property, got:\n{code}"
    );
}

#[test]
fn test_vue_link_export_preserves_style_and_animation() {
    let code = VueGenerator.generate(&[styled_link()], &[]).unwrap();

    assert!(
        code.contains("href=\"https://example.com/guide?a=1&amp;b=2\""),
        "the href must be preserved, got:\n{code}"
    );
    assert!(code.contains(&link_text()));
    assert!(
        code.contains("color: #2563eb") && code.contains("animation: fadeIn 0.5s"),
        "the ComponentStyle and animation must be exported inline, got:\n{code}"
    );
}

#[test]
fn test_svelte_link_export_preserves_style_and_animation() {
    let code = SvelteGenerator.generate(&[styled_link()], &[]).unwrap();

    assert!(code.contains("href=\"https://example.com/guide?a=1&amp;b=2\""));
    assert!(code.contains(&link_text()));
    assert!(
        code.contains("color: #2563eb") && code.contains("animation: fadeIn 0.5s"),
        "the ComponentStyle and animation must be exported inline, got:\n{code}"
    );
}

#[test]
fn test_tailwind_link_export_preserves_style_and_animation() {
    let html = TailwindHtmlGenerator
        .generate(&[styled_link()], &[])
        .unwrap();

    assert!(html.contains("href=\"https://example.com/guide?a=1&amp;b=2\""));
    assert!(html.contains(&link_text()));
    assert!(
        html.contains("color: #2563eb") && html.contains("animation: fadeIn 0.5s"),
        "Tailwind output must keep per-component style and animation, got:\n{html}"
    );
}

#[test]
fn test_json_schema_includes_link_style_surface() {
    let schema = JsonSchemaGenerator.generate(&[styled_link()], &[]).unwrap();

    assert!(schema.contains("LinkComponent"));
    assert!(
        schema.contains("\"style\"") && schema.contains("\"animation\""),
        "the Link schema must describe style/animation, got:\n{schema}"
    );
}

#[test]
fn test_typescript_link_type_includes_style_and_animation() {
    let types = TypeScriptGenerator.generate(&[], &[]).unwrap();

    assert!(types.contains("export interface LinkComponent"));
    assert!(
        types.contains("export interface ComponentStyle"),
        "ComponentStyle must be declared so LinkComponent can reference it"
    );
    assert!(types.contains("export interface Animation"));
    assert!(
        types.contains("style?: ComponentStyle") && types.contains("animation?: Animation"),
        "LinkComponent must expose style and animation, got:\n{types}"
    );
}

/// A href binding must survive into the binding-aware generators.
#[test]
fn test_link_href_binding_is_exported_in_react() {
    let mut link = match styled_link() {
        CanvasComponent::Link(l) => l,
        _ => unreachable!(),
    };
    link.bindings
        .insert("href".to_string(), "next_url".to_string());
    let code = ReactGenerator
        .generate(&[CanvasComponent::Link(link)], &[])
        .unwrap();

    assert!(
        code.contains("vars['next_url']"),
        "the href binding must be exported, got:\n{code}"
    );
}

/// A quote inside a style value must not break the generated JS literal.
#[test]
fn test_react_link_export_escapes_style_values() {
    let mut link = match styled_link() {
        CanvasComponent::Link(l) => l,
        _ => unreachable!(),
    };
    link.style.font_weight = Some("bold'; alert('x".to_string());

    let code = ReactGenerator
        .generate(&[CanvasComponent::Link(link)], &[])
        .unwrap();

    assert!(
        code.contains("bold\\'; alert(\\'x"),
        "a quote in a style value must be escaped in the JS literal, got:\n{code}"
    );
}

// ---------------------------------------------------------------------
// `ComponentStyle.custom_css`: reserved, not an applied visual setting.
//
// The field has no defined semantics (nothing writes or reads it), so it
// must not leak into any visual output — neither a `style` attribute nor a
// `class` — while the JSON and TypeScript exporters must keep representing
// it so existing project data round-trips. These tests pin both halves of
// that decision; if `custom_css` is ever given real semantics they must be
// rewritten together with the renderer.
// ---------------------------------------------------------------------

/// The same link, with and without the reserved `custom_css` field.
fn link_with_custom_css(value: Option<&str>) -> CanvasComponent {
    let mut link = match styled_link() {
        CanvasComponent::Link(l) => l,
        _ => unreachable!(),
    };
    link.style.custom_css = value.map(str::to_string);
    CanvasComponent::Link(link)
}

#[test]
fn custom_css_is_not_a_visual_setting() {
    use canvas_codegen::generators::JsonGenerator;
    use canvas_codegen::generators::{HtmlGenerator, LeptosGenerator};
    use canvas_codegen::preset::ExportPreset;

    // A value that would be obvious in the output if it were ever emitted as
    // either inline CSS or a class name.
    let needle = "sentinel-custom-css-do-not-emit";
    let (without, with) = (
        &[link_with_custom_css(None)],
        &[link_with_custom_css(Some(needle))],
    );

    let visual_outputs: Vec<(&str, String)> = vec![
        (
            "Leptos",
            LeptosGenerator::new(ExportPreset::Plain)
                .generate(with, &[])
                .unwrap(),
        ),
        ("HTML", HtmlGenerator.generate(with, &[]).unwrap()),
        ("React", ReactGenerator.generate(with, &[]).unwrap()),
        ("Vue", VueGenerator.generate(with, &[]).unwrap()),
        ("Svelte", SvelteGenerator.generate(with, &[]).unwrap()),
        (
            "Tailwind HTML",
            TailwindHtmlGenerator.generate(with, &[]).unwrap(),
        ),
    ];

    for (name, output) in &visual_outputs {
        assert!(
            !output.contains(needle),
            "{name} must not emit the reserved custom_css field, got:\n{output}"
        );
    }

    // ...and the reserved field must not change the visual output at all:
    // the canvas ignores it, so every visual exporter must too.
    let without_react = ReactGenerator.generate(without, &[]).unwrap();
    let with_react = ReactGenerator.generate(with, &[]).unwrap();
    assert_eq!(
        without_react, with_react,
        "React output must be identical whether or not custom_css is set"
    );

    let without_html = canvas_codegen::generators::HtmlGenerator
        .generate(without, &[])
        .unwrap();
    assert_eq!(
        without_html, visual_outputs[1].1,
        "HTML output must be identical whether or not custom_css is set"
    );

    // The JSON exporter must preserve it losslessly — that is the one
    // supported surface, and existing files must not lose the data.
    let json = JsonGenerator.generate(with, &[]).unwrap();
    assert!(
        json.contains(needle),
        "JSON must preserve custom_css losslessly, got:\n{json}"
    );
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(
        parsed[0]["Link"]["style"]["custom_css"],
        serde_json::json!(needle),
        "custom_css must round-trip as its own style field"
    );

    // TypeScript must keep declaring it so generated code can carry the data.
    let types = TypeScriptGenerator.generate(&[], &[]).unwrap();
    assert!(
        types.contains("custom_css?: string;"),
        "ComponentStyle must still declare custom_css, got:\n{types}"
    );
}
