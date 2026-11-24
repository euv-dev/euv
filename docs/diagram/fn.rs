use super::*;

/// Renders one fenced block as a diagram, when the fence is a diagram.
///
/// Only a mermaid fence is a diagram; every other fence is source code.
/// A diagram the engine cannot lay out yields `None`, and the caller
/// keeps the ordinary code block so the source stays readable.
///
/// # Arguments
///
/// - `&str` - The fence language of the block.
/// - `&str` - The raw source of the fence.
/// - `usize` - The ordinal of the diagram on the page.
///
/// # Returns
///
/// - `Option<String>` - The inline SVG of the diagram, or `None` when
///   the fence is not a diagram or the engine cannot lay it out.
pub(crate) fn render_diagram(lang: &str, code: &str, ordinal: usize) -> Option<String> {
    if lang != MERMAID_LANG {
        return None;
    }
    render_mermaid(code, ordinal)
}

/// Renders one mermaid source to an inline SVG string.
///
/// The diagram is rendered at build time, so the published wasm bundle
/// never links the mermaid engine: the engine is a build dependency of
/// `euv-docs`, and only the finished SVG travels into the page data.
///
/// A diagram the engine cannot lay out yields `None`, and the caller
/// keeps the ordinary code block so the source stays readable.
///
/// # Arguments
///
/// - `&str` - The mermaid source of the fence.
/// - `usize` - The ordinal of the diagram on the page.
///
/// # Returns
///
/// - `Option<String>` - The SVG, or `None` when the engine cannot lay
///   the diagram out.
pub(crate) fn render_mermaid(source: &str, ordinal: usize) -> Option<String> {
    let diagram_id: String = format!("{DIAGRAM_ID_PREFIX}{ordinal}");
    let mut request: SvgRequest = SvgRequest::default();
    request.options.diagram_id = Some(diagram_id.clone());
    request.pipeline = Some(engine_pipeline());
    let output: Result<RenderOutput, RenderError> = Renderer::new().render(
        RenderRequest::svg(source, OperationControl::new(), request).with_theme(fallback_theme()),
    );
    let svg: String = match output {
        Ok(RenderOutput::Svg(Some(svg))) => svg.svg().to_string(),
        _ => return None,
    };
    Some(apply_theme_sheet(&svg, &diagram_id))
}

/// Appends the theme sheet to a rendered diagram.
///
/// The engine paints every box and every label from its own stylesheet,
/// scoped to the diagram's id and written with literal colours. A rule
/// the page stylesheet declares without that id scope loses to the
/// engine's rule, which is why the sheet is scoped here rather than left
/// to the page: an unscoped `.actor` cannot override
/// `#docs-mermaid-0 .actor`.
///
/// The sheet is appended after the engine's own `<style>`, so where the
/// two declare the same selector the sheet wins on source order rather
/// than on a specificity the engine could still raise.
///
/// # Arguments
///
/// - `&str` - The rendered SVG.
/// - `&str` - The diagram's scoped id.
///
/// # Returns
///
/// - `String` - The SVG carrying the theme sheet.
fn apply_theme_sheet(svg: &str, diagram_id: &str) -> String {
    let scoped: String = scope_theme_sheet(SCOPED_CSS, diagram_id);
    let sheet: String = format!("<style>{scoped}</style>");
    match svg.find("</svg>") {
        Some(position) => format!("{}{}{}", &svg[..position], sheet, &svg[position..]),
        None => format!("{svg}{sheet}"),
    }
}

/// Scopes every selector of the theme sheet to one diagram.
///
/// A rule may name several selectors, and each of them is scoped on its
/// own, so `.messageLine0,.messageLine1` becomes
/// `#docs-mermaid-0 .messageLine0,#docs-mermaid-0 .messageLine1`.
///
/// # Arguments
///
/// - `&str` - The unscoped sheet.
/// - `&str` - The diagram's scoped id.
///
/// # Returns
///
/// - `String` - The sheet whose every selector carries the id.
fn scope_theme_sheet(sheet: &str, diagram_id: &str) -> String {
    let mut scoped: String = String::new();
    for rule in sheet
        .split('}')
        .filter(|rule: &&str| !rule.trim().is_empty())
    {
        let rule: &str = rule.trim();
        let Some((selectors, declarations)) = rule.split_once('{') else {
            continue;
        };
        let prefixed: Vec<String> = selectors
            .split(',')
            .map(|selector: &str| format!("#{diagram_id} {}", selector.trim()))
            .collect();
        scoped.push_str(&prefixed.join(","));
        scoped.push('{');
        scoped.push_str(declarations);
        scoped.push_str("}\n");
    }
    scoped
}

/// Builds the theme whose colours the scoped stylesheet overrides.
///
/// The engine's default theme paints the diagram's boxes with an
/// inline `style`, and an inline style beats every stylesheet rule, so
/// a diagram rendered with the default theme ignores the page's theme
/// tokens entirely. A preset theme paints the same boxes from the
/// stylesheet, which is what lets the scoped sheet — and with it the
/// page's own tokens — decide the colours.
///
/// The preset is the fallback: it names the same roles the scoped sheet
/// overrides, so a diagram the sheet does not reach still reads as a
/// diagram rather than as a hole.
///
/// # Returns
///
/// - `DiagramTheme` - The theme the render request carries.
fn fallback_theme() -> DiagramTheme {
    DiagramThemeCompiler::new()
        .compile_preset(ThemePreset::EditorLight)
        .unwrap_or_else(|_: ThemeDesignCompileError| {
            unreachable!("the engine ships a theme it can always compile")
        })
}

/// Builds the pipeline that prepares the engine's stylesheet.
///
/// The engine writes some of its own declarations as important, and an
/// important declaration beats a later rule of any specificity, so the
/// engine's markers are stripped here for the theme sheet to be able to
/// win. The sheet itself is not handed to the engine: a caller-supplied
/// `scoped_css` never reaches the rendered output, so the sheet is
/// appended to the finished SVG instead.
///
/// # Returns
///
/// - `SvgPipeline` - The pipeline the render request carries.
fn engine_pipeline() -> SvgPipeline {
    SvgOutputPolicy {
        preset: SvgPipelinePreset::Parity,
        css_override_policy: CssOverridePolicy::StripExistingImportant,
        root_background_color: None,
        scoped_css: None,
        ..SvgOutputPolicy::default()
    }
    .pipeline()
}
