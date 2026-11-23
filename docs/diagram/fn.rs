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
    request.options.diagram_id = Some(diagram_id);
    request.pipeline = Some(scoped_pipeline());
    let output: Result<RenderOutput, RenderError> = Renderer::new().render(
        RenderRequest::svg(source, OperationControl::new(), request).with_theme(fallback_theme()),
    );
    match output {
        Ok(RenderOutput::Svg(Some(svg))) => Some(svg.svg().to_string()),
        _ => None,
    }
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

/// Builds the render pipeline that appends the theme stylesheet.
///
/// The stylesheet is scoped to the diagram's own id, so two diagrams on
/// one page never share a rule, and every colour it names is a docs
/// theme token — the diagram therefore follows the root theme class.
///
/// # Returns
///
/// - `SvgPipeline` - The pipeline the render request carries.
fn scoped_pipeline() -> SvgPipeline {
    SvgOutputPolicy {
        preset: SvgPipelinePreset::Parity,
        css_override_policy: CssOverridePolicy::StripExistingImportant,
        root_background_color: None,
        scoped_css: Some(SCOPED_CSS.to_string()),
        ..SvgOutputPolicy::default()
    }
    .pipeline()
}
