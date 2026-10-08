use super::*;

/// A zero-sized namespace struct providing static math utility methods.
///
/// This struct follows the same pattern as `App`, serving as a namespace for
/// free-standing mathematical functions such as `clamp`, `lerp`, angle conversions,
/// and interpolation helpers.
#[derive(Clone, Copy, Data, Debug, Default, Eq, Hash, New, Ord, PartialEq, PartialOrd)]
pub struct Numeric;

/// A 2D vector with `x` and `y` components represented as `f64`.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq, PartialOrd)]
pub struct Vector2D {
    /// The horizontal component of the vector.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) x: f64,
    /// The vertical component of the vector.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) y: f64,
}

/// An axis-aligned rectangle defined by its top-left corner and dimensions.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq, PartialOrd)]
pub struct Rect {
    /// The x coordinate of the top-left corner.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) x: f64,
    /// The y coordinate of the top-left corner.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) y: f64,
    /// The width of the rectangle.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) width: f64,
    /// The height of the rectangle.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) height: f64,
}

/// A circle defined by its center point and radius.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq, PartialOrd)]
pub struct Circle {
    /// The center point of the circle.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) center: Vector2D,
    /// The radius of the circle.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) radius: f64,
}

/// A 2D transform composed of position, rotation (in radians), and scale.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct Transform2D {
    /// The position offset of the transform.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) position: Vector2D,
    /// The rotation angle in radians.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) rotation: f64,
    /// The scale factors for each axis.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) scale: Vector2D,
}

/// A color represented by red, green, blue, and alpha channels in the range 0.0 to 1.0.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct Color {
    /// The red channel value in the range 0.0 to 1.0.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) red: f64,
    /// The green channel value in the range 0.0 to 1.0.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) green: f64,
    /// The blue channel value in the range 0.0 to 1.0.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) blue: f64,
    /// The alpha (opacity) channel value in the range 0.0 to 1.0.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) alpha: f64,
}

/// A 3D vector with `x`, `y`, and `z` components represented as `f64`.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq, PartialOrd)]
pub struct Vector3D {
    /// The horizontal component of the vector.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) x: f64,
    /// The vertical component of the vector.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) y: f64,
    /// The depth component of the vector.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) z: f64,
}

/// A quaternion representing a 3D rotation with `x`, `y`, `z`, and `w` components.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct Quaternion {
    /// The x component of the quaternion.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) x: f64,
    /// The y component of the quaternion.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) y: f64,
    /// The z component of the quaternion.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) z: f64,
    /// The w (scalar) component of the quaternion.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) w: f64,
}

/// A 4x4 matrix stored in column-major order, used for 3D transformations.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct Matrix4x4 {
    /// The 16 elements of the matrix in column-major order.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) elements: [f64; 16],
}

/// A 3D transform composed of position, rotation (as a quaternion), and scale.
#[derive(Clone, Copy, Data, Debug, New, PartialEq, PartialOrd)]
pub struct Transform3D {
    /// The position offset of the transform.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) position: Vector3D,
    /// The rotation as a quaternion.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) rotation: Quaternion,
    /// The scale factors for each axis.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) scale: Vector3D,
}

/// A 3D axis-aligned bounding box defined by its minimum and maximum corners.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq, PartialOrd)]
pub struct AABB3D {
    /// The minimum corner (smallest x, y, z).
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) min: Vector3D,
    /// The maximum corner (largest x, y, z).
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) max: Vector3D,
}

/// A 3D sphere defined by its center point and radius.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq, PartialOrd)]
pub struct Sphere {
    /// The center point of the sphere.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) center: Vector3D,
    /// The radius of the sphere.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) radius: f64,
}

/// A 3D plane defined by a normal vector and a distance from the origin.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq, PartialOrd)]
pub struct Plane {
    /// The normal vector of the plane (should be normalized).
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) normal: Vector3D,
    /// The signed distance from the origin along the normal.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) distance: f64,
}

/// A 3D ray defined by an origin point and a direction vector.
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq, PartialOrd)]
pub struct Ray3D {
    /// The origin point of the ray.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) origin: Vector3D,
    /// The direction vector of the ray (should be normalized).
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) direction: Vector3D,
}

/// A 2D ray defined by an origin point and a direction vector, used for
/// raycasting against 2D shapes (`Rect`, `Circle`).
#[derive(Clone, Copy, Data, Debug, Default, New, PartialEq, PartialOrd)]
pub struct Ray2D {
    /// The origin point of the ray.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) origin: Vector2D,
    /// The direction vector of the ray (should be normalized).
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) direction: Vector2D,
}
