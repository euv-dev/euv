use super::*;

/// A chart, bound to one canvas element.
///
/// The chart owns its option, its layout, and its display list. Setting
/// an option invalidates the layout; rendering rebuilds both and hands
/// the resulting display list to the backend.
///
/// The backend is chosen when the chart is created: WebGPU if the host
/// has it, WebGL2 if it does not, Canvas2D otherwise. From the outside
/// the three behave identically — the same option produces the same
/// picture, and the same API draws it.
#[derive(Data)]
pub struct Chart {
    /// The canvas the chart draws into.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) canvas: HtmlCanvasElement,
    /// The configuration of the chart.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(skip)]
    pub(crate) option: ChartOption,
    /// The rendering backend.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) backend: Box<dyn ChartBackend>,
    /// The measurer and the glyph atlas the text is drawn with.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) shaper: TextShaper,
    /// The layout of the chart.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) layout: ChartLayout,
    /// The display list the backend replays.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) display_list: DisplayList,
    /// The renderer that turns the layout into the display list.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) renderer: ChartRenderer,
    /// The logical width of the canvas, in CSS pixels.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) width: f64,
    /// The logical height of the canvas, in CSS pixels.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) height: f64,
    /// The device pixel ratio the chart renders at.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) dpr: f64,
    /// Whether the layout has to be recomputed.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) layout_dirty: bool,
    /// Whether the display list has to be rebuilt.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) display_dirty: bool,
    /// The event handlers registered on the chart.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) handlers: Vec<EventRegistration>,
    /// The animator that moves the series between two states.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) animator: Animator,
    /// The tooltip the chart currently shows.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) tooltip: Option<TooltipState>,
    /// The maps the chart can draw.
    ///
    /// The geometry of a map is far too large to live in an option: the
    /// same map is drawn by every chart of an application, and it is
    /// registered once.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) maps: MapRegistry,
    /// The commands that arrived since the last frame.
    ///
    /// The commands travel through a signal, so an application that
    /// holds nothing but a handle can still drive the chart.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) commands: Signal<Vec<ChartCommand>>,
}

/// A handle to a chart.
///
/// The handle is what an application keeps when the chart itself has
/// to live somewhere else — in a component, in a closure, in a
/// callback. The commands it carries travel through a signal, so the
/// chart picks them up on its next frame, and an application that
/// holds nothing but a signal can still drive a chart.
///
/// The handle is a copy: every copy talks to the same chart.
#[derive(Clone, Copy, Data, Debug)]
pub struct ChartHandle {
    /// The commands waiting for the chart.
    pub commands: Signal<Vec<ChartCommand>>,
}

/// The state of the chart's tooltip.
///
/// The tooltip is a first-class part of the chart: an application shows
/// it with an action, and the chart draws it above everything else.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct TooltipState {
    /// The index of the series the tooltip describes.
    #[get(type(copy))]
    pub series_index: usize,
    /// The index of the data point the tooltip describes.
    #[get(type(copy))]
    pub data_index: usize,
    /// The horizontal position of the tooltip, in canvas pixels.
    #[get(type(copy))]
    pub x: f64,
    /// The vertical position of the tooltip, in canvas pixels.
    #[get(type(copy))]
    pub y: f64,
    /// The lines the tooltip shows.
    pub lines: Vec<(String, String)>,
}

/// Turns a chart's option and layout into a display list.
///
/// The renderer is the only place that knows how a chart is put
/// together: the background, the grid, the axes, the series, the title,
/// and the legend. It writes into a display list, which the backends
/// replay — so a new backend never has to know what a chart looks like.
///
/// The renderer is public so that a benchmark — or a server-side render —
/// can measure the cost of a layout without a canvas.
#[derive(Clone, Data, Debug)]
pub struct ChartRenderer {
    /// The renderer that draws the series.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) series: SeriesRenderer,
}
