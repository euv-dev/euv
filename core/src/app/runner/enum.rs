/// Diagnostic reason explaining why a rendered frame promoted to full-viewport damage.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum FullRedrawReason {
    /// Partial dirty-rect rendering succeeded or the frame was skipped while idle.
    #[default]
    None,
    /// Initial frame rendered before any prior framebuffer baseline existed.
    FirstFrame,
    /// Viewport logical or physical dimensions changed since the previous rendered frame.
    Resize,
    /// Display DPI scale factor changed since the previous rendered frame.
    DpiChange,
    /// Layout geometry changes fell back to full-viewport redraw.
    LayoutFallback,
    /// A visible top-layer portal or fixed overlay intersected the dirty region.
    PortalBackdrop,
    /// Dirty node count exceeded the bounded partial-damage tracking threshold.
    DamageOverflow,
    /// Explicit full redraw requested by configuration, lifecycle, theme, or mode change.
    Forced,
}
