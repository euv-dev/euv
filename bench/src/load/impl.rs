use super::*;

/// Inherent implementation of [`CoreLoad`].
impl CoreLoad {
    /// Builds stable payloads and one real reactive subscription.
    ///
    /// # Arguments
    ///
    /// - `usize` - Workload multiplier.
    ///
    /// # Returns
    ///
    /// - `Self` - The initialized reactive fixture.
    fn create(scale: usize) -> Self {
        let attributes: Vec<AttributeValue> = (0..CORE_SIGNAL_COUNT * scale)
            .map(|_: usize| AttributeValue::Signal(Signal::create(CORE_SEGMENT.repeat(8))))
            .collect();
        let signal: Signal<u64> = Signal::create(u64::MAX);
        let notifications: Rc<Cell<u64>> = Rc::new(Cell::new(0));
        let callback: Rc<Cell<u64>> = Rc::clone(&notifications);
        signal.subscribe(move || callback.set(callback.get().wrapping_add(1)));
        Self::new(attributes, signal, notifications)
    }

    /// Updates a signal and materializes reactive attributes as in a render pass.
    ///
    /// # Arguments
    ///
    /// - `u64` - Monotonically increasing frame number.
    ///
    /// # Returns
    ///
    /// - `u64` - Checksum of the emitted snapshots and delivered notification.
    fn tick(&mut self, frame: u64) -> u64 {
        self.get_signal().set(frame);
        let classes: AttributeValue = AttributeValue::merge_class(self.get_attributes());
        let styles: AttributeValue = AttributeValue::merge_style(self.get_attributes());
        let mut checksum: u64 = self.get_notifications().get();
        for value in [classes, styles] {
            if let AttributeValue::Text(text) = value {
                checksum = checksum.wrapping_add(text.len() as u64);
                black_box(text);
            }
        }
        checksum
    }
}

/// Inherent implementation of [`EngineLoad`].
impl EngineLoad {
    /// Creates a deterministic particle population and a non-overlapping body grid.
    ///
    /// # Arguments
    ///
    /// - `usize` - Workload multiplier.
    ///
    /// # Returns
    ///
    /// - `Self` - The initialized simulation and reusable draw buffer.
    fn create(scale: usize) -> Self {
        let mut config: ParticleConfig = ParticleConfig::default();
        config.set_emission_rate(0.0);
        config.set_lifetime_min(1_000_000.0);
        config.set_lifetime_max(1_000_000.0);
        config.set_max_particles(ENGINE_PARTICLE_COUNT * scale);
        let mut emitter: ParticleEmitter = ParticleEmitter::create(Vector2D::zero(), config);
        emitter.burst(ENGINE_PARTICLE_COUNT * scale);
        let mut world: PhysicsWorld2D =
            PhysicsWorld2D::with_config(PhysicsConfig::new(Vector2D::zero(), 0.0, 0.0));
        for index in 0..ENGINE_BODY_COUNT * scale {
            let mut body: RigidBody2D = RigidBody2D::new_dynamic(
                index as u64,
                Vector2D::new((index % 32) as f64 * 16.0, (index / 32) as f64 * 16.0),
            );
            body.update_collider(BodyCollider::Aabb(AabbCollider::new(Rect::new(
                -1.0, -1.0, 2.0, 2.0,
            ))));
            world.add_body(body);
        }
        Self::new(emitter, DrawList::default(), world)
    }

    /// Simulates physics and particles, then records a complete particle frame.
    ///
    /// # Returns
    ///
    /// - `u64` - Checksum of live objects and generated draw commands.
    fn tick(&mut self) -> u64 {
        self.get_mut_world().step(1.0 / 60.0);
        self.get_mut_emitter().update(1.0 / 60.0);
        self.get_mut_emitter().burst(0);
        let count: usize = self.get_emitter().alive_count();
        let mut draw: DrawList = take(self.get_mut_draw());
        draw.clear();
        self.get_emitter().render(&mut draw);
        black_box(&draw);
        let checksum: u64 =
            count as u64 + draw.len() as u64 + self.get_world().get_bodies().len() as u64;
        self.set_draw(draw);
        checksum
    }
}

/// Inherent implementation of [`ChartLoad`].
impl ChartLoad {
    /// Creates scalar, tuple and object series to exercise every extent path.
    ///
    /// # Arguments
    ///
    /// - `usize` - Workload multiplier.
    ///
    /// # Returns
    ///
    /// - `Self` - The initialized chart fixture.
    fn create(scale: usize) -> Self {
        let count: usize = CHART_POINT_COUNT * scale;
        let values: Vec<DataValue> = (0..count)
            .map(|index: usize| DataValue::Number((index % 100) as f64))
            .collect();
        let tuples: Vec<DataValue> = (0..count)
            .map(|index: usize| {
                DataValue::List(vec![
                    DataValue::Number(index as f64),
                    DataValue::Number((index % 70) as f64),
                ])
            })
            .collect();
        let objects: Vec<DataValue> = (0..count)
            .map(|index: usize| {
                DataValue::Object(vec![(
                    DATA_VALUE_KEY.to_string(),
                    DataValue::Number((index % 90) as f64),
                )])
            })
            .collect();
        let option: ChartOption = ChartOption::new()
            .x_axis(AxisOption::category())
            .y_axis(AxisOption::value())
            .series(SeriesOption::line().data(values))
            .series(SeriesOption::line().data(tuples))
            .series(SeriesOption::line().data(objects));
        Self::new(
            option,
            TextMeasurer::approximate(),
            SeriesRenderer::new(),
            DisplayList::new(),
            count * 3,
        )
    }

    /// Updates data, recomputes layout and emits actual chart draw instructions.
    ///
    /// # Arguments
    ///
    /// - `u64` - Monotonically increasing frame number.
    ///
    /// # Returns
    ///
    /// - `u64` - Checksum of the generated chart layout and display list.
    fn tick(&mut self, frame: u64) -> u64 {
        let index: usize = frame as usize % self.get_option().get_series()[0].get_data().len();
        self.get_mut_option().get_mut_series()[0].get_mut_data()[index] =
            DataValue::Number((frame % 100) as f64);
        let layout: ChartLayout =
            ChartLayout::compute(self.get_option(), 1280.0, 720.0, self.get_measurer());
        let mut list: DisplayList = replace(self.get_mut_list(), DisplayList::new());
        let mut renderer: SeriesRenderer = replace(self.get_mut_renderer(), SeriesRenderer::new());
        list.clear();
        renderer.draw(&mut list, self.get_option(), &layout);
        let bounds: BBox = list.bounds();
        let checksum: u64 = (list.len() as u64)
            .wrapping_add(bounds.width().to_bits())
            .wrapping_add(self.get_point_count() as u64);
        black_box(&layout);
        black_box(&list);
        self.set_renderer(renderer);
        self.set_list(list);
        checksum
    }
}

/// Inherent implementation of [`Workload`].
impl Workload {
    /// Enables precisely the subsystems required by the selected suite.
    ///
    /// # Arguments
    ///
    /// - `Suite` - Selected subsystem or combined frame.
    /// - `usize` - Workload multiplier.
    ///
    /// # Returns
    ///
    /// - `Self` - Initialized fixture.
    pub(crate) fn create(suite: Suite, scale: usize) -> Self {
        Self::new(
            if matches!(suite, Suite::Core | Suite::Mixed) {
                Some(CoreLoad::create(scale))
            } else {
                None
            },
            if matches!(suite, Suite::Engine | Suite::Mixed) {
                Some(EngineLoad::create(scale))
            } else {
                None
            },
            if matches!(suite, Suite::Chart | Suite::Mixed) {
                Some(ChartLoad::create(scale))
            } else {
                None
            },
        )
    }

    /// Executes the enabled subsystems within one shared frame.
    ///
    /// # Arguments
    ///
    /// - `u64` - Monotonically increasing frame number.
    ///
    /// # Returns
    ///
    /// - `Work` - Actual subsystem invocation counts and output checksum.
    pub(crate) fn tick(&mut self, frame: u64) -> Work {
        let mut work: Work = Work::default();
        if let Some(core) = self.get_mut_core().as_mut() {
            work.set_core_ticks(1);
            let checksum: u64 = work.get_checksum().wrapping_add(core.tick(frame));
            work.set_checksum(checksum);
        }
        if let Some(engine) = self.get_mut_engine().as_mut() {
            work.set_engine_ticks(1);
            let checksum: u64 = work.get_checksum().wrapping_add(engine.tick());
            work.set_checksum(checksum);
        }
        if let Some(chart) = self.get_mut_chart().as_mut() {
            work.set_chart_ticks(1);
            let checksum: u64 = work.get_checksum().wrapping_add(chart.tick(frame));
            work.set_checksum(checksum);
        }
        black_box(work)
    }
}

/// Inherent implementation of [`Work`].
impl Work {
    /// Accumulates actual work without allocating inside a measured frame.
    ///
    /// # Arguments
    ///
    /// - `Self` - One frame's subsystem work.
    pub(crate) fn add(&mut self, work: Self) {
        *self.get_mut_core_ticks() += work.get_core_ticks();
        *self.get_mut_engine_ticks() += work.get_engine_ticks();
        *self.get_mut_chart_ticks() += work.get_chart_ticks();
        let checksum: u64 = self.get_checksum().wrapping_add(work.get_checksum());
        self.set_checksum(checksum);
    }
}
