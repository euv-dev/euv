#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../env/common.sh
source "${SCRIPT_DIR}/../env/common.sh"
load_local_env

SIM_UDID=""
if [ "${EUV_IOS_SKIP_LAUNCH:-0}" != "1" ]; then
  SIM_UDID="${EUV_IOS_SIM_UDID:-$(xcrun simctl list devices booted | grep -oE '[0-9A-F-]{36}' | head -n 1 || true)}"
  if [ -z "${SIM_UDID}" ]; then
    SIM_UDID="$(xcrun simctl list devices available | grep 'iPhone' | grep -oE '[0-9A-F-]{36}' | head -n 1)"
  fi
fi
BUILD_DIR="${REPO_ROOT}/target/mobile-hosts/ios-host"
APP_BUNDLE="${REPO_ROOT}/dist/ios/EuvShowcase.app"
SDK_PATH="$(xcrun --sdk iphonesimulator --show-sdk-path)"
CLANG="$(xcrun --sdk iphonesimulator --find clang)"

mkdir -p "${BUILD_DIR}/src"
cat > "${BUILD_DIR}/Cargo.toml" <<EOF
[workspace]

[package]
name = "euv-ios-host"
version = "0.45.0"
edition = "2024"

[lib]
crate-type = ["staticlib"]

[dependencies]
euv = { path = "${REPO_ROOT}" }
euv-example = { path = "${REPO_ROOT}/example" }

[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
panic = "abort"
EOF

cat > "${BUILD_DIR}/src/lib.rs" <<'EOF'
use std::ffi::{CStr, CString, c_char};
use euv::{
    App, ImeEvent, KeyCode, KeyEvent, MemoryPressure, Modifiers, NativeSystemFont, NetworkStatus,
    PlatformKind, PointerEvent, RenderMode, SafeAreaInsets, SystemEvent, SystemTheme, WindowEvent,
    WindowId, register_native_action_bridge, reset_reactive_runtime,
};
use euv_example::{ShowcaseState, create_showcase_app_for_platform};

unsafe extern "C" {
    fn euv_ios_invoke_native_action(
        kind: *const c_char,
        arg1: *const c_char,
        arg2: *const c_char,
    ) -> i32;
}

fn ios_native_action_bridge(kind: &str, arg1: &str, arg2: &str) -> bool {
    let Ok(c_kind) = CString::new(kind) else {
        return false;
    };
    let Ok(c_a1) = CString::new(arg1) else {
        return false;
    };
    let Ok(c_a2) = CString::new(arg2) else {
        return false;
    };
    unsafe { euv_ios_invoke_native_action(c_kind.as_ptr(), c_a1.as_ptr(), c_a2.as_ptr()) != 0 }
}

pub struct EuvIosHandle {
    app: App,
    _state: ShowcaseState,
    wid: WindowId,
    width: u32,
    height: u32,
}

#[unsafe(no_mangle)]
pub extern "C" fn euv_ios_create(
    phys_width: u32,
    phys_height: u32,
    scale: f32,
    is_ipad: i32,
    refresh_hz: f32,
    safe_top: f32,
    safe_right: f32,
    safe_bottom: f32,
    safe_left: f32,
) -> *mut EuvIosHandle {
    register_native_action_bridge(ios_native_action_bridge);
    reset_reactive_runtime();
    let platform: PlatformKind = if is_ipad != 0 {
        PlatformKind::IpadOs
    } else {
        PlatformKind::Ios
    };
    let Ok((mut app, state)) = create_showcase_app_for_platform(platform, RenderMode::Software) else {
        return std::ptr::null_mut();
    };
    app.get_mut_renderer()
        .get_mut_software()
        .set_font_provider(NativeSystemFont::system());
    let hz: f32 = refresh_hz.clamp(24.0, 480.0);
    app.update_display_refresh_rate_hz(hz);
    let wid: WindowId = app.get_window_state().get_id();
    let pw: u32 = phys_width.max(200);
    let ph: u32 = phys_height.max(200);
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
    let handle = Box::new(EuvIosHandle {
        app,
        _state: state,
        wid,
        width: 0,
        height: 0,
    });
    Box::into_raw(handle)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_destroy(handle: *mut EuvIosHandle) {
    if handle.is_null() {
        return;
    }
    let boxed: Box<EuvIosHandle> = unsafe { Box::from_raw(handle) };
    drop(boxed);
    reset_reactive_runtime();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_is_running(handle: *mut EuvIosHandle) -> i32 {
    if handle.is_null() {
        return 0;
    }
    let h: &EuvIosHandle = unsafe { &*handle };
    if h.app.get_running() { 1 } else { 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_update_refresh_rate(handle: *mut EuvIosHandle, refresh_hz: f32) {
    if handle.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    h.app
        .update_display_refresh_rate_hz(refresh_hz.clamp(24.0, 480.0));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_ime_disabled(handle: *mut EuvIosHandle) {
    if handle.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    h.app
        .dispatch_window_event(WindowEvent::Ime(h.wid, ImeEvent::Disabled));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_system_lifecycle(handle: *mut EuvIosHandle, tag: i32) {
    if handle.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    let ev: SystemEvent = match tag {
        1 => SystemEvent::Appear,
        2 => SystemEvent::Disappear,
        3 => SystemEvent::Foreground,
        4 => SystemEvent::Inactive,
        5 => SystemEvent::Background,
        6 => SystemEvent::Terminate,
        _ => return,
    };
    h.app.dispatch_system_event(ev);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_focus(handle: *mut EuvIosHandle, focused: i32) {
    if handle.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    h.app
        .dispatch_system_event(SystemEvent::FocusChanged(focused != 0));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_keyboard(handle: *mut EuvIosHandle, visible: i32, height: f32) {
    if handle.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    h.app
        .dispatch_system_event(SystemEvent::KeyboardVisibilityChanged {
            visible: visible != 0,
            height: height.max(0.0),
        });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_theme(handle: *mut EuvIosHandle, is_dark: i32) {
    if handle.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    let mode: SystemTheme = if is_dark != 0 {
        SystemTheme::Dark
    } else {
        SystemTheme::Light
    };
    h.app.dispatch_system_event(SystemEvent::ThemeChanged(mode));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_memory_warning(handle: *mut EuvIosHandle) {
    if handle.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    h.app
        .dispatch_system_event(SystemEvent::MemoryWarning(MemoryPressure::Critical));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_low_power(handle: *mut EuvIosHandle, enabled: i32) {
    if handle.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    h.app
        .dispatch_system_event(SystemEvent::LowPowerModeChanged(enabled != 0));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_locale(handle: *mut EuvIosHandle, locale: *const c_char) {
    if handle.is_null() || locale.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    if let Ok(s) = unsafe { CStr::from_ptr(locale) }.to_str()
        && !s.is_empty()
    {
        h.app
            .dispatch_system_event(SystemEvent::LocaleChanged(s.to_string()));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_deep_link(handle: *mut EuvIosHandle, uri: *const c_char) {
    if handle.is_null() || uri.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    if let Ok(s) = unsafe { CStr::from_ptr(uri) }.to_str()
        && !s.is_empty()
    {
        h.app
            .dispatch_system_event(SystemEvent::DeepLink(s.to_string()));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_clipboard_changed(handle: *mut EuvIosHandle) {
    if handle.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    h.app.dispatch_system_event(SystemEvent::ClipboardChanged);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_network(handle: *mut EuvIosHandle, status_tag: i32) {
    if handle.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    let status: NetworkStatus = match status_tag {
        0 => NetworkStatus::Offline,
        1 => NetworkStatus::Online,
        2 => NetworkStatus::Wifi,
        3 => NetworkStatus::Cellular,
        _ => NetworkStatus::Unknown,
    };
    h.app
        .dispatch_system_event(SystemEvent::NetworkStatusChanged(status));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_resize(
    handle: *mut EuvIosHandle,
    phys_width: u32,
    phys_height: u32,
    scale: f32,
) {
    if handle.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    let pw: u32 = phys_width.max(200);
    let ph: u32 = phys_height.max(200);
    let s: f32 = scale.clamp(1.0, 4.5);
    h.app.set_exact_physical_viewport(pw, ph, s);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_set_safe_area(
    handle: *mut EuvIosHandle,
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
) {
    if handle.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    h.app.dispatch_window_event(WindowEvent::SafeAreaChanged(
        h.wid,
        SafeAreaInsets::new(top.max(0.0), right.max(0.0), bottom.max(0.0), left.max(0.0)),
    ));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_touch_down(
    handle: *mut EuvIosHandle,
    pointer_id: u64,
    x: f32,
    y: f32,
) {
    if handle.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    h.app.dispatch_window_event(WindowEvent::PointerDown(
        h.wid,
        PointerEvent::touch(pointer_id.max(1), x, y),
    ));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_touch_move(
    handle: *mut EuvIosHandle,
    pointer_id: u64,
    x: f32,
    y: f32,
) {
    if handle.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    h.app.dispatch_window_event(WindowEvent::PointerMove(
        h.wid,
        PointerEvent::touch(pointer_id.max(1), x, y),
    ));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_touch_up(
    handle: *mut EuvIosHandle,
    pointer_id: u64,
    x: f32,
    y: f32,
) {
    if handle.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    h.app.dispatch_window_event(WindowEvent::PointerUp(
        h.wid,
        PointerEvent::touch(pointer_id.max(1), x, y),
    ));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_touch_cancel(handle: *mut EuvIosHandle, pointer_id: u64) {
    if handle.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    h.app
        .dispatch_window_event(WindowEvent::PointerCancel(h.wid, pointer_id));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_touch_sync(
    handle: *mut EuvIosHandle,
    ids: *const u64,
    count: usize,
) {
    if handle.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    let live: Vec<u64> = if ids.is_null() || count == 0 {
        Vec::new()
    } else {
        let slice: &[u64] = unsafe { std::slice::from_raw_parts(ids, count.min(32)) };
        slice.iter().map(|id: &u64| (*id).max(1)).collect()
    };
    h.app.reconcile_active_touch_ids(&live);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_text_input(handle: *mut EuvIosHandle, text: *const c_char) {
    if handle.is_null() || text.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    if let Ok(s) = unsafe { CStr::from_ptr(text) }.to_str()
        && !s.is_empty()
    {
        h.app
            .dispatch_window_event(WindowEvent::TextInput(h.wid, s.to_string()));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_key_down(handle: *mut EuvIosHandle, key_tag: i32) {
    if handle.is_null() {
        return;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    let code: KeyCode = match key_tag {
        1 => KeyCode::Backspace,
        2 => KeyCode::Enter,
        3 => KeyCode::Tab,
        4 => KeyCode::Escape,
        _ => return,
    };
    h.app.dispatch_window_event(WindowEvent::KeyDown(
        h.wid,
        KeyEvent::new(code, Modifiers::default(), false),
    ));
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn euv_ios_step(
    handle: *mut EuvIosHandle,
    dt: f32,
    out_ptr: *mut *const u8,
    out_w: *mut u32,
    out_h: *mut u32,
    out_dx: *mut u32,
    out_dy: *mut u32,
    out_dw: *mut u32,
    out_dh: *mut u32,
) -> i32 {
    if handle.is_null() {
        return 0;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    let Ok(stats) = h.app.step_game_frame(dt.clamp(1.0 / 480.0, 0.05)) else {
        return 0;
    };
    let prev_w: u32 = h.width;
    let prev_h: u32 = h.height;
    let changed: bool = stats.get_pixels_shaded() > 0 || prev_w == 0;
    let mut dx: u32 = 0;
    let mut dy: u32 = 0;
    let mut dw: u32 = 0;
    let mut dh: u32 = 0;
    if let Some(fb) = h.app.get_renderer().framebuffer() {
        let w: u32 = fb.get_width();
        let ht: u32 = fb.get_height();
        h.width = w;
        h.height = ht;
        if !out_ptr.is_null() {
            unsafe { *out_ptr = fb.get_pixels().as_ptr() };
        }
        dw = w;
        dh = ht;
        if prev_w == w && prev_h == ht {
            let dmg = h.app.last_frame_damage();
            if !dmg.get_full_damage()
                && let Some(r) = dmg.resolve(w, ht)
            {
                let rx0: u32 = (r.get_x().floor().max(0.0) as u32).min(w);
                let ry0: u32 = (r.get_y().floor().max(0.0) as u32).min(ht);
                let rx1: u32 = (r.right().ceil().max(0.0) as u32).clamp(rx0, w);
                let ry1: u32 = (r.bottom().ceil().max(0.0) as u32).clamp(ry0, ht);
                if rx1 > rx0 && ry1 > ry0 {
                    dx = rx0;
                    dy = ry0;
                    dw = rx1 - rx0;
                    dh = ry1 - ry0;
                }
            }
        }
    } else if !out_ptr.is_null() {
        unsafe { *out_ptr = std::ptr::null() };
    }
    if !out_w.is_null() {
        unsafe { *out_w = h.width };
    }
    if !out_h.is_null() {
        unsafe { *out_h = h.height };
    }
    if !out_dx.is_null() {
        unsafe { *out_dx = dx };
    }
    if !out_dy.is_null() {
        unsafe { *out_dy = dy };
    }
    if !out_dw.is_null() {
        unsafe { *out_dw = dw };
    }
    if !out_dh.is_null() {
        unsafe { *out_dh = dh };
    }
    if changed { 1 } else { 0 }
}
EOF

cat > "${BUILD_DIR}/main.m" <<'EOF'
#import <UIKit/UIKit.h>
#import <CoreGraphics/CoreGraphics.h>
#import <Metal/Metal.h>
#import <QuartzCore/QuartzCore.h>
#import <UserNotifications/UserNotifications.h>
#include <stdint.h>

@class EuvViewController;

@interface EuvCanvasView : UIView <UIKeyInput>
@property (nonatomic, assign) BOOL keyboardVisible;
@property (nonatomic, assign) BOOL immersiveFullscreen;
- (void)setDisplayLinkPaused:(BOOL)paused;
- (void)ensureLiveSession;
- (void)shutdownSession;
- (void)syncSafeArea;
- (void)syncViewportScaleAndBounds;
- (void)presentFrameWithDt:(float)dt force:(BOOL)force;
@end

@interface EuvViewController : UIViewController
@property (nonatomic, strong) EuvCanvasView *canvasView;
@property (nonatomic, assign) BOOL immersiveFullscreen;
- (void)applyImmersiveFullscreen:(BOOL)enabled;
@end

static __weak EuvCanvasView *gEuvActiveCanvasView = nil;
static __weak EuvViewController *gEuvActiveViewController = nil;

int32_t euv_ios_invoke_native_action(const char *kind, const char *arg1, const char *arg2) {
    NSString *k = kind ? [NSString stringWithUTF8String:kind] : @"";
    NSString *a1 = arg1 ? [NSString stringWithUTF8String:arg1] : @"";
    NSString *a2 = arg2 ? [NSString stringWithUTF8String:arg2] : @"";
    NSLog(@"[euv-native] ios action kind=%@ arg1=%@ arg2=%@", k, a1, a2);
    dispatch_async(dispatch_get_main_queue(), ^{
        if ([k isEqualToString:@"ime"]) {
            EuvCanvasView *v = gEuvActiveCanvasView;
            if (v) {
                if ([a1 isEqualToString:@"show"]) {
                    if ([v isFirstResponder] && !v.keyboardVisible) {
                        [v resignFirstResponder];
                    }
                    [v becomeFirstResponder];
                } else {
                    [v resignFirstResponder];
                }
            }
        } else if ([k isEqualToString:@"fullscreen"]) {
            BOOL enableFs = [a1 isEqualToString:@"on"];
            EuvViewController *vc = gEuvActiveViewController;
            if (vc) {
                [vc applyImmersiveFullscreen:enableFs];
            } else if (gEuvActiveCanvasView) {
                gEuvActiveCanvasView.immersiveFullscreen = enableFs;
                [gEuvActiveCanvasView syncSafeArea];
            }
        } else if ([k isEqualToString:@"clipboard"]) {
            if (a2.length > 0) {
                [UIPasteboard generalPasteboard].string = a2;
            }
        } else if ([k isEqualToString:@"haptic"]) {
            UIImpactFeedbackGenerator *gen = [[UIImpactFeedbackGenerator alloc] initWithStyle:UIImpactFeedbackStyleMedium];
            [gen prepare];
            [gen impactOccurred];
        } else if ([k isEqualToString:@"notify"] || [k isEqualToString:@"island"]) {
            UNUserNotificationCenter *center = [UNUserNotificationCenter currentNotificationCenter];
            [center requestAuthorizationWithOptions:(UNAuthorizationOptionAlert | UNAuthorizationOptionSound | UNAuthorizationOptionBadge)
                                  completionHandler:^(BOOL granted, NSError * _Nullable err) {
                if (granted) {
                    UNMutableNotificationContent *content = [[UNMutableNotificationContent alloc] init];
                    content.title = a1.length > 0 ? a1 : @"Euv Studio";
                    content.body = a2.length > 0 ? a2 : @"Native iOS notification dispatched";
                    UNNotificationRequest *req = [UNNotificationRequest requestWithIdentifier:[[NSUUID UUID] UUIDString] content:content trigger:nil];
                    [center addNotificationRequest:req withCompletionHandler:nil];
                }
            }];
        }
    });
    return 1;
}

typedef struct EuvIosHandle EuvIosHandle;
extern EuvIosHandle *euv_ios_create(uint32_t phys_width, uint32_t phys_height, float scale, int32_t is_ipad, float refresh_hz, float safe_top, float safe_right, float safe_bottom, float safe_left);
extern void euv_ios_destroy(EuvIosHandle *handle);
extern int32_t euv_ios_is_running(EuvIosHandle *handle);
extern void euv_ios_update_refresh_rate(EuvIosHandle *handle, float refresh_hz);
extern void euv_ios_ime_disabled(EuvIosHandle *handle);
extern void euv_ios_system_lifecycle(EuvIosHandle *handle, int32_t tag);
extern void euv_ios_focus(EuvIosHandle *handle, int32_t focused);
extern void euv_ios_keyboard(EuvIosHandle *handle, int32_t visible, float height);
extern void euv_ios_theme(EuvIosHandle *handle, int32_t is_dark);
extern void euv_ios_memory_warning(EuvIosHandle *handle);
extern void euv_ios_low_power(EuvIosHandle *handle, int32_t enabled);
extern void euv_ios_locale(EuvIosHandle *handle, const char *locale);
extern void euv_ios_deep_link(EuvIosHandle *handle, const char *uri);
extern void euv_ios_clipboard_changed(EuvIosHandle *handle);
extern void euv_ios_network(EuvIosHandle *handle, int32_t status_tag);
extern void euv_ios_resize(EuvIosHandle *handle, uint32_t phys_width, uint32_t phys_height, float scale);
extern void euv_ios_set_safe_area(EuvIosHandle *handle, float top, float right, float bottom, float left);
extern void euv_ios_touch_down(EuvIosHandle *handle, uint64_t pointer_id, float x, float y);
extern void euv_ios_touch_move(EuvIosHandle *handle, uint64_t pointer_id, float x, float y);
extern void euv_ios_touch_up(EuvIosHandle *handle, uint64_t pointer_id, float x, float y);
extern void euv_ios_touch_cancel(EuvIosHandle *handle, uint64_t pointer_id);
extern void euv_ios_touch_sync(EuvIosHandle *handle, const uint64_t *ids, size_t count);
extern void euv_ios_text_input(EuvIosHandle *handle, const char *text);
extern void euv_ios_key_down(EuvIosHandle *handle, int32_t key_tag);
extern int32_t euv_ios_step(EuvIosHandle *handle, float dt, const uint8_t **out_ptr, uint32_t *out_w, uint32_t *out_h, uint32_t *out_dx, uint32_t *out_dy, uint32_t *out_dw, uint32_t *out_dh);

@interface EuvCanvasView ()
@property (nonatomic, assign) EuvIosHandle *handle;
@property (nonatomic, strong) CADisplayLink *displayLink;
@property (nonatomic, assign) CFTimeInterval lastTimestamp;
@property (nonatomic, assign) CGColorSpaceRef colorSpace;
@property (nonatomic, strong) id<MTLDevice> metalDevice;
@property (nonatomic, strong) id<MTLCommandQueue> metalQueue;
@property (nonatomic, strong) id<MTLTexture> stagingTexture;
@property (nonatomic, assign) uint32_t stagingW;
@property (nonatomic, assign) uint32_t stagingH;
@property (nonatomic, assign) CGFloat lastLogicalW;
@property (nonatomic, assign) CGFloat lastLogicalH;
@property (nonatomic, assign) CGFloat lastScale;
@property (nonatomic, assign) float targetRefreshHz;
@property (nonatomic, assign) UIEdgeInsets lastDispatchedSafeArea;
@property (nonatomic, assign) uint64_t renderedFrames;
@property (nonatomic, assign) double accumRenderMs;
@property (nonatomic, assign) double maxRenderMs;
@property (nonatomic, assign) uint64_t droppedFrames;
@end

@implementation EuvCanvasView

+ (Class)layerClass {
    return [CAMetalLayer class];
}

- (void)shutdownSession {
    [[NSNotificationCenter defaultCenter] removeObserver:self];
    if (_displayLink) {
        [_displayLink invalidate];
        _displayLink = nil;
    }
    if (gEuvActiveCanvasView == self) {
        gEuvActiveCanvasView = nil;
    }
    if (_handle) {
        EuvIosHandle *closing = _handle;
        _handle = NULL;
        euv_ios_touch_cancel(closing, 0);
        euv_ios_system_lifecycle(closing, 6);
        euv_ios_destroy(closing);
    }
    _stagingTexture = nil;
    _stagingW = 0;
    _stagingH = 0;
}

- (void)ensureLiveSession {
    if (_handle && euv_ios_is_running(_handle) != 0) {
        return;
    }
    if (_handle) {
        EuvIosHandle *stale = _handle;
        _handle = NULL;
        euv_ios_destroy(stale);
    }
    CGFloat rawScale = self.traitCollection.displayScale > 0
        ? self.traitCollection.displayScale
        : (self.window.screen.scale > 0 ? self.window.screen.scale : UIScreen.mainScreen.scale);
    CGFloat scale = fmin(4.5, rawScale);
    CGFloat w = self.bounds.size.width > 0 ? self.bounds.size.width : 393.0;
    CGFloat h = self.bounds.size.height > 0 ? self.bounds.size.height : 852.0;
    uint32_t physW = (uint32_t)llround(w * scale);
    uint32_t physH = (uint32_t)llround(h * scale);
    int32_t isIpad = (UIDevice.currentDevice.userInterfaceIdiom == UIUserInterfaceIdiomPad) ? 1 : 0;
    UIEdgeInsets initSafe = [self resolveEffectiveSafeArea];
    _lastDispatchedSafeArea = initSafe;
    _handle = euv_ios_create(
        physW,
        physH,
        (float)scale,
        isIpad,
        _targetRefreshHz,
        (float)initSafe.top,
        (float)initSafe.right,
        (float)initSafe.bottom,
        (float)initSafe.left
    );
    [self syncThemeTrait];
}

- (void)dealloc {
    [self shutdownSession];
    if (_colorSpace) {
        CGColorSpaceRelease(_colorSpace);
        _colorSpace = NULL;
    }
}

- (void)setDisplayLinkPaused:(BOOL)paused {
    if (_displayLink) {
        _displayLink.paused = paused;
        if (!paused) {
            _lastTimestamp = 0.0;
        }
    }
}

- (float)nominalFrameDt {
    float hz = _targetRefreshHz >= 24.0f ? _targetRefreshHz : 60.0f;
    return 1.0f / hz;
}

- (void)onStatusBarOrCaptureChanged:(NSNotification *)note {
    (void)note;
    [self syncSafeArea];
    [self presentFrameWithDt:[self nominalFrameDt] force:YES];
}

- (void)onKeyboardWillShow:(NSNotification *)note {
    _keyboardVisible = YES;
    CGFloat kbH = 0.0;
    NSValue *rectVal = note.userInfo[UIKeyboardFrameEndUserInfoKey];
    if (rectVal) {
        kbH = [rectVal CGRectValue].size.height;
    }
    if (_handle) {
        euv_ios_keyboard(_handle, 1, (float)kbH);
    }
}

- (void)onKeyboardDidHide:(NSNotification *)note {
    (void)note;
    _keyboardVisible = NO;
    if (_handle) {
        euv_ios_keyboard(_handle, 0, 0.0f);
        euv_ios_ime_disabled(_handle);
    }
}

- (void)onMemoryWarning:(NSNotification *)note {
    (void)note;
    if (_handle) {
        euv_ios_memory_warning(_handle);
    }
}

- (void)onPowerStateChanged:(NSNotification *)note {
    (void)note;
    if (_handle) {
        BOOL lowPower = [[NSProcessInfo processInfo] isLowPowerModeEnabled];
        euv_ios_low_power(_handle, lowPower ? 1 : 0);
    }
}

- (void)onPasteboardChanged:(NSNotification *)note {
    (void)note;
    if (_handle) {
        euv_ios_clipboard_changed(_handle);
    }
}

- (void)onLocaleChanged:(NSNotification *)note {
    (void)note;
    if (_handle) {
        NSString *loc = [[NSLocale currentLocale] localeIdentifier];
        if (loc.length > 0) {
            euv_ios_locale(_handle, [loc UTF8String]);
        }
    }
}

- (void)syncThemeTrait {
    if (!_handle) return;
    BOOL isDark = (self.traitCollection.userInterfaceStyle == UIUserInterfaceStyleDark);
    euv_ios_theme(_handle, isDark ? 1 : 0);
}

- (UIEdgeInsets)resolveEffectiveSafeArea {
    if (_immersiveFullscreen) {
        return UIEdgeInsetsZero;
    }
    UIEdgeInsets insets = self.safeAreaInsets;
    if (self.window) {
        UIEdgeInsets winInsets = self.window.safeAreaInsets;
        if (winInsets.top > insets.top) insets.top = winInsets.top;
        if (winInsets.right > insets.right) insets.right = winInsets.right;
        if (winInsets.bottom > insets.bottom) insets.bottom = winInsets.bottom;
        if (winInsets.left > insets.left) insets.left = winInsets.left;
        if (self.window.windowScene && self.window.windowScene.statusBarManager) {
            CGFloat sbH = self.window.windowScene.statusBarManager.statusBarFrame.size.height;
            if (sbH > insets.top) {
                insets.top = sbH;
            }
        }
    }
    if (@available(iOS 26.0, *)) {
        UIViewLayoutRegion *region = [UIViewLayoutRegion safeAreaLayoutRegionWithCornerAdaptation:UIViewLayoutRegionAdaptivityAxisVertical];
        UIEdgeInsets adapted = [self edgeInsetsForLayoutRegion:region];
        if (adapted.top > insets.top) insets.top = adapted.top;
        if (adapted.right > insets.right) insets.right = adapted.right;
        if (adapted.bottom > insets.bottom) insets.bottom = adapted.bottom;
        if (adapted.left > insets.left) insets.left = adapted.left;
    }
    BOOL isIpad = (UIDevice.currentDevice.userInterfaceIdiom == UIUserInterfaceIdiomPad);
    if (isIpad) {
        if (insets.top < 24.0) {
            insets.top = 40.0;
        }
    } else if (insets.top < 20.0) {
        CGFloat screenH = fmax(UIScreen.mainScreen.bounds.size.width, UIScreen.mainScreen.bounds.size.height);
        insets.top = (screenH >= 812.0) ? 47.0 : 20.0;
        if (insets.bottom < 1.0 && screenH >= 812.0) {
            insets.bottom = 34.0;
        }
    }
    return insets;
}

- (instancetype)initWithFrame:(CGRect)frame {
    self = [super initWithFrame:frame];
    if (self) {
        self.backgroundColor = [UIColor systemBackgroundColor];
        self.opaque = YES;
        self.multipleTouchEnabled = YES;
        gEuvActiveCanvasView = self;
        _keyboardVisible = NO;
        _immersiveFullscreen = NO;
        _lastDispatchedSafeArea = UIEdgeInsetsMake(-1, -1, -1, -1);
        [[NSNotificationCenter defaultCenter] addObserver:self
                                                 selector:@selector(onKeyboardWillShow:)
                                                     name:UIKeyboardWillShowNotification
                                                   object:nil];
        [[NSNotificationCenter defaultCenter] addObserver:self
                                                 selector:@selector(onKeyboardDidHide:)
                                                     name:UIKeyboardDidHideNotification
                                                   object:nil];
        [[NSNotificationCenter defaultCenter] addObserver:self
                                                 selector:@selector(onMemoryWarning:)
                                                     name:UIApplicationDidReceiveMemoryWarningNotification
                                                   object:nil];
        [[NSNotificationCenter defaultCenter] addObserver:self
                                                 selector:@selector(onPowerStateChanged:)
                                                     name:NSProcessInfoPowerStateDidChangeNotification
                                                   object:nil];
        [[NSNotificationCenter defaultCenter] addObserver:self
                                                 selector:@selector(onPasteboardChanged:)
                                                     name:UIPasteboardChangedNotification
                                                   object:nil];
        [[NSNotificationCenter defaultCenter] addObserver:self
                                                 selector:@selector(onLocaleChanged:)
                                                     name:NSCurrentLocaleDidChangeNotification
                                                   object:nil];
        [[NSNotificationCenter defaultCenter] addObserver:self
                                                 selector:@selector(onStatusBarOrCaptureChanged:)
                                                     name:UIScreenCapturedDidChangeNotification
                                                   object:nil];
        [[NSNotificationCenter defaultCenter] addObserver:self
                                                 selector:@selector(onStatusBarOrCaptureChanged:)
                                                     name:UIApplicationDidChangeStatusBarFrameNotification
                                                   object:nil];
        _colorSpace = CGColorSpaceCreateDeviceRGB();
        CGFloat scale = fmin(4.5, UIScreen.mainScreen.scale);
        self.layer.contentsScale = scale;
        self.layer.magnificationFilter = kCAFilterNearest;
        self.layer.minificationFilter = kCAFilterLinear;
        if ([self.layer isKindOfClass:[CAMetalLayer class]]) {
            _metalDevice = MTLCreateSystemDefaultDevice();
            if (_metalDevice) {
                CAMetalLayer *ml = (CAMetalLayer *)self.layer;
                ml.device = _metalDevice;
                ml.pixelFormat = MTLPixelFormatRGBA8Unorm;
                ml.framebufferOnly = NO;
                ml.maximumDrawableCount = 3;
                ml.presentsWithTransaction = NO;
                ml.contentsScale = scale;
                _metalQueue = [_metalDevice newCommandQueue];
            }
        }
        NSInteger maxHz = UIScreen.mainScreen.maximumFramesPerSecond;
        if (maxHz < 60) maxHz = 120;
        _targetRefreshHz = (float)maxHz;
        CGFloat w = frame.size.width > 0 ? frame.size.width : 393.0;
        CGFloat h = frame.size.height > 0 ? frame.size.height : 852.0;
        uint32_t physW = (uint32_t)llround(w * scale);
        uint32_t physH = (uint32_t)llround(h * scale);
        int32_t isIpad = (UIDevice.currentDevice.userInterfaceIdiom == UIUserInterfaceIdiomPad) ? 1 : 0;
        UIEdgeInsets initSafe = [self resolveEffectiveSafeArea];
        _lastDispatchedSafeArea = initSafe;
        _handle = euv_ios_create(
            physW,
            physH,
            (float)scale,
            isIpad,
            _targetRefreshHz,
            (float)initSafe.top,
            (float)initSafe.right,
            (float)initSafe.bottom,
            (float)initSafe.left
        );
        if (@available(iOS 17.0, *)) {
            [self registerForTraitChanges:@[UITraitDisplayScale.class] withTarget:self action:@selector(syncViewportScaleAndBounds)];
            [self registerForTraitChanges:@[UITraitUserInterfaceStyle.class] withTarget:self action:@selector(syncThemeTrait)];
        }
        [self syncThemeTrait];
        [self presentFrameWithDt:[self nominalFrameDt] force:YES];
        _displayLink = [CADisplayLink displayLinkWithTarget:self selector:@selector(onDisplayLink:)];
        if (@available(iOS 15.0, *)) {
            float minHz = (_targetRefreshHz >= 120.0f) ? 80.0f : 60.0f;
            _displayLink.preferredFrameRateRange = CAFrameRateRangeMake(minHz, _targetRefreshHz, _targetRefreshHz);
        } else {
            _displayLink.preferredFramesPerSecond = (NSInteger)_targetRefreshHz;
        }
        [_displayLink addToRunLoop:[NSRunLoop mainRunLoop] forMode:NSRunLoopCommonModes];
    }
    return self;
}

- (void)syncSafeArea {
    if (!_handle) return;
    UIEdgeInsets insets = [self resolveEffectiveSafeArea];
    if (fabs(insets.top - _lastDispatchedSafeArea.top) < 0.25
        && fabs(insets.right - _lastDispatchedSafeArea.right) < 0.25
        && fabs(insets.bottom - _lastDispatchedSafeArea.bottom) < 0.25
        && fabs(insets.left - _lastDispatchedSafeArea.left) < 0.25) {
        return;
    }
    _lastDispatchedSafeArea = insets;
    euv_ios_set_safe_area(_handle, (float)insets.top, (float)insets.right, (float)insets.bottom, (float)insets.left);
}

- (void)safeAreaInsetsDidChange {
    [super safeAreaInsetsDidChange];
    [self syncSafeArea];
    [self presentFrameWithDt:[self nominalFrameDt] force:YES];
}

- (void)didMoveToWindow {
    [super didMoveToWindow];
    if (!self.window && _handle) {
        euv_ios_touch_cancel(_handle, 0);
        return;
    }
    if (self.window.screen) {
        NSInteger maxHz = self.window.screen.maximumFramesPerSecond;
        if (maxHz >= 60 && fabs((float)maxHz - _targetRefreshHz) > 0.5f) {
            _targetRefreshHz = (float)maxHz;
            if (_handle) {
                euv_ios_update_refresh_rate(_handle, _targetRefreshHz);
            }
            if (_displayLink) {
                if (@available(iOS 15.0, *)) {
                    float minHz = (_targetRefreshHz >= 120.0f) ? 80.0f : 60.0f;
                    _displayLink.preferredFrameRateRange = CAFrameRateRangeMake(minHz, _targetRefreshHz, _targetRefreshHz);
                } else {
                    _displayLink.preferredFramesPerSecond = (NSInteger)_targetRefreshHz;
                }
            }
        }
    }
    [self syncViewportScaleAndBounds];
}

- (void)syncViewportScaleAndBounds {
    if (!_handle) return;
    if (self.window) {
        CGRect wb = self.window.bounds;
        if (wb.size.width > 0 && wb.size.height > 0 && !CGRectEqualToRect(self.bounds, wb)) {
            [super setFrame:wb];
        }
    }
    if (self.bounds.size.width <= 0 || self.bounds.size.height <= 0) return;
    CGFloat rawScale = self.traitCollection.displayScale > 0
        ? self.traitCollection.displayScale
        : (self.window.screen.scale > 0 ? self.window.screen.scale : UIScreen.mainScreen.scale);
    CGFloat scale = fmin(4.5, rawScale);
    _lastLogicalW = self.bounds.size.width;
    _lastLogicalH = self.bounds.size.height;
    _lastScale = scale;
    self.layer.contentsScale = scale;
    uint32_t physW = (uint32_t)llround(self.bounds.size.width * scale);
    uint32_t physH = (uint32_t)llround(self.bounds.size.height * scale);
    if ([self.layer isKindOfClass:[CAMetalLayer class]]) {
        CAMetalLayer *ml = (CAMetalLayer *)self.layer;
        ml.contentsScale = scale;
        ml.drawableSize = CGSizeMake(physW, physH);
    }
    euv_ios_resize(_handle, physW, physH, (float)scale);
    [self syncSafeArea];
    [self presentFrameWithDt:[self nominalFrameDt] force:YES];
}

- (void)layoutSubviews {
    [super layoutSubviews];
    [self syncViewportScaleAndBounds];
}

- (void)setBounds:(CGRect)bounds {
    [super setBounds:bounds];
    [self syncViewportScaleAndBounds];
}

- (void)setFrame:(CGRect)frame {
    [super setFrame:frame];
    [self syncViewportScaleAndBounds];
}

- (void)onDisplayLink:(CADisplayLink *)link {
    CFTimeInterval now = link.timestamp;
    float fallbackHz = _targetRefreshHz > 0.0f ? _targetRefreshHz : 60.0f;
    float dt = (_lastTimestamp > 0.0) ? (float)(now - _lastTimestamp) : (1.0f / fallbackHz);
    _lastTimestamp = now;
    CGFloat curW = self.window && self.window.bounds.size.width > 0 ? self.window.bounds.size.width : self.bounds.size.width;
    CGFloat curH = self.window && self.window.bounds.size.height > 0 ? self.window.bounds.size.height : self.bounds.size.height;
    CGFloat rawScale = self.traitCollection.displayScale > 0
        ? self.traitCollection.displayScale
        : (self.window.screen.scale > 0 ? self.window.screen.scale : UIScreen.mainScreen.scale);
    CGFloat curScale = fmin(4.5, rawScale);
    if (fabs(curW - _lastLogicalW) > 0.5 || fabs(curH - _lastLogicalH) > 0.5 || fabs(curScale - _lastScale) > 0.01) {
        [self syncViewportScaleAndBounds];
        return;
    }
    [self presentFrameWithDt:dt force:NO];
}

- (BOOL)presentMetalPixels:(const uint8_t *)pixels width:(uint32_t)w height:(uint32_t)h dirtyX:(uint32_t)dx dirtyY:(uint32_t)dy dirtyW:(uint32_t)dw dirtyH:(uint32_t)dh forceFull:(BOOL)forceFull {
    if (!_metalDevice || !_metalQueue || ![self.layer isKindOfClass:[CAMetalLayer class]]) {
        return NO;
    }
    CAMetalLayer *ml = (CAMetalLayer *)self.layer;
    if ((uint32_t)llround(ml.drawableSize.width) != w || (uint32_t)llround(ml.drawableSize.height) != h) {
        ml.drawableSize = CGSizeMake(w, h);
    }
    BOOL fullUpload = forceFull || !_stagingTexture || _stagingW != w || _stagingH != h || dw == 0 || dh == 0;
    if (!_stagingTexture || _stagingW != w || _stagingH != h) {
        MTLTextureDescriptor *desc = [MTLTextureDescriptor texture2DDescriptorWithPixelFormat:MTLPixelFormatRGBA8Unorm
                                                                                        width:w
                                                                                       height:h
                                                                                    mipmapped:NO];
        desc.usage = MTLTextureUsageShaderRead;
        desc.storageMode = MTLStorageModeShared;
        _stagingTexture = [_metalDevice newTextureWithDescriptor:desc];
        _stagingW = w;
        _stagingH = h;
        fullUpload = YES;
    }
    if (!_stagingTexture) {
        return NO;
    }
    NSUInteger rowBytes = (NSUInteger)w * 4;
    if (fullUpload) {
        [_stagingTexture replaceRegion:MTLRegionMake2D(0, 0, w, h)
                           mipmapLevel:0
                             withBytes:pixels
                           bytesPerRow:rowBytes];
    } else {
        uint32_t cx = dx < w ? dx : 0;
        uint32_t cy = dy < h ? dy : 0;
        uint32_t cw = (cx + dw <= w) ? dw : (w - cx);
        uint32_t ch = (cy + dh <= h) ? dh : (h - cy);
        const uint8_t *subPtr = pixels + ((size_t)cy * (size_t)w + (size_t)cx) * 4;
        [_stagingTexture replaceRegion:MTLRegionMake2D(cx, cy, cw, ch)
                           mipmapLevel:0
                             withBytes:subPtr
                           bytesPerRow:rowBytes];
    }
    id<CAMetalDrawable> drawable = [ml nextDrawable];
    if (!drawable || !drawable.texture) {
        return NO;
    }
    if (drawable.texture.width != w || drawable.texture.height != h) {
        return NO;
    }
    id<MTLCommandBuffer> cmd = [_metalQueue commandBuffer];
    id<MTLBlitCommandEncoder> blit = [cmd blitCommandEncoder];
    [blit copyFromTexture:_stagingTexture
              sourceSlice:0
              sourceLevel:0
             sourceOrigin:MTLOriginMake(0, 0, 0)
               sourceSize:MTLSizeMake(w, h, 1)
                toTexture:drawable.texture
         destinationSlice:0
         destinationLevel:0
        destinationOrigin:MTLOriginMake(0, 0, 0)];
    [blit endEncoding];
    [cmd presentDrawable:drawable];
    [cmd commit];
    return YES;
}

- (void)presentFrameWithDt:(float)dt force:(BOOL)force {
    if (!_handle) return;
    CFTimeInterval t0 = CACurrentMediaTime();
    const uint8_t *pixels = NULL;
    uint32_t w = 0, h = 0, dx = 0, dy = 0, dw = 0, dh = 0;
    int32_t changed = euv_ios_step(_handle, dt, &pixels, &w, &h, &dx, &dy, &dw, &dh);
    if ((changed || force) && pixels && w > 0 && h > 0) {
        BOOL metalOk = [self presentMetalPixels:pixels width:w height:h dirtyX:dx dirtyY:dy dirtyW:dw dirtyH:dh forceFull:force];
        if (!metalOk) {
            size_t bytes = (size_t)w * (size_t)h * 4;
            NSData *copyData = [NSData dataWithBytes:pixels length:bytes];
            CGDataProviderRef provider = CGDataProviderCreateWithCFData((__bridge CFDataRef)copyData);
            CGBitmapInfo info = kCGBitmapByteOrder32Big | kCGImageAlphaLast;
            CGImageRef img = CGImageCreate(w, h, 8, 32, (size_t)w * 4, _colorSpace, info, provider, NULL, false, kCGRenderingIntentDefault);
            if (img) {
                self.layer.contents = (__bridge id)img;
                self.layer.contentsGravity = kCAGravityResize;
                CGImageRelease(img);
            }
            CGDataProviderRelease(provider);
        }
        double renderMs = (CACurrentMediaTime() - t0) * 1000.0;
        _renderedFrames += 1;
        _accumRenderMs += renderMs;
        if (renderMs > _maxRenderMs) _maxRenderMs = renderMs;
        double budgetMs = 1000.0 / (_targetRefreshHz > 0.0f ? _targetRefreshHz : 60.0f);
        if (renderMs > budgetMs) _droppedFrames += 1;
        if (_renderedFrames % 60 == 0) {
            double avg = _accumRenderMs / 60.0;
            NSLog(@"[euv-fps] ios rendered=%llu avg=%.2fms max=%.2fms dropped=%llu target=%.0fHz res=%ux%u",
                _renderedFrames, avg, _maxRenderMs, _droppedFrames, _targetRefreshHz, w, h);
            _accumRenderMs = 0.0;
            _maxRenderMs = 0.0;
        }
    }
}

- (void)syncLiveTouchesFromEvent:(UIEvent *)event {
    if (!_handle) return;
    uint64_t liveIds[32];
    size_t count = 0;
    if (event) {
        for (UITouch *t in event.allTouches) {
            UITouchPhase phase = t.phase;
            if ((phase == UITouchPhaseBegan || phase == UITouchPhaseMoved || phase == UITouchPhaseStationary) && count < 32) {
                liveIds[count++] = (uint64_t)(uintptr_t)t;
            }
        }
    }
    euv_ios_touch_sync(_handle, liveIds, count);
}

- (void)touchesBegan:(NSSet<UITouch *> *)touches withEvent:(UIEvent *)event {
    if (!_handle) return;
    for (UITouch *t in touches) {
        CGPoint p = [t locationInView:self];
        euv_ios_touch_down(_handle, (uint64_t)(uintptr_t)t, (float)p.x, (float)p.y);
    }
    [self syncLiveTouchesFromEvent:event];
}

- (void)touchesMoved:(NSSet<UITouch *> *)touches withEvent:(UIEvent *)event {
    if (!_handle) return;
    for (UITouch *t in touches) {
        CGPoint p = [t locationInView:self];
        euv_ios_touch_move(_handle, (uint64_t)(uintptr_t)t, (float)p.x, (float)p.y);
    }
    [self syncLiveTouchesFromEvent:event];
}

- (void)touchesEnded:(NSSet<UITouch *> *)touches withEvent:(UIEvent *)event {
    if (!_handle) return;
    for (UITouch *t in touches) {
        CGPoint p = [t locationInView:self];
        euv_ios_touch_up(_handle, (uint64_t)(uintptr_t)t, (float)p.x, (float)p.y);
    }
    [self syncLiveTouchesFromEvent:event];
}

- (void)touchesCancelled:(NSSet<UITouch *> *)touches withEvent:(UIEvent *)event {
    if (!_handle) return;
    for (UITouch *t in touches) {
        euv_ios_touch_cancel(_handle, (uint64_t)(uintptr_t)t);
    }
    [self syncLiveTouchesFromEvent:event];
    BOOL anyLive = NO;
    if (event) {
        for (UITouch *t in event.allTouches) {
            UITouchPhase phase = t.phase;
            if (phase == UITouchPhaseBegan || phase == UITouchPhaseMoved || phase == UITouchPhaseStationary) {
                anyLive = YES;
                break;
            }
        }
    }
    if (!anyLive) {
        euv_ios_touch_cancel(_handle, 0);
    }
}

- (BOOL)canBecomeFirstResponder {
    return YES;
}

- (BOOL)hasText {
    return YES;
}

- (void)insertText:(NSString *)text {
    if (!_handle || text.length == 0) return;
    if ([text isEqualToString:@"\n"]) {
        euv_ios_key_down(_handle, 2);
        return;
    }
    const char *utf8 = [text UTF8String];
    if (utf8) {
        euv_ios_text_input(_handle, utf8);
    }
}

- (void)deleteBackward {
    if (!_handle) return;
    euv_ios_key_down(_handle, 1);
}

@end

@implementation EuvViewController

- (void)loadView {
    gEuvActiveViewController = self;
    CGRect initialFrame = UIScreen.mainScreen.bounds;
    self.canvasView = [[EuvCanvasView alloc] initWithFrame:initialFrame];
    self.canvasView.autoresizingMask = UIViewAutoresizingFlexibleWidth | UIViewAutoresizingFlexibleHeight;
    self.view = self.canvasView;
}

- (void)applyImmersiveFullscreen:(BOOL)enabled {
    self.immersiveFullscreen = enabled;
    self.canvasView.immersiveFullscreen = enabled;
    [self setNeedsStatusBarAppearanceUpdate];
    [self setNeedsUpdateOfHomeIndicatorAutoHidden];
    [self.canvasView syncSafeArea];
    [self.canvasView presentFrameWithDt:(1.0f / (self.canvasView.targetRefreshHz > 0.0f ? self.canvasView.targetRefreshHz : 60.0f)) force:YES];
}

- (BOOL)prefersStatusBarHidden {
    return self.immersiveFullscreen;
}

- (BOOL)prefersHomeIndicatorAutoHidden {
    return self.immersiveFullscreen;
}

- (UIStatusBarAnimation)preferredStatusBarUpdateAnimation {
    return UIStatusBarAnimationFade;
}

- (void)viewSafeAreaInsetsDidChange {
    [super viewSafeAreaInsetsDidChange];
    [self.canvasView syncSafeArea];
}

- (void)viewDidAppear:(BOOL)animated {
    [super viewDidAppear:animated];
    [self.canvasView syncViewportScaleAndBounds];
}

- (void)viewDidLayoutSubviews {
    [super viewDidLayoutSubviews];
    [self.canvasView syncViewportScaleAndBounds];
}

- (void)viewWillTransitionToSize:(CGSize)size withTransitionCoordinator:(id<UIViewControllerTransitionCoordinator>)coordinator {
    [super viewWillTransitionToSize:size withTransitionCoordinator:coordinator];
    [coordinator animateAlongsideTransition:^(id<UIViewControllerTransitionCoordinatorContext> ctx) {
        [self.canvasView syncViewportScaleAndBounds];
    } completion:^(id<UIViewControllerTransitionCoordinatorContext> ctx) {
        [self.canvasView syncViewportScaleAndBounds];
    }];
}

@end

@interface EuvSceneDelegate : UIResponder <UIWindowSceneDelegate>
@property (nonatomic, strong) UIWindow *window;
@end

@implementation EuvSceneDelegate

- (void)syncWindowGeometry:(UIWindowScene *)windowScene {
    (void)windowScene;
    if (!self.window) return;
    [self.window layoutIfNeeded];
    if ([self.window.rootViewController isKindOfClass:[EuvViewController class]]) {
        EuvViewController *vc = (EuvViewController *)self.window.rootViewController;
        [vc.canvasView syncViewportScaleAndBounds];
    }
}

- (void)scene:(UIScene *)scene willConnectToSession:(UISceneSession *)session options:(UISceneConnectionOptions *)connectionOptions {
    if (![scene isKindOfClass:[UIWindowScene class]]) return;
    UIWindowScene *windowScene = (UIWindowScene *)scene;
    if (UIDevice.currentDevice.userInterfaceIdiom == UIUserInterfaceIdiomPad && windowScene.sizeRestrictions != nil) {
        windowScene.sizeRestrictions.minimumSize = CGSizeMake(820.0, 680.0);
        windowScene.sizeRestrictions.maximumSize = CGSizeMake(CGFLOAT_MAX, CGFLOAT_MAX);
    }
    self.window = [[UIWindow alloc] initWithWindowScene:windowScene];
    self.window.backgroundColor = [UIColor systemBackgroundColor];
    self.window.rootViewController = [[EuvViewController alloc] init];
    [self.window makeKeyAndVisible];
    [self syncWindowGeometry:windowScene];
    if (connectionOptions && connectionOptions.URLContexts.count > 0) {
        [self scene:scene openURLContexts:connectionOptions.URLContexts];
    }
}

- (void)sceneWillEnterForeground:(UIScene *)scene {
    (void)scene;
    if (self.window && [self.window.rootViewController isKindOfClass:[EuvViewController class]]) {
        EuvViewController *vc = (EuvViewController *)self.window.rootViewController;
        if (vc.canvasView) {
            [vc.canvasView ensureLiveSession];
            if (vc.canvasView.handle) {
                [vc.canvasView setDisplayLinkPaused:NO];
                euv_ios_system_lifecycle(vc.canvasView.handle, 1);
                [vc.canvasView syncSafeArea];
            }
        }
    }
}

- (void)sceneDidBecomeActive:(UIScene *)scene {
    if ([scene isKindOfClass:[UIWindowScene class]]) {
        [self syncWindowGeometry:(UIWindowScene *)scene];
    }
    if (self.window && [self.window.rootViewController isKindOfClass:[EuvViewController class]]) {
        EuvViewController *vc = (EuvViewController *)self.window.rootViewController;
        if (vc.canvasView) {
            [vc.canvasView ensureLiveSession];
            if (vc.canvasView.handle) {
                [vc.canvasView setDisplayLinkPaused:NO];
                euv_ios_system_lifecycle(vc.canvasView.handle, 3);
                euv_ios_focus(vc.canvasView.handle, 1);
                [vc.canvasView syncSafeArea];
            }
        }
    }
}

- (void)sceneDidDisconnect:(UIScene *)scene {
    (void)scene;
    if (self.window && [self.window.rootViewController isKindOfClass:[EuvViewController class]]) {
        EuvViewController *vc = (EuvViewController *)self.window.rootViewController;
        if (vc.canvasView) {
            [vc.canvasView shutdownSession];
        }
        if (gEuvActiveViewController == vc) {
            gEuvActiveViewController = nil;
        }
    }
    self.window = nil;
}

- (void)sceneWillResignActive:(UIScene *)scene {
    (void)scene;
    if (self.window && [self.window.rootViewController isKindOfClass:[EuvViewController class]]) {
        EuvViewController *vc = (EuvViewController *)self.window.rootViewController;
        if (vc.canvasView && vc.canvasView.handle) {
            euv_ios_touch_cancel(vc.canvasView.handle, 0);
            euv_ios_focus(vc.canvasView.handle, 0);
            euv_ios_system_lifecycle(vc.canvasView.handle, 4);
        }
    }
}

- (void)sceneDidEnterBackground:(UIScene *)scene {
    (void)scene;
    if (self.window && [self.window.rootViewController isKindOfClass:[EuvViewController class]]) {
        EuvViewController *vc = (EuvViewController *)self.window.rootViewController;
        if (vc.canvasView && vc.canvasView.handle) {
            [vc.canvasView setDisplayLinkPaused:YES];
            euv_ios_system_lifecycle(vc.canvasView.handle, 5);
        }
    }
}

- (void)scene:(UIScene *)scene openURLContexts:(NSSet<UIOpenURLContext *> *)URLContexts {
    (void)scene;
    if (!self.window || ![self.window.rootViewController isKindOfClass:[EuvViewController class]]) return;
    EuvViewController *vc = (EuvViewController *)self.window.rootViewController;
    if (!vc.canvasView || !vc.canvasView.handle) return;
    for (UIOpenURLContext *ctx in URLContexts) {
        NSString *urlStr = ctx.URL.absoluteString;
        if (urlStr.length > 0) {
            euv_ios_deep_link(vc.canvasView.handle, [urlStr UTF8String]);
        }
    }
}

- (void)windowScene:(UIWindowScene *)windowScene didUpdateEffectiveGeometry:(UIWindowSceneGeometry *)previousEffectiveGeometry {
    [self syncWindowGeometry:windowScene];
}

@end

@interface EuvAppDelegate : UIResponder <UIApplicationDelegate>
@end

@implementation EuvAppDelegate

- (UISceneConfiguration *)application:(UIApplication *)application
    configurationForConnectingSceneSession:(UISceneSession *)connectingSceneSession
    options:(UISceneConnectionOptions *)options {
    UISceneConfiguration *cfg = [[UISceneConfiguration alloc] initWithName:@"Default Configuration"
                                                              sessionRole:connectingSceneSession.role];
    cfg.delegateClass = [EuvSceneDelegate class];
    return cfg;
}

- (void)applicationWillTerminate:(UIApplication *)application {
    (void)application;
    EuvCanvasView *v = gEuvActiveCanvasView;
    if (v && v.handle) {
        euv_ios_system_lifecycle(v.handle, 6);
    }
}

@end

int main(int argc, char *argv[]) {
    @autoreleasepool {
        return UIApplicationMain(argc, argv, nil, NSStringFromClass([EuvAppDelegate class]));
    }
}
EOF

cat > "${BUILD_DIR}/Info.plist" <<'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key><string>en</string>
    <key>CFBundleDisplayName</key><string>Euv Studio</string>
    <key>CFBundleExecutable</key><string>EuvShowcase</string>
    <key>CFBundleIconFiles</key>
    <array>
        <string>AppIcon60x60@2x.png</string>
        <string>AppIcon60x60@3x.png</string>
        <string>AppIcon.png</string>
    </array>
    <key>CFBundleIcons</key>
    <dict>
        <key>CFBundlePrimaryIcon</key>
        <dict>
            <key>CFBundleIconFiles</key>
            <array>
                <string>AppIcon60x60</string>
            </array>
        </dict>
    </dict>
    <key>CFBundleIdentifier</key><string>vip.ltpp.euv.showcase</string>
    <key>CFBundleInfoDictionaryVersion</key><string>6.0</string>
    <key>CFBundleName</key><string>Euv Studio</string>
    <key>CFBundlePackageType</key><string>APPL</string>
    <key>CFBundleShortVersionString</key><string>0.45.0</string>
    <key>CFBundleVersion</key><string>7</string>
    <key>CADisableMinimumFrameDurationOnPhone</key><true/>
    <key>LSRequiresIPhoneOS</key><true/>
    <key>UIApplicationSceneManifest</key>
    <dict>
        <key>UIApplicationSupportsMultipleScenes</key><true/>
        <key>UISceneConfigurations</key>
        <dict>
            <key>UIWindowSceneSessionRoleApplication</key>
            <array>
                <dict>
                    <key>UISceneConfigurationName</key><string>Default Configuration</string>
                    <key>UISceneDelegateClassName</key><string>EuvSceneDelegate</string>
                </dict>
            </array>
        </dict>
    </dict>
    <key>UIApplicationSupportsIndirectInputEvents</key><true/>
    <key>UIDeviceFamily</key>
    <array>
        <integer>1</integer>
        <integer>2</integer>
    </array>
    <key>UIRequiresFullScreen</key><false/>
    <key>UILaunchScreen</key><dict/>
    <key>UISupportedInterfaceOrientations</key>
    <array>
        <string>UIInterfaceOrientationPortrait</string>
        <string>UIInterfaceOrientationLandscapeLeft</string>
        <string>UIInterfaceOrientationLandscapeRight</string>
    </array>
    <key>UISupportedInterfaceOrientations~ipad</key>
    <array>
        <string>UIInterfaceOrientationPortrait</string>
        <string>UIInterfaceOrientationPortraitUpsideDown</string>
        <string>UIInterfaceOrientationLandscapeLeft</string>
        <string>UIInterfaceOrientationLandscapeRight</string>
    </array>
</dict>
</plist>
EOF

cd "${BUILD_DIR}"
log_info "Compiling Release aarch64-apple-ios-sim staticlib..."
cargo build --release --target aarch64-apple-ios-sim
STATIC_LIB="${BUILD_DIR}/target/aarch64-apple-ios-sim/release/libeuv_ios_host.a"

rm -rf "${APP_BUNDLE}" && mkdir -p "${APP_BUNDLE}"
cp "${BUILD_DIR}/Info.plist" "${APP_BUNDLE}/Info.plist"
python3 "${REPO_ROOT}/scripts/env/generate-icon.py" "${APP_BUNDLE}/AppIcon60x60@2x.png" 120 "${EUV_APP_ICON:-}"
python3 "${REPO_ROOT}/scripts/env/generate-icon.py" "${APP_BUNDLE}/AppIcon60x60@3x.png" 180 "${EUV_APP_ICON:-}"
python3 "${REPO_ROOT}/scripts/env/generate-icon.py" "${APP_BUNDLE}/AppIcon.png" 1024 "${EUV_APP_ICON:-}"
"${CLANG}" -arch arm64 -isysroot "${SDK_PATH}" -mios-simulator-version-min=17.0 -fobjc-arc \
  "${BUILD_DIR}/main.m" "${STATIC_LIB}" \
  -framework UIKit -framework UserNotifications -framework Metal -framework CoreGraphics -framework QuartzCore -framework CoreText -framework CoreFoundation -framework Foundation \
  -o "${APP_BUNDLE}/EuvShowcase"
codesign --force --sign - "${APP_BUNDLE}"

if [ "${EUV_IOS_SKIP_LAUNCH:-0}" != "1" ]; then
  xcrun simctl boot "${SIM_UDID}" 2>/dev/null || true
  xcrun simctl terminate "${SIM_UDID}" vip.ltpp.euv.showcase 2>/dev/null || true
  xcrun simctl install "${SIM_UDID}" "${APP_BUNDLE}"
  xcrun simctl launch "${SIM_UDID}" vip.ltpp.euv.showcase
  log_info "Installed and launched immersive @3x EuvShowcase.app on Simulator ${SIM_UDID}."
else
  log_info "Signed iOS Simulator bundle ready: ${APP_BUNDLE}"
fi
