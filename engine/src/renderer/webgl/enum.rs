use super::*;

/// Which shader stage a GLSL source is compiled for.
///
/// WebGL identifies a shader by the raw `VERTEX_SHADER` /
/// `FRAGMENT_SHADER` enum rather than by a string, and the same raw
/// `u32` is threaded through the [`GlProgram`] path.
/// This enum names the two stages so a caller cannot pass a meaningless
/// value and have the browser reject the compile later.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GlShaderKind {
    /// The per-vertex stage, which runs once for every vertex of every
    /// instance.
    #[default]
    Vertex,
    /// The per-fragment stage, which runs once for every surviving
    /// fragment after rasterization.
    Fragment,
}

/// The completeness verdict a framebuffer reports after its attachments
/// are bound.
///
/// WebGL returns a single `u32` status code rather than an error, so the
/// distinct failure modes a caller must actually react to are lifted
/// into named variants: one means "an attachment is missing", one means
/// "the attachments disagree with each other", and one means "this
/// driver cannot do what you asked for at all".
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum GlFramebufferStatus {
    /// `FRAMEBUFFER_COMPLETE`. Every attachment is present, every
    /// attachment agrees on its dimensions, and the combination is
    /// renderable.
    #[default]
    Complete,
    /// `FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT`: an attachment enum
    /// names nothing bound. A binding mistake, almost always.
    Incomplete,
    /// `FRAMEBUFFER_INCOMPLETE_DIMENSIONS` or
    /// `FRAMEBUFFER_INCOMPLETE_MULTISAMPLE`: the attachments disagree on
    /// size or sample count. A resource-allocation mistake, almost
    /// always a renderbuffer that was never resized alongside its color
    /// texture.
    Unsupported,
    /// Any other non-complete status the driver reports, including
    /// `FRAMEBUFFER_UNSUPPORTED` and every implementation-specific code.
    /// Treated as unsupported because the frame cannot be rendered into.
    Other,
}

/// Why a [`WebGl2Backend`] could not be constructed.
///
/// WebGL reports context-creation failure by returning `null` rather than
/// by throwing, so the three ways it fails, a missing canvas, a
/// non-matching selector, and a browser with no WebGL 2 support, would
/// otherwise all collapse into one opaque `None`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum WebGl2InitError {
    /// The configured selector matched no element in the document.
    CanvasNotFound(String),
    /// The selector could not be evaluated, because the document has no
    /// body yet or the selector is not valid.
    CanvasQuery(String),
    /// `getContext("webgl2")` threw rather than returning `None`.
    ContextLookup(String),
    /// `getContext("webgl2")` returned `None`, which means the browser
    /// has no WebGL 2 support, or the canvas already holds a context of a
    /// different kind.
    ContextUnavailable,
    /// A `webgl2` context was returned but was not a
    /// `WebGL2RenderingContext`, which should not be reachable.
    ContextCast,
}

/// Errors that can occur while building a WebGL shader program.
///
/// Each variant carries the browser-provided info log so the caller can
/// surface the exact GLSL diagnostic without losing fidelity.
#[derive(Clone, Debug)]
pub enum WebGlProgramError {
    /// Vertex or fragment shader compilation failed.
    ///
    /// Carries the shader info log returned by `getShaderInfoLog`.
    ShaderCompile(String),
    /// Program linking failed (or `createProgram` returned `None`).
    ///
    /// Carries the program info log returned by `getProgramInfoLog`.
    ProgramLink(String),
}
