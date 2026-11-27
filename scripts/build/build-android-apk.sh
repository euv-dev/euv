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
version = "0.44.0"
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
use std::cell::Cell;
use std::ffi::{CString, c_char, c_void};
use std::sync::Mutex;
use std::sync::atomic::{AtomicPtr, Ordering};
use euv::{
    App, GlyphKey, ImeEvent, KeyCode, KeyEvent, MemoryPressure, Modifiers, NativeSystemFont,
    NetworkStatus, PlatformKind, PointerEvent, RasterizedGlyph, RenderMode, SafeAreaInsets,
    SystemEvent, SystemTheme, WindowEvent, WindowId, register_external_glyph_rasterizer,
    register_native_action_bridge, reset_reactive_runtime,
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

thread_local! {
    static ACTIVE_JNI_ENV: Cell<*mut c_void> = const { Cell::new(std::ptr::null_mut()) };
}
static BRIDGE_INIT_LOCK: Mutex<()> = Mutex::new(());
static BRIDGE_CLASS_REF: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static RASTERIZE_METHOD: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());
static ACTION_METHOD: AtomicPtr<c_void> = AtomicPtr::new(std::ptr::null_mut());

struct JniEnvGuard(*mut c_void);

impl JniEnvGuard {
    fn enter(env: JniEnv) -> Self {
        let prev: *mut c_void = ACTIVE_JNI_ENV.with(|cell: &Cell<*mut c_void>| cell.replace(env.cast()));
        Self(prev)
    }
}

impl Drop for JniEnvGuard {
    fn drop(&mut self) {
        ACTIVE_JNI_ENV.with(|cell: &Cell<*mut c_void>| cell.set(self.0));
    }
}

fn android_native_action_bridge(kind: &str, arg1: &str, arg2: &str) -> bool {
    let env_ptr: *mut c_void = ACTIVE_JNI_ENV.with(Cell::get);
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
    let env_ptr: *mut c_void = ACTIVE_JNI_ENV.with(Cell::get);
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

pub struct EglPresenter {
    display: *mut c_void,
    context: *mut c_void,
    surface: *mut c_void,
    program: u32,
    texture: u32,
    vao: u32,
    tex_w: u32,
    tex_h: u32,
    surface_w: u32,
    surface_h: u32,
}

const EGL_NO_DISPLAY: *mut c_void = std::ptr::null_mut();
const EGL_NO_CONTEXT: *mut c_void = std::ptr::null_mut();
const EGL_NO_SURFACE: *mut c_void = std::ptr::null_mut();
const EGL_DEFAULT_DISPLAY: *mut c_void = std::ptr::null_mut();
const EGL_SURFACE_TYPE: i32 = 0x3033;
const EGL_WINDOW_BIT: i32 = 0x0004;
const EGL_RENDERABLE_TYPE: i32 = 0x3040;
const EGL_OPENGL_ES3_BIT: i32 = 0x0040;
const EGL_RED_SIZE: i32 = 0x3024;
const EGL_GREEN_SIZE: i32 = 0x3023;
const EGL_BLUE_SIZE: i32 = 0x3022;
const EGL_ALPHA_SIZE: i32 = 0x3021;
const EGL_NONE: i32 = 0x3038;
const EGL_CONTEXT_CLIENT_VERSION: i32 = 0x3098;

const GL_VERTEX_SHADER: u32 = 0x8B31;
const GL_FRAGMENT_SHADER: u32 = 0x8B30;
const GL_COMPILE_STATUS: u32 = 0x8B81;
const GL_LINK_STATUS: u32 = 0x8B82;
const GL_TEXTURE_2D: u32 = 0x0DE1;
const GL_TEXTURE_MIN_FILTER: u32 = 0x2801;
const GL_TEXTURE_MAG_FILTER: u32 = 0x2800;
const GL_TEXTURE_WRAP_S: u32 = 0x2802;
const GL_TEXTURE_WRAP_T: u32 = 0x2803;
const GL_LINEAR: i32 = 0x2601;
const GL_CLAMP_TO_EDGE: i32 = 0x812F;
const GL_RGBA8: i32 = 0x8058;
const GL_RGBA: u32 = 0x1908;
const GL_UNSIGNED_BYTE: u32 = 0x1401;
const GL_UNPACK_ALIGNMENT: u32 = 0x0CF5;
const GL_UNPACK_ROW_LENGTH: u32 = 0x0CF2;
const GL_TEXTURE0: u32 = 0x84C0;
const GL_TRIANGLES: u32 = 0x0004;

#[link(name = "EGL")]
unsafe extern "C" {
    fn eglGetDisplay(display_id: *mut c_void) -> *mut c_void;
    fn eglInitialize(dpy: *mut c_void, major: *mut i32, minor: *mut i32) -> u32;
    fn eglChooseConfig(
        dpy: *mut c_void,
        attrib_list: *const i32,
        configs: *mut *mut c_void,
        config_size: i32,
        num_config: *mut i32,
    ) -> u32;
    fn eglCreateContext(
        dpy: *mut c_void,
        config: *mut c_void,
        share_context: *mut c_void,
        attrib_list: *const i32,
    ) -> *mut c_void;
    fn eglCreateWindowSurface(
        dpy: *mut c_void,
        config: *mut c_void,
        win: *mut ANativeWindow,
        attrib_list: *const i32,
    ) -> *mut c_void;
    fn eglMakeCurrent(
        dpy: *mut c_void,
        draw: *mut c_void,
        read: *mut c_void,
        ctx: *mut c_void,
    ) -> u32;
    fn eglSwapInterval(dpy: *mut c_void, interval: i32) -> u32;
    fn eglSwapBuffers(dpy: *mut c_void, surface: *mut c_void) -> u32;
    fn eglDestroySurface(dpy: *mut c_void, surface: *mut c_void) -> u32;
    fn eglDestroyContext(dpy: *mut c_void, ctx: *mut c_void) -> u32;
}

#[link(name = "GLESv3")]
unsafe extern "C" {
    fn glCreateShader(shader_type: u32) -> u32;
    fn glShaderSource(shader: u32, count: i32, string: *const *const c_char, length: *const i32);
    fn glCompileShader(shader: u32);
    fn glGetShaderiv(shader: u32, pname: u32, params: *mut i32);
    fn glDeleteShader(shader: u32);
    fn glCreateProgram() -> u32;
    fn glAttachShader(program: u32, shader: u32);
    fn glLinkProgram(program: u32);
    fn glGetProgramiv(program: u32, pname: u32, params: *mut i32);
    fn glDeleteProgram(program: u32);
    fn glUseProgram(program: u32);
    fn glGetUniformLocation(program: u32, name: *const c_char) -> i32;
    fn glUniform1i(location: i32, v0: i32);
    fn glGenVertexArrays(n: i32, arrays: *mut u32);
    fn glBindVertexArray(array: u32);
    fn glDeleteVertexArrays(n: i32, arrays: *const u32);
    fn glGenTextures(n: i32, textures: *mut u32);
    fn glActiveTexture(texture: u32);
    fn glBindTexture(target: u32, texture: u32);
    fn glDeleteTextures(n: i32, textures: *const u32);
    fn glTexParameteri(target: u32, pname: u32, param: i32);
    fn glPixelStorei(pname: u32, param: i32);
    fn glTexImage2D(
        target: u32,
        level: i32,
        internalformat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        ty: u32,
        pixels: *const c_void,
    );
    fn glTexSubImage2D(
        target: u32,
        level: i32,
        xoffset: i32,
        yoffset: i32,
        width: i32,
        height: i32,
        format: u32,
        ty: u32,
        pixels: *const c_void,
    );
    fn glViewport(x: i32, y: i32, width: i32, height: i32);
    fn glDrawArrays(mode: u32, first: i32, count: i32);
}

impl EglPresenter {
    fn compile_shader(ty: u32, src: &std::ffi::CStr) -> Option<u32> {
        unsafe {
            let sh: u32 = glCreateShader(ty);
            if sh == 0 {
                return None;
            }
            let ptr: *const c_char = src.as_ptr();
            glShaderSource(sh, 1, &ptr, std::ptr::null());
            glCompileShader(sh);
            let mut ok: i32 = 0;
            glGetShaderiv(sh, GL_COMPILE_STATUS, &mut ok);
            if ok == 0 {
                glDeleteShader(sh);
                return None;
            }
            Some(sh)
        }
    }

    fn create(win: *mut ANativeWindow, surface_w: u32, surface_h: u32) -> Option<Self> {
        if win.is_null() {
            return None;
        }
        unsafe {
            let display: *mut c_void = eglGetDisplay(EGL_DEFAULT_DISPLAY);
            if display == EGL_NO_DISPLAY {
                return None;
            }
            let mut maj: i32 = 0;
            let mut min: i32 = 0;
            if eglInitialize(display, &mut maj, &mut min) == 0 {
                return None;
            }
            let cfg_attribs: [i32; 13] = [
                EGL_SURFACE_TYPE,
                EGL_WINDOW_BIT,
                EGL_RENDERABLE_TYPE,
                EGL_OPENGL_ES3_BIT,
                EGL_RED_SIZE,
                8,
                EGL_GREEN_SIZE,
                8,
                EGL_BLUE_SIZE,
                8,
                EGL_ALPHA_SIZE,
                8,
                EGL_NONE,
            ];
            let mut config: *mut c_void = std::ptr::null_mut();
            let mut num_cfg: i32 = 0;
            if eglChooseConfig(display, cfg_attribs.as_ptr(), &mut config, 1, &mut num_cfg) == 0
                || num_cfg <= 0
                || config.is_null()
            {
                return None;
            }
            let ctx_attribs: [i32; 3] = [EGL_CONTEXT_CLIENT_VERSION, 3, EGL_NONE];
            let context: *mut c_void =
                eglCreateContext(display, config, EGL_NO_CONTEXT, ctx_attribs.as_ptr());
            if context == EGL_NO_CONTEXT {
                return None;
            }
            let surface: *mut c_void =
                eglCreateWindowSurface(display, config, win, std::ptr::null());
            if surface == EGL_NO_SURFACE {
                eglDestroyContext(display, context);
                return None;
            }
            if eglMakeCurrent(display, surface, surface, context) == 0 {
                eglDestroySurface(display, surface);
                eglDestroyContext(display, context);
                return None;
            }
            let _ = eglSwapInterval(display, 0);
            let vs_src = c"#version 300 es\nout vec2 v_uv;\nvoid main() {\n    vec2 pos = vec2((gl_VertexID == 1) ? 3.0 : -1.0, (gl_VertexID == 2) ? 3.0 : -1.0);\n    v_uv = vec2((pos.x + 1.0) * 0.5, 1.0 - (pos.y + 1.0) * 0.5);\n    gl_Position = vec4(pos, 0.0, 1.0);\n}\n";
            let fs_src = c"#version 300 es\nprecision mediump float;\nin vec2 v_uv;\nuniform sampler2D u_tex;\nout vec4 frag_color;\nvoid main() {\n    frag_color = texture(u_tex, v_uv);\n}\n";
            let Some(vs) = Self::compile_shader(GL_VERTEX_SHADER, vs_src) else {
                eglMakeCurrent(display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
                eglDestroySurface(display, surface);
                eglDestroyContext(display, context);
                return None;
            };
            let Some(fs) = Self::compile_shader(GL_FRAGMENT_SHADER, fs_src) else {
                glDeleteShader(vs);
                eglMakeCurrent(display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
                eglDestroySurface(display, surface);
                eglDestroyContext(display, context);
                return None;
            };
            let program: u32 = glCreateProgram();
            glAttachShader(program, vs);
            glAttachShader(program, fs);
            glLinkProgram(program);
            glDeleteShader(vs);
            glDeleteShader(fs);
            let mut linked: i32 = 0;
            glGetProgramiv(program, GL_LINK_STATUS, &mut linked);
            if linked == 0 {
                glDeleteProgram(program);
                eglMakeCurrent(display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
                eglDestroySurface(display, surface);
                eglDestroyContext(display, context);
                return None;
            }
            let mut vao: u32 = 0;
            glGenVertexArrays(1, &mut vao);
            let mut texture: u32 = 0;
            glGenTextures(1, &mut texture);
            glActiveTexture(GL_TEXTURE0);
            glBindTexture(GL_TEXTURE_2D, texture);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
            glUseProgram(program);
            let loc: i32 = glGetUniformLocation(program, c"u_tex".as_ptr());
            if loc >= 0 {
                glUniform1i(loc, 0);
            }
            Some(Self {
                display,
                context,
                surface,
                program,
                texture,
                vao,
                tex_w: 0,
                tex_h: 0,
                surface_w,
                surface_h,
            })
        }
    }

    fn present(&mut self, pixels: &[u8], fb_w: u32, fb_h: u32, dirty: Option<ARect>) -> bool {
        let needed: usize = (fb_w as usize).saturating_mul(fb_h as usize).saturating_mul(4);
        if fb_w == 0 || fb_h == 0 || pixels.len() < needed {
            return false;
        }
        unsafe {
            glActiveTexture(GL_TEXTURE0);
            glBindTexture(GL_TEXTURE_2D, self.texture);
            glPixelStorei(GL_UNPACK_ALIGNMENT, 4);
            if self.tex_w != fb_w || self.tex_h != fb_h {
                glPixelStorei(GL_UNPACK_ROW_LENGTH, 0);
                glTexImage2D(
                    GL_TEXTURE_2D,
                    0,
                    GL_RGBA8,
                    fb_w as i32,
                    fb_h as i32,
                    0,
                    GL_RGBA,
                    GL_UNSIGNED_BYTE,
                    pixels.as_ptr().cast::<c_void>(),
                );
                self.tex_w = fb_w;
                self.tex_h = fb_h;
            } else if let Some(r) = dirty {
                let x0: i32 = r.left.clamp(0, fb_w as i32);
                let y0: i32 = r.top.clamp(0, fb_h as i32);
                let x1: i32 = r.right.clamp(x0, fb_w as i32);
                let y1: i32 = r.bottom.clamp(y0, fb_h as i32);
                let dw: i32 = x1 - x0;
                let dh: i32 = y1 - y0;
                if dw > 0 && dh > 0 {
                    let byte_off: usize =
                        ((y0 as usize).saturating_mul(fb_w as usize).saturating_add(x0 as usize))
                            .saturating_mul(4);
                    glPixelStorei(GL_UNPACK_ROW_LENGTH, fb_w as i32);
                    glTexSubImage2D(
                        GL_TEXTURE_2D,
                        0,
                        x0,
                        y0,
                        dw,
                        dh,
                        GL_RGBA,
                        GL_UNSIGNED_BYTE,
                        pixels.as_ptr().add(byte_off).cast::<c_void>(),
                    );
                    glPixelStorei(GL_UNPACK_ROW_LENGTH, 0);
                }
            } else {
                glPixelStorei(GL_UNPACK_ROW_LENGTH, 0);
                glTexSubImage2D(
                    GL_TEXTURE_2D,
                    0,
                    0,
                    0,
                    fb_w as i32,
                    fb_h as i32,
                    GL_RGBA,
                    GL_UNSIGNED_BYTE,
                    pixels.as_ptr().cast::<c_void>(),
                );
            }
            glViewport(0, 0, self.surface_w as i32, self.surface_h as i32);
            glUseProgram(self.program);
            glBindVertexArray(self.vao);
            glDrawArrays(GL_TRIANGLES, 0, 3);
            eglSwapBuffers(self.display, self.surface) != 0
        }
    }

    fn destroy(&mut self) {
        unsafe {
            if self.display != EGL_NO_DISPLAY {
                let _ = eglMakeCurrent(self.display, self.surface, self.surface, self.context);
                if self.texture != 0 {
                    glDeleteTextures(1, &self.texture);
                    self.texture = 0;
                }
                if self.vao != 0 {
                    glDeleteVertexArrays(1, &self.vao);
                    self.vao = 0;
                }
                if self.program != 0 {
                    glDeleteProgram(self.program);
                    self.program = 0;
                }
                let _ = eglMakeCurrent(
                    self.display,
                    EGL_NO_SURFACE,
                    EGL_NO_SURFACE,
                    EGL_NO_CONTEXT,
                );
                if self.surface != EGL_NO_SURFACE {
                    let _ = eglDestroySurface(self.display, self.surface);
                    self.surface = EGL_NO_SURFACE;
                }
                if self.context != EGL_NO_CONTEXT {
                    let _ = eglDestroyContext(self.display, self.context);
                    self.context = EGL_NO_CONTEXT;
                }
                self.display = EGL_NO_DISPLAY;
            }
        }
    }
}

pub struct EuvAndroidHandle {
    app: App,
    _state: ShowcaseState,
    wid: WindowId,
    native_window: *mut ANativeWindow,
    egl: Option<EglPresenter>,
    target_refresh_hz: f32,
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
        if let Some(mut egl) = self.egl.take() {
            egl.destroy();
        }
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
    refresh_hz: f32,
    safe_top: f32,
    safe_right: f32,
    safe_bottom: f32,
    safe_left: f32,
) -> i64 {
    if !env.is_null() && !cls.is_null() {
        let _init_guard = BRIDGE_INIT_LOCK.lock().ok();
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
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    reset_reactive_runtime();
    let Ok((mut app, state)) = create_showcase_app_for_platform(PlatformKind::Android, RenderMode::Software) else {
        return 0;
    };
    app.get_mut_renderer()
        .get_mut_software()
        .set_font_provider(NativeSystemFont::system());
    let hz: f32 = refresh_hz.clamp(24.0, 480.0);
    app.update_display_refresh_rate_hz(hz);
    let wid: WindowId = app.get_window_state().get_id();
    let pw: u32 = phys_width.max(200) as u32;
    let ph: u32 = phys_height.max(200) as u32;
    let s: f32 = scale.clamp(1.0, 4.5);
    app.set_exact_physical_viewport(pw, ph, s);
    app.dispatch_window_event(WindowEvent::SafeAreaChanged(
        wid,
        SafeAreaInsets::new(
            safe_top.max(0.0),
            safe_right.max(0.0),
            safe_bottom.max(0.0),
            safe_left.max(0.0),
        ),
    ));
    let handle = Box::new(EuvAndroidHandle {
        app,
        _state: state,
        wid,
        native_window: std::ptr::null_mut(),
        egl: None,
        target_refresh_hz: hz,
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
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeDestroy(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
) {
    if ptr == 0 {
        return;
    }
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let handle: Box<EuvAndroidHandle> = unsafe { Box::from_raw(ptr as *mut EuvAndroidHandle) };
    drop(handle);
    reset_reactive_runtime();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeIsRunning(
    _env: JniEnv,
    _cls: JObject,
    ptr: i64,
) -> i32 {
    if ptr == 0 {
        return 0;
    }
    let h: &EuvAndroidHandle = unsafe { &*(ptr as *const EuvAndroidHandle) };
    if h.app.get_running() { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeUpdateRefreshRate(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    refresh_hz: f32,
) {
    if ptr == 0 {
        return;
    }
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    let hz: f32 = refresh_hz.clamp(24.0, 480.0);
    h.target_refresh_hz = hz;
    h.app.update_display_refresh_rate_hz(hz);
    if !h.native_window.is_null() {
        let _: bool = euv::android_set_native_window_frame_rate(h.native_window as usize, hz);
    }
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
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    if let Some(mut egl) = h.egl.take() {
        egl.destroy();
    }
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
            let _: bool = euv::android_set_native_window_frame_rate(win as usize, h.target_refresh_hz);
            h.egl = EglPresenter::create(win, pw, ph);
            if h.egl.is_none() {
                unsafe {
                    let _ = ANativeWindow_setBuffersGeometry(
                        win,
                        pw as i32,
                        ph as i32,
                        WINDOW_FORMAT_RGBA_8888,
                    );
                }
            }
            h.native_window = win;
            h.surface_w = pw;
            h.surface_h = ph;
            h.app.set_exact_physical_viewport(pw, ph, s);
        }
    }
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
    let step_res = {
        let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
        h.app.step_game_frame(dt.clamp(1.0 / 480.0, 0.05))
    };
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
                    (rx1 - rx0) > 0 && (ry1 - ry0) > 0
                }).unwrap_or(false);
            let mut presented_egl: bool = false;
            if let Some(egl) = h.egl.as_mut() {
                presented_egl = egl.present(
                    fb.get_pixels().as_slice(),
                    w,
                    ht,
                    if use_partial { Some(dirty_box) } else { None },
                );
            }
            if !presented_egl {
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
                let lock_rc: i32 = unsafe {
                    ANativeWindow_lock(h.native_window, &mut buf, std::ptr::null_mut())
                };
                if lock_rc == 0 {
                    copy_rgba_rect_to_native_window(
                        fb.get_pixels().as_slice(),
                        w as usize,
                        ht as usize,
                        &buf,
                        None,
                    );
                    unsafe {
                        let _ = ANativeWindow_unlockAndPost(h.native_window);
                    }
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
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    let pw: u32 = phys_width.max(200) as u32;
    let ph: u32 = phys_height.max(200) as u32;
    let s: f32 = scale.clamp(1.0, 4.5);
    h.app.set_exact_physical_viewport(pw, ph, s);
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
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app.dispatch_window_event(WindowEvent::SafeAreaChanged(
        h.wid,
        SafeAreaInsets::new(top.max(0.0), right.max(0.0), bottom.max(0.0), left.max(0.0)),
    ));
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
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app.dispatch_window_event(WindowEvent::PointerDown(
        h.wid,
        PointerEvent::touch((pointer_id as u64).saturating_add(1), x, y),
    ));
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
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app.dispatch_window_event(WindowEvent::PointerMove(
        h.wid,
        PointerEvent::touch((pointer_id as u64).saturating_add(1), x, y),
    ));
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
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app.dispatch_window_event(WindowEvent::PointerUp(
        h.wid,
        PointerEvent::touch((pointer_id as u64).saturating_add(1), x, y),
    ));
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
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    let mapped_id: u64 = if pointer_id < 0 {
        0
    } else {
        (pointer_id as u64).saturating_add(1)
    };
    h.app
        .dispatch_window_event(WindowEvent::PointerCancel(h.wid, mapped_id));
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
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    let bits: u64 = active_mask as u64;
    let mut live_ids: Vec<u64> = Vec::with_capacity(bits.count_ones() as usize);
    for bit in 0_u64..63_u64 {
        if (bits & (1_u64 << bit)) != 0 {
            live_ids.push(bit + 1);
        }
    }
    h.app.reconcile_active_touch_ids(&live_ids);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeImeDisabled(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
) {
    if ptr == 0 {
        return;
    }
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app
        .dispatch_window_event(WindowEvent::Ime(h.wid, ImeEvent::Disabled));
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
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app.dispatch_window_event(WindowEvent::KeyDown(
        h.wid,
        KeyEvent::new(KeyCode::Back, Modifiers::default(), false),
    ));
}

fn read_jni_utf8(env: JniEnv, utf8_bytes: JByteArray) -> Option<String> {
    if env.is_null() || utf8_bytes.is_null() {
        return None;
    }
    unsafe {
        let table: &JniNativeInterface = &*(*env);
        let len: i32 = (table.get_array_length)(env, utf8_bytes);
        if len > 0 {
            let mut buf: Vec<u8> = vec![0; len as usize];
            (table.get_byte_array_region)(env, utf8_bytes, 0, len, buf.as_mut_ptr().cast::<i8>());
            String::from_utf8(buf).ok()
        } else {
            None
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeSystemLifecycle(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    tag: i32,
) {
    if ptr == 0 {
        return;
    }
    let ev: SystemEvent = match tag {
        1 => SystemEvent::Appear,
        2 => SystemEvent::Disappear,
        3 => SystemEvent::Foreground,
        4 => SystemEvent::Inactive,
        5 => SystemEvent::Background,
        6 => SystemEvent::Terminate,
        _ => return,
    };
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app.dispatch_system_event(ev);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeFocusChanged(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    focused: i32,
) {
    if ptr == 0 {
        return;
    }
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app
        .dispatch_system_event(SystemEvent::FocusChanged(focused != 0));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeKeyboardChanged(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    visible: i32,
    height: f32,
) {
    if ptr == 0 {
        return;
    }
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app
        .dispatch_system_event(SystemEvent::KeyboardVisibilityChanged {
            visible: visible != 0,
            height: height.max(0.0),
        });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeThemeChanged(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    is_dark: i32,
) {
    if ptr == 0 {
        return;
    }
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    let mode: SystemTheme = if is_dark != 0 {
        SystemTheme::Dark
    } else {
        SystemTheme::Light
    };
    h.app.dispatch_system_event(SystemEvent::ThemeChanged(mode));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeMemoryWarning(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    level: i32,
) {
    if ptr == 0 {
        return;
    }
    let pressure: MemoryPressure = if level >= 2 {
        MemoryPressure::Critical
    } else {
        MemoryPressure::Warning
    };
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app
        .dispatch_system_event(SystemEvent::MemoryWarning(pressure));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeLowPowerChanged(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    enabled: i32,
) {
    if ptr == 0 {
        return;
    }
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app
        .dispatch_system_event(SystemEvent::LowPowerModeChanged(enabled != 0));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeNetworkChanged(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    status_tag: i32,
) {
    if ptr == 0 {
        return;
    }
    let status: NetworkStatus = match status_tag {
        0 => NetworkStatus::Offline,
        1 => NetworkStatus::Online,
        2 => NetworkStatus::Wifi,
        3 => NetworkStatus::Cellular,
        _ => NetworkStatus::Unknown,
    };
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app
        .dispatch_system_event(SystemEvent::NetworkStatusChanged(status));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeLocaleChanged(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    utf8_bytes: JByteArray,
) {
    if ptr == 0 {
        return;
    }
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    if let Some(locale) = read_jni_utf8(env, utf8_bytes)
        && !locale.is_empty()
    {
        h.app
            .dispatch_system_event(SystemEvent::LocaleChanged(locale));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeDeepLink(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
    utf8_bytes: JByteArray,
) {
    if ptr == 0 {
        return;
    }
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    if let Some(uri) = read_jni_utf8(env, utf8_bytes)
        && !uri.is_empty()
    {
        h.app.dispatch_system_event(SystemEvent::DeepLink(uri));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_vip_ltpp_euv_showcase_EuvNativeBridge_nativeClipboardChanged(
    env: JniEnv,
    _cls: JObject,
    ptr: i64,
) {
    if ptr == 0 {
        return;
    }
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app.dispatch_system_event(SystemEvent::ClipboardChanged);
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
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
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
    let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
    let h: &mut EuvAndroidHandle = unsafe { &mut *(ptr as *mut EuvAndroidHandle) };
    h.app.dispatch_window_event(WindowEvent::KeyDown(
        h.wid,
        KeyEvent::new(code, Modifiers::default(), false),
    ));
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
    let step_res = {
        let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
        h.app.step_game_frame(dt.clamp(1.0 / 480.0, 0.05))
    };
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
    let step_res = {
        let _jni_guard: JniEnvGuard = JniEnvGuard::enter(env);
        h.app.step_game_frame(dt.clamp(1.0 / 480.0, 0.05))
    };
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
import android.content.ClipData;
import android.content.ClipboardManager;
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
    private static final ThreadLocal<Paint> GLYPH_PAINT = ThreadLocal.withInitial(
        () -> new Paint(Paint.ANTI_ALIAS_FLAG | Paint.SUBPIXEL_TEXT_FLAG)
    );
    private static final Typeface FACE_REGULAR = Typeface.create(Typeface.SANS_SERIF, Typeface.NORMAL);
    private static final Typeface FACE_BOLD = Typeface.create(Typeface.SANS_SERIF, Typeface.BOLD);
    private static final Typeface FACE_ITALIC = Typeface.create(Typeface.SANS_SERIF, Typeface.ITALIC);
    private static final Typeface FACE_BOLD_ITALIC = Typeface.create(Typeface.SANS_SERIF, Typeface.BOLD_ITALIC);
    private static final Typeface FACE_MONO_REGULAR = Typeface.create(Typeface.MONOSPACE, Typeface.NORMAL);
    private static final Typeface FACE_MONO_BOLD = Typeface.create(Typeface.MONOSPACE, Typeface.BOLD);
    private static final Typeface FACE_MONO_ITALIC = Typeface.create(Typeface.MONOSPACE, Typeface.ITALIC);
    private static final Typeface FACE_MONO_BOLD_ITALIC = Typeface.create(Typeface.MONOSPACE, Typeface.BOLD_ITALIC);
    private static final String CHANNEL_ID = "euv_native_alerts";
    private static final Object HOST_LOCK = new Object();
    private static WeakReference<Activity> hostActivity = new WeakReference<>(null);
    private static long activeSessionGen = 0L;
    private static int notifSeq = 100;

    static {
        System.loadLibrary("euv_android_host");
    }

    public static long registerHostActivity(Activity activity) {
        synchronized (HOST_LOCK) {
            activeSessionGen += 1L;
            hostActivity = new WeakReference<>(activity);
            return activeSessionGen;
        }
    }

    public static void activateHostActivity(Activity activity, long sessionGen) {
        if (activity == null || activity.isFinishing() || activity.isDestroyed()) {
            return;
        }
        synchronized (HOST_LOCK) {
            Activity current = hostActivity.get();
            if (sessionGen >= activeSessionGen || current == null || current.isFinishing() || current.isDestroyed()) {
                if (sessionGen > activeSessionGen) {
                    activeSessionGen = sessionGen;
                }
                hostActivity = new WeakReference<>(activity);
            }
        }
    }

    public static void clearHostActivity(Activity activity, long sessionGen) {
        synchronized (HOST_LOCK) {
            if (hostActivity.get() == activity && sessionGen == activeSessionGen) {
                hostActivity = new WeakReference<>(null);
            }
        }
    }

    public static void setHostActivity(Activity activity) {
        synchronized (HOST_LOCK) {
            if (activity != null) {
                activeSessionGen += 1L;
            }
            hostActivity = new WeakReference<>(activity);
        }
    }

    private static Activity currentHostActivity() {
        synchronized (HOST_LOCK) {
            return hostActivity.get();
        }
    }

    public static boolean invokeNativeAction(String kind, String arg1, String arg2) {
        Activity act = currentHostActivity();
        if (act == null || act.isFinishing() || act.isDestroyed()) {
            return false;
        }
        String k = (kind != null) ? kind : "";
        String a1 = (arg1 != null) ? arg1 : "";
        String a2 = (arg2 != null) ? arg2 : "";
        Log.i("euv-native", "invokeNativeAction kind=" + k + " arg1=" + a1 + " arg2=" + a2);
        act.runOnUiThread(() -> {
            if (act.isDestroyed()) {
                return;
            }
            try {
                switch (k) {
                    case "ime": {
                        if (act instanceof EuvActivity) {
                            ((EuvActivity) act).setSoftKeyboardVisible("show".equals(a1));
                        }
                        break;
                    }
                    case "clipboard": {
                        ClipboardManager cm = (ClipboardManager) act.getSystemService(Context.CLIPBOARD_SERVICE);
                        if (cm != null) {
                            cm.setPrimaryClip(ClipData.newPlainText("euv", a2));
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
                                .setContentTitle(a1.isEmpty() ? "Euv Studio" : a1)
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
                        boolean enableFs = "on".equals(a1);
                        if (act instanceof EuvActivity) {
                            ((EuvActivity) act).setImmersiveFullscreen(enableFs);
                        } else {
                            WindowInsetsController ctrl = act.getWindow().getInsetsController();
                            if (ctrl != null) {
                                if (enableFs) {
                                    ctrl.hide(WindowInsets.Type.statusBars() | WindowInsets.Type.navigationBars());
                                    ctrl.setSystemBarsBehavior(WindowInsetsController.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE);
                                } else {
                                    ctrl.show(WindowInsets.Type.statusBars() | WindowInsets.Type.navigationBars());
                                }
                            }
                        }
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
                        if (!act.isFinishing() && !act.isDestroyed()) {
                            act.finish();
                        }
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
        Paint paint = GLYPH_PAINT.get();
        paint.setTypeface(face);
        paint.setTextSize(Math.max(6.0f, sizePx));
        paint.setColor(Color.WHITE);
        Paint.FontMetrics fm = paint.getFontMetrics();
        float advF = paint.measureText(text);
        int adv = Math.max(2, (int) Math.ceil(advF));
        int w = Math.max(4, adv + 4);
        int h = Math.max(8, (int) Math.ceil(fm.descent - fm.ascent) + 4);
        Bitmap bmp = Bitmap.createBitmap(w, h, Bitmap.Config.ALPHA_8);
        Canvas canvas = new Canvas(bmp);
        canvas.drawText(text, 1.0f, -fm.ascent + 2.0f, paint);
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

    public static native long nativeCreate(
        int physWidth,
        int physHeight,
        float scale,
        float refreshHz,
        float safeTop,
        float safeRight,
        float safeBottom,
        float safeLeft
    );
    public static native void nativeDestroy(long handle);
    public static native int nativeIsRunning(long handle);
    public static native void nativeUpdateRefreshRate(long handle, float refreshHz);
    public static native void nativeImeDisabled(long handle);
    public static native void nativeSystemLifecycle(long handle, int tag);
    public static native void nativeFocusChanged(long handle, int focused);
    public static native void nativeKeyboardChanged(long handle, int visible, float height);
    public static native void nativeThemeChanged(long handle, int isDark);
    public static native void nativeMemoryWarning(long handle, int level);
    public static native void nativeLowPowerChanged(long handle, int enabled);
    public static native void nativeNetworkChanged(long handle, int statusTag);
    public static native void nativeLocaleChanged(long handle, byte[] utf8Bytes);
    public static native void nativeDeepLink(long handle, byte[] utf8Bytes);
    public static native void nativeClipboardChanged(long handle);
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
import android.content.ClipboardManager;
import android.content.Context;
import android.content.Intent;
import android.content.res.Configuration;
import android.graphics.Color;
import android.graphics.Insets;
import android.graphics.PixelFormat;
import android.graphics.Rect;
import android.hardware.display.DisplayManager;
import android.os.Build;
import android.os.Bundle;
import android.os.Handler;
import android.os.HandlerThread;
import android.os.PowerManager;
import android.os.Process;
import android.text.InputType;
import android.util.Log;
import android.view.Choreographer;
import android.view.Display;
import android.view.DisplayCutout;
import android.view.KeyEvent;
import android.view.MotionEvent;
import android.view.Surface;
import android.view.SurfaceHolder;
import android.view.SurfaceView;
import android.view.View;
import android.view.ViewGroup;
import android.view.Window;
import android.view.WindowInsets;
import android.view.WindowInsetsController;
import android.view.WindowManager;
import android.widget.FrameLayout;
import android.view.inputmethod.BaseInputConnection;
import android.view.inputmethod.EditorInfo;
import android.view.inputmethod.InputConnection;
import android.view.inputmethod.InputMethodManager;
import java.lang.reflect.Method;
import java.nio.charset.StandardCharsets;
import java.util.List;
import java.util.Locale;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;

public final class EuvActivity extends Activity {
    private static final int LAUNCH_BG_LIGHT = 0xFFFAFAFA;
    private static final int LAUNCH_BG_DARK = 0xFF09090B;
    private static final long LAUNCH_HOLD_TIMEOUT_MS = 1200L;
    private FrameLayout rootContainer;
    private View launchCoverView;
    private EuvCanvasView canvasView;
    private long sessionGen = 0L;
    private volatile boolean activityDestroyed = false;
    private Object backInvokedCallback = null;
    private float targetRefreshHz = 60.0f;
    private int appliedDisplayModeId = 0;
    private boolean immersiveFullscreen = false;
    private volatile boolean firstFramePresented = false;
    private ClipboardManager clipboardManager;
    private DisplayManager displayManager;
    private final DisplayManager.DisplayListener displayListener = new DisplayManager.DisplayListener() {
        @Override
        public void onDisplayAdded(int displayId) {}

        @Override
        public void onDisplayRemoved(int displayId) {}

        @Override
        public void onDisplayChanged(int displayId) {
            if (activityDestroyed || isDestroyed()) return;
            selectHighestRefreshMode(getWindow());
            if (canvasView != null) {
                canvasView.updateTargetRefreshRate(targetRefreshHz);
                canvasView.requestSafeAreaRefresh();
            }
        }
    };
    private final ClipboardManager.OnPrimaryClipChangedListener clipListener = () -> {
        if (activityDestroyed || isDestroyed()) return;
        if (canvasView != null) {
            canvasView.dispatchClipboardChanged();
        }
    };

    private Display resolveActiveDisplay() {
        try {
            Display d = getDisplay();
            if (d != null) return d;
        } catch (Throwable ignored) {
        }
        try {
            DisplayManager dm = (DisplayManager) getSystemService(Context.DISPLAY_SERVICE);
            if (dm != null) {
                Display d = dm.getDisplay(Display.DEFAULT_DISPLAY);
                if (d != null) return d;
            }
        } catch (Throwable ignored) {
        }
        try {
            WindowManager wm = getWindowManager();
            if (wm != null) {
                @SuppressWarnings("deprecation")
                Display d = wm.getDefaultDisplay();
                return d;
            }
        } catch (Throwable ignored) {
        }
        return null;
    }

    private void syncSystemConfig(Configuration cfg) {
        if (canvasView == null || cfg == null) return;
        boolean isDark = (cfg.uiMode & Configuration.UI_MODE_NIGHT_MASK) == Configuration.UI_MODE_NIGHT_YES;
        canvasView.dispatchThemeChanged(isDark);
        String localeTag = Locale.getDefault().toLanguageTag();
        if (localeTag != null && !localeTag.isEmpty()) {
            canvasView.dispatchLocaleChanged(localeTag);
        }
        PowerManager pm = (PowerManager) getSystemService(Context.POWER_SERVICE);
        if (pm != null) {
            canvasView.dispatchLowPowerChanged(pm.isPowerSaveMode());
        }
    }

    private void forwardIntentDeepLink(Intent intent) {
        if (intent == null || canvasView == null) return;
        String uri = intent.getDataString();
        if (uri != null && !uri.isEmpty()) {
            canvasView.dispatchDeepLink(uri);
        }
    }

    private void selectHighestRefreshMode(Window win) {
        try {
            Display display = resolveActiveDisplay();
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
            int bestModeId = best.getModeId();
            if (appliedDisplayModeId != bestModeId) {
                WindowManager.LayoutParams lp = win.getAttributes();
                lp.preferredDisplayModeId = bestModeId;
                lp.preferredRefreshRate = targetRefreshHz;
                win.setAttributes(lp);
                appliedDisplayModeId = bestModeId;
            }
            if (Build.VERSION.SDK_INT >= 34) {
                try {
                    Method boost = Window.class.getMethod("setFrameRateBoostOnTouchEnabled", boolean.class);
                    boost.invoke(win, Boolean.TRUE);
                } catch (Throwable ignored) {
                }
            }
            if (canvasView != null) {
                canvasView.updateTargetRefreshRate(targetRefreshHz);
            }
        } catch (Throwable t) {
            Log.w("euv-fps", "refresh mode hint skipped: " + t.getMessage());
        }
    }

    public void setImmersiveFullscreen(boolean enabled) {
        if (immersiveFullscreen == enabled && canvasView != null) {
            canvasView.setImmersiveFullscreen(enabled);
            return;
        }
        immersiveFullscreen = enabled;
        Window win = getWindow();
        if (win != null) {
            WindowInsetsController ctrl = win.getInsetsController();
            if (ctrl != null) {
                int barTypes = WindowInsets.Type.statusBars() | WindowInsets.Type.navigationBars();
                if (enabled) {
                    ctrl.hide(barTypes);
                    ctrl.setSystemBarsBehavior(WindowInsetsController.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE);
                } else {
                    ctrl.show(barTypes);
                }
            }
        }
        if (canvasView != null) {
            canvasView.setImmersiveFullscreen(enabled);
            canvasView.requestSafeAreaRefresh();
        }
    }

    private void attachDecorInsetsListener(Window win) {
        if (win == null) return;
        View decor = win.getDecorView();
        if (decor == null) return;
        decor.setOnApplyWindowInsetsListener((v, insets) -> {
            if (canvasView != null) {
                canvasView.applyWindowInsets(insets);
            }
            return v.onApplyWindowInsets(insets);
        });
    }

    private void dismissLaunchCoverOnFirstFrame() {
        if (firstFramePresented) return;
        firstFramePresented = true;
        if (activityDestroyed || isDestroyed()) return;
        runOnUiThread(() -> {
            if (activityDestroyed || isDestroyed()) return;
            if (launchCoverView != null) {
                ViewGroup parent = (ViewGroup) launchCoverView.getParent();
                if (parent != null) {
                    parent.removeView(launchCoverView);
                }
                launchCoverView = null;
            }
        });
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        activityDestroyed = false;
        sessionGen = EuvNativeBridge.registerHostActivity(this);
        Window win = getWindow();
        win.setDecorFitsSystemWindows(false);
        win.setStatusBarColor(Color.TRANSPARENT);
        win.setNavigationBarColor(Color.TRANSPARENT);
        Configuration cfg = getResources().getConfiguration();
        boolean isNight = (cfg.uiMode & Configuration.UI_MODE_NIGHT_MASK) == Configuration.UI_MODE_NIGHT_YES;
        int launchBg = isNight ? LAUNCH_BG_DARK : LAUNCH_BG_LIGHT;
        WindowManager.LayoutParams lp = win.getAttributes();
        lp.layoutInDisplayCutoutMode = WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES;
        win.setAttributes(lp);
        selectHighestRefreshMode(win);
        rootContainer = new FrameLayout(this);
        rootContainer.setBackgroundColor(launchBg);
        canvasView = new EuvCanvasView(this, win, targetRefreshHz, this::dismissLaunchCoverOnFirstFrame);
        rootContainer.addView(
            canvasView,
            new FrameLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.MATCH_PARENT
            )
        );
        launchCoverView = new View(this);
        launchCoverView.setBackgroundColor(launchBg);
        launchCoverView.setClickable(false);
        launchCoverView.setFocusable(false);
        rootContainer.addView(
            launchCoverView,
            new FrameLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.MATCH_PARENT
            )
        );
        setContentView(rootContainer);
        attachDecorInsetsListener(win);
        displayManager = (DisplayManager) getSystemService(Context.DISPLAY_SERVICE);
        if (displayManager != null) {
            displayManager.registerDisplayListener(displayListener, new Handler(getMainLooper()));
        }
        clipboardManager = (ClipboardManager) getSystemService(Context.CLIPBOARD_SERVICE);
        if (clipboardManager != null) {
            clipboardManager.addPrimaryClipChangedListener(clipListener);
        }
        forwardIntentDeepLink(getIntent());
        if (Build.VERSION.SDK_INT >= 33) {
            android.window.OnBackInvokedCallback cb = () -> {
                if (!activityDestroyed && canvasView != null) {
                    canvasView.dispatchSystemBack();
                }
            };
            backInvokedCallback = cb;
            getOnBackInvokedDispatcher().registerOnBackInvokedCallback(
                android.window.OnBackInvokedDispatcher.PRIORITY_DEFAULT,
                cb
            );
        }
        View decor = win.getDecorView();
        decor.post(() -> {
            if (activityDestroyed || isDestroyed()) return;
            selectHighestRefreshMode(win);
            WindowInsetsController ctrl = win.getInsetsController();
            if (ctrl != null) {
                int appearance = isNight
                    ? 0
                    : (WindowInsetsController.APPEARANCE_LIGHT_STATUS_BARS
                        | WindowInsetsController.APPEARANCE_LIGHT_NAVIGATION_BARS);
                ctrl.setSystemBarsAppearance(
                    appearance,
                    WindowInsetsController.APPEARANCE_LIGHT_STATUS_BARS
                        | WindowInsetsController.APPEARANCE_LIGHT_NAVIGATION_BARS
                );
            }
            if (canvasView != null) {
                canvasView.requestSafeAreaRefresh();
            }
        });
        decor.postDelayed(() -> {
            if (!activityDestroyed && !isDestroyed() && canvasView != null) {
                canvasView.requestSafeAreaRefresh();
            }
        }, 150L);
        decor.postDelayed(() -> {
            if (!activityDestroyed && !isDestroyed() && canvasView != null) {
                canvasView.requestSafeAreaRefresh();
            }
        }, 400L);
        decor.postDelayed(this::dismissLaunchCoverOnFirstFrame, LAUNCH_HOLD_TIMEOUT_MS);
    }

    @Override
    public void onAttachedToWindow() {
        super.onAttachedToWindow();
        selectHighestRefreshMode(getWindow());
        if (canvasView != null) {
            canvasView.requestSafeAreaRefresh();
        }
    }

    @Override
    protected void onNewIntent(Intent intent) {
        super.onNewIntent(intent);
        setIntent(intent);
        forwardIntentDeepLink(intent);
    }

    @Override
    protected void onStart() {
        super.onStart();
        if (canvasView != null) {
            canvasView.dispatchSystemLifecycle(1);
        }
    }

    @Override
    protected void onStop() {
        if (canvasView != null) {
            canvasView.dispatchSystemLifecycle(5);
        }
        super.onStop();
    }

    @Override
    protected void onDestroy() {
        activityDestroyed = true;
        if (Build.VERSION.SDK_INT >= 33 && backInvokedCallback instanceof android.window.OnBackInvokedCallback) {
            try {
                getOnBackInvokedDispatcher().unregisterOnBackInvokedCallback(
                    (android.window.OnBackInvokedCallback) backInvokedCallback
                );
            } catch (Throwable ignored) {
            }
            backInvokedCallback = null;
        }
        if (displayManager != null) {
            displayManager.unregisterDisplayListener(displayListener);
            displayManager = null;
        }
        if (clipboardManager != null) {
            clipboardManager.removePrimaryClipChangedListener(clipListener);
            clipboardManager = null;
        }
        EuvNativeBridge.clearHostActivity(this, sessionGen);
        if (canvasView != null) {
            canvasView.shutdown();
            canvasView = null;
        }
        super.onDestroy();
    }

    @Override
    public void onTrimMemory(int level) {
        super.onTrimMemory(level);
        if (canvasView == null) return;
        if (level >= TRIM_MEMORY_RUNNING_CRITICAL || level >= TRIM_MEMORY_COMPLETE) {
            canvasView.dispatchMemoryWarning(2);
        } else if (level >= TRIM_MEMORY_RUNNING_MODERATE) {
            canvasView.dispatchMemoryWarning(1);
        }
    }

    @Override
    public void onLowMemory() {
        super.onLowMemory();
        if (canvasView != null) {
            canvasView.dispatchMemoryWarning(2);
        }
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
        selectHighestRefreshMode(getWindow());
        if (canvasView != null) {
            canvasView.syncDisplayScaleAndResize();
            canvasView.requestSafeAreaRefresh();
            syncSystemConfig(newConfig);
        }
    }

    @Override
    protected void onResume() {
        super.onResume();
        EuvNativeBridge.activateHostActivity(this, sessionGen);
        selectHighestRefreshMode(getWindow());
        if (canvasView != null) {
            canvasView.onActivityResumed();
            syncSystemConfig(getResources().getConfiguration());
            canvasView.requestSafeAreaRefresh();
            getWindow().getDecorView().post(() -> {
                if (!activityDestroyed && !isDestroyed() && canvasView != null) {
                    canvasView.requestSafeAreaRefresh();
                }
            });
        }
    }

    @Override
    protected void onPause() {
        super.onPause();
        if (canvasView != null) {
            canvasView.onActivityPaused();
        }
    }

    @Override
    public void onWindowFocusChanged(boolean hasFocus) {
        super.onWindowFocusChanged(hasFocus);
        if (canvasView != null) {
            if (!hasFocus) {
                canvasView.cancelAllTouches();
            } else {
                selectHighestRefreshMode(getWindow());
                canvasView.requestSafeAreaRefresh();
                canvasView.requestFocus();
            }
            canvasView.dispatchFocusChanged(hasFocus);
        }
    }

    private static final class EuvCanvasView extends SurfaceView implements SurfaceHolder.Callback {
        private final Window hostWindow;
        private final Runnable onFirstFramePresented;
        private final HandlerThread renderThread;
        private final Handler renderHandler;
        private final Object moveLock = new Object();
        private final Object safeLock = new Object();
        private boolean hasPendingMove = false;
        private int pendingMoveCount = 0;
        private final int[] pendingMoveIds = new int[16];
        private final float[] pendingMoveXs = new float[16];
        private final float[] pendingMoveYs = new float[16];
        private long pendingMoveMask = 0L;
        private long handle = 0;
        private volatile boolean destroyed = false;
        private volatile boolean activityResumed = false;
        private long activeLoopGen = 0L;
        private volatile float displayDensity = 2.0f;
        private volatile float renderScale = 2.0f;
        private volatile float targetFps = 60.0f;
        private volatile boolean immersiveFullscreen = false;
        private boolean firstPresentNotified = false;
        private float cachedSafeTop = 28.0f;
        private float cachedSafeRight = 0.0f;
        private float cachedSafeBottom = 24.0f;
        private float cachedSafeLeft = 0.0f;
        private boolean surfaceReady = false;
        private int surfaceW = 0;
        private int surfaceH = 0;
        private long lastNanos = 0;
        private boolean running = false;
        private boolean imeWasVisible = false;
        private long renderedFrames = 0;
        private double accumTotalMs = 0.0;
        private double accumStepMs = 0.0;
        private double accumBlitMs = 0.0;
        private double maxRenderMs = 0.0;
        private long droppedFrames = 0;
        private final Choreographer.FrameCallback renderFrameCallback = new Choreographer.FrameCallback() {
            @Override
            public void doFrame(long frameTimeNanos) {
                if (!running || destroyed) return;
                long gen = activeLoopGen;
                Choreographer.getInstance().postFrameCallback(this);
                float hz = targetFps > 24.0f ? targetFps : 60.0f;
                float dt = (lastNanos > 0) ? ((frameTimeNanos - lastNanos) / 1_000_000_000.0f) : (1.0f / hz);
                lastNanos = frameTimeNanos;
                flushPendingMovesOnRenderThread();
                if (!running || destroyed || activeLoopGen != gen) return;
                stepAndPresentOnRenderThread(dt, false);
            }
        };

        EuvCanvasView(Context ctx, Window window, float preferredHz, Runnable firstFrameCallback) {
            super(ctx);
            hostWindow = window;
            onFirstFramePresented = firstFrameCallback;
            targetFps = Math.max(60.0f, preferredHz);
            refreshDensity(ctx);
            seedInitialSafeArea(ctx);
            renderThread = new HandlerThread("euv-render", Process.THREAD_PRIORITY_DISPLAY);
            renderThread.start();
            renderHandler = new Handler(renderThread.getLooper());
            getHolder().setFormat(PixelFormat.RGBA_8888);
            getHolder().addCallback(this);
            setFocusable(true);
            setFocusableInTouchMode(true);
            applyViewRequestedFrameRate(targetFps);
            setOnApplyWindowInsetsListener((v, insets) -> {
                applyWindowInsets(insets);
                return insets;
            });
        }

        private void applyViewRequestedFrameRate(float hz) {
            if (Build.VERSION.SDK_INT >= 35) {
                try {
                    Method setReq = View.class.getMethod("setRequestedFrameRate", float.class);
                    setReq.invoke(this, hz);
                } catch (Throwable ignored) {
                }
            }
        }

        private int queryFallbackStatusBarHeightPx(Context ctx) {
            int fallbackPx = 0;
            try {
                int resId = ctx.getResources().getIdentifier("status_bar_height", "dimen", "android");
                if (resId > 0) {
                    fallbackPx = Math.max(fallbackPx, ctx.getResources().getDimensionPixelSize(resId));
                }
            } catch (Throwable ignored) {
            }
            try {
                Display d = ctx.getDisplay();
                if (d == null) {
                    DisplayManager dm = (DisplayManager) ctx.getSystemService(Context.DISPLAY_SERVICE);
                    if (dm != null) {
                        d = dm.getDisplay(Display.DEFAULT_DISPLAY);
                    }
                }
                if (d != null) {
                    DisplayCutout cutout = d.getCutout();
                    fallbackPx = Math.max(fallbackPx, extractCutoutTopPx(cutout, ctx));
                }
            } catch (Throwable ignored) {
            }
            return fallbackPx;
        }

        private int extractCutoutTopPx(DisplayCutout cutout, Context ctx) {
            if (cutout == null) return 0;
            int topPx = Math.max(0, cutout.getSafeInsetTop());
            int screenH = ctx.getResources().getDisplayMetrics().heightPixels;
            int maxTopBand = screenH > 0 ? (screenH / 3) : 600;
            List<Rect> rects = cutout.getBoundingRects();
            if (rects != null) {
                for (Rect r : rects) {
                    if (r != null && r.top <= topPx + 32 && r.bottom > topPx && r.bottom <= maxTopBand) {
                        topPx = Math.max(topPx, r.bottom);
                    }
                }
            }
            return topPx;
        }

        private void seedInitialSafeArea(Context ctx) {
            WindowInsets rootInsets = (hostWindow != null && hostWindow.getDecorView() != null)
                ? hostWindow.getDecorView().getRootWindowInsets()
                : null;
            if (rootInsets != null) {
                applyWindowInsets(rootInsets);
                return;
            }
            float d = displayDensity;
            int topPx = queryFallbackStatusBarHeightPx(ctx);
            int navPx = 0;
            try {
                int navId = ctx.getResources().getIdentifier("navigation_bar_height", "dimen", "android");
                if (navId > 0) {
                    navPx = Math.min(ctx.getResources().getDimensionPixelSize(navId), Math.round(32.0f * d));
                }
            } catch (Throwable ignored) {
            }
            synchronized (safeLock) {
                cachedSafeTop = Math.max(28.0f, topPx / d);
                cachedSafeRight = 0.0f;
                cachedSafeBottom = Math.max(0.0f, navPx / d);
                cachedSafeLeft = 0.0f;
            }
        }

        void setImmersiveFullscreen(boolean enabled) {
            immersiveFullscreen = enabled;
            if (enabled) {
                synchronized (safeLock) {
                    cachedSafeTop = 0.0f;
                    cachedSafeRight = 0.0f;
                    cachedSafeBottom = 0.0f;
                    cachedSafeLeft = 0.0f;
                }
                renderHandler.post(() -> {
                    if (handle != 0) {
                        EuvNativeBridge.nativeSetSafeArea(handle, 0.0f, 0.0f, 0.0f, 0.0f);
                        stepAndPresentOnRenderThread(1.0f / targetFps, true);
                    }
                });
            }
        }

        void requestSafeAreaRefresh() {
            if (hostWindow != null && hostWindow.getDecorView() != null) {
                View decor = hostWindow.getDecorView();
                WindowInsets live = decor.getRootWindowInsets();
                if (live != null) {
                    applyWindowInsets(live);
                }
                decor.requestApplyInsets();
            }
            requestApplyInsets();
        }

        void applyWindowInsets(WindowInsets insets) {
            if (insets == null) return;
            Context ctx = getContext();
            refreshDensity(ctx);
            int safeTypes = WindowInsets.Type.statusBars()
                | WindowInsets.Type.displayCutout()
                | WindowInsets.Type.navigationBars()
                | WindowInsets.Type.systemBars()
                | WindowInsets.Type.mandatorySystemGestures();
            Insets visibleBars = insets.getInsets(safeTypes);
            Insets ignoringBars = insets.getInsetsIgnoringVisibility(
                WindowInsets.Type.statusBars()
                    | WindowInsets.Type.displayCutout()
                    | WindowInsets.Type.navigationBars()
                    | WindowInsets.Type.systemBars()
            );
            Insets ime = insets.getInsets(WindowInsets.Type.ime());
            boolean imeVisible = ime.bottom > 0;
            boolean imeHiddenNow = imeWasVisible && !imeVisible;
            imeWasVisible = imeVisible;
            float d = displayDensity;
            float top;
            float right;
            float bottom;
            float left;
            if (immersiveFullscreen) {
                top = 0.0f;
                right = 0.0f;
                bottom = imeVisible ? (ime.bottom / d) : 0.0f;
                left = 0.0f;
            } else {
                int cutoutTopPx = extractCutoutTopPx(insets.getDisplayCutout(), ctx);
                int fallbackTopPx = queryFallbackStatusBarHeightPx(ctx);
                int topPx = Math.max(
                    Math.max(visibleBars.top, ignoringBars.top),
                    Math.max(cutoutTopPx, fallbackTopPx)
                );
                int rightPx = Math.max(visibleBars.right, ignoringBars.right);
                int bottomPx = Math.max(Math.max(visibleBars.bottom, ignoringBars.bottom), ime.bottom);
                int leftPx = Math.max(visibleBars.left, ignoringBars.left);
                top = topPx / d;
                right = rightPx / d;
                bottom = bottomPx / d;
                left = leftPx / d;
            }
            float imeHeight = ime.bottom / d;
            boolean changed;
            synchronized (safeLock) {
                changed = Math.abs(top - cachedSafeTop) > 0.25f
                    || Math.abs(right - cachedSafeRight) > 0.25f
                    || Math.abs(bottom - cachedSafeBottom) > 0.25f
                    || Math.abs(left - cachedSafeLeft) > 0.25f;
                cachedSafeTop = top;
                cachedSafeRight = right;
                cachedSafeBottom = bottom;
                cachedSafeLeft = left;
            }
            final boolean shouldRedraw = changed;
            renderHandler.post(() -> {
                if (handle != 0) {
                    EuvNativeBridge.nativeKeyboardChanged(handle, imeVisible ? 1 : 0, imeHeight);
                    if (imeHiddenNow) {
                        EuvNativeBridge.nativeImeDisabled(handle);
                    }
                    EuvNativeBridge.nativeSetSafeArea(handle, top, right, bottom, left);
                    if (shouldRedraw && surfaceReady) {
                        stepAndPresentOnRenderThread(1.0f / targetFps, true);
                    }
                }
            });
        }

        private void refreshDensity(Context ctx) {
            displayDensity = Math.max(1.0f, ctx.getResources().getDisplayMetrics().density);
            renderScale = Math.max(1.0f, Math.min(4.5f, displayDensity));
        }

        void updateTargetRefreshRate(float hz) {
            float clamped = Math.max(60.0f, Math.min(480.0f, hz));
            targetFps = clamped;
            applyViewRequestedFrameRate(clamped);
            SurfaceHolder holder = getHolder();
            if (holder != null && holder.getSurface() != null && holder.getSurface().isValid()) {
                applySurfaceFrameRate(holder.getSurface());
            }
            renderHandler.post(() -> {
                if (handle != 0) {
                    EuvNativeBridge.nativeUpdateRefreshRate(handle, clamped);
                }
            });
        }

        private void applySurfaceFrameRate(Surface surface) {
            if (surface != null && Build.VERSION.SDK_INT >= 31) {
                try {
                    surface.setFrameRate(
                        targetFps,
                        Surface.FRAME_RATE_COMPATIBILITY_FIXED_SOURCE,
                        Surface.CHANGE_FRAME_RATE_ONLY_IF_SEAMLESS
                    );
                } catch (Throwable ignored) {
                }
            } else if (surface != null && Build.VERSION.SDK_INT >= 30) {
                try {
                    surface.setFrameRate(targetFps, Surface.FRAME_RATE_COMPATIBILITY_FIXED_SOURCE);
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
                if (Build.VERSION.SDK_INT >= 30) {
                    WindowInsetsController ctrl = getWindowInsetsController();
                    if (ctrl != null) {
                        ctrl.show(WindowInsets.Type.ime());
                    }
                }
                if (!shown) {
                    postDelayed(() -> {
                        requestFocus();
                        imm.restartInput(this);
                        imm.showSoftInput(this, InputMethodManager.SHOW_IMPLICIT);
                        if (Build.VERSION.SDK_INT >= 30) {
                            WindowInsetsController ctrl = getWindowInsetsController();
                            if (ctrl != null) {
                                ctrl.show(WindowInsets.Type.ime());
                            }
                        }
                    }, 40L);
                }
            } else {
                if (Build.VERSION.SDK_INT >= 30) {
                    WindowInsetsController ctrl = getWindowInsetsController();
                    if (ctrl != null) {
                        ctrl.hide(WindowInsets.Type.ime());
                    }
                }
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
            Surface surface = holder.getSurface();
            applySurfaceFrameRate(surface);
            int w = getWidth();
            int h = getHeight();
            if (w > 0 && h > 0 && surface != null && surface.isValid()) {
                initializeSurfaceAndPresent(surface, w, h);
            } else {
                post(this::requestSafeAreaRefresh);
            }
        }

        private void ensureLiveHandleOnRenderThread(int width, int height, float scale) {
            if (destroyed) return;
            final float initTop;
            final float initRight;
            final float initBottom;
            final float initLeft;
            synchronized (safeLock) {
                initTop = cachedSafeTop;
                initRight = cachedSafeRight;
                initBottom = cachedSafeBottom;
                initLeft = cachedSafeLeft;
            }
            if (handle != 0 && EuvNativeBridge.nativeIsRunning(handle) == 0) {
                long stale = handle;
                handle = 0;
                EuvNativeBridge.nativeSetSurface(stale, null, 0, 0, scale);
                EuvNativeBridge.nativeDestroy(stale);
            }
            if (handle == 0) {
                handle = EuvNativeBridge.nativeCreate(
                    Math.max(200, width),
                    Math.max(200, height),
                    scale,
                    targetFps,
                    initTop,
                    initRight,
                    initBottom,
                    initLeft
                );
            } else {
                EuvNativeBridge.nativeUpdateRefreshRate(handle, targetFps);
                EuvNativeBridge.nativeSetSafeArea(handle, initTop, initRight, initBottom, initLeft);
            }
        }

        private void initializeSurfaceAndPresent(Surface surface, int width, int height) {
            if (destroyed) return;
            refreshDensity(getContext());
            surfaceW = width;
            surfaceH = height;
            applySurfaceFrameRate(surface);
            float scale = renderScale;
            renderHandler.post(() -> {
                if (destroyed) return;
                ensureLiveHandleOnRenderThread(width, height, scale);
                if (handle == 0) return;
                EuvNativeBridge.nativeSetSurface(handle, surface, width, height, scale);
                surfaceReady = true;
                if (activityResumed && !running) {
                    startLoopOnRenderThread();
                }
                post(this::requestSafeAreaRefresh);
                stepAndPresentOnRenderThread(1.0f / targetFps, true);
            });
        }

        @Override
        public void surfaceChanged(SurfaceHolder holder, int format, int width, int height) {
            if (destroyed || width <= 0 || height <= 0) return;
            if (surfaceReady && surfaceW == width && surfaceH == height && handle != 0) {
                applySurfaceFrameRate(holder.getSurface());
                post(this::requestSafeAreaRefresh);
                return;
            }
            initializeSurfaceAndPresent(holder.getSurface(), width, height);
        }

        @Override
        public void surfaceDestroyed(SurfaceHolder holder) {
            if (destroyed) return;
            synchronized (moveLock) {
                hasPendingMove = false;
                pendingMoveCount = 0;
            }
            CountDownLatch latch = new CountDownLatch(1);
            float scale = renderScale;
            boolean posted = renderHandler.post(() -> {
                surfaceReady = false;
                if (handle != 0) {
                    EuvNativeBridge.nativeTouchCancel(handle, -1);
                    EuvNativeBridge.nativeSetSurface(handle, null, 0, 0, scale);
                }
                latch.countDown();
            });
            if (!posted) return;
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

        void dispatchSystemLifecycle(int tag) {
            renderHandler.post(() -> {
                if (handle != 0) {
                    EuvNativeBridge.nativeSystemLifecycle(handle, tag);
                }
            });
        }

        void dispatchSystemLifecycleSync(int tag) {
            CountDownLatch latch = new CountDownLatch(1);
            renderHandler.post(() -> {
                if (handle != 0) {
                    EuvNativeBridge.nativeSystemLifecycle(handle, tag);
                }
                latch.countDown();
            });
            try {
                latch.await(250L, TimeUnit.MILLISECONDS);
            } catch (InterruptedException ignored) {
                Thread.currentThread().interrupt();
            }
        }

        void dispatchFocusChanged(boolean focused) {
            renderHandler.post(() -> {
                if (handle != 0) {
                    EuvNativeBridge.nativeFocusChanged(handle, focused ? 1 : 0);
                }
            });
        }

        void dispatchThemeChanged(boolean isDark) {
            renderHandler.post(() -> {
                if (handle != 0) {
                    EuvNativeBridge.nativeThemeChanged(handle, isDark ? 1 : 0);
                }
            });
        }

        void dispatchMemoryWarning(int level) {
            renderHandler.post(() -> {
                if (handle != 0) {
                    EuvNativeBridge.nativeMemoryWarning(handle, level);
                }
            });
        }

        void dispatchLowPowerChanged(boolean enabled) {
            renderHandler.post(() -> {
                if (handle != 0) {
                    EuvNativeBridge.nativeLowPowerChanged(handle, enabled ? 1 : 0);
                }
            });
        }

        void dispatchLocaleChanged(String localeTag) {
            byte[] utf8 = localeTag.getBytes(StandardCharsets.UTF_8);
            if (utf8.length == 0) return;
            renderHandler.post(() -> {
                if (handle != 0) {
                    EuvNativeBridge.nativeLocaleChanged(handle, utf8);
                }
            });
        }

        void dispatchDeepLink(String uri) {
            byte[] utf8 = uri.getBytes(StandardCharsets.UTF_8);
            if (utf8.length == 0) return;
            renderHandler.post(() -> {
                if (handle != 0) {
                    EuvNativeBridge.nativeDeepLink(handle, utf8);
                }
            });
        }

        void dispatchClipboardChanged() {
            renderHandler.post(() -> {
                if (handle != 0) {
                    EuvNativeBridge.nativeClipboardChanged(handle);
                }
            });
        }

        private void startLoopOnRenderThread() {
            if (destroyed) return;
            activeLoopGen += 1L;
            running = true;
            lastNanos = 0;
            Choreographer ch = Choreographer.getInstance();
            ch.removeFrameCallback(renderFrameCallback);
            ch.postFrameCallback(renderFrameCallback);
        }

        private void stopLoopOnRenderThread() {
            activeLoopGen += 1L;
            running = false;
            Choreographer.getInstance().removeFrameCallback(renderFrameCallback);
        }

        void onActivityResumed() {
            if (destroyed) return;
            activityResumed = true;
            SurfaceHolder holder = getHolder();
            Surface liveSurface = (holder != null) ? holder.getSurface() : null;
            int curW = surfaceW > 0 ? surfaceW : getWidth();
            int curH = surfaceH > 0 ? surfaceH : getHeight();
            float scale = renderScale;
            renderHandler.post(() -> {
                if (destroyed) return;
                if (handle != 0 && EuvNativeBridge.nativeIsRunning(handle) == 0) {
                    ensureLiveHandleOnRenderThread(curW, curH, scale);
                    if (handle != 0 && liveSurface != null && liveSurface.isValid() && curW > 0 && curH > 0) {
                        EuvNativeBridge.nativeSetSurface(handle, liveSurface, curW, curH, scale);
                        surfaceReady = true;
                    }
                }
                if (handle != 0) {
                    EuvNativeBridge.nativeSystemLifecycle(handle, 3);
                }
                startLoopOnRenderThread();
                if (handle != 0 && surfaceReady) {
                    stepAndPresentOnRenderThread(1.0f / targetFps, true);
                }
            });
        }

        void onActivityPaused() {
            activityResumed = false;
            if (destroyed) return;
            synchronized (moveLock) {
                hasPendingMove = false;
                pendingMoveCount = 0;
            }
            renderHandler.post(() -> {
                if (handle != 0) {
                    EuvNativeBridge.nativeTouchCancel(handle, -1);
                    EuvNativeBridge.nativeSystemLifecycle(handle, 4);
                }
                stopLoopOnRenderThread();
            });
        }

        void startLoop() {
            if (destroyed) return;
            renderHandler.post(this::startLoopOnRenderThread);
        }

        void stopLoop() {
            if (destroyed) return;
            renderHandler.post(this::stopLoopOnRenderThread);
        }

        void shutdown() {
            if (destroyed) return;
            destroyed = true;
            activityResumed = false;
            synchronized (moveLock) {
                hasPendingMove = false;
                pendingMoveCount = 0;
            }
            float scale = renderScale;
            CountDownLatch latch = new CountDownLatch(1);
            boolean posted = renderHandler.post(() -> {
                stopLoopOnRenderThread();
                surfaceReady = false;
                if (handle != 0) {
                    long closing = handle;
                    handle = 0;
                    EuvNativeBridge.nativeTouchCancel(closing, -1);
                    EuvNativeBridge.nativeSystemLifecycle(closing, 6);
                    EuvNativeBridge.nativeSetSurface(closing, null, 0, 0, scale);
                    EuvNativeBridge.nativeDestroy(closing);
                }
                latch.countDown();
            });
            if (posted) {
                try {
                    latch.await(1000L, TimeUnit.MILLISECONDS);
                } catch (InterruptedException ignored) {
                    Thread.currentThread().interrupt();
                }
            }
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
                if (!firstPresentNotified) {
                    firstPresentNotified = true;
                    if (onFirstFramePresented != null) {
                        onFirstFramePresented.run();
                    }
                }
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
            if (destroyed) return false;
            int action = event.getActionMasked();
            int actionIdx = event.getActionIndex();
            float d = renderScale;
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
                        if (destroyed || handle == 0) return;
                        if (activityResumed && !running) {
                            startLoopOnRenderThread();
                        }
                        EuvNativeBridge.nativeTouchSyncMask(handle, 0L);
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
                        if (destroyed) return;
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
                        if (destroyed) return;
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
                        if (destroyed) return;
                        flushPendingMovesOnRenderThread();
                        if (handle == 0) return;
                        EuvNativeBridge.nativeTouchUp(handle, pid, x, y);
                        EuvNativeBridge.nativeTouchSyncMask(handle, 0L);
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
    android:versionCode="7"
    android:versionName="0.44.0">
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
        android:label="Euv Studio"
        android:usesCleartextTraffic="true"
        android:icon="@mipmap/ic_launcher"
        android:roundIcon="@mipmap/ic_launcher"
        android:theme="@style/EuvLaunchTheme"
        android:hardwareAccelerated="true">
        <activity
            android:name=".EuvActivity"
            android:exported="true"
            android:windowSoftInputMode="adjustResize|stateHidden"
            android:configChanges="orientation|screenSize|screenLayout|smallestScreenSize|density|fontScale|uiMode|locale|keyboard|keyboardHidden|navigation">
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
mkdir -p "${RES_DIR}/mipmap-xxhdpi" "${RES_DIR}/mipmap-xxxhdpi" \
  "${RES_DIR}/values" "${RES_DIR}/values-night" "${RES_DIR}/values-v31" "${RES_DIR}/values-night-v31"
cat > "${RES_DIR}/values/styles.xml" <<'EOF'
<?xml version="1.0" encoding="utf-8"?>
<resources>
    <color name="euv_launch_bg">#FAFAFA</color>
    <style name="EuvLaunchTheme" parent="@android:style/Theme.DeviceDefault.Light.NoActionBar">
        <item name="android:windowBackground">@color/euv_launch_bg</item>
        <item name="android:statusBarColor">@android:color/transparent</item>
        <item name="android:navigationBarColor">@android:color/transparent</item>
    </style>
</resources>
EOF
cat > "${RES_DIR}/values-night/styles.xml" <<'EOF'
<?xml version="1.0" encoding="utf-8"?>
<resources>
    <color name="euv_launch_bg">#09090B</color>
    <style name="EuvLaunchTheme" parent="@android:style/Theme.DeviceDefault.NoActionBar">
        <item name="android:windowBackground">@color/euv_launch_bg</item>
        <item name="android:statusBarColor">@android:color/transparent</item>
        <item name="android:navigationBarColor">@android:color/transparent</item>
    </style>
</resources>
EOF
cat > "${RES_DIR}/values-v31/styles.xml" <<'EOF'
<?xml version="1.0" encoding="utf-8"?>
<resources>
    <style name="EuvLaunchTheme" parent="@android:style/Theme.DeviceDefault.Light.NoActionBar">
        <item name="android:windowBackground">@color/euv_launch_bg</item>
        <item name="android:windowSplashScreenBackground">@color/euv_launch_bg</item>
        <item name="android:statusBarColor">@android:color/transparent</item>
        <item name="android:navigationBarColor">@android:color/transparent</item>
    </style>
</resources>
EOF
cat > "${RES_DIR}/values-night-v31/styles.xml" <<'EOF'
<?xml version="1.0" encoding="utf-8"?>
<resources>
    <style name="EuvLaunchTheme" parent="@android:style/Theme.DeviceDefault.NoActionBar">
        <item name="android:windowBackground">@color/euv_launch_bg</item>
        <item name="android:windowSplashScreenBackground">@color/euv_launch_bg</item>
        <item name="android:statusBarColor">@android:color/transparent</item>
        <item name="android:navigationBarColor">@android:color/transparent</item>
    </style>
</resources>
EOF
python3 "${REPO_ROOT}/scripts/env/generate-icon.py" "${RES_DIR}/mipmap-xxhdpi/ic_launcher.png" 144 "${EUV_APP_ICON:-}"
python3 "${REPO_ROOT}/scripts/env/generate-icon.py" "${RES_DIR}/mipmap-xxxhdpi/ic_launcher.png" 192 "${EUV_APP_ICON:-}"
"${AAPT2}" compile --dir "${RES_DIR}" -o "${RES_ZIP}"

UNALIGNED_APK="${BUILD_DIR}/unaligned.apk"
ALIGNED_APK="${BUILD_DIR}/aligned.apk"
rm -f "${UNALIGNED_APK}" "${ALIGNED_APK}" "${APK_OUT}"
"${AAPT2}" link \
  -I "${ANDROID_JAR}" \
  --manifest "${BUILD_DIR}/AndroidManifest.xml" \
  --auto-add-overlay \
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
