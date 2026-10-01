use super::*;

/// A trait for types that support linear interpolation between two values.
pub trait Interpolable {
    /// Performs linear interpolation between `self` and `other` by the factor `t`.
    ///
    /// # Arguments
    ///
    /// - `Self` - The target value to interpolate towards.
    /// - `f64` - The interpolation factor, typically in the range 0.0 to 1.0.
    ///
    /// # Returns
    ///
    /// - `Self` - The interpolated result.
    fn lerp(&self, other: Self, factor: f64) -> Self;
}

/// A trait abstracting the operations common to `Vector2D` and `Vector3D`.
///
/// Provides a single abstraction surface so that dimension-agnostic algorithms
/// (e.g. `integrate_linear` in physics) can be written once. Method signatures
/// match the inherent implementations on both vectors: taking `&self` and
/// returning new owned values where the inherent methods do, taking `self` by
/// value where they already do.
///
/// The arithmetic operator supertraits (`Add`, `Sub`, `Mul<f64>`, `Neg`, and
/// the `*Assign` variants) are required so generic code can use natural
/// `a + b`, `v * 2.0`, `-v`, `v += other`, `v *= scalar` syntax without
/// importing additional bounds.
///
/// `Vector2D` adds 2D-specific operations (`perp`, `cross -> f64`,
/// `from_angle`) and `Vector3D` adds 3D-specific ones (`cross -> Vector3D`).
/// These are intentionally **not** part of the trait — they have different
/// return types or only exist in one dimension.
pub trait Vector:
    Copy
    + Clone
    + Default
    + Debug
    + PartialEq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<f64, Output = Self>
    + Neg<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign<f64>
{
    /// Returns the additive identity (`Self::default()`).
    ///
    /// # Returns
    ///
    /// - `Self` - The zero vector for this dimension.
    fn zero() -> Self;

    /// Computes the dot product with another vector.
    ///
    /// # Arguments
    ///
    /// - `Self` - The other vector.
    ///
    /// # Returns
    ///
    /// - `f64` - The dot product.
    fn dot(&self, other: Self) -> f64;

    /// Returns the magnitude (length) of the vector.
    ///
    /// # Returns
    ///
    /// - `f64` - The magnitude.
    fn magnitude(&self) -> f64;

    /// Returns the squared magnitude of the vector.
    ///
    /// Faster than `magnitude` for comparison-only use cases because it
    /// avoids a square root.
    ///
    /// # Returns
    ///
    /// - `f64` - The squared magnitude.
    fn magnitude_squared(&self) -> f64;

    /// Returns a normalized (unit length) copy of this vector.
    ///
    /// Implementations may return the zero vector when the magnitude is
    /// below an epsilon threshold.
    ///
    /// # Returns
    ///
    /// - `Self` - The normalized vector.
    fn normalized(&self) -> Self;

    /// Returns a copy of this vector scaled by the given factor.
    ///
    /// # Arguments
    ///
    /// - `f64` - The scalar factor.
    ///
    /// # Returns
    ///
    /// - `Self` - The scaled vector.
    fn scaled(&self, scalar: f64) -> Self;

    /// Performs linear interpolation between `self` and `other` by `t`.
    ///
    /// # Arguments
    ///
    /// - `Self` - The target vector.
    /// - `f64` - The interpolation factor, typically in the range 0.0 to 1.0.
    ///
    /// # Returns
    ///
    /// - `Self` - The interpolated vector.
    fn lerp(&self, other: Self, factor: f64) -> Self;
}
