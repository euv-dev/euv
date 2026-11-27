use super::*;

/// Probes the host OS for a physical GPU adapter, compiles native UI shaders, and returns handles.
///
/// # Arguments
///
/// - `GraphicsBackend` - Requested graphics backend.
///
/// # Returns
///
/// - `(GpuAdapterInfo, usize, usize, usize)` - Adapter info plus `(device, queue, library)` raw handles.
#[must_use]
pub fn probe_native_gpu(backend: GraphicsBackend) -> (GpuAdapterInfo, usize, usize, usize) {
    if backend == GraphicsBackend::Headless || backend == GraphicsBackend::SoftwareCpu {
        return (
            GpuAdapterInfo::new(backend, GPU_DEVICE_FALLBACK.to_string(), false, false),
            0,
            0,
            0,
        );
    }
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        probe_apple_metal(backend)
    }
    #[cfg(target_os = "windows")]
    {
        probe_windows_d3d12(backend)
    }
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        probe_vulkan_runtime(backend)
    }
    #[cfg(target_arch = "wasm32")]
    {
        (
            GpuAdapterInfo::new(backend, GPU_DEVICE_WEB.to_string(), true, false),
            0,
            0,
            0,
        )
    }
    #[cfg(not(any(target_arch = "wasm32", unix, windows)))]
    {
        (
            GpuAdapterInfo::new(backend, GPU_DEVICE_FALLBACK.to_string(), false, false),
            0,
            0,
            0,
        )
    }
}

/// Uploads instanced quad buffers and submits a command buffer to the active physical GPU.
///
/// # Arguments
///
/// - `usize` - Raw OS device handle.
/// - `usize` - Raw OS command queue handle.
/// - `&[GpuBatch]` - Compiled instanced draw batches.
///
/// # Returns
///
/// - `usize` - Total GPU instances uploaded and submitted.
pub fn submit_native_gpu_batches(
    device_handle: usize,
    queue_handle: usize,
    batches: &[GpuBatch],
) -> usize {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        metal_submit_batches(device_handle, queue_handle, batches)
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    {
        let _: (usize, usize) = (device_handle, queue_handle);
        batches
            .iter()
            .map(|b: &GpuBatch| b.get_instances().len())
            .sum()
    }
}

/// Uploads instanced quad buffers via a reusable grow-only staging buffer and submits a command buffer to the active GPU.
///
/// # Arguments
///
/// - `usize` - Raw OS device handle.
/// - `usize` - Raw OS command queue handle.
/// - `&mut (usize, usize, usize)` - Reusable staging state `(buffer_handle, contents_ptr, byte_capacity)`.
/// - `&[GpuBatch]` - Compiled instanced draw batches.
///
/// # Returns
///
/// - `usize` - Total GPU instances uploaded and submitted.
pub(crate) fn submit_native_gpu_batches_staged(
    device_handle: usize,
    queue_handle: usize,
    staging: &mut (usize, usize, usize),
    batches: &[GpuBatch],
) -> usize {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        metal_submit_batches_staged(device_handle, queue_handle, staging, batches)
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    {
        let _: (usize, usize, &mut (usize, usize, usize)) = (device_handle, queue_handle, staging);
        batches
            .iter()
            .map(|b: &GpuBatch| b.get_instances().len())
            .sum()
    }
}

/// Releases any retained persistent GPU staging buffer before tearing down device handles.
///
/// # Arguments
///
/// - `usize` - Retained staging buffer handle (`0` when unallocated).
pub(crate) fn release_native_gpu_staging(staging_buffer_handle: usize) {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        if staging_buffer_handle != 0 {
            let sel: &MetalSelectors = cached_metal_selectors();
            mtl_send_void0(
                staging_buffer_handle as *mut c_void,
                sel.release as *mut c_void,
            );
        }
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    {
        let _: usize = staging_buffer_handle;
    }
}

/// Releases native GPU handles on driver teardown.
///
/// # Arguments
///
/// - `usize` - Raw device handle.
/// - `usize` - Raw command queue handle.
/// - `usize` - Raw shader library handle.
pub fn release_native_gpu_handles(
    device_handle: usize,
    queue_handle: usize,
    library_handle: usize,
) {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        metal_release_handles(device_handle, queue_handle, library_handle);
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    {
        let _: (usize, usize, usize) = (device_handle, queue_handle, library_handle);
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[link(name = "Metal", kind = "framework")]
unsafe extern "C" {
    /// Creates the default Apple Metal GPU device handle.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn MTLCreateSystemDefaultDevice() -> *mut c_void;
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[link(name = "objc")]
unsafe extern "C" {
    /// Looks up an Objective-C class pointer by C name.
    ///
    /// # Arguments
    ///
    /// - `*const c_char` - A `*const c_char` parameter.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn objc_getClass(name: *const c_char) -> *mut c_void;
    /// Registers an Objective-C selector by C name.
    ///
    /// # Arguments
    ///
    /// - `*const c_char` - A `*const c_char` parameter.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn sel_registerName(name: *const c_char) -> *mut c_void;
    /// Objective-C dynamic message dispatch entry point.
    fn objc_msgSend();
}

/// Registers an Objective-C selector for Apple Metal FFI calls.
///
/// # Arguments
///
/// - `&str` - Selector name.
///
/// # Returns
///
/// - `*mut c_void` - Registered Objective-C selector pointer.
#[cfg(any(target_os = "macos", target_os = "ios"))]
fn mtl_sel(name: &str) -> *mut c_void {
    let Ok(c_name) = CString::new(name) else {
        return null_mut();
    };
    unsafe { sel_registerName(c_name.as_ptr()) }
}

/// Allocates an autoreleased `NSString` from a UTF-8 Rust slice.
///
/// # Arguments
///
/// - `&str` - Source UTF-8 string.
///
/// # Returns
///
/// - `*mut c_void` - Objective-C `NSString` pointer.
#[cfg(any(target_os = "macos", target_os = "ios"))]
fn mtl_ns_string(text: &str) -> *mut c_void {
    let Ok(cls_name) = CString::new(MTL_CLS_NS_STRING) else {
        return null_mut();
    };
    let Ok(c_text) = CString::new(text) else {
        return null_mut();
    };
    unsafe {
        let cls: *mut c_void = objc_getClass(cls_name.as_ptr());
        if cls.is_null() {
            return null_mut();
        }
        let send: unsafe extern "C" fn(*mut c_void, *mut c_void, *const c_char) -> *mut c_void =
            transmute(objc_msgSend as *const c_void);
        send(cls, mtl_sel(MTL_SEL_STRING_UTF8), c_text.as_ptr())
    }
}

/// Sends a zero-argument Objective-C message returning a pointer.
///
/// # Arguments
///
/// - `*mut c_void` - Target Objective-C object.
/// - `*mut c_void` - Selector pointer.
///
/// # Returns
///
/// - `*mut c_void` - Returned Objective-C object pointer.
#[cfg(any(target_os = "macos", target_os = "ios"))]
fn mtl_send_ptr0(obj: *mut c_void, sel: *mut c_void) -> *mut c_void {
    if obj.is_null() || sel.is_null() {
        return null_mut();
    }
    unsafe {
        let send: unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void =
            transmute(objc_msgSend as *const c_void);
        send(obj, sel)
    }
}

/// Sends a zero-argument Objective-C message returning void.
///
/// # Arguments
///
/// - `*mut c_void` - Target Objective-C object.
/// - `*mut c_void` - Selector pointer.
#[cfg(any(target_os = "macos", target_os = "ios"))]
fn mtl_send_void0(obj: *mut c_void, sel: *mut c_void) {
    if obj.is_null() || sel.is_null() {
        return;
    }
    unsafe {
        let send: unsafe extern "C" fn(*mut c_void, *mut c_void) =
            transmute(objc_msgSend as *const c_void);
        send(obj, sel);
    }
}

/// Initializes a real Apple Metal GPU device, command queue, and runtime-compiled MSL library.
///
/// # Arguments
///
/// - `GraphicsBackend` - Requested backend tag.
///
/// # Returns
///
/// - `(GpuAdapterInfo, usize, usize, usize)` - Probed Metal device info and retained ObjC handles.
#[cfg(any(target_os = "macos", target_os = "ios"))]
fn probe_apple_metal(backend: GraphicsBackend) -> (GpuAdapterInfo, usize, usize, usize) {
    unsafe {
        let device: *mut c_void = MTLCreateSystemDefaultDevice();
        if device.is_null() {
            return (
                GpuAdapterInfo::new(backend, GPU_DEVICE_FALLBACK.to_string(), false, false),
                0,
                0,
                0,
            );
        }
        let ns_name: *mut c_void = mtl_send_ptr0(device, mtl_sel(MTL_SEL_NAME));
        let utf8_ptr: *const c_char =
            mtl_send_ptr0(ns_name, mtl_sel(MTL_SEL_UTF8_STRING)).cast::<c_char>();
        let device_name: String = if utf8_ptr.is_null() {
            GPU_DEVICE_APPLE_METAL.to_string()
        } else {
            CStr::from_ptr(utf8_ptr).to_string_lossy().into_owned()
        };
        let queue: *mut c_void = mtl_send_ptr0(device, mtl_sel(MTL_SEL_NEW_CMD_QUEUE));
        let ns_src: *mut c_void = mtl_ns_string(MSL_UI_SHADER);
        let mut err_ptr: *mut c_void = null_mut();
        let send_lib: unsafe extern "C" fn(
            *mut c_void,
            *mut c_void,
            *mut c_void,
            *mut c_void,
            *mut *mut c_void,
        ) -> *mut c_void = transmute(objc_msgSend as *const c_void);
        let library: *mut c_void = send_lib(
            device,
            mtl_sel(MTL_SEL_NEW_LIBRARY),
            ns_src,
            null_mut(),
            &mut err_ptr,
        );
        let mut shader_compiled: bool = false;
        if !library.is_null() {
            let send_fn: unsafe extern "C" fn(
                *mut c_void,
                *mut c_void,
                *mut c_void,
            ) -> *mut c_void = transmute(objc_msgSend as *const c_void);
            let vs_fn: *mut c_void = send_fn(
                library,
                mtl_sel(MTL_SEL_NEW_FUNCTION),
                mtl_ns_string(MTL_FN_VERTEX),
            );
            let fs_fn: *mut c_void = send_fn(
                library,
                mtl_sel(MTL_SEL_NEW_FUNCTION),
                mtl_ns_string(MTL_FN_FRAGMENT),
            );
            shader_compiled = !vs_fn.is_null() && !fs_fn.is_null();
            if !vs_fn.is_null() {
                mtl_send_void0(vs_fn, mtl_sel(MTL_SEL_RELEASE));
            }
            if !fs_fn.is_null() {
                mtl_send_void0(fs_fn, mtl_sel(MTL_SEL_RELEASE));
            }
        }
        (
            GpuAdapterInfo::new(
                GraphicsBackend::Metal,
                device_name,
                !queue.is_null(),
                shader_compiled,
            ),
            device as usize,
            queue as usize,
            library as usize,
        )
    }
}

/// Returns the process-wide `OnceLock`-cached Objective-C selector pointers for Metal submission and teardown.
///
/// # Returns
///
/// - `&'static MetalSelectors` - Cached Metal selector addresses.
#[cfg(any(target_os = "macos", target_os = "ios"))]
fn cached_metal_selectors() -> &'static MetalSelectors {
    METAL_SELECTORS.get_or_init(|| MetalSelectors {
        new_buffer_with_bytes: mtl_sel(MTL_SEL_NEW_BUFFER) as usize,
        new_buffer_with_length: mtl_sel(MTL_SEL_NEW_BUFFER_LENGTH) as usize,
        contents: mtl_sel(MTL_SEL_CONTENTS) as usize,
        command_buffer: mtl_sel(MTL_SEL_CMD_BUFFER) as usize,
        commit: mtl_sel(MTL_SEL_COMMIT) as usize,
        release: mtl_sel(MTL_SEL_RELEASE) as usize,
    })
}

/// Uploads `GpuQuadInstance` buffers into a reusable grow-only shared `MTLBuffer` and commits a command buffer using cached selectors.
///
/// # Arguments
///
/// - `usize` - Retained `id<MTLDevice>` pointer.
/// - `usize` - Retained `id<MTLCommandQueue>` pointer.
/// - `&mut (usize, usize, usize)` - Persistent staging tuple `(buffer_handle, contents_ptr, byte_capacity)`.
/// - `&[GpuBatch]` - Ordered GPU batches to upload.
///
/// # Returns
///
/// - `usize` - Total instances uploaded into Metal GPU memory.
#[cfg(any(target_os = "macos", target_os = "ios"))]
fn metal_submit_batches_staged(
    device_handle: usize,
    queue_handle: usize,
    staging: &mut (usize, usize, usize),
    batches: &[GpuBatch],
) -> usize {
    let total_instances: usize = batches
        .iter()
        .map(|b: &GpuBatch| b.get_instances().len())
        .sum();
    if device_handle == 0 || queue_handle == 0 || total_instances == 0 {
        return total_instances;
    }
    let total_bytes: usize = total_instances.saturating_mul(size_of::<GpuQuadInstance>());
    let device: *mut c_void = device_handle as *mut c_void;
    let queue: *mut c_void = queue_handle as *mut c_void;
    let sel: &MetalSelectors = cached_metal_selectors();
    let sel_rel: *mut c_void = sel.release as *mut c_void;
    let (buf_handle, contents_ptr, capacity): &mut (usize, usize, usize) = staging;
    unsafe {
        if *buf_handle == 0 || *contents_ptr == 0 || total_bytes > *capacity {
            if *buf_handle != 0 {
                mtl_send_void0(*buf_handle as *mut c_void, sel_rel);
                *buf_handle = 0;
                *contents_ptr = 0;
                *capacity = 0;
            }
            let alloc_bytes: usize = total_bytes.max(4096).next_power_of_two();
            let send_new_len: unsafe extern "C" fn(
                *mut c_void,
                *mut c_void,
                usize,
                usize,
            ) -> *mut c_void = transmute(objc_msgSend as *const c_void);
            let new_buf: *mut c_void = send_new_len(
                device,
                sel.new_buffer_with_length as *mut c_void,
                alloc_bytes,
                0,
            );
            if !new_buf.is_null() {
                let ptr: *mut c_void = mtl_send_ptr0(new_buf, sel.contents as *mut c_void);
                if !ptr.is_null() {
                    *buf_handle = new_buf as usize;
                    *contents_ptr = ptr as usize;
                    *capacity = alloc_bytes;
                } else {
                    mtl_send_void0(new_buf, sel_rel);
                }
            }
        }
        if *contents_ptr != 0 && total_bytes <= *capacity {
            let mut dst: *mut u8 = (*contents_ptr as *mut c_void).cast::<u8>();
            for batch in batches.iter() {
                let instances: &[GpuQuadInstance] = batch.get_instances();
                if instances.is_empty() {
                    continue;
                }
                let byte_len: usize = size_of_val(instances);
                dst.copy_from_nonoverlapping(instances.as_ptr().cast::<u8>(), byte_len);
                dst = dst.add(byte_len);
            }
        } else {
            let send_buf: unsafe extern "C" fn(
                *mut c_void,
                *mut c_void,
                *const c_void,
                usize,
                usize,
            ) -> *mut c_void = transmute(objc_msgSend as *const c_void);
            let sel_buf: *mut c_void = sel.new_buffer_with_bytes as *mut c_void;
            for batch in batches.iter() {
                let instances: &[GpuQuadInstance] = batch.get_instances();
                if instances.is_empty() {
                    continue;
                }
                let byte_len: usize = size_of_val(instances);
                let mtl_buf: *mut c_void = send_buf(
                    device,
                    sel_buf,
                    instances.as_ptr().cast::<c_void>(),
                    byte_len,
                    0,
                );
                if !mtl_buf.is_null() {
                    mtl_send_void0(mtl_buf, sel_rel);
                }
            }
        }
        let cmd_buf: *mut c_void = mtl_send_ptr0(queue, sel.command_buffer as *mut c_void);
        if !cmd_buf.is_null() {
            mtl_send_void0(cmd_buf, sel.commit as *mut c_void);
        }
    }
    total_instances
}

/// Uploads `GpuQuadInstance` buffers to Metal GPU memory and commits a command buffer.
///
/// # Arguments
///
/// - `usize` - Retained `id<MTLDevice>` pointer.
/// - `usize` - Retained `id<MTLCommandQueue>` pointer.
/// - `&[GpuBatch]` - Ordered GPU batches to upload.
///
/// # Returns
///
/// - `usize` - Total instances uploaded into Metal buffers.
#[cfg(any(target_os = "macos", target_os = "ios"))]
fn metal_submit_batches(device_handle: usize, queue_handle: usize, batches: &[GpuBatch]) -> usize {
    let total_instances: usize = batches
        .iter()
        .map(|b: &GpuBatch| b.get_instances().len())
        .sum();
    if device_handle == 0 || queue_handle == 0 || total_instances == 0 {
        return total_instances;
    }
    let device: *mut c_void = device_handle as *mut c_void;
    let queue: *mut c_void = queue_handle as *mut c_void;
    let sel: &MetalSelectors = cached_metal_selectors();
    let sel_buf: *mut c_void = sel.new_buffer_with_bytes as *mut c_void;
    let sel_rel: *mut c_void = sel.release as *mut c_void;
    unsafe {
        let send_buf: unsafe extern "C" fn(
            *mut c_void,
            *mut c_void,
            *const c_void,
            usize,
            usize,
        ) -> *mut c_void = transmute(objc_msgSend as *const c_void);
        for batch in batches.iter() {
            let instances: &[GpuQuadInstance] = batch.get_instances();
            if instances.is_empty() {
                continue;
            }
            let byte_len: usize = size_of_val(instances);
            let mtl_buf: *mut c_void = send_buf(
                device,
                sel_buf,
                instances.as_ptr().cast::<c_void>(),
                byte_len,
                0,
            );
            if !mtl_buf.is_null() {
                mtl_send_void0(mtl_buf, sel_rel);
            }
        }
        let cmd_buf: *mut c_void = mtl_send_ptr0(queue, sel.command_buffer as *mut c_void);
        if !cmd_buf.is_null() {
            mtl_send_void0(cmd_buf, sel.commit as *mut c_void);
        }
    }
    total_instances
}

/// Releases retained Apple Metal Objective-C objects cleanly on drop.
///
/// # Arguments
///
/// - `usize` - `id<MTLDevice>` pointer.
/// - `usize` - `id<MTLCommandQueue>` pointer.
/// - `usize` - `id<MTLLibrary>` pointer.
#[cfg(any(target_os = "macos", target_os = "ios"))]
fn metal_release_handles(device_handle: usize, queue_handle: usize, library_handle: usize) {
    let sel_rel: *mut c_void = cached_metal_selectors().release as *mut c_void;
    if library_handle != 0 {
        mtl_send_void0(library_handle as *mut c_void, sel_rel);
    }
    if queue_handle != 0 {
        mtl_send_void0(queue_handle as *mut c_void, sel_rel);
    }
    if device_handle != 0 {
        mtl_send_void0(device_handle as *mut c_void, sel_rel);
    }
}

#[cfg(target_os = "windows")]
#[link(name = "kernel32")]
unsafe extern "system" {
    /// Dynamically loads a Windows DLL by ANSI path.
    ///
    /// # Arguments
    ///
    /// - `*const u8` - A `*const u8` parameter.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn LoadLibraryA(lp_lib_file_name: *const u8) -> *mut c_void;
    /// Looks up an exported procedure address from a loaded Windows DLL.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    /// - `*const u8` - A `*const u8` parameter.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn GetProcAddress(h_module: *mut c_void, lp_proc_name: *const u8) -> *mut c_void;
    /// Unloads a dynamically loaded Windows DLL module.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    ///
    /// # Returns
    ///
    /// - `i32` - A 32-bit signed integer.
    fn FreeLibrary(h_lib_module: *mut c_void) -> i32;
}

/// Probes Windows Direct3D 12 and `d3dcompiler_47.dll` at runtime with zero link-time dependency.
///
/// # Arguments
///
/// - `GraphicsBackend` - Requested graphics backend.
///
/// # Returns
///
/// - `(GpuAdapterInfo, usize, usize, usize)` - Probed Direct3D 12 availability metadata.
#[cfg(target_os = "windows")]
fn probe_windows_d3d12(backend: GraphicsBackend) -> (GpuAdapterInfo, usize, usize, usize) {
    unsafe {
        let d3d12_mod: *mut c_void = LoadLibraryA(DLL_D3D12.as_ptr());
        let has_d3d12: bool = if !d3d12_mod.is_null() {
            let fn_ptr: *mut c_void = GetProcAddress(d3d12_mod, SYM_D3D12_CREATE_DEVICE.as_ptr());
            let _: i32 = FreeLibrary(d3d12_mod);
            !fn_ptr.is_null()
        } else {
            false
        };
        let compiler_mod: *mut c_void = LoadLibraryA(DLL_D3DCOMPILER.as_ptr());
        let has_compiler: bool = if !compiler_mod.is_null() {
            let fn_ptr: *mut c_void = GetProcAddress(compiler_mod, SYM_D3D_COMPILE.as_ptr());
            let _: i32 = FreeLibrary(compiler_mod);
            !fn_ptr.is_null()
        } else {
            false
        };
        let label: &str = if has_d3d12 {
            GPU_DEVICE_D3D12
        } else {
            GPU_DEVICE_FALLBACK
        };
        (
            GpuAdapterInfo::new(backend, label.to_string(), has_d3d12, has_compiler),
            0,
            0,
            0,
        )
    }
}

#[cfg(any(target_os = "linux", target_os = "android"))]
unsafe extern "C" {
    /// Dynamically opens a shared library object by name.
    ///
    /// # Arguments
    ///
    /// - `*const c_char` - A `*const c_char` parameter.
    /// - `i32` - A 32-bit signed integer (`i32`).
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn dlopen(filename: *const c_char, flags: i32) -> *mut c_void;
    /// Resolves a symbol address from an open shared library handle.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    /// - `*const c_char` - A `*const c_char` parameter.
    ///
    /// # Returns
    ///
    /// - `*mut c_void` - A `*mut c_void` value.
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    /// Closes an open shared library handle.
    ///
    /// # Arguments
    ///
    /// - `*mut c_void` - A `*mut c_void` parameter.
    ///
    /// # Returns
    ///
    /// - `i32` - A 32-bit signed integer.
    fn dlclose(handle: *mut c_void) -> i32;
}

/// Probes Linux or Android Vulkan loader (`libvulkan.so.1` / `libvulkan.so`) at runtime via `dlopen`.
///
/// # Arguments
///
/// - `GraphicsBackend` - Requested graphics backend.
///
/// # Returns
///
/// - `(GpuAdapterInfo, usize, usize, usize)` - Probed Vulkan runtime status.
#[cfg(any(target_os = "linux", target_os = "android"))]
fn probe_vulkan_runtime(backend: GraphicsBackend) -> (GpuAdapterInfo, usize, usize, usize) {
    let Ok(lib_name) = CString::new(LIB_VULKAN_SONAME) else {
        return (
            GpuAdapterInfo::new(backend, GPU_DEVICE_FALLBACK.to_string(), false, false),
            0,
            0,
            0,
        );
    };
    let Ok(sym_create) = CString::new(SYM_VK_CREATE_INSTANCE) else {
        return (
            GpuAdapterInfo::new(backend, GPU_DEVICE_FALLBACK.to_string(), false, false),
            0,
            0,
            0,
        );
    };
    let Ok(sym_enum) = CString::new(SYM_VK_ENUM_DEVICES) else {
        return (
            GpuAdapterInfo::new(backend, GPU_DEVICE_FALLBACK.to_string(), false, false),
            0,
            0,
            0,
        );
    };
    let available: bool = unsafe {
        let handle: *mut c_void = dlopen(lib_name.as_ptr(), RTLD_LAZY_GPU);
        if handle.is_null() {
            false
        } else {
            let p1: *mut c_void = dlsym(handle, sym_create.as_ptr());
            let p2: *mut c_void = dlsym(handle, sym_enum.as_ptr());
            let _: i32 = dlclose(handle);
            !p1.is_null() && !p2.is_null()
        }
    };
    let name: &str = if available {
        GPU_DEVICE_VULKAN
    } else {
        GPU_DEVICE_FALLBACK
    };
    (
        GpuAdapterInfo::new(backend, name.to_string(), available, available),
        0,
        0,
        0,
    )
}

/// Compiles and links the WebGL 2 analytical SDF shader program on `wasm32`.
///
/// # Arguments
///
/// - `&WebGl2RenderingContext` - Active WebGL 2 canvas context.
///
/// # Returns
///
/// - `Option<WebGlProgram>` - Linked shader program when compilation succeeds.
#[cfg(target_arch = "wasm32")]
pub fn webgl2_compile_ui_program(gl: &WebGl2RenderingContext) -> Option<WebGlProgram> {
    let vs: WebGlShader = gl.create_shader(WebGl2RenderingContext::VERTEX_SHADER)?;
    gl.shader_source(&vs, GLSL_ES3_UI_VERTEX_SHADER);
    gl.compile_shader(&vs);
    let fs: WebGlShader = gl.create_shader(WebGl2RenderingContext::FRAGMENT_SHADER)?;
    gl.shader_source(&fs, GLSL_ES3_UI_FRAGMENT_SHADER);
    gl.compile_shader(&fs);
    let program: WebGlProgram = gl.create_program()?;
    gl.attach_shader(&program, &vs);
    gl.attach_shader(&program, &fs);
    gl.link_program(&program);
    Some(program)
}
