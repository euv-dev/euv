use super::*;

/// Renders a mermaid diagram as inline SVG.
///
/// The diagram is rendered by `merman`, a pure-Rust mermaid engine, so
/// the published site ships no JavaScript runtime and no `.js` or `.css`
/// asset: the SVG is produced inside the wasm bundle and inlined into
/// the page.
///
/// The colours come from a scoped stylesheet the engine appends to the
/// SVG itself. Every colour it names is a docs theme token, and CSS
/// custom properties inherit into an inline SVG, so the diagram follows
/// the root theme class — light or dark — with no re-render and no
/// `!important`.
///
/// A diagram the engine cannot lay out is not rendered as a broken
/// picture: the fence falls back to the ordinary code block, so the
/// source stays readable.
///
/// # Arguments
///
/// - `VirtualNode<DocsMermaidProps>` - The props node carrying the
///   mermaid source and the diagram's ordinal on the page.
///
/// # Returns
///
/// - `VirtualNode` - The diagram virtual DOM tree.
#[component]
pub(crate) fn docs_mermaid(node: VirtualNode<DocsMermaidProps>) -> VirtualNode {
    let DocsMermaidProps { source, ordinal }: DocsMermaidProps =
        node.try_get_props().unwrap_or_default();
    let Some(svg) = render_mermaid(source, ordinal) else {
        return html! {
            docs_code_block {
                lang: MERMAID_LANG
                code: source
            }
        };
    };
    html! {
        figure {
            class: c_docs_mermaid()
            div {
                class: c_docs_mermaid_frame()
                inner_html: svg
            }
        }
    }
}

/// Renders one mermaid source to an inline SVG string.
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
