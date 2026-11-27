#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../env/common.sh
source "${SCRIPT_DIR}/../env/common.sh"
load_local_env

if [ -z "${ANDROID_HOME:-}" ] || [ ! -x "${ANDROID_HOME}/platform-tools/adb" ]; then
  log_info "Android environment missing; running scripts/env/setup-android.sh..."
  "${REPO_ROOT}/scripts/env/setup-android.sh"
  load_local_env
fi

API_LEVEL="${ANDROID_API_LEVEL:-35}"
BUILD_TOOLS="${ANDROID_BUILD_TOOLS:-35.0.0}"
AVD_NAME="${ANDROID_AVD_NAME:-euv_android_${API_LEVEL}}"
ANDROID_JAR="${ANDROID_HOME}/platforms/android-${API_LEVEL}/android.jar"
BT_DIR="${ANDROID_HOME}/build-tools/${BUILD_TOOLS}"
AAPT2="${BT_DIR}/aapt2"
D8="${BT_DIR}/d8"
ZIPALIGN="${BT_DIR}/zipalign"
APKSIGNER="${BT_DIR}/apksigner"

BUILD_DIR="${REPO_ROOT}/target/mobile-hosts/android-host"
OUT_DIR="${REPO_ROOT}/dist/android"
APK_OUT="${OUT_DIR}/EuvShowcase.apk"
mkdir -p "${BUILD_DIR}/rust/src" "${BUILD_DIR}/java/vip/ltpp/euv/showcase" "${BUILD_DIR}/stage/lib/arm64-v8a" "${OUT_DIR}"

cat > "${BUILD_DIR}/rust/Cargo.toml" <<EOF
[workspace]

[package]
name = "euv-android-host"
version = "0.42.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
euv = { path = "${REPO_ROOT}" }
euv-example = { path = "${REPO_ROOT}/example" }

[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
panic = "abort"
strip = "symbols"
EOF

cat > "${BUILD_DIR}/rust/src/lib.rs" <<'EOF'
use std::ffi::{CString, c_char, c_void};
use std::sync::atomic::{AtomicPtr, Ordering};
use euv::{
    App, GlyphKey, KeyCode, KeyEvent, Modifiers, NativeSystemFont, PlatformKind, PointerEvent,
    RasterizedGlyph, RenderMode, SafeAreaInsets, WindowEvent, WindowId,
    register_external_glyph_rasterizer, register_native_action_bridge,
};
use euv_example::{ShowcaseState, create_showcase_app_for_platform};

pub type JniEnv = *mut *const JniNativeInterface;
pub type JObject = *mut c_void;
pub type JClass = *mut c_void;
pub type JMethodId = *mut c_void;
pub type JIntArray = *mut c_void;
pub type JByteArray = *mut c_void;

#[repr(C)]
#[derive(Clone, Copy)]
pub union JValue {
    pub z: u8,
    pub i: i32,
    pub f: f32,
    pub j: i64,
    pub l: JObject,
}

#[repr(C)]
pub struct JniNativeInterface {
    _pad0: [*const c_void; 21],
    new_global_ref: unsafe extern "C" fn(JniEnv, JObject) -> JObject,
    _pad1: *const c_void,
    delete_local_ref: unsafe extern "C" fn(JniEnv, JObject),
    _pad2: [*const c_void; 89],
    get_static_method_id:
        unsafe extern "C" fn(JniEnv, JClass, *const c_char, *const c_char) -> JMethodId,
    _pad3: [*const c_void; 2],
    call_static_object_method_a:
        unsafe extern "C" fn(JniEnv, JClass, JMethodId, *const JValue) -> JObject,
    _pad3b: [*const c_void; 2],
    call_static_boolean_method_a:
        unsafe extern "C" fn(JniEnv, JClass, JMethodId, *const JValue) -> u8,
    _pad4a: [*const c_void; 47],
    new_string_utf: unsafe extern "C" fn(JniEnv, *const c_char) -> JObject,
    _pad4b: [*const c_void; 3],
    get_array_length: unsafe extern "C" fn(JniEnv, JObject) -> i32,
    _pad5: [*const c_void; 28],
    get_byte_array_region: unsafe extern "C" fn(JniEnv, JByteArray, i32, i32, *mut i8),
    _pad6: [*const c_void; 10],
    set_int_array_region: unsafe extern "C" fn(JniEnv, JIntArray, i32, i32, *const i32),
    _pad7: [*const c_void; 18],
    get_direct_buffer_address: unsafe extern "C" fn(JniEnv, JObject) -> *mut c_void,
    get_direct_buffer_capacity: unsafe extern "C" fn(JniEnv, JObject) -> i64,
}

static ACTIVE_JNI_ENV: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static BRIDGE_CLASS_REF: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static RASTERIZE_METHOD: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static ACTION_METHOD: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

fn android_native_action_bridge(kind: &str, arg1: &str, arg2: &str) -> bool {
    let env_ptr: *mut c_void = ACTIVE_JNI_ENV.load(Ordering::Acquire);
    let cls_ptr: JClass = BRIDGE_CLASS_REF.load(Ordering::Acquire);
    let mid_ptr: JMethodId = ACTION_METHOD.load(Ordering::Acquire);
    if env_ptr.is_null() || cls_ptr.is_null() || mid_ptr.is_null() {
        return false;
    }
    let Ok(c_kind) = CString::new(kind) else {
        return false;
    };
    let Ok(c_a1) = CString::new(arg1) else {
        return false;
    };
    let Ok(c_a2) = CString::new(arg2) else {
        return false;
    };
    let env: JniEnv = env_ptr.cast();
    unsafe {
        let table: &JniNativeInterface = &*(*env);
        let j_kind: JObject = (table.new_string_utf)(env, c_kind.as_ptr());
        let j_a1: JObject = (table.new_string_utf)(env, c_a1.as_ptr());
        let j_a2: JObject = (table.new_string_utf)(env, c_a2.as_ptr());
        let args: [JValue; 3] = [
            JValue { l: j_kind },
            JValue { l: j_a1 },
            JValue { l: j_a2 },
        ];
        let ok: u8 = (table.call_static_boolean_method_a)(env, cls_ptr, mid_ptr, args.as_ptr());
        if !j_kind.is_null() {
            (table.delete_local_ref)(env, j_kind);
        }
        if !j_a1.is_null() {
            (table.delete_local_ref)(env, j_a1);
        }
        if !j_a2.is_null() {
            (table.delete_local_ref)(env, j_a2);
        }
        ok != 0
    }
}

fn android_native_glyph_rasterizer(key: GlyphKey) -> Option<RasterizedGlyph> {
    let env_ptr: *mut c_void = ACTIVE_JNI_ENV.load(Ordering::Acquire);
    let cls_ptr: JClass = BRIDGE_CLASS_REF.load(Ordering::Acquire);
    let mid_ptr: JMethodId = RASTERIZE_METHOD.load(Ordering::Acquire);
    if env_ptr.is_null() || cls_ptr.is_null() || mid_ptr.is_null() {
        return None;
    }
    let env: JniEnv = env_ptr.cast();
    let size_px: f32 = ((key.get_size_tenths() as f32) * 0.1).max(6.0);
    let mut style_flags: i32 = 0;
    if key.get_bold() {
        style_flags |= 1;
    }
    if key.get_monospace() {
        style_flags |= 2;
    }
    if key.get_italic() {
        style_flags |= 4;
    }
    let args: [JValue; 3] = [
        JValue { i: key.get_codepoint() as i32 },
        JValue { f: size_px },
        JValue { i: style_flags },
    ];
    unsafe {
        let table: &JniNativeInterface = &*(*env);
        let arr: JByteArray = (table.call_static_object_method_a)(env, cls_ptr, mid_ptr, args.as_ptr());
        if arr.is_null() {
            return None;
        }
        let len: i32 = (table.get_array_length)(env, arr);
        if len < 6 {
            (table.delete_local_ref)(env, arr);
            return None;
        }
        let mut buf: Vec<u8> = vec![0; len as usize];
        (table.get_byte_array_region)(env, arr, 0, len, buf.as_mut_ptr().cast::<i8>());
        (table.delete_local_ref)(env, arr);
        let w: u32 = u16::from_le_bytes([buf[0], buf[1]]) as u32;
        let h: u32 = u16::from_le_bytes([buf[2], buf[3]]) as u32;
        let adv: u32 = u16::from_le_bytes([buf[4], buf[5]]) as u32;
        let expected: usize = (w as usize).saturating_mul(h as usize);
        if w == 0 || h == 0 || buf.len() < 6 + expected {
            return None;
        }
        Some(RasterizedGlyph::new(w, h, adv.max(1), buf[6..6 + expected].to_vec()))
    }
}

pub struct EuvAndroidHandle {
    app: App,
    _state: ShowcaseState,
    wid: WindowId,
    native_window: *mut ANativeWindow,
    surface_w: u32,
    surface_h: u32,
    argb: Vec<i32>,
    width: u32,
    height: u32,
    last_step_us: u64,
    last_blit_us: u64,
}

#[repr(C)]
pub struct ANativeWindow {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ANativeWindowBuffer {
    pub width: i32,
    pub height: i32,
    pub stride: i32,
    pub format: i32,
    pub bits: *mut c_void,
    pub reserved: [u32; 6],
}

#[repr(C)]
pub struct ARect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

const WINDOW_FORMAT_RGBA_8888: i32 = 1;

#[link(name = "android")]
unsafe extern "C" {
    fn ANativeWindow_fromSurface(env: JniEnv, surface: JObject) -> *mut ANativeWindow;
    fn ANativeWindow_release(window: *mut ANativeWindow);
    fn ANativeWindow_setBuffersGeometry(
        window: *mut ANativeWindow,
        width: i32,
        height: i32,
        format: i32,
    ) -> i32;
    fn ANativeWindow_lock(
        window: *mut ANativeWindow,
        out_buffer: *mut ANativeWindowBuffer,
        in_out_dirty_bounds: *mut ARect,
    ) -> i32;
    fn ANativeWindow_unlockAndPost(window: *mut ANativeWindow) -> i32;
}

impl Drop for EuvAndroidHandle {
    fn drop(&mut self) {
        if !self.native_window.is_null() {
            unsafe { ANativeWindow_release(self.native_window) };
            self.native_window = std::ptr::null_mut();
        }
    }
}

fn copy_rgba_rect_to_native_window(
    src: &[u8],
    w: usize,
    h: usize,
    buf: &ANativeWindowBuffer,
    dirty: Option<ARect>,
) {
    if buf.bits.is_null() || buf.width <= 0 || buf.height <= 0 || buf.stride <= 0 {
        return;
    }
    let copy_w: usize = w.min(buf.width as usize);
    let copy_h: usize = h.min(buf.height as usize);
    let src_stride: usize = w * 4;
    let dst_stride: usize = (buf.stride as usize) * 4;
    if copy_w == 0 || copy_h == 0 || src.len() < copy_h * src_stride {
        return;
    }
    let (x0, y0, x1, y1): (usize, usize, usize, usize) = match dirty {
        Some(r) => (
            (r.left.max(0) as usize).min(copy_w),
            (r.top.max(0) as usize).min(copy_h),
            (r.right.max(0) as usize).min(copy_w),
            (r.bottom.max(0) as usize).min(copy_h),
        ),
        None => (0, 0, copy_w, copy_h),
    };
    if x0 >= x1 || y0 >= y1 {
        return;
    }
    let dst_ptr: *mut u8 = buf.bits.cast::<u8>();
    let row_bytes: usize = (x1 - x0) * 4;
    if x0 == 0 && x1 == copy_w && src_stride == dst_stride && copy_w == w {
        let start_off: usize = y0 * src_stride;
        let total_bytes: usize = (y1 - y0) * row_bytes;
        unsafe {
            std::ptr::copy_nonoverlapping(
                src.as_ptr().add(start_off),
                dst_ptr.add(start_off),
                total_bytes,
            );
        }
        return;
    }
    let x_byte: usize = x0 * 4;
    for y in y0..y1 {
        unsafe {
            std::ptr::copy_nonoverlapping(
                src.as_ptr().add(y * src_stride + x_byte),
                dst_ptr.add(y * dst_stride + x_byte),
                row_bytes,
            );
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeCreate(
    env: JniEnv,
    cls: JClass,
    phys_width: i32,
    phys_height: i32,
    scale: f32,
) -> i64 {
    if !env.is_null() && !cls.is_null() {
        unsafe {
            let table: &JniNativeInterface = &*(*env);
            if BRIDGE_CLASS_REF.load(Ordering::Acquire).is_null() {
                let global_cls: JClass = (table.new_global_ref)(env, cls);
                let mid: JMethodId = (table.get_static_method_id)(
                    env,
                    global_cls,
                    c"rasterizeGlyph".as_ptr(),
                    c"(IFI)[B".as_ptr(),
                );
                let act_mid: JMethodId = (table.get_static_method_id)(
                    env,
                    global_cls,
                    c"invokeNativeAction".as_ptr(),
                    c"(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Z".as_ptr(),
                );
                if !global_cls.is_null() {
                    BRIDGE_CLASS_REF.store(global_cls, Ordering::Release);
                    if !mid.is_null() {
                        RASTERIZE_METHOD.store(mid, Ordering::Release);
                        register_external_glyph_rasterizer(android_native_glyph_rasterizer);
                    }
                    if !act_mid.is_null() {
                        ACTION_METHOD.store(act_mid, Ordering::Release);
                        register_native_action_bridge(android_native_action_bridge);
                    }
                }
            }
        }
    }
    ACTIVE_JNI_ENV.store(env.cast(), Ordering::Release);
    let Ok((mut app, state)) = create_showcase_app_for_platform(PlatformKind::Android, RenderMode::Software) else {
        ACTIVE_JNI_ENV.store(std::ptr::null_mut(), Ordering::Release);
        return 0;
    };
    app.get_mut_renderer()
        .get_mut_software()
        .set_font_provider(NativeSystemFont::system());
    let wid: WindowId = app.get_window_state().get_id();
    let pw: u32 = phys_width.max(200) as u32;
    let ph: u32 = phys_height.max(200) as u32;
    let s: f32 = scale.clamp(1.0, 4.5);
    app.set_exact_physical_viewport(pw, ph, s);
    let _ = app.step_game_frame(1.0 / 60.0);
    ACTIVE_JNI_ENV.store(std::ptr::null_mut(), Ordering::Release);
    let handle = Box::new(EuvAndroidHandle {
        app,
        _state: state,
        wid,
        native_window: std::ptr::null_mut(),
        surface_w: 0,
        surface_h: 0,
        argb: Vec::new(),
        width: 0,
        height: 0,
        last_step_us: 0,
        last_blit_us: 0,
    });
    Box::into_raw(handle) as i64
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeSetSurface(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    surface: JObject,
    phys_width: i32,
    phys_height: i32,
    scale: f32,
) {
    if ptr == 0 {
        return;
    }
    ACTIVE_JNI_ENV.store(env.cast(), Ordering::Release);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    if !h.native_window.is_null() {
        unsafe { ANativeWindow_release(h.native_window) };
        h.native_window = std::ptr::null_mut();
        h.surface_w = 0;
        h.surface_h = 0;
    }
    if !surface.is_null() && phys_width > 0 && phys_height > 0 {
        let win: *mut ANativeWindow = unsafe { ANativeWindow_fromSurface(env, surface) };
        if !win.is_null() {
            let pw: u32 = phys_width.max(200) as u32;
            let ph: u32 = phys_height.max(200) as u32;
            let s: f32 = scale.clamp(1.0, 4.5);
            unsafe {
                let _ = ANativeWindow_setBuffersGeometry(
                    win,
                    pw as i32,
                    ph as i32,
                    WINDOW_FORMAT_RGBA_8888,
                );
            }
            h.native_window = win;
            h.surface_w = pw;
            h.surface_h = ph;
            h.app.set_exact_physical_viewport(pw, ph, s);
        }
    }
    ACTIVE_JNI_ENV.store(std::ptr::null_mut(), Ordering::Release);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeStepSurface(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    dt: f32,
    force_present: i32,
) -> i32 {
    if ptr == 0 {
        return 0;
    }
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    let t_step = std::time::Instant::now();
    ACTIVE_JNI_ENV.store(env.cast(), Ordering::Release);
    let step_res = h.app.step_game_frame(dt.clamp(1.0 / 240.0, 0.05));
    ACTIVE_JNI_ENV.store(std::ptr::null_mut(), Ordering::Release);
    h.last_step_us = t_step.elapsed().as_micros() as u64;
    let Ok(stats) = step_res else {
        return 0;
    };
    let changed: bool = stats.get_pixels_shaded() > 0 || h.width == 0 || force_present != 0;
    if !changed || h.native_window.is_null() {
        h.last_blit_us = 0;
        return 0;
    }
    let t_blit = std::time::Instant::now();
    if let Some(fb) = h.app.get_renderer().framebuffer() {
        let w: u32 = fb.get_width();
        let ht: u32 = fb.get_height();
        h.width = w;
        h.height = ht;
        if w > 0 && ht > 0 {
            if h.surface_w != w || h.surface_h != ht {
                unsafe {
                    let _ = ANativeWindow_setBuffersGeometry(
                        h.native_window,
                        w as i32,
                        ht as i32,
                        WINDOW_FORMAT_RGBA_8888,
                    );
                }
                h.surface_w = w;
                h.surface_h = ht;
            }
            let mut buf: ANativeWindowBuffer = ANativeWindowBuffer {
                width: 0,
                height: 0,
                stride: 0,
                format: 0,
                bits: std::ptr::null_mut(),
                reserved: [0; 6],
            };
            let dmg = h.app.last_frame_damage();
            let mut dirty_box: ARect = ARect {
                left: 0,
                top: 0,
                right: w as i32,
                bottom: ht as i32,
            };
            let use_partial: bool = force_present == 0
                && !dmg.get_full_damage()
                && dmg.resolve(w, ht).map(|r| {
                    let rx0: i32 = (r.get_x().floor().max(0.0) as i32).clamp(0, w as i32);
                    let ry0: i32 = (r.get_y().floor().max(0.0) as i32).clamp(0, ht as i32);
                    let rx1: i32 = (r.right().ceil().max(0.0) as i32).clamp(rx0, w as i32);
                    let ry1: i32 = (r.bottom().ceil().max(0.0) as i32).clamp(ry0, ht as i32);
                    dirty_box = ARect {
                        left: rx0,
                        top: ry0,
                        right: rx1,
                        bottom: ry1,
                    };
                    (rx1 - rx0) > 0 && (ry1 - ry0) > 0 && ((rx1 - rx0) as u64) * ((ry1 - ry0) as u64) * 4 < (w as u64) * (ht as u64) * 3
                }).unwrap_or(false);
            let dirty_ptr: *mut ARect = if use_partial {
                &mut dirty_box
            } else {
                std::ptr::null_mut()
            };
            let lock_rc: i32 = unsafe {
                ANativeWindow_lock(h.native_window, &mut buf, dirty_ptr)
            };
            if lock_rc == 0 {
                copy_rgba_rect_to_native_window(
                    fb.get_pixels().as_slice(),
                    w as usize,
                    ht as usize,
                    &buf,
                    if use_partial { Some(dirty_box) } else { None },
                );
                unsafe {
                    let _ = ANativeWindow_unlockAndPost(h.native_window);
                }
            }
        }
    }
    h.last_blit_us = t_blit.elapsed().as_micros() as u64;
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeGetLastStepUs(
    _env: JniEnv,
    _cls: JObject,
    ptr: i64,
) -> i64 {
    if ptr == 0 {
        return 0;
    }
    let h: &EuvAndroidHandle = unsafe { &*(ptr as *const EuvAndroidHandle) };
    h.last_step_us as i64
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeGetLastBlitUs(
    _env: JniEnv,
    _cls: JObject,
    ptr: i64,
) -> i64 {
    if ptr == 0 {
        return 0;
    }
    let h: &EuvAndroidHandle = unsafe { &*(ptr as *const EuvAndroidHandle) };
    h.last_blit_us as i64
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeResize(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    phys_width: i32,
    phys_height: i32,
    scale: f32,
) {
    if ptr == 0 {
        return;
    }
    ACTIVE_JNI_ENV.store(env.cast(), Ordering::Release);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    let pw: u32 = phys_width.max(200) as u32;
    let ph: u32 = phys_height.max(200) as u32;
    let s: f32 = scale.clamp(1.0, 4.5);
    h.app.set_exact_physical_viewport(pw, ph, s);
    ACTIVE_JNI_ENV.store(std::ptr::null_mut(), Ordering::Release);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeSetSafeArea(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
) {
    if ptr == 0 {
        return;
    }
    ACTIVE_JNI_ENV.store(env.cast(), Ordering::Release);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app.dispatch_window_event(WindowEvent::SafeAreaChanged(
        h.wid,
        SafeAreaInsets::new(top.max(0.0), right.max(0.0), bottom.max(0.0), left.max(0.0)),
    ));
    ACTIVE_JNI_ENV.store(std::ptr::null_mut(), Ordering::Release);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeTouchDown(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    pointer_id: i32,
    x: f32,
    y: f32,
) {
    if ptr == 0 {
        return;
    }
    ACTIVE_JNI_ENV.store(env.cast(), Ordering::Release);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app.dispatch_window_event(WindowEvent::PointerDown(
        h.wid,
        PointerEvent::touch((pointer_id as u64).saturating_add(1), x, y),
    ));
    ACTIVE_JNI_ENV.store(std::ptr::null_mut(), Ordering::Release);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeTouchMove(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    pointer_id: i32,
    x: f32,
    y: f32,
) {
    if ptr == 0 {
        return;
    }
    ACTIVE_JNI_ENV.store(env.cast(), Ordering::Release);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app.dispatch_window_event(WindowEvent::PointerMove(
        h.wid,
        PointerEvent::touch((pointer_id as u64).saturating_add(1), x, y),
    ));
    ACTIVE_JNI_ENV.store(std::ptr::null_mut(), Ordering::Release);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeTouchUp(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    pointer_id: i32,
    x: f32,
    y: f32,
) {
    if ptr == 0 {
        return;
    }
    ACTIVE_JNI_ENV.store(env.cast(), Ordering::Release);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app.dispatch_window_event(WindowEvent::PointerUp(
        h.wid,
        PointerEvent::touch((pointer_id as u64).saturating_add(1), x, y),
    ));
    ACTIVE_JNI_ENV.store(std::ptr::null_mut(), Ordering::Release);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeTouchCancel(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    pointer_id: i32,
) {
    if ptr == 0 {
        return;
    }
    ACTIVE_JNI_ENV.store(env.cast(), Ordering::Release);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    let mapped_id: u64 = if pointer_id < 0 {
        0
    } else {
        (pointer_id as u64).saturating_add(1)
    };
    h.app
        .dispatch_window_event(WindowEvent::PointerCancel(h.wid, mapped_id));
    ACTIVE_JNI_ENV.store(std::ptr::null_mut(), Ordering::Release);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeTouchSyncMask(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    active_mask: i64,
) {
    if ptr == 0 {
        return;
    }
    ACTIVE_JNI_ENV.store(env.cast(), Ordering::Release);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    let bits: u64 = active_mask as u64;
    let mut live_ids: Vec<u64> = Vec::with_capacity(bits.count_ones() as usize);
    for bit in 0_u64..63_u64 {
        if (bits & (1_u64 << bit)) != 0 {
            live_ids.push(bit + 1);
        }
    }
    h.app.reconcile_active_touch_ids(&live_ids);
    ACTIVE_JNI_ENV.store(std::ptr::null_mut(), Ordering::Release);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeSystemBack(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
) {
    if ptr == 0 {
        return;
    }
    ACTIVE_JNI_ENV.store(env.cast(), Ordering::Release);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app.dispatch_window_event(WindowEvent::KeyDown(
        h.wid,
        KeyEvent::new(KeyCode::Back, Modifiers::default(), false),
    ));
    ACTIVE_JNI_ENV.store(std::ptr::null_mut(), Ordering::Release);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeTextInput(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    utf8_bytes: JByteArray,
) {
    if ptr == 0 || env.is_null() || utf8_bytes.is_null() {
        return;
    }
    ACTIVE_JNI_ENV.store(env.cast(), Ordering::Release);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    let text_opt: Option<String> = unsafe {
        let table: &JniNativeInterface = &*(*env);
        let len: i32 = (table.get_array_length)(env, utf8_bytes);
        if len > 0 {
            let mut buf: Vec<u8> = vec![0; len as usize];
            (table.get_byte_array_region)(env, utf8_bytes, 0, len, buf.as_mut_ptr().cast::<i8>());
            String::from_utf8(buf).ok()
        } else {
            None
        }
    };
    if let Some(text) = text_opt
        && !text.is_empty()
    {
        h.app
            .dispatch_window_event(WindowEvent::TextInput(h.wid, text));
    }
    ACTIVE_JNI_ENV.store(std::ptr::null_mut(), Ordering::Release);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeKeyDown(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    key_tag: i32,
) {
    if ptr == 0 {
        return;
    }
    let code: KeyCode = match key_tag {
        1 => KeyCode::Backspace,
        2 => KeyCode::Enter,
        3 => KeyCode::Tab,
        4 => KeyCode::Escape,
        _ => return,
    };
    ACTIVE_JNI_ENV.store(env.cast(), Ordering::Release);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app.dispatch_window_event(WindowEvent::KeyDown(
        h.wid,
        KeyEvent::new(code, Modifiers::default(), false),
    ));
    ACTIVE_JNI_ENV.store(std::ptr::null_mut(), Ordering::Release);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeCopyFramebufferDirect(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    out_rgba: JObject,
    max_bytes: i32,
) -> i32 {
    if ptr == 0 || env.is_null() || out_rgba.is_null() || max_bytes <= 0 {
        return 0;
    }
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    if let Some(fb) = h.app.get_renderer().framebuffer() {
        let w: u32 = fb.get_width();
        let ht: u32 = fb.get_height();
        let src: &[u8] = fb.get_pixels().as_slice();
        let byte_count: usize = (w as usize) * (ht as usize) * 4;
        h.width = w;
        h.height = ht;
        unsafe {
            let table: &JniNativeInterface = &*(*env);
            let dst_ptr: *mut c_void = (table.get_direct_buffer_address)(env, out_rgba);
            let cap: i64 = (table.get_direct_buffer_capacity)(env, out_rgba);
            if !dst_ptr.is_null() && cap >= byte_count as i64 && (max_bytes as usize) >= byte_count && src.len() >= byte_count {
                std::ptr::copy_nonoverlapping(src.as_ptr(), dst_ptr.cast::<u8>(), byte_count);
                return 1;
            }
        }
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeStepFrameDirect(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    dt: f32,
    out_rgba: JObject,
    max_bytes: i32,
) -> i32 {
    if ptr == 0 || env.is_null() || out_rgba.is_null() || max_bytes <= 0 {
        return 0;
    }
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    ACTIVE_JNI_ENV.store(env.cast(), Ordering::Release);
    let step_res = h.app.step_game_frame(dt.clamp(1.0 / 240.0, 0.05));
    ACTIVE_JNI_ENV.store(std::ptr::null_mut(), Ordering::Release);
    let Ok(stats) = step_res else {
        return 0;
    };
    let changed: bool = stats.get_pixels_shaded() > 0 || h.width == 0;
    if changed && let Some(fb) = h.app.get_renderer().framebuffer() {
        let w: u32 = fb.get_width();
        let ht: u32 = fb.get_height();
        let src: &[u8] = fb.get_pixels().as_slice();
        let byte_count: usize = (w as usize) * (ht as usize) * 4;
        h.width = w;
        h.height = ht;
        unsafe {
            let table: &JniNativeInterface = &*(*env);
            let dst_ptr: *mut c_void = (table.get_direct_buffer_address)(env, out_rgba);
            let cap: i64 = (table.get_direct_buffer_capacity)(env, out_rgba);
            if !dst_ptr.is_null() && cap > 0 {
                let copy_len: usize = byte_count
                    .min(src.len())
                    .min(max_bytes as usize)
                    .min(cap as usize);
                if copy_len > 0 {
                    std::ptr::copy_nonoverlapping(src.as_ptr(), dst_ptr.cast::<u8>(), copy_len);
                }
            }
        }
    }
    if changed { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeStepFrame(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    dt: f32,
    out_pixels: JIntArray,
    max_len: i32,
) -> i32 {
    if ptr == 0 || env.is_null() || out_pixels.is_null() {
        return 0;
    }
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    ACTIVE_JNI_ENV.store(env.cast(), Ordering::Release);
    let step_res = h.app.step_game_frame(dt.clamp(1.0 / 240.0, 0.05));
    ACTIVE_JNI_ENV.store(std::ptr::null_mut(), Ordering::Release);
    let Ok(stats) = step_res else {
        return 0;
    };
    let changed: bool = stats.get_pixels_shaded() > 0 || h.argb.is_empty();
    if changed && let Some(fb) = h.app.get_renderer().framebuffer() {
        let w: u32 = fb.get_width();
        let ht: u32 = fb.get_height();
        let src: &[u8] = fb.get_pixels().as_slice();
        let pixel_count: usize = (w as usize) * (ht as usize);
        h.width = w;
        h.height = ht;
        h.argb.resize(pixel_count, 0);
        for (idx, chunk) in src.chunks_exact(4).take(pixel_count).enumerate() {
            let r: u32 = chunk[0] as u32;
            let g: u32 = chunk[1] as u32;
            let b: u32 = chunk[2] as u32;
            let a: u32 = chunk[3] as u32;
            h.argb[idx] = ((a << 24) | (r << 16) | (g << 8) | b) as i32;
        }
        let copy_len: i32 = (pixel_count as i32).min(max_len);
        if copy_len > 0 {
            unsafe {
                let table: &JniNativeInterface = &*(*env);
                (table.set_int_array_region)(env, out_pixels, 0, copy_len, h.argb.as_ptr());
            }
        }
    }
    if changed { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeGetWidth(
    _env: JniEnv,
    _cls: JObject,
    ptr: i64,
) -> i32 {
    if ptr == 0 {
        return 0;
    }
    let h: &EuvAndroidHandle = unsafe { &*(ptr as *const EuvAndroidHandle) };
    h.width as i32
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeGetHeight(
    _env: JniEnv,
    _cls: JObject,
    ptr: i64,
) -> i32 {
    if ptr == 0 {
        return 0;
    }
    let h: &EuvAndroidHandle = unsafe { &*(ptr as *const EuvAndroidHandle) };
    h.height as i32
}
EOF

cat > "${BUILD_DIR}/java/vip/ltpp/euv/showcase/EuvNativeBridge.java" <<'EOF'
package vip.ltpp.euv.showcase;

import android.app.Activity;
import android.app.Notification;
import android.app.NotificationChannel;
import android.app.NotificationManager;
import android.content.Context;
import android.content.pm.PackageManager;
import android.graphics.Bitmap;
import android.graphics.Canvas;
import android.graphics.Color;
import android.graphics.Paint;
import android.graphics.Typeface;
import android.os.Build;
import android.os.VibrationEffect;
import android.os.Vibrator;
import android.util.Log;
import android.view.Surface;
import android.view.WindowInsets;
import android.view.WindowInsetsController;
import android.widget.Toast;
import java.lang.ref.WeakReference;
import java.nio.ByteBuffer;

public final class EuvNativeBridge {
    private static final Paint GLYPH_PAINT = new Paint(Paint.ANTI_ALIAS_FLAG | Paint.SUBPIXEL_TEXT_FLAG);
    private static final Typeface FACE_REGULAR = Typeface.create(Typeface.SANS_SERIF, Typeface.NORMAL);
    private static final Typeface FACE_BOLD = Typeface.create(Typeface.SANS_SERIF, Typeface.BOLD);
    private static final Typeface FACE_ITALIC = Typeface.create(Typeface.SANS_SERIF, Typeface.ITALIC);
    private static final Typeface FACE_BOLD_ITALIC = Typeface.create(Typeface.SANS_SERIF, Typeface.BOLD_ITALIC);
    private static final Typeface FACE_MONO_REGULAR = Typeface.create(Typeface.MONOSPACE, Typeface.NORMAL);
    private static final Typeface FACE_MONO_BOLD = Typeface.create(Typeface.MONOSPACE, Typeface.BOLD);
    private static final Typeface FACE_MONO_ITALIC = Typeface.create(Typeface.MONOSPACE, Typeface.ITALIC);
    private static final Typeface FACE_MONO_BOLD_ITALIC = Typeface.create(Typeface.MONOSPACE, Typeface.BOLD_ITALIC);
    private static final String CHANNEL_ID = "euv_native_alerts";
    private static WeakReference<Activity> hostActivity = new WeakReference<>(null);
    private static int notifSeq = 100;

    static {
        System.loadLibrary("euv_android_host");
    }

    public static void setHostActivity(Activity activity) {
        hostActivity = new WeakReference<>(activity);
    }

    public static boolean invokeNativeAction(String kind, String arg1, String arg2) {
        Activity act = hostActivity.get();
        if (act == null || act.isFinishing()) {
            return false;
        }
        String k = (kind != null) ? kind : "";
        String a1 = (arg1 != null) ? arg1 : "";
        String a2 = (arg2 != null) ? arg2 : "";
        Log.i("euv-native", "invokeNativeAction kind=" + k + " arg1=" + a1 + " arg2=" + a2);
        act.runOnUiThread(() -> {
            try {
                switch (k) {
                    case "ime": {
                        if (act instanceof EuvActivity) {
                            ((EuvActivity) act).setSoftKeyboardVisible("show".equals(a1));
                        }
                        break;
                    }
                    case "haptic": {
                        Vibrator vib = (Vibrator) act.getSystemService(Context.VIBRATOR_SERVICE);
                        if (vib != null && vib.hasVibrator()) {
                            vib.vibrate(VibrationEffect.createOneShot(45, VibrationEffect.DEFAULT_AMPLITUDE));
                        }
                        Toast.makeText(act, "Native Haptic + A11y Dispatched", Toast.LENGTH_SHORT).show();
                        break;
                    }
                    case "notify":
                    case "island": {
                        NotificationManager nm = (NotificationManager) act.getSystemService(Context.NOTIFICATION_SERVICE);
                        if ("island".equals(k) && "stop".equals(a1)) {
                            if (nm != null) {
                                nm.cancel(99);
                            }
                            Toast.makeText(act, "Dynamic Island Live Activity Stopped", Toast.LENGTH_SHORT).show();
                            break;
                        }
                        if (Build.VERSION.SDK_INT >= 33
                            && act.checkSelfPermission("android.permission.POST_NOTIFICATIONS") != PackageManager.PERMISSION_GRANTED) {
                            act.requestPermissions(new String[]{"android.permission.POST_NOTIFICATIONS"}, 201);
                        }
                        if (nm != null) {
                            NotificationChannel ch = new NotificationChannel(
                                CHANNEL_ID,
                                "Euv Native Alerts",
                                NotificationManager.IMPORTANCE_HIGH
                            );
                            nm.createNotificationChannel(ch);
                            Notification.Builder b = new Notification.Builder(act, CHANNEL_ID)
                                .setSmallIcon(android.R.drawable.ic_dialog_info)
                                .setContentTitle(a1.isEmpty() ? "Euv Showcase" : a1)
                                .setContentText(a2.isEmpty() ? "Native notification dispatched" : a2)
                                .setAutoCancel(!"island".equals(k))
                                .setOngoing("island".equals(k));
                            int nid = "island".equals(k) ? 99 : (++notifSeq);
                            nm.notify(nid, b.build());
                        }
                        Toast.makeText(act, (a1.isEmpty() ? "Notification" : a1) + ": " + a2, Toast.LENGTH_SHORT).show();
                        break;
                    }
                    case "permission": {
                        if (a1.startsWith("android.permission.")) {
                            if (act.checkSelfPermission(a1) == PackageManager.PERMISSION_GRANTED) {
                                Toast.makeText(act, "Already Granted: " + a1, Toast.LENGTH_SHORT).show();
                            } else {
                                act.requestPermissions(new String[]{a1}, 202);
                            }
                        } else {
                            Toast.makeText(act, "Native Entitlement Checked: " + a1, Toast.LENGTH_SHORT).show();
                        }
                        break;
                    }
                    case "fullscreen": {
                        WindowInsetsController ctrl = act.getWindow().getInsetsController();
                        if (ctrl != null) {
                            if ("on".equals(a1)) {
                                ctrl.hide(WindowInsets.Type.systemBars());
                                ctrl.setSystemBarsBehavior(WindowInsetsController.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE);
                            } else {
                                ctrl.show(WindowInsets.Type.systemBars());
                            }
                        }
                        Toast.makeText(act, "Fullscreen: " + a1, Toast.LENGTH_SHORT).show();
                        break;
                    }
                    case "protocol": {
                        Toast.makeText(act, "DeepLink Scheme Ready: " + a1 + ":// (" + a2 + ")", Toast.LENGTH_SHORT).show();
                        break;
                    }
                    case "toast": {
                        String msg = a1.isEmpty() ? "Press back again to exit" : a1;
                        Toast.makeText(act, msg, Toast.LENGTH_SHORT).show();
                        break;
                    }
                    case "exit": {
                        act.finish();
                        break;
                    }
                    default: {
                        Toast.makeText(act, "Native Action: " + k + " " + a1, Toast.LENGTH_SHORT).show();
                        break;
                    }
                }
            } catch (Throwable t) {
                Log.w("euv-native", "Action failed: " + t.getMessage());
            }
        });
        return true;
    }

    public static byte[] rasterizeGlyph(int codepoint, float sizePx, int styleFlags) {
        String text = new String(Character.toChars(codepoint));
        boolean bold = (styleFlags & 1) != 0;
        boolean mono = (styleFlags & 2) != 0;
        boolean italic = (styleFlags & 4) != 0;
        Typeface face;
        if (mono) {
            face = bold ? (italic ? FACE_MONO_BOLD_ITALIC : FACE_MONO_BOLD)
                        : (italic ? FACE_MONO_ITALIC : FACE_MONO_REGULAR);
        } else {
            face = bold ? (italic ? FACE_BOLD_ITALIC : FACE_BOLD)
                        : (italic ? FACE_ITALIC : FACE_REGULAR);
        }
        GLYPH_PAINT.setTypeface(face);
        GLYPH_PAINT.setTextSize(Math.max(6.0f, sizePx));
        GLYPH_PAINT.setColor(Color.WHITE);
        Paint.FontMetrics fm = GLYPH_PAINT.getFontMetrics();
        float advF = GLYPH_PAINT.measureText(text);
        int adv = Math.max(2, (int) Math.ceil(advF));
        int w = Math.max(4, adv + 4);
        int h = Math.max(8, (int) Math.ceil(fm.descent - fm.ascent) + 4);
        Bitmap bmp = Bitmap.createBitmap(w, h, Bitmap.Config.ALPHA_8);
        Canvas canvas = new Canvas(bmp);
        canvas.drawText(text, 1.0f, -fm.ascent + 2.0f, GLYPH_PAINT);
        ByteBuffer alpha = ByteBuffer.allocate(w * h);
        bmp.copyPixelsToBuffer(alpha);
        bmp.recycle();
        byte[] out = new byte[6 + w * h];
        out[0] = (byte) (w & 0xFF);
        out[1] = (byte) ((w >> 8) & 0xFF);
        out[2] = (byte) (h & 0xFF);
        out[3] = (byte) ((h >> 8) & 0xFF);
        out[4] = (byte) (adv & 0xFF);
        out[5] = (byte) ((adv >> 8) & 0xFF);
        System.arraycopy(alpha.array(), 0, out, 6, w * h);
        return out;
    }

    public static native long nativeCreate(int physWidth, int physHeight, float scale);
    public static native void nativeResize(long handle, int physWidth, int physHeight, float scale);
    public static native void nativeSetSurface(long handle, Surface surface, int physWidth, int physHeight, float scale);
    public static native int nativeStepSurface(long handle, float dt, int forcePresent);
    public static native long nativeGetLastStepUs(long handle);
    public static native long nativeGetLastBlitUs(long handle);
    public static native void nativeSetSafeArea(long handle, float top, float right, float bottom, float left);
    public static native void nativeTouchDown(long handle, int pointerId, float x, float y);
    public static native void nativeTouchMove(long handle, int pointerId, float x, float y);
    public static native void nativeTouchUp(long handle, int pointerId, float x, float y);
    public static native void nativeTouchCancel(long handle, int pointerId);
    public static native void nativeTouchSyncMask(long handle, long activeMask);
    public static native void nativeSystemBack(long handle);
    public static native void nativeTextInput(long handle, byte[] utf8Bytes);
    public static native void nativeKeyDown(long handle, int keyTag);
    public static native int nativeCopyFramebufferDirect(long handle, ByteBuffer outRgba, int maxBytes);
    public static native int nativeStepFrameDirect(long handle, float dt, ByteBuffer outRgba, int maxBytes);
    public static native int nativeStepFrame(long handle, float dt, int[] outPixels, int maxLen);
    public static native int nativeGetWidth(long handle);
    public static native int nativeGetHeight(long handle);
}
EOF

cat > "${BUILD_DIR}/java/vip/ltpp/euv/showcase/EuvActivity.java" <<'EOF'
package vip.ltpp.euv.showcase;

import android.app.Activity;
import android.content.Context;
import android.content.res.Configuration;
import android.graphics.Color;
import android.graphics.Insets;
import android.graphics.PixelFormat;
import android.os.Build;
import android.os.Bundle;
import android.os.Handler;
import android.os.HandlerThread;
import android.os.Process;
import android.text.InputType;
import android.util.Log;
import android.view.Choreographer;
import android.view.Display;
import android.view.KeyEvent;
import android.view.MotionEvent;
import android.view.Surface;
import android.view.SurfaceHolder;
import android.view.SurfaceView;
import android.view.Window;
import android.view.WindowInsets;
import android.view.WindowInsetsController;
import android.view.WindowManager;
import android.view.inputmethod.BaseInputConnection;
import android.view.inputmethod.EditorInfo;
import android.view.inputmethod.InputConnection;
import android.view.inputmethod.InputMethodManager;
import java.nio.charset.StandardCharsets;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;

public final class EuvActivity extends Activity {
    private EuvCanvasView canvasView;
    private float targetRefreshHz = 60.0f;

    private void selectHighestRefreshMode(Window win) {
        try {
            Display display = getDisplay();
            if (display == null) return;
            Display.Mode current = display.getMode();
            Display.Mode best = current;
            for (Display.Mode m : display.getSupportedModes()) {
                if (m.getPhysicalWidth() == current.getPhysicalWidth()
                    && m.getPhysicalHeight() == current.getPhysicalHeight()
                    && m.getRefreshRate() > best.getRefreshRate()) {
                    best = m;
                }
            }
            targetRefreshHz = Math.max(60.0f, best.getRefreshRate());
            WindowManager.LayoutParams lp = win.getAttributes();
            lp.preferredDisplayModeId = best.getModeId();
            win.setAttributes(lp);
        } catch (Throwable t) {
            Log.w("euv-fps", "refresh mode hint skipped: " + t.getMessage());
        }
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        EuvNativeBridge.setHostActivity(this);
        Window win = getWindow();
        win.setDecorFitsSystemWindows(false);
        win.setStatusBarColor(Color.TRANSPARENT);
        win.setNavigationBarColor(Color.TRANSPARENT);
        WindowManager.LayoutParams lp = win.getAttributes();
        lp.layoutInDisplayCutoutMode = WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES;
        win.setAttributes(lp);
        selectHighestRefreshMode(win);
        canvasView = new EuvCanvasView(this, targetRefreshHz);
        setContentView(canvasView);
        if (Build.VERSION.SDK_INT >= 33) {
            getOnBackInvokedDispatcher().registerOnBackInvokedCallback(
                android.window.OnBackInvokedDispatcher.PRIORITY_DEFAULT,
                () -> {
                    if (canvasView != null) {
                        canvasView.dispatchSystemBack();
                    }
                }
            );
        }
        canvasView.post(() -> {
            WindowInsetsController ctrl = win.getInsetsController();
            if (ctrl != null) {
                ctrl.setSystemBarsAppearance(
                    WindowInsetsController.APPEARANCE_LIGHT_STATUS_BARS | WindowInsetsController.APPEARANCE_LIGHT_NAVIGATION_BARS,
                    WindowInsetsController.APPEARANCE_LIGHT_STATUS_BARS | WindowInsetsController.APPEARANCE_LIGHT_NAVIGATION_BARS
                );
            }
        });
    }

    @Override
    protected void onDestroy() {
        EuvNativeBridge.setHostActivity(null);
        if (canvasView != null) {
            canvasView.shutdown();
        }
        super.onDestroy();
    }

    public void setSoftKeyboardVisible(boolean show) {
        if (canvasView != null) {
            canvasView.setSoftKeyboardVisible(show);
        }
    }

    @Override
    @SuppressWarnings("deprecation")
    public void onBackPressed() {
        if (canvasView != null && canvasView.dispatchSystemBack()) {
            return;
        }
        super.onBackPressed();
    }

    @Override
    public void onConfigurationChanged(Configuration newConfig) {
        super.onConfigurationChanged(newConfig);
        if (canvasView != null) {
            canvasView.syncDisplayScaleAndResize();
        }
    }

    @Override
    protected void onResume() {
        super.onResume();
        EuvNativeBridge.setHostActivity(this);
        if (canvasView != null) {
            canvasView.startLoop();
        }
    }

    @Override
    protected void onPause() {
        super.onPause();
        if (canvasView != null) {
            canvasView.cancelAllTouches();
            canvasView.stopLoop();
        }
    }

    @Override
    public void onWindowFocusChanged(boolean hasFocus) {
        super.onWindowFocusChanged(hasFocus);
        if (!hasFocus && canvasView != null) {
            canvasView.cancelAllTouches();
        }
    }

    private static final class EuvCanvasView extends SurfaceView implements SurfaceHolder.Callback {
        private final HandlerThread renderThread;
        private final Handler renderHandler;
        private final Object moveLock = new Object();
        private boolean hasPendingMove = false;
        private int pendingMoveCount = 0;
        private final int[] pendingMoveIds = new int[16];
        private final float[] pendingMoveXs = new float[16];
        private final float[] pendingMoveYs = new float[16];
        private long pendingMoveMask = 0L;
        private long handle = 0;
        private volatile float displayDensity = 2.0f;
        private volatile float renderScale = 2.0f;
        private final float targetFps;
        private boolean surfaceReady = false;
        private int surfaceW = 0;
        private int surfaceH = 0;
        private long lastNanos = 0;
        private boolean running = false;
        private long renderedFrames = 0;
        private double accumTotalMs = 0.0;
        private double accumStepMs = 0.0;
        private double accumBlitMs = 0.0;
        private double maxRenderMs = 0.0;
        private long droppedFrames = 0;
        private final Choreographer.FrameCallback renderFrameCallback = new Choreographer.FrameCallback() {
            @Override
            public void doFrame(long frameTimeNanos) {
                if (!running) return;
                float dt = (lastNanos > 0) ? ((frameTimeNanos - lastNanos) / 1_000_000_000.0f) : (1.0f / targetFps);
                lastNanos = frameTimeNanos;
                flushPendingMovesOnRenderThread();
                stepAndPresentOnRenderThread(dt, false);
                Choreographer.getInstance().postFrameCallback(this);
            }
        };

        EuvCanvasView(Context ctx, float preferredHz) {
            super(ctx);
            targetFps = Math.max(60.0f, preferredHz);
            refreshDensity(ctx);
            renderThread = new HandlerThread("euv-render", Process.THREAD_PRIORITY_DISPLAY);
            renderThread.start();
            renderHandler = new Handler(renderThread.getLooper());
            getHolder().setFormat(PixelFormat.RGBA_8888);
            getHolder().addCallback(this);
            setFocusable(true);
            setFocusableInTouchMode(true);
            setOnApplyWindowInsetsListener((v, insets) -> {
                Insets bars = insets.getInsets(WindowInsets.Type.systemBars() | WindowInsets.Type.displayCutout());
                Insets ime = insets.getInsets(WindowInsets.Type.ime());
                float d = displayDensity;
                float top = bars.top / d;
                float right = bars.right / d;
                float bottom = Math.max(bars.bottom, ime.bottom) / d;
                float left = bars.left / d;
                renderHandler.post(() -> {
                    if (handle != 0) {
                        EuvNativeBridge.nativeSetSafeArea(handle, top, right, bottom, left);
                    }
                });
                return insets;
            });
        }

        private void refreshDensity(Context ctx) {
            displayDensity = Math.max(1.0f, ctx.getResources().getDisplayMetrics().density);
            renderScale = Math.max(1.0f, Math.min(4.5f, displayDensity));
        }

        private void applySurfaceFrameRate(Surface surface) {
            if (surface != null && Build.VERSION.SDK_INT >= 30) {
                try {
                    surface.setFrameRate(targetFps, Surface.FRAME_RATE_COMPATIBILITY_DEFAULT);
                } catch (Throwable ignored) {
                }
            }
        }

        @Override
        public boolean onCheckIsTextEditor() {
            return true;
        }

        @Override
        public InputConnection onCreateInputConnection(EditorInfo outAttrs) {
            outAttrs.inputType = InputType.TYPE_CLASS_TEXT | InputType.TYPE_TEXT_FLAG_NO_SUGGESTIONS;
            outAttrs.imeOptions = EditorInfo.IME_FLAG_NO_EXTRACT_UI | EditorInfo.IME_ACTION_DONE;
            return new BaseInputConnection(this, false) {
                @Override
                public boolean commitText(CharSequence text, int newCursorPosition) {
                    if (text != null && text.length() > 0) {
                        dispatchTextInput(text.toString());
                    }
                    return true;
                }

                @Override
                public boolean deleteSurroundingText(int beforeLength, int afterLength) {
                    int count = Math.max(1, Math.min(32, beforeLength));
                    for (int i = 0; i < count; i++) {
                        dispatchKeyTag(1);
                    }
                    return true;
                }

                @Override
                public boolean sendKeyEvent(KeyEvent event) {
                    if (event != null && event.getAction() == KeyEvent.ACTION_DOWN) {
                        if (handleHardwareKeyDown(event.getKeyCode(), event)) {
                            return true;
                        }
                    }
                    return super.sendKeyEvent(event);
                }
            };
        }

        @Override
        public boolean onKeyDown(int keyCode, KeyEvent event) {
            if (handleHardwareKeyDown(keyCode, event)) {
                return true;
            }
            return super.onKeyDown(keyCode, event);
        }

        private boolean handleHardwareKeyDown(int keyCode, KeyEvent event) {
            if (keyCode == KeyEvent.KEYCODE_DEL) {
                dispatchKeyTag(1);
                return true;
            }
            if (keyCode == KeyEvent.KEYCODE_ENTER || keyCode == KeyEvent.KEYCODE_NUMPAD_ENTER) {
                dispatchKeyTag(2);
                return true;
            }
            if (keyCode == KeyEvent.KEYCODE_TAB) {
                dispatchKeyTag(3);
                return true;
            }
            if (keyCode == KeyEvent.KEYCODE_ESCAPE) {
                dispatchKeyTag(4);
                return true;
            }
            if (event != null) {
                int uni = event.getUnicodeChar();
                if (uni >= 32) {
                    dispatchTextInput(new String(Character.toChars(uni)));
                    return true;
                }
            }
            return false;
        }

        void setSoftKeyboardVisible(boolean show) {
            InputMethodManager imm = (InputMethodManager) getContext().getSystemService(Context.INPUT_METHOD_SERVICE);
            if (imm == null) return;
            if (show) {
                requestFocus();
                imm.restartInput(this);
                boolean shown = imm.showSoftInput(this, InputMethodManager.SHOW_IMPLICIT);
                if (!shown) {
                    postDelayed(() -> {
                        requestFocus();
                        imm.showSoftInput(this, InputMethodManager.SHOW_IMPLICIT);
                    }, 40L);
                }
            } else {
                imm.hideSoftInputFromWindow(getWindowToken(), 0);
            }
        }

        private void dispatchTextInput(String text) {
            byte[] utf8 = text.getBytes(StandardCharsets.UTF_8);
            if (utf8.length == 0) return;
            renderHandler.post(() -> {
                if (handle != 0) {
                    EuvNativeBridge.nativeTextInput(handle, utf8);
                    stepAndPresentOnRenderThread(1.0f / targetFps, false);
                }
            });
        }

        private void dispatchKeyTag(int keyTag) {
            renderHandler.post(() -> {
                if (handle != 0) {
                    EuvNativeBridge.nativeKeyDown(handle, keyTag);
                    stepAndPresentOnRenderThread(1.0f / targetFps, false);
                }
            });
        }

        @Override
        public void surfaceCreated(SurfaceHolder holder) {
            applySurfaceFrameRate(holder.getSurface());
        }

        @Override
        public void surfaceChanged(SurfaceHolder holder, int format, int width, int height) {
            if (width <= 0 || height <= 0) return;
            refreshDensity(getContext());
            surfaceW = width;
            surfaceH = height;
            Surface surface = holder.getSurface();
            applySurfaceFrameRate(surface);
            float scale = renderScale;
            renderHandler.post(() -> {
                if (handle == 0) {
                    handle = EuvNativeBridge.nativeCreate(width, height, scale);
                }
                EuvNativeBridge.nativeSetSurface(handle, surface, width, height, scale);
                surfaceReady = true;
                post(this::requestApplyInsets);
                stepAndPresentOnRenderThread(1.0f / targetFps, true);
            });
        }

        @Override
        public void surfaceDestroyed(SurfaceHolder holder) {
            CountDownLatch latch = new CountDownLatch(1);
            float scale = renderScale;
            renderHandler.post(() -> {
                surfaceReady = false;
                if (handle != 0) {
                    EuvNativeBridge.nativeSetSurface(handle, null, 0, 0, scale);
                }
                latch.countDown();
            });
            try {
                latch.await(500L, TimeUnit.MILLISECONDS);
            } catch (InterruptedException ignored) {
                Thread.currentThread().interrupt();
            }
        }

        void syncDisplayScaleAndResize() {
            refreshDensity(getContext());
            int w = surfaceW > 0 ? surfaceW : getWidth();
            int h = surfaceH > 0 ? surfaceH : getHeight();
            Surface surface = getHolder().getSurface();
            float scale = renderScale;
            if (w > 0 && h > 0 && surface != null) {
                renderHandler.post(() -> {
                    if (handle != 0 && surfaceReady) {
                        EuvNativeBridge.nativeSetSurface(handle, surface, w, h, scale);
                        post(this::requestApplyInsets);
                        stepAndPresentOnRenderThread(1.0f / targetFps, true);
                    }
                });
            }
        }

        boolean dispatchSystemBack() {
            renderHandler.post(() -> {
                if (handle != 0) {
                    EuvNativeBridge.nativeSystemBack(handle);
                    stepAndPresentOnRenderThread(1.0f / targetFps, false);
                }
            });
            return true;
        }

        void startLoop() {
            renderHandler.post(() -> {
                if (!running) {
                    running = true;
                    lastNanos = 0;
                    Choreographer.getInstance().postFrameCallback(renderFrameCallback);
                }
            });
        }

        void stopLoop() {
            renderHandler.post(() -> {
                running = false;
                Choreographer.getInstance().removeFrameCallback(renderFrameCallback);
            });
        }

        void shutdown() {
            stopLoop();
            renderThread.quitSafely();
        }

        private void flushPendingMovesOnRenderThread() {
            int count;
            int[] ids = null;
            float[] xs = null;
            float[] ys = null;
            long mask;
            synchronized (moveLock) {
                if (!hasPendingMove) return;
                hasPendingMove = false;
                count = pendingMoveCount;
                mask = pendingMoveMask;
                if (count > 0) {
                    ids = new int[count];
                    xs = new float[count];
                    ys = new float[count];
                    System.arraycopy(pendingMoveIds, 0, ids, 0, count);
                    System.arraycopy(pendingMoveXs, 0, xs, 0, count);
                    System.arraycopy(pendingMoveYs, 0, ys, 0, count);
                }
            }
            if (handle == 0 || count <= 0) return;
            for (int i = 0; i < count; i++) {
                EuvNativeBridge.nativeTouchMove(handle, ids[i], xs[i], ys[i]);
            }
            EuvNativeBridge.nativeTouchSyncMask(handle, mask);
        }

        private void stepAndPresentOnRenderThread(float dt, boolean force) {
            if (handle == 0 || !surfaceReady) return;
            long t0 = System.nanoTime();
            int presented = EuvNativeBridge.nativeStepSurface(handle, dt, force ? 1 : 0);
            if (presented != 0) {
                double totalMs = (System.nanoTime() - t0) / 1_000_000.0;
                double stepMs = EuvNativeBridge.nativeGetLastStepUs(handle) / 1000.0;
                double blitMs = EuvNativeBridge.nativeGetLastBlitUs(handle) / 1000.0;
                renderedFrames += 1;
                accumTotalMs += totalMs;
                accumStepMs += stepMs;
                accumBlitMs += blitMs;
                if (totalMs > maxRenderMs) maxRenderMs = totalMs;
                double budgetMs = 1000.0 / targetFps;
                if (totalMs > budgetMs) droppedFrames += 1;
                if (renderedFrames % 60 == 0) {
                    int bw = EuvNativeBridge.nativeGetWidth(handle);
                    int bh = EuvNativeBridge.nativeGetHeight(handle);
                    Log.i("euv-fps", String.format(
                        "android rendered=%d avg=%.2fms (step=%.2fms blit=%.2fms) max=%.2fms dropped=%d target=%.0fHz res=%dx%d",
                        renderedFrames,
                        accumTotalMs / 60.0,
                        accumStepMs / 60.0,
                        accumBlitMs / 60.0,
                        maxRenderMs,
                        droppedFrames,
                        targetFps,
                        bw,
                        bh
                    ));
                    accumTotalMs = 0.0;
                    accumStepMs = 0.0;
                    accumBlitMs = 0.0;
                    maxRenderMs = 0.0;
                }
            }
        }

        void cancelAllTouches() {
            synchronized (moveLock) {
                hasPendingMove = false;
                pendingMoveCount = 0;
            }
            renderHandler.post(() -> {
                if (handle != 0) {
                    EuvNativeBridge.nativeTouchCancel(handle, -1);
                }
            });
        }

        private long buildActiveTouchMask(MotionEvent event, int excludeIndex) {
            long mask = 0L;
            int count = event.getPointerCount();
            for (int i = 0; i < count; i++) {
                if (i == excludeIndex) continue;
                int pid = event.getPointerId(i);
                if (pid >= 0 && pid < 63) {
                    mask |= (1L << pid);
                }
            }
            return mask;
        }

        @Override
        public boolean onTouchEvent(MotionEvent event) {
            int action = event.getActionMasked();
            int actionIdx = event.getActionIndex();
            float d = displayDensity;
            switch (action) {
                case MotionEvent.ACTION_DOWN: {
                    int pid = event.getPointerId(actionIdx);
                    float x = event.getX(actionIdx) / d;
                    float y = event.getY(actionIdx) / d;
                    synchronized (moveLock) {
                        hasPendingMove = false;
                        pendingMoveCount = 0;
                    }
                    renderHandler.post(() -> {
                        if (handle == 0) return;
                        EuvNativeBridge.nativeTouchCancel(handle, -1);
                        EuvNativeBridge.nativeTouchDown(handle, pid, x, y);
                    });
                    return true;
                }
                case MotionEvent.ACTION_POINTER_DOWN: {
                    int pid = event.getPointerId(actionIdx);
                    float x = event.getX(actionIdx) / d;
                    float y = event.getY(actionIdx) / d;
                    long mask = buildActiveTouchMask(event, -1);
                    renderHandler.post(() -> {
                        flushPendingMovesOnRenderThread();
                        if (handle == 0) return;
                        EuvNativeBridge.nativeTouchDown(handle, pid, x, y);
                        EuvNativeBridge.nativeTouchSyncMask(handle, mask);
                    });
                    return true;
                }
                case MotionEvent.ACTION_MOVE: {
                    int count = Math.min(16, event.getPointerCount());
                    long mask = buildActiveTouchMask(event, -1);
                    synchronized (moveLock) {
                        pendingMoveCount = count;
                        pendingMoveMask = mask;
                        for (int i = 0; i < count; i++) {
                            pendingMoveIds[i] = event.getPointerId(i);
                            pendingMoveXs[i] = event.getX(i) / d;
                            pendingMoveYs[i] = event.getY(i) / d;
                        }
                        hasPendingMove = true;
                    }
                    return true;
                }
                case MotionEvent.ACTION_POINTER_UP: {
                    int pid = event.getPointerId(actionIdx);
                    float x = event.getX(actionIdx) / d;
                    float y = event.getY(actionIdx) / d;
                    long mask = buildActiveTouchMask(event, actionIdx);
                    renderHandler.post(() -> {
                        flushPendingMovesOnRenderThread();
                        if (handle == 0) return;
                        EuvNativeBridge.nativeTouchUp(handle, pid, x, y);
                        EuvNativeBridge.nativeTouchSyncMask(handle, mask);
                    });
                    return true;
                }
                case MotionEvent.ACTION_UP: {
                    int pid = event.getPointerId(actionIdx);
                    float x = event.getX(actionIdx) / d;
                    float y = event.getY(actionIdx) / d;
                    renderHandler.post(() -> {
                        flushPendingMovesOnRenderThread();
                        if (handle == 0) return;
                        EuvNativeBridge.nativeTouchUp(handle, pid, x, y);
                        EuvNativeBridge.nativeTouchCancel(handle, -1);
                    });
                    return true;
                }
                case MotionEvent.ACTION_CANCEL: {
                    cancelAllTouches();
                    return true;
                }
                default:
                    return super.onTouchEvent(event);
            }
        }
    }
}
EOF

cat > "${BUILD_DIR}/AndroidManifest.xml" <<'EOF'
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="vip.ltpp.euv.showcase"
    android:versionCode="5"
    android:versionName="0.42.0">
    <uses-sdk android:minSdkVersion="30" android:targetSdkVersion="35" />
    <uses-permission android:name="android.permission.VIBRATE" />
    <uses-permission android:name="android.permission.POST_NOTIFICATIONS" />
    <uses-permission android:name="android.permission.INTERNET" />
    <uses-permission android:name="android.permission.CAMERA" />
    <uses-permission android:name="android.permission.RECORD_AUDIO" />
    <uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />
    <uses-permission android:name="android.permission.READ_CONTACTS" />
    <uses-permission android:name="android.permission.READ_CALENDAR" />
    <uses-permission android:name="android.permission.READ_MEDIA_IMAGES" />
    <uses-permission android:name="android.permission.BLUETOOTH_CONNECT" />
    <uses-permission android:name="android.permission.USE_BIOMETRIC" />
    <uses-permission android:name="android.permission.BODY_SENSORS" />
    <uses-permission android:name="android.permission.NEARBY_WIFI_DEVICES" />
    <application
        android:label="Euv Showcase"
        android:usesCleartextTraffic="true"
        android:icon="@mipmap/ic_launcher"
        android:roundIcon="@mipmap/ic_launcher"
        android:theme="@android:style/Theme.DeviceDefault.Light.NoActionBar"
        android:hardwareAccelerated="true">
        <activity
            android:name=".EuvActivity"
            android:exported="true"
            android:windowSoftInputMode="adjustResize|stateHidden"
            android:configChanges="orientation|screenSize|screenLayout|smallestScreenSize|density|fontScale|uiMode|keyboard|keyboardHidden|navigation">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
            <intent-filter>
                <action android:name="android.intent.action.VIEW" />
                <category android:name="android.intent.category.DEFAULT" />
                <category android:name="android.intent.category.BROWSABLE" />
                <data android:scheme="euv" />
            </intent-filter>
        </activity>
    </application>
</manifest>
EOF

log_info "Building Release aarch64-linux-android native shared library..."
(
  cd "${BUILD_DIR}/rust"
  cargo build --release --target aarch64-linux-android
)
SO_PATH="${BUILD_DIR}/rust/target/aarch64-linux-android/release/libeuv_android_host.so"
ANDROID_SCHEMA="$(strings "${SO_PATH}" | grep -c "euv.ai.schema.v1" || true)"
log_info "Built ${SO_PATH} ($(du -h "${SO_PATH}" | cut -f1)), release ai-schema matches=${ANDROID_SCHEMA}"
[ "${ANDROID_SCHEMA}" -eq 0 ]

log_info "Compiling Android Java host & DEX..."
rm -rf "${BUILD_DIR}/classes" "${BUILD_DIR}/dex"
mkdir -p "${BUILD_DIR}/classes" "${BUILD_DIR}/dex"
javac -source 11 -target 11 -classpath "${ANDROID_JAR}" -d "${BUILD_DIR}/classes" \
  "${BUILD_DIR}/java/vip/ltpp/euv/showcase/EuvNativeBridge.java" \
  "${BUILD_DIR}/java/vip/ltpp/euv/showcase/EuvActivity.java"
"${D8}" --min-api 30 --lib "${ANDROID_JAR}" --output "${BUILD_DIR}/dex" \
  "${BUILD_DIR}/classes/vip/ltpp/euv/showcase/"*.class

log_info "Generating app icon resources and packaging EuvShowcase.apk..."
RES_DIR="${BUILD_DIR}/res"
RES_ZIP="${BUILD_DIR}/res.zip"
rm -rf "${RES_DIR}" "${RES_ZIP}"
mkdir -p "${RES_DIR}/mipmap-xxhdpi" "${RES_DIR}/mipmap-xxxhdpi"
python3 "${REPO_ROOT}/scripts/env/generate-icon.py" "${RES_DIR}/mipmap-xxhdpi/ic_launcher.png" 144 "${EUV_APP_ICON:-}"
python3 "${REPO_ROOT}/scripts/env/generate-icon.py" "${RES_DIR}/mipmap-xxxhdpi/ic_launcher.png" 192 "${EUV_APP_ICON:-}"
"${AAPT2}" compile --dir "${RES_DIR}" -o "${RES_ZIP}"

UNALIGNED_APK="${BUILD_DIR}/unaligned.apk"
ALIGNED_APK="${BUILD_DIR}/aligned.apk"
rm -f "${UNALIGNED_APK}" "${ALIGNED_APK}" "${APK_OUT}"
"${AAPT2}" link \
  -I "${ANDROID_JAR}" \
  --manifest "${BUILD_DIR}/AndroidManifest.xml" \
  -R "${RES_ZIP}" \
  -o "${UNALIGNED_APK}"

rm -rf "${BUILD_DIR}/stage" && mkdir -p "${BUILD_DIR}/stage/lib/arm64-v8a"
cp "${BUILD_DIR}/dex/classes.dex" "${BUILD_DIR}/stage/classes.dex"
cp "${SO_PATH}" "${BUILD_DIR}/stage/lib/arm64-v8a/libeuv_android_host.so"
(
  cd "${BUILD_DIR}/stage"
  zip -q -0 -r "${UNALIGNED_APK}" classes.dex lib/arm64-v8a/libeuv_android_host.so
)
"${ZIPALIGN}" -f -p 4 "${UNALIGNED_APK}" "${ALIGNED_APK}"

KEYSTORE="${HOME}/.android/debug.keystore"
if [ ! -f "${KEYSTORE}" ]; then
  mkdir -p "${HOME}/.android"
  keytool -genkeypair -v -keystore "${KEYSTORE}" -storepass android -alias androiddebugkey \
    -keypass android -keyalg RSA -keysize 2048 -validity 10000 \
    -dname "CN=Android Debug,O=Android,C=US" >/dev/null 2>&1
fi
"${APKSIGNER}" sign --ks "${KEYSTORE}" --ks-pass pass:android --key-pass pass:android --out "${APK_OUT}" "${ALIGNED_APK}"
"${APKSIGNER}" verify "${APK_OUT}"
log_info "Signed APK ready: ${APK_OUT} ($(du -h "${APK_OUT}" | cut -f1))"

if [ "${1:-}" = "--install" ]; then
  adb start-server >/dev/null 2>&1 || true
  DEV_COUNT="$(adb devices | awk 'NR>1 && $2=="device" {print $1}' | wc -l | tr -d ' ')"
  if [ "${DEV_COUNT}" -eq 0 ]; then
    log_info "No online ADB device detected; booting AVD '${AVD_NAME}'..."
    nohup "${ANDROID_HOME}/emulator/emulator" -avd "${AVD_NAME}" -no-boot-anim -gpu auto >/tmp/euv_android_emulator.log 2>&1 &
    adb wait-for-device
    for _ in $(seq 1 90); do
      if [ "$(adb shell getprop sys.boot_completed 2>/dev/null | tr -d '\r')" = "1" ]; then
        break
      fi
      sleep 2
    done
    sleep 3
  fi
  log_info "Installing ${APK_OUT} via adb..."
  adb devices -l
  ONLINE_SERIALS="$(adb devices | awk 'NR>1 && $2=="device" {print $1}')"
  for SERIAL in ${ONLINE_SERIALS}; do
    log_info "Deploying to Android target ${SERIAL}..."
    adb -s "${SERIAL}" install -r "${APK_OUT}"
    adb -s "${SERIAL}" shell am start -n vip.ltpp.euv.showcase/.EuvActivity
  done
  log_info "Launched vip.ltpp.euv.showcase/.EuvActivity on all online Android targets."
fi
