use super::*;

/// The Canvas 2D reference backend: a CPU trace blitted through one
/// reused `ImageData`.
///
/// The surface is two JavaScript objects, not one: the `ImageData`
/// the browser reads at blit time, and the `Uint8ClampedArray` that is
/// its pixel backing store. Both are allocated once per surface size
/// and reused by every later frame.
///
/// The two are distinct on purpose. `ImageData::data` is declared in
/// `web-sys` as `Clamped<Vec<u8>>`, which is a Rust copy of the
/// JavaScript array: writing into that copy cannot change the array the
/// `ImageData` still points at, so a blit after such a write is a
/// blit of the original, still transparent, pixels. Uploading through
/// the backing array's own `copy_from` is what actually reaches the
/// image, and it copies the traced bytes straight out of linear memory
/// without an intermediate Rust `Vec`.
#[derive(Data, Debug)]
pub struct CanvasRayTraceBackend {
    /// The 2D context every blit goes through.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) context: CanvasRenderingContext2d,
    /// The `ImageData` covering the whole surface, reallocated only
    /// when the surface size changes.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) image: ImageData,
    /// The JavaScript `Uint8ClampedArray` the `ImageData` reads, held
    /// so a frame is written into the real backing store rather than
    /// into a Rust copy of it.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) backing: Uint8ClampedArray,
    /// The width the current `ImageData` was allocated for.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) image_width: u32,
    /// The height the current `ImageData` was allocated for.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) image_height: u32,
}
