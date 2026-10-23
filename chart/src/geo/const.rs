/// The factor that keeps a map from looking stretched.
///
/// ECharts shortens the height of a map by the same factor, and so
/// does this crate: the distance of a degree of longitude shrinks
/// towards the poles, and a map drawn without the correction looks
/// stretched sideways.
pub(crate) const DEFAULT_ASPECT_SCALE: f64 = 0.75;

/// The share of the plot area a map takes up.
///
/// The rest of the area stays empty, so that a map never touches the
/// edges of the chart.
pub(crate) const DEFAULT_MAP_FILL: f64 = 0.8;

/// The GeoJSON field that names the kind of a document or a geometry.
pub(crate) const GEO_FIELD_TYPE: &str = "type";

/// The GeoJSON field that carries the properties of a feature.
pub(crate) const GEO_FIELD_PROPERTIES: &str = "properties";

/// The GeoJSON field that carries the name of a feature.
pub(crate) const GEO_FIELD_NAME: &str = "name";

/// The GeoJSON field that carries the uppercase name of a feature.
pub(crate) const GEO_FIELD_NAME_UPPER: &str = "NAME";

/// The GeoJSON field that carries the identifier of a feature.
pub(crate) const GEO_FIELD_ID: &str = "id";

/// The GeoJSON field that carries the geometry of a feature.
pub(crate) const GEO_FIELD_GEOMETRY: &str = "geometry";

/// The GeoJSON field that carries the coordinates of a geometry.
pub(crate) const GEO_FIELD_COORDINATES: &str = "coordinates";

/// The GeoJSON field that carries the features of a collection.
pub(crate) const GEO_FIELD_FEATURES: &str = "features";

/// The GeoJSON kind of a collection of features.
pub(crate) const GEO_KIND_FEATURE_COLLECTION: &str = "FeatureCollection";

/// The GeoJSON kind of a single feature.
pub(crate) const GEO_KIND_FEATURE: &str = "Feature";

/// The GeoJSON kind of a polygon.
pub(crate) const GEO_KIND_POLYGON: &str = "Polygon";

/// The GeoJSON kind of a list of polygons.
pub(crate) const GEO_KIND_MULTI_POLYGON: &str = "MultiPolygon";

/// The GeoJSON kind of a single point.
pub(crate) const GEO_KIND_POINT: &str = "Point";

/// The GeoJSON kind of a list of points.
pub(crate) const GEO_KIND_MULTI_POINT: &str = "MultiPoint";

/// The GeoJSON kind of a line of points.
pub(crate) const GEO_KIND_LINE_STRING: &str = "LineString";

/// The GeoJSON kind of a list of lines of points.
pub(crate) const GEO_KIND_MULTI_LINE_STRING: &str = "MultiLineString";

/// The message of a geometry that carries no coordinates.
pub(crate) const GEO_ERROR_NO_COORDINATES: &str = "the geometry has no coordinates";

/// The message of a multi polygon that is not a list.
pub(crate) const GEO_ERROR_MULTI_POLYGON_NOT_A_LIST: &str = "a multi polygon has to be a list";

/// The message of a polygon that is not a list of rings.
pub(crate) const GEO_ERROR_POLYGON_NOT_A_LIST: &str = "a polygon has to be a list of rings";

/// The message of a ring that is not a list of points.
pub(crate) const GEO_ERROR_RING_NOT_A_LIST: &str = "a ring has to be a list of points";

/// The message of a point that is not a pair of numbers.
pub(crate) const GEO_ERROR_POINT_NOT_A_PAIR: &str = "a point has to be a pair of numbers";

/// The message of a polygon that has no outer ring.
pub(crate) const GEO_ERROR_NO_OUTER_RING: &str = "a polygon needs an outer ring";
