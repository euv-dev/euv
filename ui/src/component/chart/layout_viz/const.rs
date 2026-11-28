/// Header bar height in logical pixels for nested treemap group boxes.
pub(crate) const LAYOUT_TREEMAP_HEADER_H: f32 = 16.0;

/// Golden angle in radians used for deterministic force-directed graph seeding.
pub(crate) const LAYOUT_GOLDEN_ANGLE: f32 = 2.399_963_1;

/// Built-in stylized North America polygon ring `(lon, lat)`.
pub(crate) const GEO_POLY_NORTH_AMERICA: &[(f32, f32)] = &[
    (-166.0, 66.0),
    (-140.0, 70.0),
    (-96.0, 70.0),
    (-62.0, 56.0),
    (-68.0, 44.0),
    (-80.0, 26.0),
    (-96.0, 18.0),
    (-112.0, 24.0),
    (-124.0, 42.0),
    (-146.0, 60.0),
];

/// Built-in stylized South America polygon ring `(lon, lat)`.
pub(crate) const GEO_POLY_SOUTH_AMERICA: &[(f32, f32)] = &[
    (-80.0, 10.0),
    (-60.0, 8.0),
    (-35.0, -6.0),
    (-40.0, -24.0),
    (-58.0, -40.0),
    (-68.0, -54.0),
    (-75.0, -42.0),
    (-81.0, -6.0),
];

/// Built-in stylized Europe polygon ring `(lon, lat)`.
pub(crate) const GEO_POLY_EUROPE: &[(f32, f32)] = &[
    (-10.0, 38.0),
    (-8.0, 56.0),
    (8.0, 68.0),
    (32.0, 70.0),
    (44.0, 58.0),
    (38.0, 44.0),
    (22.0, 36.0),
    (6.0, 42.0),
];

/// Built-in stylized Africa polygon ring `(lon, lat)`.
pub(crate) const GEO_POLY_AFRICA: &[(f32, f32)] = &[
    (-16.0, 16.0),
    (-6.0, 35.0),
    (12.0, 36.0),
    (34.0, 30.0),
    (50.0, 11.0),
    (40.0, -14.0),
    (26.0, -34.0),
    (14.0, -28.0),
    (8.0, 4.0),
    (-12.0, 6.0),
];

/// Built-in stylized Asia polygon ring `(lon, lat)`.
pub(crate) const GEO_POLY_ASIA: &[(f32, f32)] = &[
    (42.0, 42.0),
    (46.0, 66.0),
    (112.0, 72.0),
    (168.0, 64.0),
    (142.0, 44.0),
    (124.0, 22.0),
    (106.0, 10.0),
    (82.0, 8.0),
    (64.0, 24.0),
    (48.0, 28.0),
];

/// Built-in stylized Oceania / Australia polygon ring `(lon, lat)`.
pub(crate) const GEO_POLY_OCEANIA: &[(f32, f32)] = &[
    (114.0, -22.0),
    (132.0, -12.0),
    (152.0, -20.0),
    (150.0, -37.0),
    (134.0, -35.0),
    (116.0, -34.0),
];

/// Built-in stylized China sub-region polygon ring `(lon, lat)` for World preset.
pub(crate) const GEO_POLY_CHINA: &[(f32, f32)] = &[
    (76.0, 38.0),
    (96.0, 44.0),
    (124.0, 48.0),
    (122.0, 30.0),
    (112.0, 21.0),
    (98.0, 23.0),
    (82.0, 30.0),
];

/// Built-in stylized United States sub-region polygon ring `(lon, lat)` for World preset.
pub(crate) const GEO_POLY_UNITED_STATES: &[(f32, f32)] = &[
    (-124.0, 48.0),
    (-94.0, 48.0),
    (-68.0, 45.0),
    (-80.0, 26.0),
    (-98.0, 27.0),
    (-118.0, 33.0),
];

/// Built-in stylized Brazil sub-region polygon ring `(lon, lat)` for World preset.
pub(crate) const GEO_POLY_BRAZIL: &[(f32, f32)] = &[
    (-72.0, 2.0),
    (-50.0, 2.0),
    (-36.0, -8.0),
    (-42.0, -23.0),
    (-56.0, -30.0),
    (-68.0, -12.0),
];

/// Built-in geographic region names for built-in world/continent presets.
pub(crate) const GEO_NAME_NORTH_AMERICA: &str = "North America";
/// Built-in geographic region name for South America.
pub(crate) const GEO_NAME_SOUTH_AMERICA: &str = "South America";
/// Built-in geographic region name for Europe.
pub(crate) const GEO_NAME_EUROPE: &str = "Europe";
/// Built-in geographic region name for Africa.
pub(crate) const GEO_NAME_AFRICA: &str = "Africa";
/// Built-in geographic region name for Asia.
pub(crate) const GEO_NAME_ASIA: &str = "Asia";
/// Built-in geographic region name for Oceania.
pub(crate) const GEO_NAME_OCEANIA: &str = "Oceania";
/// Built-in geographic region name for China.
pub(crate) const GEO_NAME_CHINA: &str = "China";
/// Built-in geographic region name for United States.
pub(crate) const GEO_NAME_UNITED_STATES: &str = "United States";
/// Built-in geographic region name for Brazil.
pub(crate) const GEO_NAME_BRAZIL: &str = "Brazil";
/// Short code for North America.
pub(crate) const GEO_CODE_NA: &str = "NA";
/// Short code for South America.
pub(crate) const GEO_CODE_SA: &str = "SA";
/// Short code for Europe.
pub(crate) const GEO_CODE_EU: &str = "EU";
/// Short code for Africa.
pub(crate) const GEO_CODE_AF: &str = "AF";
/// Short code for Asia.
pub(crate) const GEO_CODE_AS: &str = "AS";
/// Short code for Oceania.
pub(crate) const GEO_CODE_OC: &str = "OC";
/// Short code for China.
pub(crate) const GEO_CODE_CN: &str = "CN";
/// Short code for United States.
pub(crate) const GEO_CODE_US: &str = "US";
/// Short code for Brazil.
pub(crate) const GEO_CODE_BR: &str = "BR";
