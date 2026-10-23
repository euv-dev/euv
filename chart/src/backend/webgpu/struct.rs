use super::*;

/// The WebGPU rendering backend for the chart.
///
/// This is the backend the chart prefers: vector geometry is tessellated
/// into triangles once, uploaded to the GPU once, and replayed with a
/// handful of draw calls. The display list is retained between frames, so a
/// chart that has not changed does not re-tessellate and does not re-upload
/// anything — the backend keeps the GPU buffers alive and reuses them as
/// long as the list is unchanged.
///
/// The backend is created asynchronously, because acquiring a GPU adapter
/// and a GPU device returns JavaScript promises. When the host cannot
/// provide WebGPU, [`WebGpuChartBackend::create`] returns `None` and the
/// factory falls back to WebGL2 and then to Canvas2D.
#[derive(Data)]
pub struct WebGpuChartBackend {
    /// The canvas the chart draws into.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) canvas: HtmlCanvasElement,
    /// The GPU device the resources are created on.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) device: JsValue,
    /// The GPU queue the command buffers are submitted to.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) queue: JsValue,
    /// The canvas's WebGPU context.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) context: JsValue,
    /// The swap-chain texture format, as reported by
    /// `getPreferredCanvasFormat`.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) format: String,
    /// The JavaScript methods the backend calls every frame.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) api: WebGpuApi,
    /// The methods of the command encoder, fetched on the first frame.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) encoder_api: Option<WebGpuEncoderApi>,
    /// The methods of the render pass, fetched on the first frame.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) pass_api: Option<WebGpuPassApi>,
    /// The pipeline that draws flat-coloured triangles.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) solid_pipeline: JsValue,
    /// The pipeline that draws textured triangles.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) textured_pipeline: JsValue,
    /// The bind group layout of the texture and the sampler.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) texture_layout: JsValue,
    /// The sampler bound to the texture bind group.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) sampler: JsValue,
    /// The buffer holding the CSS-pixel to clip-space transform.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) uniform_buffer: JsValue,
    /// The bind group that exposes the transform to the pipelines.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) uniform_bind_group: JsValue,
    /// The render pass descriptor, reused across frames.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) pass_descriptor: JsValue,
    /// The colour attachment inside the render pass descriptor.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) pass_attachment: JsValue,
    /// The glyph atlas texture and its bind group.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) atlas: WebGpuAtlas,
    /// The buffer holding the flat-coloured vertices.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) solid_buffer: WebGpuBuffer,
    /// The buffer holding the indices of the flat-coloured vertices.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) solid_index_buffer: WebGpuBuffer,
    /// The buffer holding the textured vertices.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) textured_buffer: WebGpuBuffer,
    /// The buffer holding the indices of the textured vertices.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) textured_index_buffer: WebGpuBuffer,
    /// The batches to draw, in paint order.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) batches: Vec<WebGpuBatch>,
    /// The images the display list refers to.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) images: Vec<WebGpuImage>,
    /// The identifier handed to the next image that shows up.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) next_image_id: u32,
    /// The number of glyphs the atlas held at the last render.
    ///
    /// The atlas starts over when it runs out of room, which moves the
    /// glyphs around. A shrinking count is the signal that the text
    /// geometry has to be built again.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) atlas_glyphs: usize,
    /// The fingerprint of the display list the buffers currently hold.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) fingerprint: Option<u64>,
    /// The tessellator, reused across frames so its scratch buffers are
    /// allocated once.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) tessellator: Tessellator,
    /// The logical width of the canvas, in CSS pixels.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) width: f64,
    /// The logical height of the canvas, in CSS pixels.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) height: f64,
    /// The device pixel ratio the canvas renders at.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) dpr: f64,
    /// The scratch storage for the flat-coloured vertices.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) solid_vertices: Vec<u8>,
    /// The scratch storage for the indices of the flat-coloured vertices.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) solid_indices: Vec<u32>,
    /// The scratch storage for the textured vertices.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) textured_vertices: Vec<u8>,
    /// The scratch storage for the indices of the textured vertices.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) textured_indices: Vec<u32>,
    /// The clip rectangles currently in effect.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) clips: Vec<WebGpuScissor>,
}

/// A GPU buffer whose capacity the backend manages.
///
/// The buffer is only re-created when the data outgrows it, and the new
/// capacity is rounded up to a power of two so that a chart whose data
/// grows a little at a time does not re-create the buffer on every change.
#[derive(Data)]
pub(crate) struct WebGpuBuffer {
    /// The GPU buffer, or `None` before the first allocation.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) buffer: Option<JsValue>,
    /// The capacity of the buffer, in bytes.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) capacity: u32,
}

/// The GPU-side state of the glyph atlas.
///
/// The atlas is a canvas the text shaper rasterises glyphs into. The
/// backend uploads it as a texture and binds it to the textured pipeline,
/// which is how the GPU backends draw text at all — WebGPU has no text
/// API.
#[derive(Clone, Debug)]
pub(crate) struct WebGpuAtlas {
    /// The atlas texture, or `None` before the first upload.
    pub(crate) texture: Option<JsValue>,
    /// The bind group that binds the atlas to group one.
    pub(crate) bind_group: Option<JsValue>,
    /// The width of the atlas texture, in pixels.
    pub(crate) width: u32,
    /// The height of the atlas texture, in pixels.
    pub(crate) height: u32,
}

/// The GPU-side state of one image.
///
/// Images are uploaded with `copyExternalImageToTexture` and drawn with the
/// textured pipeline, using a bind group of their own.
pub(crate) struct WebGpuImage {
    /// The image element the texture was made from.
    pub(crate) element: JsValue,
    /// A stable identifier, used to notice that the display list changed.
    pub(crate) id: u32,
    /// The texture holding the image, or `None` before the first upload.
    pub(crate) texture: Option<JsValue>,
    /// The bind group that binds the texture to group one.
    pub(crate) bind_group: Option<JsValue>,
    /// The width of the texture, in pixels.
    pub(crate) width: u32,
    /// The height of the texture, in pixels.
    pub(crate) height: u32,
    /// The source the texture was made from, used to notice a reload.
    pub(crate) source: String,
}

/// One contiguous run of triangles that costs a single draw call.
///
/// The batches are kept in paint order, and consecutive batches usually
/// share a pipeline, so the render pass only has to rebind when the kind
/// changes.
pub(crate) struct WebGpuBatch {
    /// The pipeline the batch is drawn with.
    pub(crate) kind: WebGpuBatchKind,
    /// The index of the batch's first triangle.
    pub(crate) first_index: u32,
    /// The number of indices in the batch.
    pub(crate) index_count: u32,
    /// The bind group bound to group one, for textured batches.
    pub(crate) bind_group: Option<JsValue>,
    /// The scissor rectangle the batch is clipped to, if any.
    pub(crate) scissor: Option<WebGpuScissor>,
}

/// A scissor rectangle, in physical pixels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WebGpuScissor {
    /// The left edge of the rectangle.
    pub(crate) x: u32,
    /// The top edge of the rectangle.
    pub(crate) y: u32,
    /// The width of the rectangle.
    pub(crate) width: u32,
    /// The height of the rectangle.
    pub(crate) height: u32,
}

/// The JavaScript methods the backend calls on the device, the queue, and
/// the canvas context.
///
/// The methods are fetched once, when the backend is created: JavaScript
/// class methods live on the prototype, so every call returns the same
/// `Function` object, and caching them keeps a steady-state frame free of
/// property lookups.
#[derive(Clone)]
pub(crate) struct WebGpuApi {
    /// `GPUDevice.createCommandEncoder`.
    pub(crate) create_command_encoder: Function,
    /// `GPUQueue.submit`.
    pub(crate) submit: Function,
    /// `GPUQueue.writeBuffer`.
    pub(crate) write_buffer: Function,
    /// `GPUQueue.copyExternalImageToTexture`.
    pub(crate) copy_external_image: Function,
    /// `GPUCanvasContext.getCurrentTexture`.
    pub(crate) get_current_texture: Function,
}

/// The methods of the command encoder.
///
/// The encoder is created per frame, but its methods live on the
/// prototype, so they are fetched from the first encoder and then reused.
#[derive(Clone)]
pub(crate) struct WebGpuEncoderApi {
    /// `GPUCommandEncoder.beginRenderPass`.
    pub(crate) begin_render_pass: Function,
    /// `GPUCommandEncoder.finish`.
    pub(crate) finish_encoder: Function,
}

/// The methods of the render pass.
///
/// The pass is created per frame, but its methods live on the prototype,
/// so they are fetched from the first pass and then reused.
#[derive(Clone)]
pub(crate) struct WebGpuPassApi {
    /// `GPURenderPassEncoder.setPipeline`.
    pub(crate) set_pipeline: Function,
    /// `GPURenderPassEncoder.setVertexBuffer`.
    pub(crate) set_vertex_buffer: Function,
    /// `GPURenderPassEncoder.setIndexBuffer`.
    pub(crate) set_index_buffer: Function,
    /// `GPURenderPassEncoder.setBindGroup`.
    pub(crate) set_bind_group: Function,
    /// `GPURenderPassEncoder.setScissorRect`.
    pub(crate) set_scissor_rect: Function,
    /// `GPURenderPassEncoder.drawIndexed`.
    pub(crate) draw_indexed: Function,
    /// `GPURenderPassEncoder.end`.
    pub(crate) end_pass: Function,
}

/// A small FNV-1a hasher, used to fingerprint the display list.
///
/// The fingerprint tells the backend whether the geometry it holds on the
/// GPU is still the geometry the display list asks for. Hashing the actual
/// coordinates — rather than just counting the shapes — means a chart that
/// moves its data but keeps the same number of points is still noticed.
#[derive(Data)]
pub(crate) struct GpuHasher {
    /// The running hash state.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) state: u64,
}
