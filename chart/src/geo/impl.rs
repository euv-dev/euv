use super::*;

/// Inherent implementation of [`GeoPoint`].
impl GeoPoint {
    /// Returns a point of the globe.
    ///
    /// # Arguments
    ///
    /// - `f64` - A 64-bit float (`f64`).
    pub fn new(lng: f64, lat: f64) -> Self {
        Self { lng, lat }
    }
}

/// Default construction for [`GeoBounds`].
impl Default for GeoBounds {
    /// Constructs a default [`GeoBounds`] value.
    fn default() -> Self {
        Self {
            min_lng: f64::INFINITY,
            min_lat: f64::INFINITY,
            max_lng: f64::NEG_INFINITY,
            max_lat: f64::NEG_INFINITY,
        }
    }
}

/// Inherent implementation of [`GeoBounds`].
impl GeoBounds {
    /// Returns the width of the box, in degrees.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    pub fn width(&self) -> f64 {
        self.get_max_lng() - self.get_min_lng()
    }

    /// Returns the height of the box, in degrees.
    ///
    /// # Returns
    ///
    /// - `f64` - A 64-bit float.
    pub fn height(&self) -> f64 {
        self.get_max_lat() - self.get_min_lat()
    }

    /// Returns `true` when the box holds no point at all.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn is_empty(&self) -> bool {
        !(self.get_min_lng() <= self.get_max_lng() && self.get_min_lat() <= self.get_max_lat())
    }

    /// Extends the box so that it holds the given point.
    ///
    /// # Arguments
    ///
    /// - `GeoPoint` - A `GeoPoint` parameter.
    pub fn extend(&mut self, point: GeoPoint) {
        self.set_min_lng(self.get_min_lng().min(point.lng));
        self.set_min_lat(self.get_min_lat().min(point.lat));
        self.set_max_lng(self.get_max_lng().max(point.lng));
        self.set_max_lat(self.get_max_lat().max(point.lat));
    }
}

/// Inherent implementation of [`MapRegion`].
impl MapRegion {
    /// Returns the middle of the region, in geographic coordinates.
    ///
    /// The middle is the centre of gravity of the outline, which is
    /// where a label of the region belongs. A region without a usable
    /// outline has no middle.
    ///
    /// # Returns
    ///
    /// - `Option<GeoPoint>` - `Some(...)` on success, `None` otherwise.
    pub fn center(&self) -> Option<GeoPoint> {
        let polygon: &RegionPolygon = self.get_polygons().first()?;
        ring_center(&polygon.outer)
    }
}

/// Inherent implementation of [`MapData`].
impl MapData {
    /// Builds a map out of its regions, and measures the box they cover.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    /// - `Vec<MapRegion>` - A `Vec<MapRegion>` parameter.
    pub fn from_regions(name: &str, regions: Vec<MapRegion>) -> Self {
        let mut bounds: GeoBounds = GeoBounds::default();
        for region in &regions {
            for polygon in &region.polygons {
                for point in &polygon.outer {
                    bounds.extend(*point);
                }
            }
        }
        Self {
            name: name.to_string(),
            regions,
            bounds,
        }
    }

    /// Reads a GeoJSON document into a map.
    ///
    /// The document is the shape GeoJSON has: a collection of features,
    /// each with a geometry and a name. The name is taken from the
    /// `name` property of the feature, because that is where GeoJSON
    /// keeps it.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    /// - `&DataValue` - Shared reference to a `DataValue`.
    ///
    /// # Returns
    ///
    /// - `Result<Self, GeoError>` - Result of the operation; an `Err` variant on failure.
    pub fn from_geojson(name: &str, value: &DataValue) -> Result<Self, GeoError> {
        parse_geojson(name, value)
    }

    /// Returns the region with the given name.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `Option<&MapRegion>` - `Some(...)` on success, `None` otherwise.
    pub fn region(&self, name: &str) -> Option<&MapRegion> {
        self.get_regions()
            .iter()
            .find(|region: &&MapRegion| region.name == name)
    }
}

/// Inherent implementation of [`MapRegistry`].
impl MapRegistry {
    /// Returns an empty registry.
    pub fn new() -> Self {
        Self {
            maps: BTreeMap::new(),
        }
    }

    /// Adds a map to the registry, replacing the one of the same name.
    ///
    /// # Arguments
    ///
    /// - `MapData` - A `MapData` parameter.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A `mut Self` value.
    pub fn register(&mut self, data: MapData) -> &mut Self {
        self.get_mut_maps().insert(data.name.clone(), data);
        self
    }

    /// Reads a GeoJSON document and adds the map it holds.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    /// - `&DataValue` - Shared reference to a `DataValue`.
    ///
    /// # Returns
    ///
    /// - `Result<&mut Self, GeoError>` - Result of the operation; an `Err` variant on failure.
    pub fn register_geojson(
        &mut self,
        name: &str,
        value: &DataValue,
    ) -> Result<&mut Self, GeoError> {
        let data: MapData = MapData::from_geojson(name, value)?;
        self.get_mut_maps().insert(name.to_string(), data);
        Ok(self)
    }

    /// Returns the map with the given name.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `Option<&MapData>` - `Some(...)` on success, `None` otherwise.
    pub fn get(&self, name: &str) -> Option<&MapData> {
        self.get_maps().get(name)
    }

    /// Returns the names of the maps of the registry.
    ///
    /// # Returns
    ///
    /// - `Vec<String>` - A `Vec<String>` value.
    pub fn names(&self) -> Vec<String> {
        self.get_maps().keys().cloned().collect()
    }

    /// Returns the number of maps in the registry.
    ///
    /// # Returns
    ///
    /// - `usize` - A non-negative integer.
    pub fn len(&self) -> usize {
        self.get_maps().len()
    }

    /// Returns `true` when the registry holds no map.
    ///
    /// # Returns
    ///
    /// - `bool` - A boolean.
    pub fn is_empty(&self) -> bool {
        self.get_maps().is_empty()
    }

    /// Removes every map from the registry.
    pub fn clear(&mut self) {
        self.get_mut_maps().clear();
    }
}

/// Inherent implementation of [`GeoLayout`].
impl GeoLayout {
    /// Projects a map into a rectangle of the chart.
    ///
    /// The projection is the one ECharts uses: the longitude and the
    /// latitude are scaled evenly, and the height of the map is
    /// shortened by the aspect scale so that a map of the mid-latitudes
    /// does not look stretched sideways. The map is centred in the
    /// rectangle and takes most of it.
    ///
    /// # Arguments
    ///
    /// - `&MapData` - Shared reference to a `MapData`.
    /// - `BBox` - A `BBox` parameter.
    /// - `f64` - A 64-bit float (`f64`).
    pub fn compute(data: &MapData, rect: BBox, aspect_scale: f64) -> Self {
        let bounds: GeoBounds = data.bounds;
        let geo_width: f64 = bounds.width().max(f64::EPSILON);
        let geo_height: f64 = bounds.height().max(f64::EPSILON);
        // The aspect of the map: its width over its height, shortened
        // by the aspect scale.
        let aspect: f64 = geo_width / geo_height * aspect_scale.max(f64::EPSILON);
        let view: BBox = fit_aspect(rect, aspect);
        // The map is scaled evenly. The direction that binds leaves the
        // map smaller than the view, and the map is centred in it.
        let scale: f64 = (view.width() / geo_width).min(view.height() / geo_height);
        let drawn_width: f64 = geo_width * scale;
        let drawn_height: f64 = geo_height * scale;
        let origin_x: f64 = view.min_x + (view.width() - drawn_width) * 0.5;
        let origin_y: f64 = view.min_y + (view.height() - drawn_height) * 0.5;
        // The longitude grows to the right and the latitude grows
        // upwards, while the screen grows downwards: the vertical axis
        // is flipped.
        let transform: Transform2D = Transform2D {
            a: scale,
            b: 0.0,
            c: 0.0,
            d: -scale,
            e: origin_x - bounds.min_lng * scale,
            f: origin_y + bounds.max_lat * scale,
        };
        let regions: Vec<GeoRegionLayout> = data
            .regions
            .iter()
            .map(|region: &MapRegion| {
                let path: Path = region_path(region, &transform);
                let center: Point = region_center(region, &transform);
                GeoRegionLayout {
                    name: region.name.clone(),
                    path,
                    center,
                }
            })
            .collect();
        Self {
            name: data.name.clone(),
            regions,
            transform,
        }
    }

    /// Returns the region with the given name.
    ///
    /// # Arguments
    ///
    /// - `&str` - Shared reference to a `str`.
    ///
    /// # Returns
    ///
    /// - `Option<&GeoRegionLayout>` - `Some(...)` on success, `None` otherwise.
    pub fn region(&self, name: &str) -> Option<&GeoRegionLayout> {
        self.get_regions()
            .iter()
            .find(|region: &&GeoRegionLayout| region.name == name)
    }
}

/// Implements `impl std::fmt::Display for GeoError`.
impl Display for GeoError {
    /// Formats the [`GeoError`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut std::fmt::Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `std::fmt::Result` - A `std::fmt::Result` value.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeoError::MissingType => write!(formatter, "the map has no `type` field"),
            GeoError::UnsupportedType(kind) => {
                write!(formatter, "the map kind `{}` is not supported", kind)
            }
            GeoError::MissingGeometry => write!(formatter, "the map feature has no geometry"),
            GeoError::MalformedGeometry(reason) => {
                write!(formatter, "the map geometry is malformed: {}", reason)
            }
        }
    }
}

/// Implements `impl std::error::Error for GeoError {}`.
impl Error for GeoError {}
