use super::*;

/// Reads a GeoJSON document into a map.
///
/// # Arguments
///
/// - `&str` - Shared reference to a `str`.
/// - `&DataValue` - Shared reference to a `DataValue`.
///
/// # Returns
///
/// - `Result<MapData, GeoError>` - Result of the operation; an `Err` variant on failure.
pub(crate) fn parse_geojson(name: &str, value: &DataValue) -> Result<MapData, GeoError> {
    let kind: String = value
        .get(GEO_FIELD_TYPE)
        .and_then(|kind: &DataValue| kind.as_text())
        .unwrap_or_default()
        .to_string();
    match kind.as_str() {
        GEO_KIND_FEATURE_COLLECTION => {
            let features: &[DataValue] = value
                .get(GEO_FIELD_FEATURES)
                .and_then(|features: &DataValue| features.as_list())
                .unwrap_or(&[]);
            let mut regions: Vec<MapRegion> = Vec::with_capacity(features.len());
            for feature in features {
                regions.push(parse_feature(feature)?);
            }
            Ok(MapData::from_regions(name, regions))
        }
        GEO_KIND_FEATURE => {
            let region: MapRegion = parse_feature(value)?;
            Ok(MapData::from_regions(name, vec![region]))
        }
        "" => Err(GeoError::MissingType),
        other => Err(GeoError::UnsupportedType(other.to_string())),
    }
}

/// Reads one feature of a GeoJSON document.
///
/// # Arguments
///
/// - `&DataValue` - Shared reference to a `DataValue`.
///
/// # Returns
///
/// - `Result<MapRegion, GeoError>` - Result of the operation; an `Err` variant on failure.
pub(crate) fn parse_feature(value: &DataValue) -> Result<MapRegion, GeoError> {
    let name: String = feature_name(value);
    let geometry: &DataValue = value
        .get(GEO_FIELD_GEOMETRY)
        .ok_or(GeoError::MissingGeometry)?;
    let polygons: Vec<RegionPolygon> = parse_geometry(geometry)?;
    Ok(MapRegion { name, polygons })
}

/// Returns the name of a feature.
///
/// The name of a GeoJSON feature is the `name` of its properties. The
/// other places are fallbacks, for the documents that keep the name
/// somewhere else.
///
/// # Arguments
///
/// - `&DataValue` - Shared reference to a `DataValue`.
///
/// # Returns
///
/// - `String` - A `String` value.
pub(crate) fn feature_name(value: &DataValue) -> String {
    if let Some(properties) = value.get(GEO_FIELD_PROPERTIES)
        && let Some(name) = properties
            .get(GEO_FIELD_NAME)
            .and_then(|name: &DataValue| name.as_text())
        && !name.is_empty()
    {
        return name.to_string();
    }
    for field in [GEO_FIELD_NAME, GEO_FIELD_NAME_UPPER, GEO_FIELD_ID] {
        if let Some(name) = value.get(field).and_then(|name: &DataValue| name.as_text())
            && !name.is_empty()
        {
            return name.to_string();
        }
    }
    String::new()
}

/// Reads the geometry of a feature.
///
/// # Arguments
///
/// - `&DataValue` - Shared reference to a `DataValue`.
///
/// # Returns
///
/// - `Result<Vec<RegionPolygon>, GeoError>` - Result of the operation; an `Err` variant on failure.
pub(crate) fn parse_geometry(value: &DataValue) -> Result<Vec<RegionPolygon>, GeoError> {
    let kind: String = value
        .get(GEO_FIELD_TYPE)
        .and_then(|kind: &DataValue| kind.as_text())
        .unwrap_or_default()
        .to_string();
    let coordinates: &DataValue = value
        .get(GEO_FIELD_COORDINATES)
        .ok_or_else(|| GeoError::MalformedGeometry(GEO_ERROR_NO_COORDINATES.to_string()))?;
    match kind.as_str() {
        GEO_KIND_POLYGON => Ok(vec![parse_polygon(coordinates)?]),
        GEO_KIND_MULTI_POLYGON => {
            let list: &[DataValue] = coordinates.as_list().ok_or_else(|| {
                GeoError::MalformedGeometry(GEO_ERROR_MULTI_POLYGON_NOT_A_LIST.to_string())
            })?;
            let mut polygons: Vec<RegionPolygon> = Vec::with_capacity(list.len());
            for item in list {
                polygons.push(parse_polygon(item)?);
            }
            Ok(polygons)
        }
        GEO_KIND_POINT
        | GEO_KIND_MULTI_POINT
        | GEO_KIND_LINE_STRING
        | GEO_KIND_MULTI_LINE_STRING => Err(GeoError::UnsupportedType(kind)),
        "" => Err(GeoError::MissingType),
        other => Err(GeoError::UnsupportedType(other.to_string())),
    }
}

/// Reads one outline of a region: an outer ring and its holes.
///
/// # Arguments
///
/// - `&DataValue` - Shared reference to a `DataValue`.
///
/// # Returns
///
/// - `Result<RegionPolygon, GeoError>` - Result of the operation; an `Err` variant on failure.
pub(crate) fn parse_polygon(value: &DataValue) -> Result<RegionPolygon, GeoError> {
    let rings: &[DataValue] = value
        .as_list()
        .ok_or_else(|| GeoError::MalformedGeometry(GEO_ERROR_POLYGON_NOT_A_LIST.to_string()))?;
    let mut outer: Vec<GeoPoint> = Vec::new();
    let mut holes: Vec<Vec<GeoPoint>> = Vec::new();
    for (index, ring) in rings.iter().enumerate() {
        let points: Vec<GeoPoint> = parse_ring(ring)?;
        // A ring with less than three points encloses nothing.
        if points.len() < 3 {
            continue;
        }
        if index == 0 {
            outer = points;
        } else {
            holes.push(points);
        }
    }
    if outer.is_empty() {
        return Err(GeoError::MalformedGeometry(
            GEO_ERROR_NO_OUTER_RING.to_string(),
        ));
    }
    Ok(RegionPolygon { outer, holes })
}

/// Reads one ring of an outline.
///
/// # Arguments
///
/// - `&DataValue` - Shared reference to a `DataValue`.
///
/// # Returns
///
/// - `Result<Vec<GeoPoint>, GeoError>` - Result of the operation; an `Err` variant on failure.
pub(crate) fn parse_ring(value: &DataValue) -> Result<Vec<GeoPoint>, GeoError> {
    let list: &[DataValue] = value
        .as_list()
        .ok_or_else(|| GeoError::MalformedGeometry(GEO_ERROR_RING_NOT_A_LIST.to_string()))?;
    let mut points: Vec<GeoPoint> = Vec::with_capacity(list.len());
    for item in list {
        let pair: &[DataValue] = item
            .as_list()
            .ok_or_else(|| GeoError::MalformedGeometry(GEO_ERROR_POINT_NOT_A_PAIR.to_string()))?;
        let lng: f64 = pair
            .first()
            .and_then(|value: &DataValue| value.to_number())
            .ok_or_else(|| GeoError::MalformedGeometry(GEO_ERROR_POINT_NOT_A_PAIR.to_string()))?;
        let lat: f64 = pair
            .get(1)
            .and_then(|value: &DataValue| value.to_number())
            .ok_or_else(|| GeoError::MalformedGeometry(GEO_ERROR_POINT_NOT_A_PAIR.to_string()))?;
        points.push(GeoPoint { lng, lat });
    }
    Ok(points)
}

/// Returns the middle of a ring.
///
/// The middle is the centre of gravity of the ring, which is where the
/// label of a region belongs. A ring that encloses no area has no
/// centre of gravity, and the plain average of its points is used
/// instead.
///
/// # Arguments
///
/// - `&[GeoPoint]` - Shared reference to a `[GeoPoint]`.
///
/// # Returns
///
/// - `Option<GeoPoint>` - `Some(...)` on success, `None` otherwise.
pub(crate) fn ring_center(ring: &[GeoPoint]) -> Option<GeoPoint> {
    if ring.is_empty() {
        return None;
    }
    let mut area: f64 = 0.0;
    let mut lng_sum: f64 = 0.0;
    let mut lat_sum: f64 = 0.0;
    for index in 0..ring.len() {
        let current: GeoPoint = ring[index];
        let next: GeoPoint = ring[(index + 1) % ring.len()];
        let cross: f64 = current.lng * next.lat - next.lng * current.lat;
        area += cross;
        lng_sum += (current.lng + next.lng) * cross;
        lat_sum += (current.lat + next.lat) * cross;
    }
    if area.abs() > f64::EPSILON {
        return Some(GeoPoint {
            lng: lng_sum / (3.0 * area),
            lat: lat_sum / (3.0 * area),
        });
    }
    let count: f64 = ring.len() as f64;
    Some(GeoPoint {
        lng: ring.iter().map(|point: &GeoPoint| point.lng).sum::<f64>() / count,
        lat: ring.iter().map(|point: &GeoPoint| point.lat).sum::<f64>() / count,
    })
}

/// Returns the largest rectangle of a given aspect that fits into a
/// rectangle.
///
/// The rectangle is centred in the one it is fitted into, and takes
/// most of it: a map that touches the edges of the chart has no room
/// for the labels around it.
///
/// # Arguments
///
/// - `BBox` - A `BBox` parameter.
/// - `f64` - A 64-bit float (`f64`).
///
/// # Returns
///
/// - `BBox` - A `BBox` value.
pub(crate) fn fit_aspect(rect: BBox, aspect: f64) -> BBox {
    let width: f64 = rect.width();
    let height: f64 = rect.height();
    if aspect <= 0.0 || width <= 0.0 || height <= 0.0 {
        return rect;
    }
    let target_width: f64 = width * DEFAULT_MAP_FILL;
    let target_height: f64 = height * DEFAULT_MAP_FILL;
    let (map_width, map_height) = if aspect >= target_width / target_height {
        (target_width, target_width / aspect)
    } else {
        (target_height * aspect, target_height)
    };
    BBox::from_corners(
        rect.min_x + (width - map_width) * 0.5,
        rect.min_y + (height - map_height) * 0.5,
        rect.min_x + (width + map_width) * 0.5,
        rect.min_y + (height + map_height) * 0.5,
    )
}

/// Builds the outline of a region, in pixel space.
///
/// The holes of the region are drawn as part of the same path, and the
/// path is filled with the even-odd rule: a point inside two rings is
/// outside the region, and a point inside one ring is inside it.
///
/// # Arguments
///
/// - `&MapRegion` - Shared reference to a `MapRegion`.
/// - `&Transform2D` - Shared reference to a `Transform2D`.
///
/// # Returns
///
/// - `Path` - A `Path` value.
pub(crate) fn region_path(region: &MapRegion, transform: &Transform2D) -> Path {
    let mut builder: PathBuilder = Path::builder().fill_rule(FillRule::EvenOdd);
    for polygon in &region.polygons {
        for ring in once(&polygon.outer).chain(polygon.holes.iter()) {
            if ring.len() < 3 {
                continue;
            }
            let (x, y) = transform.apply(ring[0].lng, ring[0].lat);
            builder = builder.move_to(x as f32, y as f32);
            for point in &ring[1..] {
                let (x, y) = transform.apply(point.lng, point.lat);
                builder = builder.line_to(x as f32, y as f32);
            }
            builder = builder.close();
        }
    }
    builder.build()
}

/// Returns the middle of a region, in pixel space.
///
/// # Arguments
///
/// - `&MapRegion` - Shared reference to a `MapRegion`.
/// - `&Transform2D` - Shared reference to a `Transform2D`.
///
/// # Returns
///
/// - `Point` - A `Point` value.
pub(crate) fn region_center(region: &MapRegion, transform: &Transform2D) -> Point {
    let Some(center) = region.center() else {
        return Point::new(0.0, 0.0);
    };
    let (x, y) = transform.apply(center.lng, center.lat);
    Point::new(x as f32, y as f32)
}
