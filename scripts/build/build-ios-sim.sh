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
version = "0.38.0"
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
use std::ffi::{CString, c_char};
use euv::{
    App, NativeSystemFont, PlatformKind, PointerEvent, RenderMode, SafeAreaInsets, WindowEvent,
    WindowId, register_native_action_bridge,
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
) -> *mut EuvIosHandle {
    register_native_action_bridge(ios_native_action_bridge);
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
    let wid: WindowId = app.get_window_state().get_id();
    let pw: u32 = phys_width.max(200);
    let ph: u32 = phys_height.max(200);
    let s: f32 = scale.clamp(1.0, 4.5);
    app.set_exact_physical_viewport(pw, ph, s);
    let _ = app.step_game_frame(1.0 / 60.0);
    let mut handle = Box::new(EuvIosHandle {
        app,
        _state: state,
        wid,
        width: 0,
        height: 0,
    });
    if let Some(fb) = handle.app.get_renderer().framebuffer() {
        handle.width = fb.get_width();
        handle.height = fb.get_height();
    }
    Box::into_raw(handle)
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
pub unsafe extern "C" fn euv_ios_step(
    handle: *mut EuvIosHandle,
    dt: f32,
    out_ptr: *mut *const u8,
    out_w: *mut u32,
    out_h: *mut u32,
) -> i32 {
    if handle.is_null() {
        return 0;
    }
    let h: &mut EuvIosHandle = unsafe { &mut *handle };
    let Ok(stats) = h.app.step_game_frame(dt.clamp(1.0 / 240.0, 0.05)) else {
        return 0;
    };
    let changed: bool = stats.get_pixels_shaded() > 0 || h.width == 0;
    if let Some(fb) = h.app.get_renderer().framebuffer() {
        h.width = fb.get_width();
        h.height = fb.get_height();
        if !out_ptr.is_null() {
            unsafe { *out_ptr = fb.get_pixels().as_ptr() };
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
    if changed { 1 } else { 0 }
}
EOF

cat > "${BUILD_DIR}/main.m" <<'EOF'
#import <UIKit/UIKit.h>
#import <CoreGraphics/CoreGraphics.h>
#import <QuartzCore/QuartzCore.h>
#import <UserNotifications/UserNotifications.h>
#include <stdint.h>

int32_t euv_ios_invoke_native_action(const char *kind, const char *arg1, const char *arg2) {
    NSString *k = kind ? [NSString stringWithUTF8String:kind] : @"";
    NSString *a1 = arg1 ? [NSString stringWithUTF8String:arg1] : @"";
    NSString *a2 = arg2 ? [NSString stringWithUTF8String:arg2] : @"";
    NSLog(@"[euv-native] ios action kind=%@ arg1=%@ arg2=%@", k, a1, a2);
    dispatch_async(dispatch_get_main_queue(), ^{
        if ([k isEqualToString:@"haptic"]) {
            UIImpactFeedbackGenerator *gen = [[UIImpactFeedbackGenerator alloc] initWithStyle:UIImpactFeedbackStyleMedium];
            [gen prepare];
            [gen impactOccurred];
        } else if ([k isEqualToString:@"notify"] || [k isEqualToString:@"island"]) {
            UNUserNotificationCenter *center = [UNUserNotificationCenter currentNotificationCenter];
            [center requestAuthorizationWithOptions:(UNAuthorizationOptionAlert | UNAuthorizationOptionSound | UNAuthorizationOptionBadge)
                                  completionHandler:^(BOOL granted, NSError * _Nullable err) {
                if (granted) {
                    UNMutableNotificationContent *content = [[UNMutableNotificationContent alloc] init];
                    content.title = a1.length > 0 ? a1 : @"Euv Showcase";
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
extern EuvIosHandle *euv_ios_create(uint32_t phys_width, uint32_t phys_height, float scale, int32_t is_ipad);
extern void euv_ios_resize(EuvIosHandle *handle, uint32_t phys_width, uint32_t phys_height, float scale);
extern void euv_ios_set_safe_area(EuvIosHandle *handle, float top, float right, float bottom, float left);
extern void euv_ios_touch_down(EuvIosHandle *handle, uint64_t pointer_id, float x, float y);
extern void euv_ios_touch_move(EuvIosHandle *handle, uint64_t pointer_id, float x, float y);
extern void euv_ios_touch_up(EuvIosHandle *handle, uint64_t pointer_id, float x, float y);
extern void euv_ios_touch_cancel(EuvIosHandle *handle, uint64_t pointer_id);
extern void euv_ios_touch_sync(EuvIosHandle *handle, const uint64_t *ids, size_t count);
extern int32_t euv_ios_step(EuvIosHandle *handle, float dt, const uint8_t **out_ptr, uint32_t *out_w, uint32_t *out_h);

@interface EuvCanvasView : UIView
@property (nonatomic, assign) EuvIosHandle *handle;
@property (nonatomic, strong) CADisplayLink *displayLink;
@property (nonatomic, assign) CFTimeInterval lastTimestamp;
@property (nonatomic, assign) CGColorSpaceRef colorSpace;
@property (nonatomic, assign) CGFloat lastLogicalW;
@property (nonatomic, assign) CGFloat lastLogicalH;
@property (nonatomic, assign) CGFloat lastScale;
@property (nonatomic, assign) uint64_t renderedFrames;
@property (nonatomic, assign) double accumRenderMs;
@property (nonatomic, assign) double maxRenderMs;
@property (nonatomic, assign) uint64_t droppedFrames;
@end

@implementation EuvCanvasView

- (instancetype)initWithFrame:(CGRect)frame {
    self = [super initWithFrame:frame];
    if (self) {
        self.backgroundColor = [UIColor whiteColor];
        self.multipleTouchEnabled = YES;
        _colorSpace = CGColorSpaceCreateDeviceRGB();
        CGFloat scale = fmin(4.5, UIScreen.mainScreen.scale);
        self.layer.contentsScale = scale;
        self.layer.magnificationFilter = kCAFilterNearest;
        self.layer.minificationFilter = kCAFilterLinear;
        CGFloat w = frame.size.width > 0 ? frame.size.width : 393.0;
        CGFloat h = frame.size.height > 0 ? frame.size.height : 852.0;
        uint32_t physW = (uint32_t)llround(w * scale);
        uint32_t physH = (uint32_t)llround(h * scale);
        int32_t isIpad = (UIDevice.currentDevice.userInterfaceIdiom == UIUserInterfaceIdiomPad) ? 1 : 0;
        _handle = euv_ios_create(physW, physH, (float)scale, isIpad);
        if (@available(iOS 17.0, *)) {
            [self registerForTraitChanges:@[UITraitDisplayScale.class] withTarget:self action:@selector(syncViewportScaleAndBounds)];
        }
        [self syncSafeArea];
        [self presentFrameWithDt:1.0f / 60.0f force:YES];
        _displayLink = [CADisplayLink displayLinkWithTarget:self selector:@selector(onDisplayLink:)];
        if (@available(iOS 15.0, *)) {
            _displayLink.preferredFrameRateRange = CAFrameRateRangeMake(60.0f, 120.0f, 120.0f);
        }
        [_displayLink addToRunLoop:[NSRunLoop mainRunLoop] forMode:NSRunLoopCommonModes];
    }
    return self;
}

- (void)syncSafeArea {
    if (!_handle) return;
    UIEdgeInsets insets = self.safeAreaInsets;
    if (@available(iOS 26.0, *)) {
        UIViewLayoutRegion *region = [UIViewLayoutRegion safeAreaLayoutRegionWithCornerAdaptation:UIViewLayoutRegionAdaptivityAxisVertical];
        UIEdgeInsets adapted = [self edgeInsetsForLayoutRegion:region];
        if (adapted.top > insets.top) insets.top = adapted.top;
        if (adapted.right > insets.right) insets.right = adapted.right;
        if (adapted.bottom > insets.bottom) insets.bottom = adapted.bottom;
        if (adapted.left > insets.left) insets.left = adapted.left;
    }
    if (UIDevice.currentDevice.userInterfaceIdiom == UIUserInterfaceIdiomPad && insets.top < 40.0) {
        insets.top = 40.0;
    }
    euv_ios_set_safe_area(_handle, (float)insets.top, (float)insets.right, (float)insets.bottom, (float)insets.left);
}

- (void)safeAreaInsetsDidChange {
    [super safeAreaInsetsDidChange];
    [self syncSafeArea];
    [self presentFrameWithDt:1.0f / 60.0f force:YES];
}

- (void)didMoveToWindow {
    [super didMoveToWindow];
    if (!self.window && _handle) {
        euv_ios_touch_cancel(_handle, 0);
        return;
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
    euv_ios_resize(_handle, physW, physH, (float)scale);
    [self syncSafeArea];
    [self presentFrameWithDt:1.0f / 60.0f force:YES];
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
    float dt = (_lastTimestamp > 0.0) ? (float)(now - _lastTimestamp) : (1.0f / 60.0f);
    _lastTimestamp = now;
    CGFloat curW = self.window && self.window.bounds.size.width > 0 ? self.window.bounds.size.width : self.bounds.size.width;
    CGFloat curH = self.window && self.window.bounds.size.height > 0 ? self.window.bounds.size.height : self.bounds.size.height;
    CGFloat rawScale = self.traitCollection.displayScale > 0
        ? self.traitCollection.displayScale
        : (self.window.screen.scale > 0 ? self.window.screen.scale : UIScreen.mainScreen.scale);
    CGFloat curScale = fmin(4.0, rawScale);
    if (fabs(curW - _lastLogicalW) > 0.5 || fabs(curH - _lastLogicalH) > 0.5 || fabs(curScale - _lastScale) > 0.01) {
        [self syncViewportScaleAndBounds];
        return;
    }
    [self presentFrameWithDt:dt force:NO];
}

- (void)presentFrameWithDt:(float)dt force:(BOOL)force {
    if (!_handle) return;
    CFTimeInterval t0 = CACurrentMediaTime();
    const uint8_t *pixels = NULL;
    uint32_t w = 0, h = 0;
    int32_t changed = euv_ios_step(_handle, dt, &pixels, &w, &h);
    if ((changed || force) && pixels && w > 0 && h > 0) {
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
        double renderMs = (CACurrentMediaTime() - t0) * 1000.0;
        _renderedFrames += 1;
        _accumRenderMs += renderMs;
        if (renderMs > _maxRenderMs) _maxRenderMs = renderMs;
        if (renderMs > 16.67) _droppedFrames += 1;
        if (_renderedFrames % 60 == 0) {
            double avg = _accumRenderMs / 60.0;
            NSLog(@"[euv-fps] ios rendered=%llu avg=%.2fms max=%.2fms dropped=%llu res=%ux%u",
                _renderedFrames, avg, _maxRenderMs, _droppedFrames, w, h);
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
}

@end

@interface EuvViewController : UIViewController
@property (nonatomic, strong) EuvCanvasView *canvasView;
@end

@implementation EuvViewController

- (void)loadView {
    CGRect initialFrame = UIScreen.mainScreen.bounds;
    self.canvasView = [[EuvCanvasView alloc] initWithFrame:initialFrame];
    self.canvasView.autoresizingMask = UIViewAutoresizingFlexibleWidth | UIViewAutoresizingFlexibleHeight;
    self.view = self.canvasView;
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
    self.window.backgroundColor = [UIColor whiteColor];
    self.window.rootViewController = [[EuvViewController alloc] init];
    [self.window makeKeyAndVisible];
    [self syncWindowGeometry:windowScene];
}

- (void)sceneDidBecomeActive:(UIScene *)scene {
    if ([scene isKindOfClass:[UIWindowScene class]]) {
        [self syncWindowGeometry:(UIWindowScene *)scene];
    }
}

- (void)sceneWillResignActive:(UIScene *)scene {
    (void)scene;
    if (self.window && [self.window.rootViewController isKindOfClass:[EuvViewController class]]) {
        EuvViewController *vc = (EuvViewController *)self.window.rootViewController;
        if (vc.canvasView && vc.canvasView.handle) {
            euv_ios_touch_cancel(vc.canvasView.handle, 0);
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
    <key>CFBundleDisplayName</key><string>Euv Showcase</string>
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
    <key>CFBundleName</key><string>EuvShowcase</string>
    <key>CFBundlePackageType</key><string>APPL</string>
    <key>CFBundleShortVersionString</key><string>0.38.0</string>
    <key>CFBundleVersion</key><string>1</string>
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
