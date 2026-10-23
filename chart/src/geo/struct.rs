use super::*;

/// A point on the globe.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeoPoint {
    /// The longitude, in degrees.
    pub lng: f64,
    /// The latitude, in degrees.
    pub lat: f64,
}

/// The box a map covers.
#[derive(Clone, Copy, Debug, PartialEq, lombok_macros::Data)]
pub struct GeoBounds {
    /// The westernmost longitude.
    #[get(type(copy))]
    pub min_lng: f64,
    /// The southernmost latitude.
    #[get(type(copy))]
    pub min_lat: f64,
    /// The easternmost longitude.
    #[get(type(copy))]
    pub max_lng: f64,
    /// The northernmost latitude.
    #[get(type(copy))]
    pub max_lat: f64,
}

/// One outline of a region: an outer ring, and the holes in it.
///
/// A region is made of one or more outlines: a country with an
/// enclave, or an archipelago, has more than one.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RegionPolygon {
    /// The outer ring of the outline.
    pub outer: Vec<GeoPoint>,
    /// The holes of the outline — the lakes and the enclaves that are
    /// not part of the region.
    pub holes: Vec<Vec<GeoPoint>>,
}

/// One region of a map.
///
/// The name of the region is what the series' data is matched
/// against: a data item whose name is the name of a region gives that
/// region its value.
#[derive(Clone, Debug, Default, PartialEq, lombok_macros::Data)]
pub struct MapRegion {
    /// The name of the region.
    pub name: String,
    /// The outlines of the region.
    pub polygons: Vec<RegionPolygon>,
}

/// One map: the regions it is made of, and the box they cover.
#[derive(Clone, Debug, Default, PartialEq, lombok_macros::Data)]
pub struct MapData {
    /// The name the map is registered under.
    pub name: String,
    /// The regions of the map.
    pub regions: Vec<MapRegion>,
    /// The box the regions cover.
    pub bounds: GeoBounds,
}

/// The maps a chart can draw.
///
/// The registry is what turns a map series into a picture: the series
/// names the map it wants, and the registry holds the geometry. The
/// geometry is kept out of the option on purpose — a map of a country
/// is megabytes of coordinates, and it is the same for every chart
/// that draws it.
#[derive(Clone, Debug, Default, PartialEq, lombok_macros::Data)]
pub struct MapRegistry {
    /// The maps of the registry, by name.
    pub maps: BTreeMap<String, MapData>,
}

/// One region of a map, in pixel space.
#[derive(Clone, Debug, PartialEq, lombok_macros::Data)]
pub struct GeoRegionLayout {
    /// The name of the region.
    pub name: String,
    /// The outline of the region, projected into the chart.
    pub path: Path,
    /// The middle of the region, where its label goes.
    pub center: Point,
}

/// A map, in pixel space.
#[derive(Clone, Debug, PartialEq, lombok_macros::Data)]
pub struct GeoLayout {
    /// The name of the map.
    pub name: String,
    /// The regions of the map, projected into the chart.
    pub regions: Vec<GeoRegionLayout>,
    /// The transform that takes a longitude and a latitude to a point of
    /// the chart.
    pub transform: Transform2D,
}
