use super::*;

/// Inherent implementation of [`DisplayItem`].
impl DisplayItem {
    /// Returns the kind of the command.
    ///
    /// # Returns
    ///
    /// - `DrawKind` - A `DrawKind` value.
    pub fn kind(&self) -> DrawKind {
        match self.get_cmd() {
            DrawCmd::Fill { .. } => DrawKind::Fill,
            DrawCmd::Stroke { .. } => DrawKind::Stroke,
            DrawCmd::Text { .. } => DrawKind::Text,
            DrawCmd::Image { .. } => DrawKind::Image,
            DrawCmd::Clip { .. } => DrawKind::Clip,
            DrawCmd::PopClip => DrawKind::PopClip,
            DrawCmd::Strip { .. } => DrawKind::Strip,
        }
    }
}

/// Inherent implementation of [`DisplayList`].
impl DisplayList {
    /// Returns an empty display list.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            bounds: BBox::empty(),
        }
    }

    /// Returns the items, in paint order.
    ///
    /// # Returns
    ///
    /// - `&[DisplayItem]` - A `[DisplayItem]` value.
    pub fn items(&self) -> &[DisplayItem] {
        self.get_items()
    }

    /// Returns the number of items in the list.
    ///
    /// # Returns
    ///
    /// - `usize` - A non-negative integer.
    pub fn len(&self) -> usize {
        self.get_items().len()
    }

    /// Returns `true` when the list holds no items.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn is_empty(&self) -> bool {
        self.get_items().is_empty()
    }

    /// Returns the bounds of everything in the list.
    ///
    /// # Returns
    ///
    /// - `BBox` - A `BBox` value.
    pub fn bounds(&self) -> BBox {
        *self.get_bounds()
    }

    /// Removes every item from the list, keeping the allocated capacity.
    pub fn clear(&mut self) {
        self.get_mut_items().clear();
        self.set_bounds(BBox::empty());
    }

    /// Adds a filled path.
    ///
    /// # Arguments
    ///
    /// - `Path` - A `Path` parameter.
    /// - `Fill` - A `Fill` parameter.
    /// - `f64` - A 64-bit float (`f64`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    pub fn fill(&mut self, path: Path, paint: Fill, opacity: f64, z: i32) {
        self.get_mut_bounds().extend_box(&path.bounds());
        self.get_mut_items().push(DisplayItem {
            cmd: DrawCmd::Fill {
                path,
                paint,
                opacity,
            },
            z,
            element_id: 0,
            series_index: None,
            data_index: None,
        });
    }

    /// Adds a stroked path.
    ///
    /// # Arguments
    ///
    /// - `Path` - A `Path` parameter.
    /// - `Stroke` - A `Stroke` parameter.
    /// - `LineStyle` - A `LineStyle` parameter.
    /// - `f64` - A 64-bit float (`f64`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    pub fn stroke(&mut self, path: Path, paint: Stroke, style: LineStyle, opacity: f64, z: i32) {
        self.get_mut_bounds().extend_box(&path.bounds());
        self.get_mut_items().push(DisplayItem {
            cmd: DrawCmd::Stroke {
                path,
                paint,
                style,
                opacity,
            },
            z,
            element_id: 0,
            series_index: None,
            data_index: None,
        });
    }

    /// Adds a text run.
    ///
    /// # Arguments
    ///
    /// - `String` - A `String` parameter.
    /// - `f64` - A 64-bit float (`f64`).
    /// - `TextStyle` - A `TextStyle` parameter.
    /// - `i32` - A 32-bit signed integer (`i32`).
    pub fn text(&mut self, text: String, x: f64, y: f64, style: TextStyle, z: i32) {
        self.get_mut_items().push(DisplayItem {
            cmd: DrawCmd::Text { text, x, y, style },
            z,
            element_id: 0,
            series_index: None,
            data_index: None,
        });
    }

    /// Adds an image.
    ///
    /// # Arguments
    ///
    /// - `HtmlImageElement` - A `HtmlImageElement` parameter.
    /// - `f64` - A 64-bit float (`f64`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    pub fn image(
        &mut self,
        image: HtmlImageElement,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        z: i32,
    ) {
        self.get_mut_items().push(DisplayItem {
            cmd: DrawCmd::Image {
                image,
                x,
                y,
                width,
                height,
            },
            z,
            element_id: 0,
            series_index: None,
            data_index: None,
        });
    }

    /// Adds a clip instruction.
    ///
    /// # Arguments
    ///
    /// - `Path` - A `Path` parameter.
    /// - `i32` - A 32-bit signed integer (`i32`).
    pub fn clip(&mut self, path: Path, z: i32) {
        self.get_mut_bounds().extend_box(&path.bounds());
        self.get_mut_items().push(DisplayItem {
            cmd: DrawCmd::Clip { path },
            z,
            element_id: 0,
            series_index: None,
            data_index: None,
        });
    }

    /// Adds a clip-restore instruction.
    ///
    /// # Arguments
    ///
    /// - `i32` - A 32-bit signed integer (`i32`).
    pub fn pop_clip(&mut self, z: i32) {
        self.get_mut_items().push(DisplayItem {
            cmd: DrawCmd::PopClip,
            z,
            element_id: 0,
            series_index: None,
            data_index: None,
        });
    }

    /// Adds a filled triangle strip.
    ///
    /// # Arguments
    ///
    /// - `Vec<Point>` - A `Vec<Point>` parameter.
    /// - `f32` - A 32-bit float (`f32`).
    /// - `Fill` - A `Fill` parameter.
    /// - `f64` - A 64-bit float (`f64`).
    /// - `i32` - A 32-bit signed integer (`i32`).
    pub fn strip(&mut self, points: Vec<Point>, baseline: f32, paint: Fill, opacity: f64, z: i32) {
        for point in &points {
            self.get_mut_bounds()
                .extend_point(point.x as f64, point.y as f64);
        }
        self.get_mut_bounds().extend_point(0.0, baseline as f64);
        self.get_mut_items().push(DisplayItem {
            cmd: DrawCmd::Strip {
                points,
                baseline,
                paint,
                opacity,
            },
            z,
            element_id: 0,
            series_index: None,
            data_index: None,
        });
    }

    /// Marks the item at the given index as belonging to a series and a
    /// data point.
    ///
    /// # Arguments
    ///
    /// - `usize` - A non-negative integer (`usize`).
    /// - `u32` - A 32-bit unsigned integer (`u32`).
    pub fn mark(&mut self, index: usize, element_id: u32, series_index: usize, data_index: usize) {
        if let Some(item) = self.get_mut_items().get_mut(index) {
            item.set_element_id(element_id);
            item.set_series_index(Some(series_index));
            item.set_data_index(Some(data_index));
        }
    }
}
