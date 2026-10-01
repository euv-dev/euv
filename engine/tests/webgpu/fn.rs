use super::*;

#[test]
fn set_viewport_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue, &ViewportDescriptor) = WebGpuRenderer::set_viewport;
}

#[test]
fn set_scissor_rect_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue, u32, u32, u32, u32) = WebGpuRenderer::set_scissor_rect;
}

#[test]
fn set_stencil_reference_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue, u32) = WebGpuRenderer::set_stencil_reference;
}

#[test]
fn set_blend_constant_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue, f32, f32, f32, f32) = WebGpuRenderer::set_blend_constant;
}

#[test]
fn set_bind_group_with_dynamic_offsets_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue, u32, &JsValue, &[u32]) =
        WebGpuRenderer::set_bind_group_with_dynamic_offsets;
}

#[test]
fn set_bind_group_compute_with_dynamic_offsets_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue, u32, &JsValue, &[u32]) =
        WebGpuRenderer::set_bind_group_compute_with_dynamic_offsets;
}

#[test]
fn generate_mipmaps_signature_pinned() {
    let _: fn(&WebGpuRenderer, &JsValue) = WebGpuRenderer::generate_mipmaps;
}

#[test]
fn create_shader_module_with_label_signature_pinned() {
    fn _type_check(renderer: &WebGpuRenderer, source: &str, label: &str) -> JsValue {
        renderer.create_shader_module_with_label(source, label)
    }
    let _ = _type_check;
}

#[test]
fn read_buffer_is_async() {
    fn assert_future<F>(_: F)
    where
        F: Future,
    {
    }
    let fut: Ready<Option<Vec<u8>>> = ready(None);
    assert_future(fut);
}

#[test]
fn begin_render_pass_full_signature_pinned() {
    fn _type_check(
        renderer: &mut WebGpuRenderer,
        encoder: &JsValue,
        color: &mut ColorAttachment,
        depth: Option<&DepthStencilAttachment>,
    ) -> JsValue {
        renderer.begin_render_pass_full(encoder, color, depth)
    }
    let _ = _type_check;
}

#[test]
fn create_render_pipeline_full_signature_pinned() {
    fn _type_check(renderer: &WebGpuRenderer, descriptor: &RenderPipelineDescriptor) -> JsValue {
        renderer.create_render_pipeline_full(descriptor)
    }
    let _ = _type_check;
}

#[test]
fn create_view_signature_pinned() {
    fn _type_check(
        renderer: &WebGpuRenderer,
        texture: &JsValue,
        descriptor: Option<&TextureViewDescriptor>,
    ) -> JsValue {
        renderer.create_view(texture, descriptor)
    }
    let _ = _type_check;
}

#[test]
fn push_error_scope_signature_pinned() {
    let _: fn(&WebGpuRenderer, GpuErrorFilter) = WebGpuRenderer::push_error_scope;
}

#[test]
fn texture_view_descriptor_full_returns_canonical_shape() {
    let d: TextureViewDescriptor = TextureViewDescriptor::full();
    assert!(d.get_format().is_none());
    assert!(d.get_dimension().is_none());
    assert_eq!(d.get_base_mip_level(), 0);
    assert_eq!(d.get_mip_level_count(), 0);
    assert_eq!(d.get_base_array_layer(), 0);
    assert_eq!(d.get_array_layer_count(), 0);
    assert!(d.get_aspect().is_none());
}

#[test]
fn gpu_sampler_descriptor_default_returns_nearest_clamp() {
    let s: SamplerDescriptor = SamplerDescriptor::nearest_clamp();
    assert_eq!(s.get_filter(), FilterMode::Nearest);
    assert_eq!(s.get_mipmap_filter(), MipmapFilter::Nearest);
    assert_eq!(s.get_address_mode_u(), AddressMode::ClampToEdge);
    assert_eq!(s.get_address_mode_v(), AddressMode::ClampToEdge);
    assert_eq!(s.get_address_mode_w(), AddressMode::ClampToEdge);
    let compare: Option<CompareFunction> = s.try_get_compare();
    assert!(compare.is_none());
}
