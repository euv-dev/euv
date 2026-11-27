/// Fallback device label when executing on headless or software-only hosts.
pub const GPU_DEVICE_FALLBACK: &str = "euv-software-fallback";
/// Device label for Web WebGL2 / WebGPU browsers.
pub const GPU_DEVICE_WEB: &str = "Browser WebGL2 / WebGPU";
/// Device label when Windows Direct3D 12 runtime is present.
pub const GPU_DEVICE_D3D12: &str = "Windows Direct3D 12";
/// Device label when Vulkan ICD runtime is present.
pub const GPU_DEVICE_VULKAN: &str = "Vulkan Physical GPU";

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const MTL_CLS_NS_STRING: &str = "NSString";
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const MTL_SEL_STRING_UTF8: &str = "stringWithUTF8String:";
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const MTL_SEL_UTF8_STRING: &str = "UTF8String";
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const MTL_SEL_NAME: &str = "name";
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const MTL_SEL_NEW_CMD_QUEUE: &str = "newCommandQueue";
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const MTL_SEL_NEW_LIBRARY: &str = "newLibraryWithSource:options:error:";
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const MTL_SEL_NEW_FUNCTION: &str = "newFunctionWithName:";
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const MTL_SEL_NEW_BUFFER: &str = "newBufferWithBytes:length:options:";
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const MTL_SEL_NEW_BUFFER_LENGTH: &str = "newBufferWithLength:options:";
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const MTL_SEL_CONTENTS: &str = "contents";
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const MTL_SEL_CMD_BUFFER: &str = "commandBuffer";
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const MTL_SEL_COMMIT: &str = "commit";
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const MTL_SEL_RELEASE: &str = "release";
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const MTL_FN_VERTEX: &str = "euv_ui_vertex";
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const MTL_FN_FRAGMENT: &str = "euv_ui_fragment";
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub(crate) const GPU_DEVICE_APPLE_METAL: &str = "Apple Metal GPU";

#[cfg(target_os = "windows")]
pub(crate) const DLL_D3D12: &[u8] = b"d3d12.dll\0";
#[cfg(target_os = "windows")]
pub(crate) const DLL_D3DCOMPILER: &[u8] = b"d3dcompiler_47.dll\0";
#[cfg(target_os = "windows")]
pub(crate) const SYM_D3D12_CREATE_DEVICE: &[u8] = b"D3D12CreateDevice\0";
#[cfg(target_os = "windows")]
pub(crate) const SYM_D3D_COMPILE: &[u8] = b"D3DCompile\0";

#[cfg(target_os = "linux")]
pub(crate) const LIB_VULKAN_SONAME: &str = "libvulkan.so.1";
#[cfg(target_os = "android")]
pub(crate) const LIB_VULKAN_SONAME: &str = "libvulkan.so";
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) const SYM_VK_CREATE_INSTANCE: &str = "vkCreateInstance";
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) const SYM_VK_ENUM_DEVICES: &str = "vkEnumeratePhysicalDevices";
#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) const RTLD_LAZY_GPU: i32 = 1;
