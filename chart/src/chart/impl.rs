use super::*;

/// Inherent implementation of [`Chart`].
impl Chart {
    /// Creates a chart on the given canvas.
    ///
    /// The backend is chosen automatically: WebGPU when the host has it,
    /// WebGL2 when it does not, and Canvas2D everywhere else. The chart
    /// looks the same whichever one it gets.
    ///
    /// Returns `None` when the host has no DOM, or when the canvas
    /// cannot give out a rendering context at all.
    ///
    /// # Arguments
    ///
    /// - `HtmlCanvasElement` - The `HtmlCanvasElement` value.
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - The `Option<Self>` value.
    pub async fn create(canvas: HtmlCanvasElement) -> Option<Self> {
        Self::create_with_backend(canvas, BackendKind::WebGpu).await
    }

    /// Creates a chart on the given canvas, preferring a particular
    /// backend.
    ///
    /// The preference is honoured when the host can provide it; the
    /// factory falls back to the next backend down when it cannot.
    ///
    /// # Arguments
    ///
    /// - `HtmlCanvasElement` - The `HtmlCanvasElement` value.
    /// - `BackendKind` - The `BackendKind` value.
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - The `Option<Self>` value.
    pub async fn create_with_backend(
        canvas: HtmlCanvasElement,
        preferred: BackendKind,
    ) -> Option<Self> {
        let backend: Box<dyn ChartBackend> = BackendFactory::create(&canvas, preferred).await?;
        let measurer: TextMeasurer = TextMeasurer::new()?;
        let atlas: GlyphAtlas = GlyphAtlas::new(DEFAULT_ATLAS_SIZE, DEFAULT_ATLAS_SIZE)?;
        let shaper: TextShaper = TextShaper::new(measurer, atlas);

        let (width, height) = canvas_size(&canvas);
        let mut chart: Chart = Self {
            canvas,
            option: ChartOption::new(),
            backend,
            shaper,
            layout: ChartLayout::default(),
            display_list: DisplayList::new(),
            renderer: ChartRenderer::new(),
            width,
            height,
            dpr: detect_dpr(),
            layout_dirty: true,
            display_dirty: true,
            handlers: Vec::new(),
            animator: Animator::new(),
            tooltip: None,
            maps: MapRegistry::new(),
            commands: Signal::create(Vec::new()),
        };
        chart.resize(width, height);
        Some(chart)
    }

    /// Sets the configuration of the chart.
    ///
    /// The chart is re-laid out and re-rendered. When the chart
    /// animates, the new configuration is the target of the animation and
    /// the chart moves towards it; call [`Chart::advance`] to move it
    /// along.
    ///
    /// # Arguments
    ///
    /// - `ChartOption` - A `ChartOption` parameter.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A `mut Self` value.
    pub fn set_option(&mut self, option: ChartOption) -> &mut Self {
        self.get_mut_animator().begin(&option);
        *self.get_mut_option() = option;
        self.set_layout_dirty(true);
        self.set_display_dirty(true);
        self.render()
    }

    /// Advances the animation by the given number of milliseconds.
    ///
    /// Returns `true` when the animation is still running. The call
    /// re-renders the chart, so an animation loop is a call to
    /// `advance` once per frame.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn advance(&mut self, delta: f64) -> bool {
        if !self.get_mut_animator().advance(delta) {
            return false;
        }
        let mut option: ChartOption = self.get_option().clone();
        self.get_mut_animator().apply(&mut option);
        *self.get_mut_option() = option;
        self.set_layout_dirty(true);
        self.set_display_dirty(true);
        self.render();
        true
    }

    /// Returns the animator of the chart.
    ///
    /// # Returns
    ///
    /// - `&mut Animator` - A `mut Animator` value.
    pub fn animator(&mut self) -> &mut Animator {
        self.get_mut_animator()
    }

    /// Returns the configuration of the chart.
    ///
    /// # Returns
    ///
    /// - `&ChartOption` - A `ChartOption` value.
    pub fn option(&self) -> &ChartOption {
        self.get_option()
    }

    /// Returns the configuration of the chart, mutably.
    ///
    /// Changing the configuration marks the chart as needing a new
    /// layout; call [`Chart::render`] to apply the change.
    ///
    /// # Returns
    ///
    /// - `&mut ChartOption` - A `mut ChartOption` value.
    pub fn option_mut(&mut self) -> &mut ChartOption {
        self.set_layout_dirty(true);
        self.set_display_dirty(true);
        self.get_mut_option()
    }

    /// Lays the chart out and draws it.
    ///
    /// The call is cheap when nothing changed: the layout and the
    /// display list are only rebuilt when the option or the size
    /// changed.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A `mut Self` value.
    pub fn render(&mut self) -> &mut Self {
        // The commands that arrived since the last frame are performed
        // first, so that the frame that follows them draws what they
        // asked for.
        self.drain_commands();
        if self.get_layout_dirty() {
            let layout: ChartLayout = ChartLayout::compute_with_maps(
                self.get_option(),
                self.get_width(),
                self.get_height(),
                self.get_shaper().measurer(),
                self.get_maps(),
            );
            self.set_layout(layout);
            self.set_layout_dirty(false);
            self.set_display_dirty(true);
        }
        if self.get_display_dirty() {
            // The renderer, the display list and the option are taken out
            // of the chart together: the renderer writes into the list
            // while it reads the option.
            let Self {
                renderer,
                display_list,
                option,
                layout,
                tooltip,
                ..
            } = self;
            renderer.build_with_tooltip(display_list, option, layout, tooltip.as_ref());
            self.set_display_dirty(false);
        }
        // The backend takes the display list and the shaper at the same
        // time, so the two are taken out of the chart together.
        let Self {
            backend,
            display_list,
            shaper,
            ..
        } = self;
        backend.render(display_list, shaper);
        self
    }

    /// Performs the commands that arrived since the last frame.
    ///
    /// The queue is emptied before the commands are performed, so that a
    /// command that draws the chart does not run the queue twice.
    fn drain_commands(&mut self) {
        let waiting: bool = self
            .get_commands()
            .with(|queue: &Vec<ChartCommand>| !queue.is_empty());
        if !waiting {
            return;
        }
        let queue: Vec<ChartCommand> = self.get_commands().get();
        self.get_commands().set(Vec::new());
        for command in queue {
            match command {
                ChartCommand::SetOption(option) => {
                    self.get_mut_animator().begin(&option);
                    *self.get_mut_option() = *option;
                    self.set_layout_dirty(true);
                    self.set_display_dirty(true);
                }
                ChartCommand::Resize(width, height) => {
                    self.set_width(width.max(1.0));
                    self.set_height(height.max(1.0));
                    self.set_dpr(detect_dpr());
                    let dpr: f64 = self.get_dpr();
                    let chart_width: f64 = self.get_width();
                    let chart_height: f64 = self.get_height();
                    self.get_mut_backend()
                        .resize(chart_width, chart_height, dpr);
                    self.set_layout_dirty(true);
                    self.set_display_dirty(true);
                }
                ChartCommand::Action(action) => {
                    self.dispatch_action(action);
                }
            }
        }
    }

    /// Returns a handle to the chart.
    ///
    /// The handle is how an application that does not hold the chart
    /// talks to it: the commands it sends travel through a signal, and
    /// the chart performs them on its next frame.
    ///
    /// # Returns
    ///
    /// - `ChartHandle` - A `ChartHandle` value.
    pub fn handle(&self) -> ChartHandle {
        ChartHandle {
            commands: *self.get_commands(),
        }
    }

    /// Resizes the chart.
    ///
    /// The size is in CSS pixels; the chart multiplies it by the device
    /// pixel ratio to get the size of the drawing buffer.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A `mut Self` value.
    pub fn resize(&mut self, width: f64, height: f64) -> &mut Self {
        self.set_width(width.max(1.0));
        self.set_height(height.max(1.0));
        self.set_dpr(detect_dpr());
        let dpr: f64 = self.get_dpr();
        let chart_width: f64 = self.get_width();
        let chart_height: f64 = self.get_height();
        self.get_mut_backend()
            .resize(chart_width, chart_height, dpr);
        self.set_layout_dirty(true);
        self.set_display_dirty(true);
        self.render()
    }

    /// Returns the kind of the backend the chart runs on.
    ///
    /// # Returns
    ///
    /// - `BackendKind` - A `BackendKind` value.
    pub fn backend_kind(&self) -> BackendKind {
        self.get_backend().kind()
    }

    /// Returns the layout of the chart.
    ///
    /// # Returns
    ///
    /// - `&ChartLayout` - A `ChartLayout` value.
    pub fn layout(&self) -> &ChartLayout {
        self.get_layout()
    }

    /// Returns the display list the chart draws.
    ///
    /// # Returns
    ///
    /// - `&DisplayList` - A `DisplayList` value.
    pub fn display_list(&self) -> &DisplayList {
        self.get_display_list()
    }

    /// Returns the canvas the chart draws into.
    ///
    /// # Returns
    ///
    /// - `&HtmlCanvasElement` - A `HtmlCanvasElement` value.
    pub fn canvas(&self) -> &HtmlCanvasElement {
        self.get_canvas()
    }

    /// Registers a handler for a kind of event.
    ///
    /// # Arguments
    ///
    /// - `ChartEventType` - A `ChartEventType` parameter.
    /// - `EventHandler` - A `EventHandler` parameter.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A `mut Self` value.
    pub fn on(&mut self, event_type: ChartEventType, handler: EventHandler) -> &mut Self {
        self.get_mut_handlers().push(EventRegistration {
            event_type,
            handler,
        });
        self
    }

    /// Removes every handler registered for a kind of event.
    ///
    /// # Arguments
    ///
    /// - `ChartEventType` - A `ChartEventType` parameter.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A `mut Self` value.
    pub fn off(&mut self, event_type: ChartEventType) -> &mut Self {
        self.get_mut_handlers()
            .retain(|registration: &EventRegistration| registration.event_type != event_type);
        self
    }

    /// Dispatches an event to the registered handlers.
    ///
    /// # Arguments
    ///
    /// - `ChartEvent` - A `ChartEvent` parameter.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A `mut Self` value.
    pub fn emit(&mut self, event: ChartEvent) -> &mut Self {
        let handlers: Vec<EventHandler> = self
            .get_handlers()
            .iter()
            .filter(|registration: &&EventRegistration| registration.event_type == event.event_type)
            .map(|registration: &EventRegistration| registration.handler.clone())
            .collect();
        for handler in handlers {
            handler(&event);
        }
        self
    }

    /// Performs an action.
    ///
    /// Actions are how an application drives a chart: the same mechanism
    /// ECharts calls `dispatchAction`, but expressed as a method on the
    /// chart rather than a free function.
    ///
    /// # Arguments
    ///
    /// - `ChartAction` - A `ChartAction` parameter.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A `mut Self` value.
    pub fn dispatch_action(&mut self, action: ChartAction) -> &mut Self {
        match action {
            ChartAction::DataZoom { start, end } => {
                self.apply_data_zoom(start, end);
            }
            ChartAction::LegendToggle { name } => {
                self.toggle_legend(&name);
            }
            ChartAction::Restore => {
                self.restore();
            }
            ChartAction::Highlight {
                series_index,
                data_index,
            } => {
                self.highlight(series_index, data_index);
            }
            ChartAction::Downplay {
                series_index,
                data_index,
            } => {
                self.downplay(series_index, data_index);
            }
            ChartAction::ShowTip {
                series_index,
                data_index,
                x,
                y,
            } => {
                self.show_tip(series_index, data_index, x, y);
            }
            ChartAction::HideTip => {
                self.hide_tip();
            }
        }
        self
    }

    /// Shows the tooltip over a data point.
    ///
    /// # Arguments
    ///
    /// - `usize` - A non-negative integer (`usize`).
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A `mut Self` value.
    pub fn show_tip(
        &mut self,
        series_index: usize,
        data_index: usize,
        x: f64,
        y: f64,
    ) -> &mut Self {
        let lines: Vec<(String, String)> = self.tooltip_lines(series_index, data_index);
        self.set_tooltip(Some(TooltipState {
            series_index,
            data_index,
            x,
            y,
            lines,
        }));
        self.set_display_dirty(true);
        self
    }

    /// Hides the tooltip.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A `mut Self` value.
    pub fn hide_tip(&mut self) -> &mut Self {
        self.set_tooltip(None);
        self.set_display_dirty(true);
        self
    }

    /// Returns the tooltip the chart currently shows.
    ///
    /// # Returns
    ///
    /// - `Option<&TooltipState>` - `Some(...)` on success, `None` otherwise.
    pub fn tooltip(&self) -> Option<&TooltipState> {
        self.try_get_tooltip().as_ref()
    }

    /// Finds the data point under the given position.
    ///
    /// The search walks the chart's series from the last one to the
    /// first, so that the series drawn on top wins — which is what the
    /// user expects to click on.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `Option<ChartEvent>` - `Some(...)` on success, `None` otherwise.
    pub fn hit_test(&self, x: f64, y: f64) -> Option<ChartEvent> {
        for series in self.get_layout().series.iter().rev() {
            for point in series.points.iter().rev() {
                let distance: f64 = ((point.x - x).powi(2) + (point.y - y).powi(2)).sqrt();
                if distance <= HIT_RADIUS {
                    return Some(ChartEvent {
                        event_type: ChartEventType::Click,
                        series_index: Some(series.index),
                        data_index: Some(point.data_index),
                        series_name: series.name.clone(),
                        name: point.name.clone(),
                        value: Some(point.value),
                        x,
                        y,
                        offset_x: x,
                        offset_y: y,
                    });
                }
            }
        }
        None
    }

    /// Builds the lines the tooltip shows for a data point.
    ///
    /// # Arguments
    ///
    /// - `usize` - A non-negative integer (`usize`).
    ///
    /// # Returns
    ///
    /// - `Vec<(String, String)>` - A `Vec<(String, String)>` value.
    fn tooltip_lines(&self, series_index: usize, data_index: usize) -> Vec<(String, String)> {
        let Some(series) = self.get_option().series.get(series_index) else {
            return Vec::new();
        };
        let Some(data) = series.data.get(data_index) else {
            return Vec::new();
        };
        let value: String = match data {
            DataValue::Number(number) => format_number(*number),
            DataValue::List(items) => items
                .get(1)
                .and_then(|item: &DataValue| item.to_number())
                .map(format_number)
                .unwrap_or_default(),
            DataValue::Object(_) => data
                .get("value")
                .and_then(|item: &DataValue| item.to_number())
                .map(format_number)
                .unwrap_or_default(),
            _ => String::new(),
        };
        let name: String = match data {
            DataValue::List(items) => items
                .first()
                .and_then(|item: &DataValue| item.as_text())
                .unwrap_or_default()
                .to_string(),
            DataValue::Object(_) => data
                .get("name")
                .and_then(|item: &DataValue| item.as_text())
                .unwrap_or_default()
                .to_string(),
            _ => String::new(),
        };
        let mut lines: Vec<(String, String)> = Vec::new();
        if !series.name.is_empty() {
            lines.push((series.name.clone(), value.clone()));
        }
        if !name.is_empty() {
            lines.push((name, value));
        }
        lines
    }

    /// Highlights a data point.
    ///
    /// # Arguments
    ///
    /// - `usize` - A non-negative integer (`usize`).
    /// - `Option<usize>` - A `Option<usize>` parameter.
    fn highlight(&mut self, series_index: usize, data_index: Option<usize>) {
        let _: (usize, Option<usize>) = (series_index, data_index);
        self.set_display_dirty(true);
    }

    /// Removes the highlight from a data point.
    ///
    /// # Arguments
    ///
    /// - `usize` - A non-negative integer (`usize`).
    /// - `Option<usize>` - A `Option<usize>` parameter.
    fn downplay(&mut self, series_index: usize, data_index: Option<usize>) {
        let _: (usize, Option<usize>) = (series_index, data_index);
        self.set_display_dirty(true);
    }

    /// Zooms the chart to a window of the data.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    fn apply_data_zoom(&mut self, start: f64, end: f64) {
        let start: f64 = start.clamp(0.0, 100.0);
        let end: f64 = end.clamp(0.0, 100.0);
        if end <= start {
            return;
        }
        for control in self.get_mut_option().data_zoom.iter_mut() {
            control.start = Some(start);
            control.end = Some(end);
        }
        self.set_layout_dirty(true);
        self.set_display_dirty(true);
    }

    /// Toggles a legend entry.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    fn toggle_legend(&mut self, name: &str) {
        let Some(legend) = self.get_mut_option().legend.as_mut() else {
            return;
        };
        let mut selected: Vec<String> = legend.selected.take().unwrap_or_default();
        if let Some(position) = selected.iter().position(|item: &String| item == name) {
            selected.remove(position);
        } else {
            selected.push(name.to_string());
        }
        legend.selected = Some(selected);
        self.set_layout_dirty(true);
        self.set_display_dirty(true);
    }

    /// Restores the chart to its initial state.
    fn restore(&mut self) {
        for control in self.get_mut_option().data_zoom.iter_mut() {
            control.start = None;
            control.end = None;
        }
        if let Some(legend) = self.get_mut_option().legend.as_mut() {
            legend.selected = None;
        }
        self.set_layout_dirty(true);
        self.set_display_dirty(true);
    }

    /// Releases the resources the chart holds.
    pub fn dispose(&mut self) {
        self.get_mut_backend().dispose();
        self.get_mut_display_list().clear();
    }

    /// Registers a map, so that a series can draw it.
    ///
    /// The map is given as a GeoJSON document: a collection of
    /// features, each with a geometry and a name. The name of the map
    /// is the one a series asks for.
    ///
    /// The geometry is kept out of the option on purpose: a map of a
    /// country is megabytes of coordinates, and every chart of the
    /// application draws the very same one.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    /// - `&DataValue` - Shared reference to a `DataValue`.
    ///
    /// # Returns
    ///
    /// - `Result<&mut Self, GeoError>` - Result of the operation; an `Err` variant on failure.
    pub fn register_map(&mut self, name: &str, geojson: &DataValue) -> Result<&mut Self, GeoError> {
        self.get_mut_maps().register_geojson(name, geojson)?;
        self.set_layout_dirty(true);
        self.set_display_dirty(true);
        Ok(self)
    }

    /// Registers a map that was built by hand.
    ///
    /// # Arguments
    ///
    /// - `MapData` - A `MapData` parameter.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A `mut Self` value.
    pub fn register_map_data(&mut self, data: MapData) -> &mut Self {
        self.get_mut_maps().register(data);
        self.set_layout_dirty(true);
        self.set_display_dirty(true);
        self
    }

    /// Returns the maps the chart can draw.
    ///
    /// # Returns
    ///
    /// - `&MapRegistry` - A `MapRegistry` value.
    pub fn maps(&self) -> &MapRegistry {
        self.get_maps()
    }

    /// Returns the names of the maps the chart can draw.
    ///
    /// # Returns
    ///
    /// - `Vec<String>` - A `Vec<String>` value.
    pub fn map_names(&self) -> Vec<String> {
        self.get_maps().names()
    }

    /// Returns the chart as a data URL.
    ///
    /// The PNG is produced by the browser, so it is only available in a
    /// browser; a native build returns `None`.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - `Some(...)` on success, `None` otherwise.
    pub fn to_data_url(&self) -> Option<String> {
        self.get_canvas().to_data_url_with_type("image/png").ok()
    }
}

/// Inherent implementation of [`ChartHandle`].
impl ChartHandle {
    /// Sends a command to the chart.
    ///
    /// The chart performs the command on its next frame.
    ///
    /// # Arguments
    ///
    /// - `ChartCommand` - A `ChartCommand` parameter.
    ///
    /// # Returns
    ///
    /// - `&Self` - The `&Self` value.
    pub fn push(&self, command: ChartCommand) -> &Self {
        let mut queue: Vec<ChartCommand> = self.get_commands().get();
        queue.push(command);
        self.get_commands().set(queue);
        self
    }

    /// Sends an option to the chart.
    ///
    /// The chart draws the option on its next frame.
    ///
    /// # Arguments
    ///
    /// - `ChartOption` - A `ChartOption` parameter.
    ///
    /// # Returns
    ///
    /// - `&Self` - The `&Self` value.
    pub fn set_option(&self, option: ChartOption) -> &Self {
        self.push(ChartCommand::SetOption(Box::new(option)))
    }

    /// Resizes the chart, in CSS pixels.
    ///
    /// The chart draws at the new size on its next frame.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    ///
    /// # Returns
    ///
    /// - `&Self` - The `&Self` value.
    pub fn resize(&self, width: f64, height: f64) -> &Self {
        self.push(ChartCommand::Resize(width, height))
    }

    /// Performs an action on the chart.
    ///
    /// The chart performs the action on its next frame.
    ///
    /// # Arguments
    ///
    /// - `ChartAction` - A `ChartAction` parameter.
    ///
    /// # Returns
    ///
    /// - `&Self` - The `&Self` value.
    pub fn dispatch_action(&self, action: ChartAction) -> &Self {
        self.push(ChartCommand::Action(action))
    }

    /// Returns the number of commands waiting for the chart.
    ///
    /// # Returns
    ///
    /// - `usize` - A non-negative integer.
    pub fn pending(&self) -> usize {
        self.get_commands().get().len()
    }
}

/// Default construction for [`ChartRenderer`].
impl Default for ChartRenderer {
    /// Returns a renderer.
    fn default() -> Self {
        Self::new()
    }
}

/// Inherent implementation of [`ChartRenderer`].
impl ChartRenderer {
    /// Returns a renderer.
    pub fn new() -> Self {
        Self {
            series: SeriesRenderer::new(),
        }
    }

    /// Builds the display list of a chart.
    ///
    /// # Arguments
    ///
    /// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
    /// - `&ChartOption` - Shared reference to a `ChartOption`.
    /// - `&ChartLayout` - Shared reference to a `ChartLayout`.
    pub fn build(&mut self, list: &mut DisplayList, option: &ChartOption, layout: &ChartLayout) {
        self.build_with_tooltip(list, option, layout, None)
    }

    /// Builds the display list of a chart, with a tooltip on top.
    ///
    /// # Arguments
    ///
    /// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
    /// - `&ChartOption` - Shared reference to a `ChartOption`.
    /// - `&ChartLayout` - Shared reference to a `ChartLayout`.
    /// - `Option<&TooltipState>` - A `Option<&TooltipState>` parameter.
    pub fn build_with_tooltip(
        &mut self,
        list: &mut DisplayList,
        option: &ChartOption,
        layout: &ChartLayout,
        tooltip: Option<&TooltipState>,
    ) {
        list.clear();
        self.draw_background(list, option);
        self.draw_title(list, option, layout);
        self.draw_grid(list, option, layout);
        self.draw_axes(list, option, layout);
        self.get_mut_series().draw(list, option, layout);
        self.draw_legend(list, option, layout);
        if let Some(tooltip) = tooltip {
            self.draw_tooltip(list, option, layout, tooltip);
        }
    }

    /// Draws the tooltip.
    ///
    /// The tooltip is a rounded rectangle with one line per entry, and
    /// it is placed next to the point it describes — flipping to the
    /// other side when it would run off the edge of the canvas.
    ///
    /// # Arguments
    ///
    /// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
    /// - `&ChartOption` - Shared reference to a `ChartOption`.
    /// - `&ChartLayout` - Shared reference to a `ChartLayout`.
    /// - `&TooltipState` - Shared reference to a `TooltipState`.
    fn draw_tooltip(
        &mut self,
        list: &mut DisplayList,
        option: &ChartOption,
        layout: &ChartLayout,
        tooltip: &TooltipState,
    ) {
        if tooltip.lines.is_empty() {
            return;
        }
        let font_size: f64 = option
            .tooltip
            .as_ref()
            .and_then(|tooltip: &TooltipOption| tooltip.text_style.as_ref())
            .and_then(|style: &LabelOption| style.font_size)
            .unwrap_or(DEFAULT_TOOLTIP_FONT_SIZE);
        let padding: (f64, f64, f64, f64) = option
            .tooltip
            .as_ref()
            .and_then(|tooltip: &TooltipOption| tooltip.padding)
            .unwrap_or((8.0, 12.0, 8.0, 12.0));
        let background: Color = option
            .tooltip
            .as_ref()
            .and_then(|tooltip: &TooltipOption| tooltip.background_color.as_deref())
            .and_then(Color::parse)
            .unwrap_or_else(|| {
                Color::parse(DEFAULT_BACKGROUND).unwrap_or(Color::rgb(255, 255, 255))
            });
        let border: Color = option
            .tooltip
            .as_ref()
            .and_then(|tooltip: &TooltipOption| tooltip.border_color.as_deref())
            .and_then(Color::parse)
            .unwrap_or(Color::rgb(217, 217, 217));

        let mut width: f64 = 0.0_f64;
        for (name, value) in &tooltip.lines {
            let text = format!("{}: {}", name, value);
            width = width.max(text.len() as f64 * font_size * 0.6);
        }
        let line_height: f64 = font_size * 1.4;
        let height: f64 = line_height * tooltip.lines.len() as f64;
        let box_width: f64 = width + padding.1 + padding.3;
        let box_height: f64 = height + padding.0 + padding.2;

        // The tooltip sits to the right of the point, and flips to the
        // left when it would run off the canvas.
        let mut left: f64 = tooltip.x + 12.0;
        if left + box_width > layout.width {
            left = tooltip.x - 12.0 - box_width;
        }
        let mut top: f64 = tooltip.y - box_height * 0.5;
        if top < 0.0 {
            top = 0.0;
        }
        if top + box_height > layout.height {
            top = (layout.height - box_height).max(0.0);
        }

        let path: Path = Path::builder()
            .round_rect(
                left as f32,
                top as f32,
                box_width as f32,
                box_height as f32,
                4.0,
            )
            .build();
        list.fill(path.clone(), Fill::Solid(background), 1.0, Z_TOOLTIP);
        let border_width: f64 = option
            .tooltip
            .as_ref()
            .and_then(|tooltip: &TooltipOption| tooltip.border_width)
            .unwrap_or(1.0);
        if border_width > 0.0 {
            list.stroke(
                path,
                Stroke::Solid(border),
                LineStyle {
                    width: border_width,
                    ..LineStyle::new()
                },
                1.0,
                Z_TOOLTIP + 1,
            );
        }

        let mut style: TextStyle = TextStyle::new();
        style.size = font_size;
        style.align = TextAlign::Left;
        style.vertical_align = TextVerticalAlign::Middle;
        style.color = Color::rgb(51, 51, 51);
        for (index, (name, value)) in tooltip.lines.iter().enumerate() {
            let baseline: f64 = top + padding.0 + line_height * (index as f64 + 0.5);
            list.text(
                format!("{}: {}", name, value),
                left + padding.3,
                baseline,
                style.clone(),
                Z_TOOLTIP + 2,
            );
        }
    }

    /// Draws the chart's background.
    ///
    /// # Arguments
    ///
    /// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
    /// - `&ChartOption` - Shared reference to a `ChartOption`.
    fn draw_background(&mut self, list: &mut DisplayList, option: &ChartOption) {
        let Some(background) = option.background_color.as_deref() else {
            return;
        };
        let Some(color) = Color::parse(background) else {
            return;
        };
        let rect: Path = Path::builder().rect(0.0, 0.0, 1.0, 1.0).build();
        let mut scaled: Path = rect;
        // The background covers the whole canvas; the path is built with
        // the canvas' size so that the fill is not affected by the
        // device pixel ratio.
        scaled = scaled.transformed(&Transform2D::scale(1.0, 1.0));
        list.fill(scaled, Fill::Solid(color), 1.0, Z_BACKGROUND);
    }

    /// Draws the chart's title and subtitle.
    ///
    /// # Arguments
    ///
    /// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
    /// - `&ChartOption` - Shared reference to a `ChartOption`.
    /// - `&ChartLayout` - Shared reference to a `ChartLayout`.
    fn draw_title(&mut self, list: &mut DisplayList, option: &ChartOption, layout: &ChartLayout) {
        let Some(title) = &option.title else {
            return;
        };
        if !title.show || title.text.is_empty() {
            return;
        }
        let font_size: f64 = title
            .text_style
            .as_ref()
            .and_then(|style: &LabelOption| style.font_size)
            .unwrap_or(DEFAULT_TITLE_FONT_SIZE);
        let mut style: TextStyle = TextStyle::new();
        style.size = font_size;
        style.align = TextAlign::Center;
        style.vertical_align = TextVerticalAlign::Top;
        if let Some(text_style) = &title.text_style {
            if let Some(color) = text_style
                .color
                .as_ref()
                .and_then(|color: &String| Color::parse(color))
            {
                style.color = color;
            }
            if let Some(family) = &text_style.font_family {
                style.family = family.clone();
            }
            if let Some(weight) = &text_style.font_weight {
                style.weight = weight.clone();
            }
        }
        let center_x: f64 = layout.width * 0.5;
        list.text(title.text.clone(), center_x, 0.0, style, Z_TITLE);
    }

    /// Draws the chart's grid and the split lines of its axes.
    ///
    /// # Arguments
    ///
    /// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
    /// - `&ChartOption` - Shared reference to a `ChartOption`.
    /// - `&ChartLayout` - Shared reference to a `ChartLayout`.
    fn draw_grid(&mut self, list: &mut DisplayList, option: &ChartOption, layout: &ChartLayout) {
        let grid_option: GridOption = option.grid.clone().unwrap_or_default();
        if let Some(background) = grid_option.background_color.as_deref()
            && let Some(color) = Color::parse(background)
        {
            for grid in &layout.grids {
                let path: Path = Path::builder()
                    .rect(
                        grid.rect.min_x as f32,
                        grid.rect.min_y as f32,
                        grid.rect.width() as f32,
                        grid.rect.height() as f32,
                    )
                    .build();
                list.fill(path, Fill::Solid(color), 1.0, Z_GRID);
            }
        }
        if !grid_option.show {
            return;
        }
        if let Some(border) = grid_option.border_color.as_deref().and_then(Color::parse) {
            let width: f32 = grid_option.border_width.unwrap_or(1.0) as f32;
            for grid in &layout.grids {
                let path: Path = Path::builder()
                    .rect(
                        grid.rect.min_x as f32,
                        grid.rect.min_y as f32,
                        grid.rect.width() as f32,
                        grid.rect.height() as f32,
                    )
                    .build();
                list.stroke(
                    path,
                    Stroke::Solid(border),
                    LineStyle {
                        width: width as f64,
                        ..LineStyle::new()
                    },
                    1.0,
                    Z_GRID + 1,
                );
            }
        }
    }

    /// Draws the chart's axes: their lines, their ticks, their labels,
    /// and their names.
    ///
    /// # Arguments
    ///
    /// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
    /// - `&ChartOption` - Shared reference to a `ChartOption`.
    /// - `&ChartLayout` - Shared reference to a `ChartLayout`.
    fn draw_axes(&mut self, list: &mut DisplayList, option: &ChartOption, layout: &ChartLayout) {
        for grid in &layout.grids {
            for axis in &grid.x_axes {
                self.draw_axis(list, option, grid, axis, true);
            }
            for axis in &grid.y_axes {
                self.draw_axis(list, option, grid, axis, false);
            }
        }
    }

    /// Draws one axis.
    ///
    /// A chart that does not name its axes still gets ECharts' default
    /// axes: a category axis on the bottom with its line, its tick
    /// marks and its labels, and a value axis on the left with its
    /// labels and its split lines — and without a line of its own,
    /// because ECharts hides the line of a value axis whose other axis
    /// is a category axis.
    ///
    /// # Arguments
    ///
    /// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
    /// - `&ChartOption` - Shared reference to a `ChartOption`.
    /// - `&GridLayout` - Shared reference to a `GridLayout`.
    /// - `&AxisLayout` - Shared reference to a `AxisLayout`.
    /// - `bool` - A boolean (`bool`).
    fn draw_axis(
        &mut self,
        list: &mut DisplayList,
        option: &ChartOption,
        grid: &GridLayout,
        axis: &AxisLayout,
        horizontal: bool,
    ) {
        if !axis.show {
            return;
        }
        let axis_option: Option<&AxisOption> = if horizontal {
            option.x_axis.get(axis.index)
        } else {
            option.y_axis.get(axis.index)
        };

        // The split lines run across the grid, perpendicular to the
        // axis.
        let split_line: Option<&SplitLineOption> =
            axis_option.and_then(|option: &AxisOption| option.split_line.as_ref());
        let split_line_show: bool = match split_line {
            Some(split_line) => split_line.show,
            // A value axis draws its split lines, a category axis does
            // not: ECharts leaves the grid of a category axis alone.
            None => matches!(axis.kind, AxisType::Value | AxisType::Log),
        };
        if split_line_show {
            let color: Color = split_line
                .and_then(|split_line: &SplitLineOption| split_line.line_style.as_ref())
                .and_then(|style: &LineStyleOption| style.color.as_deref())
                .and_then(Color::parse)
                .unwrap_or_else(|| {
                    Color::parse(DEFAULT_SPLIT_LINE_COLOR).unwrap_or(Color::rgb(217, 217, 217))
                });
            let width: f64 = split_line
                .and_then(|split_line: &SplitLineOption| split_line.line_style.as_ref())
                .and_then(|style: &LineStyleOption| style.width)
                .unwrap_or(DEFAULT_SPLIT_LINE_WIDTH);
            for (index, tick) in axis.ticks.iter().enumerate() {
                // A category axis with more categories than fit
                // draws only every Nth split line, so that a chart
                // with a million points does not ask for a million
                // strokes.
                if index % axis.label_interval.max(1) != 0 {
                    continue;
                }
                let path: Path = if horizontal {
                    Path::builder()
                        .move_to(tick.position as f32, grid.rect.min_y as f32)
                        .line_to(tick.position as f32, grid.rect.max_y as f32)
                        .build()
                } else {
                    Path::builder()
                        .move_to(grid.rect.min_x as f32, tick.position as f32)
                        .line_to(grid.rect.max_x as f32, tick.position as f32)
                        .build()
                };
                list.stroke(
                    path,
                    Stroke::Solid(color),
                    LineStyle {
                        width,
                        ..LineStyle::new()
                    },
                    1.0,
                    Z_GRID,
                );
            }
        }

        // The axis' own line.
        let axis_line: Option<&AxisLineOption> =
            axis_option.and_then(|option: &AxisOption| option.axis_line.as_ref());
        let axis_line_show: bool = match axis_line {
            Some(axis_line) => axis_line.show,
            // ECharts draws the line of a category axis, and hides the
            // line of a value axis whose other axis is a category
            // axis: the grid of the chart says it better.
            None => draws_own_line(axis, grid, horizontal),
        };
        if axis_line_show {
            let color: Color = axis_line
                .and_then(|axis_line: &AxisLineOption| axis_line.line_style.as_ref())
                .and_then(|style: &LineStyleOption| style.color.as_deref())
                .and_then(Color::parse)
                .unwrap_or_else(|| {
                    Color::parse(DEFAULT_AXIS_LINE_COLOR).unwrap_or(Color::rgb(217, 217, 217))
                });
            let width: f64 = axis_line
                .and_then(|axis_line: &AxisLineOption| axis_line.line_style.as_ref())
                .and_then(|style: &LineStyleOption| style.width)
                .unwrap_or(DEFAULT_AXIS_LINE_WIDTH);
            let path: Path = if horizontal {
                Path::builder()
                    .move_to(grid.rect.min_x as f32, axis.offset as f32)
                    .line_to(grid.rect.max_x as f32, axis.offset as f32)
                    .build()
            } else {
                Path::builder()
                    .move_to(axis.offset as f32, grid.rect.min_y as f32)
                    .line_to(axis.offset as f32, grid.rect.max_y as f32)
                    .build()
            };
            list.stroke(
                path,
                Stroke::Solid(color),
                LineStyle {
                    width,
                    ..LineStyle::new()
                },
                1.0,
                Z_AXIS,
            );
        }

        // The tick marks: a short line at every tick, outside the grid.
        let tick_option: Option<&AxisTickOption> =
            axis_option.and_then(|option: &AxisOption| option.axis_tick.as_ref());
        let tick_show: bool = match tick_option {
            Some(tick) => tick.show,
            // The tick marks follow the axis' line: ECharts draws them
            // wherever it draws the line.
            None => draws_own_line(axis, grid, horizontal),
        };
        if tick_show {
            let length: f64 = tick_option
                .and_then(|tick: &AxisTickOption| tick.length)
                .unwrap_or(DEFAULT_AXIS_TICK_LENGTH);
            let color: Color = tick_option
                .and_then(|tick: &AxisTickOption| tick.line_style.as_ref())
                .and_then(|style: &LineStyleOption| style.color.as_deref())
                .and_then(Color::parse)
                .or_else(|| {
                    axis_line
                        .and_then(|axis_line: &AxisLineOption| axis_line.line_style.as_ref())
                        .and_then(|style: &LineStyleOption| style.color.as_deref())
                        .and_then(Color::parse)
                })
                .unwrap_or_else(|| {
                    Color::parse(DEFAULT_AXIS_LINE_COLOR).unwrap_or(Color::rgb(217, 217, 217))
                });
            let width: f64 = tick_option
                .and_then(|tick: &AxisTickOption| tick.line_style.as_ref())
                .and_then(|style: &LineStyleOption| style.width)
                .unwrap_or(DEFAULT_AXIS_LINE_WIDTH);
            for (index, tick) in axis.ticks.iter().enumerate() {
                if index % axis.label_interval.max(1) != 0 {
                    continue;
                }
                let path: Path = if horizontal {
                    Path::builder()
                        .move_to(tick.position as f32, axis.offset as f32)
                        .line_to(tick.position as f32, (axis.offset + length) as f32)
                        .build()
                } else {
                    Path::builder()
                        .move_to(axis.offset as f32, tick.position as f32)
                        .line_to((axis.offset - length) as f32, tick.position as f32)
                        .build()
                };
                list.stroke(
                    path,
                    Stroke::Solid(color),
                    LineStyle {
                        width,
                        ..LineStyle::new()
                    },
                    1.0,
                    Z_AXIS,
                );
            }
        }

        // The labels.
        let label_option: Option<LabelOption> =
            axis_option.and_then(|option: &AxisOption| option.axis_label.clone());
        let show_labels: bool = label_option
            .as_ref()
            .map(|label: &LabelOption| label.show)
            .unwrap_or(true);
        let font_size: f64 = label_option
            .as_ref()
            .and_then(|label: &LabelOption| label.font_size)
            .unwrap_or(DEFAULT_AXIS_LABEL_FONT_SIZE);
        let gap: f64 = DEFAULT_AXIS_LABEL_GAP;
        if show_labels {
            let mut style: TextStyle = TextStyle::new();
            style.size = font_size;
            style.color = Color::parse(DEFAULT_TEXT_COLOR).unwrap_or(Color::rgb(110, 112, 121));
            if let Some(label) = &label_option {
                if let Some(color) = label
                    .color
                    .as_ref()
                    .and_then(|color: &String| Color::parse(color))
                {
                    style.color = color;
                }
                if let Some(family) = &label.font_family {
                    style.family = family.clone();
                }
                if let Some(weight) = &label.font_weight {
                    style.weight = weight.clone();
                }
            }
            for (index, tick) in axis.ticks.iter().enumerate() {
                // The labels are drawn every Nth tick, so that they do
                // not overlap each other.
                if index % axis.label_interval.max(1) != 0 {
                    continue;
                }
                let (x, y) = if horizontal {
                    (tick.position, axis.offset + gap + font_size)
                } else {
                    (axis.offset - gap, tick.position)
                };
                let mut label_style: TextStyle = style.clone();
                label_style.align = if horizontal {
                    TextAlign::Center
                } else {
                    TextAlign::Right
                };
                label_style.vertical_align = if horizontal {
                    TextVerticalAlign::Top
                } else {
                    TextVerticalAlign::Middle
                };
                list.text(tick.label.clone(), x, y, label_style, Z_AXIS + 1);
            }
        }

        // The axis' name.
        if let Some(name) = &axis.name
            && !name.is_empty()
        {
            let mut style: TextStyle = TextStyle::new();
            style.size = DEFAULT_AXIS_LABEL_FONT_SIZE;
            style.color = Color::parse(DEFAULT_TEXT_COLOR).unwrap_or(Color::rgb(110, 112, 121));
            style.align = TextAlign::Center;
            style.vertical_align = TextVerticalAlign::Middle;
            let (x, y) = if horizontal {
                (grid.rect.center_x(), axis.offset + gap + font_size * 2.0)
            } else {
                (
                    axis.offset - gap - axis.label_space * 0.5,
                    grid.rect.center_y(),
                )
            };
            list.text(name.clone(), x, y, style, Z_AXIS + 2);
        }
    }

    /// Draws the chart's legend.
    ///
    /// # Arguments
    ///
    /// - `&mut DisplayList` - Mutable reference to a `DisplayList` (mutated in place).
    /// - `&ChartOption` - Shared reference to a `ChartOption`.
    /// - `&ChartLayout` - Shared reference to a `ChartLayout`.
    fn draw_legend(&mut self, list: &mut DisplayList, option: &ChartOption, layout: &ChartLayout) {
        let Some(legend) = &option.legend else {
            return;
        };
        if !legend.show || legend.data.is_empty() {
            return;
        }
        let font_size: f64 = legend
            .text_style
            .as_ref()
            .and_then(|style: &LabelOption| style.font_size)
            .unwrap_or(DEFAULT_LEGEND_FONT_SIZE);
        let item_width: f64 = legend.item_width.unwrap_or(25.0);
        let item_height: f64 = legend.item_height.unwrap_or(14.0);
        let item_gap: f64 = legend.item_gap.unwrap_or(10.0);
        let mut style: TextStyle = TextStyle::new();
        style.size = font_size;
        style.color = Color::parse(DEFAULT_TEXT_COLOR).unwrap_or(Color::rgb(110, 112, 121));
        style.align = TextAlign::Left;
        style.vertical_align = TextVerticalAlign::Middle;
        if let Some(text_style) = &legend.text_style
            && let Some(color) = text_style
                .color
                .as_ref()
                .and_then(|color: &String| Color::parse(color))
        {
            style.color = color;
        }
        let horizontal: bool =
            legend.orient.unwrap_or(Orientation::Horizontal) == Orientation::Horizontal;
        let mut cursor_x: f64 = legend.left.as_ref().map(|_| 10.0).unwrap_or(10.0);
        let mut cursor_y: f64 = layout
            .title_rect
            .as_ref()
            .map(|rect: &BBox| rect.max_y + 10.0)
            .unwrap_or(10.0);
        for (index, name) in legend.data.iter().enumerate() {
            let color: Color = palette_color(option, index);
            let symbol: Path = Path::builder()
                .rect(
                    cursor_x as f32,
                    cursor_y as f32,
                    item_width as f32,
                    item_height as f32,
                )
                .build();
            list.fill(symbol, Fill::Solid(color), 1.0, Z_LEGEND);
            list.text(
                name.clone(),
                cursor_x + item_width + item_gap * 0.5,
                cursor_y + item_height * 0.5,
                style.clone(),
                Z_LEGEND + 1,
            );
            let advance: f64 = item_width + item_gap + name.len() as f64 * font_size * 0.6;
            if horizontal {
                cursor_x += advance;
            } else {
                cursor_y += item_height + item_gap;
                cursor_x = 10.0;
            }
        }
    }
}
